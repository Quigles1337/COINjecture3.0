[CmdletBinding()]
param()

# Gate G0-F ruling, clause 4.5c: every build of cj3-types in this repository uses
# curve25519-dalek's serial back end, and a build that does not fails to compile.
#
# This check proves both halves on every D6 run. It builds cj3-types as the repository
# configures it and requires success. Then it builds it with the setting absent and
# wrong, and requires each of those builds to fail with the crate's own message. A build
# that fails for another reason does not pass.
#
# The last case gives the setting two values at once. On x86_64 the library itself then
# fails to compile, before cj3-types is reached; on a target with no vector back end the
# crate's own exact comparison refuses it. Either is a build that does not compile, and
# the check accepts those two reasons and no other, and prints which one it saw.
#
# A RUSTFLAGS environment variable replaces the flags of .cargo/config.toml, which is
# how the other settings are produced here. The builds go to a temporary directory, so
# the workspace's own build output is not touched.

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

$serial = '--cfg curve25519_dalek_backend="serial"'
$refusal = 'cj3-types must be built with exactly'
$libraryFailure = 'could not compile `curve25519-dalek`'

$repoRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot '..\..')).Path

# The committed configuration must carry the flag for compiling and for documenting.
$config = Get-Content -Raw -LiteralPath (Join-Path $repoRoot '.cargo/config.toml')
foreach ($key in @('rustflags', 'rustdocflags')) {
    $pattern = '(?m)^' + $key + ' = \["--cfg", "curve25519_dalek_backend=\\"serial\\""\]\s*$'
    if ($config -notmatch $pattern) {
        throw ".cargo/config.toml does not set $key to the serial back end."
    }
}

# With one of these set, the first build below would not test the configuration file.
foreach ($name in @('RUSTFLAGS', 'CARGO_ENCODED_RUSTFLAGS', 'CARGO_BUILD_RUSTFLAGS')) {
    if (Test-Path -LiteralPath "Env:$name") {
        throw "$name is set in this environment. This check must start without it."
    }
}

# Each case: a name, the RUSTFLAGS value ($null leaves the variable unset, so the
# configuration file applies), whether cj3-types must build, and, for a case that must
# not build, whether the library's own failure to compile is also an accepted reason.
$cases = @(
    @{ Name = 'configuration-file'; Flags = $null; Builds = $true; LibraryMayFail = $false }
    @{ Name = 'rustflags-serial'; Flags = $serial; Builds = $true; LibraryMayFail = $false }
    @{ Name = 'rustflags-without-the-setting'; Flags = '--cfg cj3_backend_probe'; Builds = $false; LibraryMayFail = $false }
    @{ Name = 'rustflags-simd'; Flags = '--cfg curve25519_dalek_backend="simd"'; Builds = $false; LibraryMayFail = $false }
    @{ Name = 'rustflags-fiat'; Flags = '--cfg curve25519_dalek_backend="fiat"'; Builds = $false; LibraryMayFail = $false }
    @{ Name = 'rustflags-serial-and-simd'; Flags = "$serial --cfg curve25519_dalek_backend=`"simd`""; Builds = $false; LibraryMayFail = $true }
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

        if ($case.Builds) {
            if ($exitCode -ne 0) {
                Write-Output $output
                throw "cj3-types did not build in case $($case.Name)."
            }
            if ($output.Contains($refusal)) {
                Write-Output $output
                throw "cj3-types printed its refusal in case $($case.Name), which must build."
            }
            $built++
            Write-Output "ARITHMETIC_BACKEND_CASE=$($case.Name) EXPECTED=builds RESULT=PASS"
        }
        else {
            if ($exitCode -eq 0) {
                throw "cj3-types built in case $($case.Name). It must refuse to compile."
            }
            if ($output.Contains($refusal)) {
                $reason = 'cj3-types-refuses'
            }
            elseif ($case.LibraryMayFail -and $output.Contains($libraryFailure)) {
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
}
finally {
    Pop-Location
    if (Test-Path -LiteralPath $targetDirectory) {
        Remove-Item -LiteralPath $targetDirectory -Recurse -Force
    }
}

Write-Output "ARITHMETIC_BACKEND=PASS backend=serial builds=$built refusals=$refused"

# The last build above is one that must fail, so it leaves a failing exit code behind.
# A workflow step ends with the exit code of the last program that ran, so the code is
# cleared here: every case has been judged, and reaching this line is the pass.
$global:LASTEXITCODE = 0
