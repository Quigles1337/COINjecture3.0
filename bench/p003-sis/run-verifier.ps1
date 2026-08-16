[CmdletBinding()]
param(
    [ValidateRange(1, 1000000)]
    [int] $Samples = 200
)

$ErrorActionPreference = 'Stop'
$repoRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot '..\..')).Path
Push-Location $repoRoot
try {
    & cargo build --release --locked --example p003_check_bench -p cj3-classes
    if ($LASTEXITCODE -ne 0) {
        throw 'Release checker benchmark build failed.'
    }
    $runner = (Resolve-Path -LiteralPath 'target\release\examples\p003_check_bench.exe').Path
    $tuples = @(
        @(8, 112, 67, 112),
        @(12, 192, 149, 192),
        @(16, 288, 257, 288),
        @(24, 480, 577, 480),
        @(32, 704, 1031, 704),
        @(48, 1152, 2309, 1152),
        @(64, 1664, 4099, 1664),
        @(80, 2080, 6421, 2080),
        @(96, 2688, 9221, 2688),
        @(128, 3840, 16411, 3840)
    )
    foreach ($tuple in $tuples) {
        & $runner $tuple[0] $tuple[1] $tuple[2] $tuple[3] $Samples
        if ($LASTEXITCODE -ne 0) {
            throw "Checker benchmark failed for tuple ($($tuple -join ', '))."
        }
    }
}
finally {
    Pop-Location
}
