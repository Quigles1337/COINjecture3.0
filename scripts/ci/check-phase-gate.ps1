[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)]
    [ValidateSet('conservation-invariant', 'lean-conformance', 'codec-fuzz-smoke', 'genesis-spend-test', 'full-fuzz-corpus')]
    [string] $Gate
)

$ErrorActionPreference = 'Stop'
$owners = @{
    'conservation-invariant' = 'P-101 / Phase 1 kernel'
    'lean-conformance' = 'P-101 after P-005 vectors'
    'codec-fuzz-smoke' = 'P-007 canonical codecs'
    'genesis-spend-test' = 'Phase 1 genesis packet'
    # The nightly long fuzz run needs a nightly toolchain. The P-007 resume ruling
    # (clause 4.5) allows one to the D6 codec-fuzz-smoke job alone, so this gate has no
    # handler until Al rules otherwise.
    'full-fuzz-corpus' = "Al's ruling on a nightly-toolchain job outside D6 (P-007 resume ruling, clause 4.5)"
}

$handler = Join-Path $PSScriptRoot "active\$Gate.ps1"
if (-not (Test-Path -LiteralPath $handler -PathType Leaf)) {
    Write-Output "GATE=$Gate STATUS=NOT_YET_ADMITTED OWNER=$($owners[$Gate])"
    Write-Output 'This is an explicit phase deferral, not a claim that the named test ran.'
    exit 0
}

Write-Output "GATE=$Gate STATUS=ACTIVE HANDLER=$handler"
& pwsh -NoProfile -File $handler
if ($LASTEXITCODE -ne 0) {
    throw "Active phase gate failed: $Gate"
}
