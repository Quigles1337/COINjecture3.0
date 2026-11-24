[CmdletBinding()]
param()

# Gate G0-F ruling, clause 4.5c: every build of cj3-types in this repository uses
# curve25519-dalek's serial back end, and a build that does not fails to compile.
#
# Ruling "P-101 TO ITS FRAME", clause 4.3a: every build of cj3-types in this repository
# sets curve25519_dalek_bits to exactly 64, and a build that does not fails to compile.
#
# This check proves both halves of both rules on every D6 run. It builds cj3-types as the
# repository configures it and requires success. Then it builds it with each setting
# absent, wrong and doubled, and requires each of those builds to fail with the crate's
# own message for that setting. A build that fails for another reason does not pass.
#
# A doubled case gives one setting two values at once. For the back end, on x86_64, the
# library itself then fails to compile, before cj3-types is reached; on a target with no
# vector back end the crate's own exact comparison refuses it. For the word size, the
# library treats the pair as if no word size were set: it either fails to compile, with
# both bodies of code switched on, or compiles, and then the crate's own exact
# comparison refuses it. Either is a build that does not compile. For those two cases the
# check accepts those two reasons and no other, and prints which one it saw.
#
# A RUSTFLAGS environment variable replaces the flags of .cargo/config.toml, which is
# how the other settings are produced here. The builds go to a temporary directory, so
# the workspace's own build output is not touched.

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

$serial = '--cfg curve25519_dalek_backend="serial"'
$bits64 = '--cfg curve25519_dalek_bits="64"'
$backendRefusal = 'cj3-types must be built with exactly `--cfg curve25519_dalek_backend="serial"`'
$bitsRefusal = 'cj3-types must be built with exactly `--cfg curve25519_dalek_bits="64"`'
$libraryFailure = 'could not compile `curve25519-dalek`'

$repoRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot '..\..')).Path

# The committed configuration must carry both flags, for compiling and for documenting.
$config = Get-Content -Raw -LiteralPath (Join-Path $repoRoot '.cargo/config.toml')
$configured = '\["--cfg", "curve25519_dalek_backend=\\"serial\\"", "--cfg", "curve25519_dalek_bits=\\"64\\""\]'
foreach ($key in @('rustflags', 'rustdocflags')) {
    $pattern = '(?m)^' + $key + ' = ' + $configured + '\s*$'
    if ($config -notmatch $pattern) {
        throw ".cargo/config.toml does not set $key to the serial back end at 64 bits."
    }
}

# With one of these set, the first build below would not test the configuration file.
foreach ($name in @('RUSTFLAGS', 'CARGO_ENCODED_RUSTFLAGS', 'CARGO_BUILD_RUSTFLAGS')) {
    if (Test-Path -LiteralPath "Env:$name") {
        throw "$name is set in this environment. This check must start without it."
    }
}

