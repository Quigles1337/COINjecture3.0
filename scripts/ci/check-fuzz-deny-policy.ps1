[CmdletBinding()]
param()

# Gate G0-F ruling, clause 4.5b: `cargo deny` runs over the fuzz workspace with a
# configuration of its own, which is the root policy plus the one licence term
# libfuzzer-sys needs, for that crate alone.
#
# This check holds fuzz/deny.toml to that. Comment lines and blank lines aside, it must
# be deny.toml line for line with exactly one line added: the exception below, inside
# the [licenses] table. A second exception, a changed rule, or a root policy that
# itself names NCSA fails the check. It runs no dependency check itself: the D6 job
# runs `cargo deny` with the file once this passes.

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

$exception = 'exceptions = [{ allow = ["NCSA"], crate = "libfuzzer-sys@0.4.13" }]'

function Get-PolicyLine {
    param([Parameter(Mandatory = $true)] [string] $Path)

    @(
        Get-Content -LiteralPath $Path |
            ForEach-Object { $_.TrimEnd() } |
            Where-Object { $_ -ne '' -and -not $_.StartsWith('#') }
    )
}

$repoRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot '..\..')).Path
$root = Get-PolicyLine -Path (Join-Path $repoRoot 'deny.toml')
$fuzz = Get-PolicyLine -Path (Join-Path $repoRoot 'fuzz/deny.toml')

if ($root.Count -eq 0) {
    throw 'deny.toml has no policy lines.'
}
if (@($root | Where-Object { $_ -match 'NCSA|libfuzzer' }).Count -ne 0) {
    throw 'The root deny.toml names NCSA or libfuzzer-sys. The root policy must not carry the fuzz exception.'
}

$positions = @(0..($fuzz.Count - 1) | Where-Object { $fuzz[$_] -ceq $exception })
if ($positions.Count -ne 1) {
    throw "fuzz/deny.toml must hold the exception line exactly once. Found $($positions.Count) time(s). Expected: $exception"
}
$position = $positions[0]

$table = ''
for ($index = $position; $index -ge 0; $index--) {
    if ($fuzz[$index] -match '^\[([^\]]+)\]$') {
        $table = $Matches[1]
        break
    }
}
if ($table -cne 'licenses') {
    throw "The exception line must be inside the [licenses] table. It is inside [$table]."
}

$rest = [System.Collections.Generic.List[string]]::new()
for ($index = 0; $index -lt $fuzz.Count; $index++) {
    if ($index -ne $position) {
        $rest.Add($fuzz[$index])
    }
}

if ($rest.Count -ne $root.Count) {
    throw "fuzz/deny.toml differs from deny.toml by more than the one exception: $($rest.Count) other policy lines against $($root.Count)."
}
for ($index = 0; $index -lt $root.Count; $index++) {
    if ($rest[$index] -cne $root[$index]) {
        throw "fuzz/deny.toml differs from deny.toml at policy line $($index + 1). Root: '$($root[$index])'. Fuzz: '$($rest[$index])'."
    }
}

Write-Output "FUZZ_DENY_POLICY=PASS root_lines=$($root.Count) fuzz_lines=$($fuzz.Count) added=1 exception=libfuzzer-sys@0.4.13:NCSA"