# Each case: a name; the RUSTFLAGS value ($null leaves the variable unset, so the
# configuration file applies); whether cj3-types must build; for a case that must not
# build, the refusals the crate must print, each by its own message; and whether the
# library's own failure to compile is also an accepted reason.
$cases = @(
    @{ Name = 'configuration-file'; Flags = $null; Builds = $true }
    @{ Name = 'rustflags-serial-64'; Flags = "$serial $bits64"; Builds = $true }
    @{ Name = 'rustflags-without-either-setting'; Flags = '--cfg cj3_backend_probe'; Builds = $false; Refusals = @($backendRefusal, $bitsRefusal); LibraryMayFail = $false }
    @{ Name = 'backend-absent'; Flags = $bits64; Builds = $false; Refusals = @($backendRefusal); LibraryMayFail = $false }
    @{ Name = 'backend-simd'; Flags = "--cfg curve25519_dalek_backend=`"simd`" $bits64"; Builds = $false; Refusals = @($backendRefusal); LibraryMayFail = $false }
    @{ Name = 'backend-fiat'; Flags = "--cfg curve25519_dalek_backend=`"fiat`" $bits64"; Builds = $false; Refusals = @($backendRefusal); LibraryMayFail = $false }
    @{ Name = 'backend-serial-and-simd'; Flags = "$serial --cfg curve25519_dalek_backend=`"simd`" $bits64"; Builds = $false; Refusals = @($backendRefusal); LibraryMayFail = $true }
    @{ Name = 'bits-absent'; Flags = $serial; Builds = $false; Refusals = @($bitsRefusal); LibraryMayFail = $false }
    @{ Name = 'bits-32'; Flags = "$serial --cfg curve25519_dalek_bits=`"32`""; Builds = $false; Refusals = @($bitsRefusal); LibraryMayFail = $false }
    @{ Name = 'bits-32-and-64'; Flags = "$serial --cfg curve25519_dalek_bits=`"32`" $bits64"; Builds = $false; Refusals = @($bitsRefusal); LibraryMayFail = $true }
)

$targetDirectory = Join-Path ([IO.Path]::GetTempPath()) "cj3-backend-check-$([Guid]::NewGuid().ToString('N'))"
$built = 0
$refused = 0

Push-Location $repoRoot
try {
    foreach ($case in $cases) {
        if ($null -eq $case.Flags) {
            Remove-Item -LiteralPath Env:RUSTFLAGS -ErrorAction SilentlyContinue
        }
        else {
            $env:RUSTFLAGS = $case.Flags
        }

        # Both streams are kept: cargo reports a compile error on its error stream.
        $previousPreference = $ErrorActionPreference
        $ErrorActionPreference = 'Continue'
        try {
            $output = (& cargo check --package cj3-types --locked --target-dir $targetDirectory 2>&1 | Out-String)
            $exitCode = $LASTEXITCODE
        }
        finally {
            $ErrorActionPreference = $previousPreference
            Remove-Item -LiteralPath Env:RUSTFLAGS -ErrorAction SilentlyContinue
        }

        $sawBackendRefusal = $output.Contains($backendRefusal)
        $sawBitsRefusal = $output.Contains($bitsRefusal)

        if ($case.Builds) {
            if ($exitCode -ne 0) {
                Write-Output $output
                throw "cj3-types did not build in case $($case.Name)."
            }
            if ($sawBackendRefusal -or $sawBitsRefusal) {
                Write-Output $output
                throw "cj3-types printed a refusal in case $($case.Name), which must build."
            }
            $built++
            Write-Output "ARITHMETIC_BACKEND_CASE=$($case.Name) EXPECTED=builds RESULT=PASS"
            continue
        }

        if ($exitCode -eq 0) {
            throw "cj3-types built in case $($case.Name). It must refuse to compile."
        }

        # The crate must print exactly the refusals this case names: every one of them,
        # and not the other.
        $expectsBackend = $case.Refusals -contains $backendRefusal
        $expectsBits = $case.Refusals -contains $bitsRefusal
        if (($sawBackendRefusal -eq $expectsBackend) -and ($sawBitsRefusal -eq $expectsBits)) {
            $names = @()
            if ($expectsBackend) { $names += 'back-end' }
            if ($expectsBits) { $names += 'word-size' }
            $reason = "cj3-types-refuses-the-$($names -join '-and-the-')"
        }
        elseif ($case.LibraryMayFail -and
            (-not $sawBackendRefusal) -and (-not $sawBitsRefusal) -and
            $output.Contains($libraryFailure)) {
            $reason = 'the-library-does-not-compile'
        }
        else {
            Write-Output $output
            throw "The build failed in case $($case.Name), but not for an accepted reason."
        }
        $refused++
        Write-Output "ARITHMETIC_BACKEND_CASE=$($case.Name) EXPECTED=does-not-compile REASON=$reason RESULT=PASS"
    }
}
finally {
    Pop-Location
    if (Test-Path -LiteralPath $targetDirectory) {
        Remove-Item -LiteralPath $targetDirectory -Recurse -Force
    }
}

Write-Output "ARITHMETIC_BACKEND=PASS backend=serial bits=64 builds=$built refusals=$refused"

# The last build above is one that must fail, so it leaves a failing exit code behind.
# A workflow step ends with the exit code of the last program that ran, so the code is
# cleared here: every case has been judged, and reaching this line is the pass.
$global:LASTEXITCODE = 0
