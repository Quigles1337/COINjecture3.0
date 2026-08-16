[CmdletBinding()]
param()

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

$repoRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot '..\..\..')).Path
$specRoot = Join-Path $repoRoot 'spec'
$artifact = Join-Path $specRoot 'vectors\p005-draft.json'
$requiredHeader = 'NORMATIVE STATUS: draft — pending human ratification; formal-verification ownership reserved per LEDGER D16.'
$requiredFixtureStatus = 'NON-NORMATIVE TEST FIXTURE — ABSTRACT INTERFACE, NOT CANONICAL BYTES'
$requiredCases = @(
    'v1-v8-baseline-pass',
    'v1-size-at-limit-pass',
    'v1-size-over-limit-reject',
    'v1-unknown-field-reject',
    'v2-signature-mismatch-reject',
    'v3-address-binding-mismatch-reject',
    'v4-nonce-equal-pass',
    'v4-nonce-one-below-reject',
    'v4-nonce-one-above-reject',
    'v5-fee-at-minimum-pass',
    'v5-fee-below-minimum-reject',
    'v6-exact-balance-pass',
    'v6-insufficient-by-one-reject',
    'v6-u64-max-plus-one-reject',
    'v6-u64-max-boundary-pass',
    'v7-destination-32-bytes-pass',
    'v7-destination-31-bytes-reject',
    'v8-network-match-pass',
    'v8-network-mismatch-reject',
    'v9-self-send-pass-and-fee-applies',
    'block-atomicity-late-transaction-failure',
    'block-atomicity-reward-credit-overflow',
    'reward-threshold-quality-floor',
    'reward-at-quality-cap',
    'reward-above-quality-cap',
    'reward-r-max-one-degeneration',
    'conservation-uses-realized-reward'
)

if (-not (Test-Path -LiteralPath $artifact -PathType Leaf)) {
    throw "Missing committed P-005 vector artifact: $artifact"
}

$toolchain = (Get-Content -LiteralPath (Join-Path $specRoot 'lean-toolchain') -Raw).Trim()
$expectedVersion = ($toolchain -split ':')[-1].TrimStart('v')

$leanFiles = @(Get-ChildItem -LiteralPath (Join-Path $specRoot 'Spec') -Filter '*.lean' -File)
if ($leanFiles.Count -eq 0) {
    throw 'No Spec/*.lean files were found.'
}

foreach ($file in $leanFiles) {
    $source = Get-Content -LiteralPath $file.FullName -Raw
    if (-not $source.Contains($requiredHeader)) {
        throw "Required draft/ownership header missing from $($file.FullName)."
    }
    if ($source -match '\b(sorry|admit|axiom)\b') {
        throw "Forbidden proof placeholder found in $($file.FullName)."
    }
}

$tempArtifact = Join-Path ([IO.Path]::GetTempPath()) "cj3-p005-vectors-$([Guid]::NewGuid().ToString('N')).json"
try {
    Push-Location $specRoot
    try {
        # Resolve Lean while the pinned `spec/lean-toolchain` file is in scope. This
        # avoids accidentally accepting a machine-wide default toolchain in CI.
        $actualLean = (& lean --version | Out-String).Trim()
        if ($LASTEXITCODE -ne 0 -or $actualLean -notmatch [regex]::Escape($expectedVersion)) {
            throw "Lean toolchain mismatch. Expected $toolchain; observed '$actualLean'."
        }

        & lake build
        if ($LASTEXITCODE -ne 0) {
            throw 'Pinned P-005 Lake build failed.'
        }

        & lake exe vectors -- $tempArtifact
        if ($LASTEXITCODE -ne 0) {
            throw 'P-005 vector exporter failed.'
        }
    }
    finally {
        Pop-Location
    }

    $committedHash = (Get-FileHash -Algorithm SHA256 -LiteralPath $artifact).Hash
    $generatedHash = (Get-FileHash -Algorithm SHA256 -LiteralPath $tempArtifact).Hash
    if ($committedHash -ne $generatedHash) {
        throw "Generated vector drift: committed=$committedHash generated=$generatedHash"
    }

    $vectors = @(Get-Content -LiteralPath $tempArtifact -Raw | ConvertFrom-Json)
    $names = @($vectors | ForEach-Object { $_.name })
    foreach ($requiredCase in $requiredCases) {
        if ($requiredCase -notin $names) {
            throw "Required §14/Model 4 vector case is missing: $requiredCase"
        }
    }

    foreach ($vector in $vectors) {
        if ($null -eq $vector.input_bytes -or $vector.input_bytes -is [string]) {
            throw "Vector '$($vector.name)' exposes concrete input bytes instead of an abstract interface."
        }
        if ($vector.input_bytes.normative -ne $false) {
            throw "Vector '$($vector.name)' is not explicitly non-normative."
        }
        if ($vector.input_bytes.status -ne $requiredFixtureStatus) {
            throw "Vector '$($vector.name)' has an invalid fixture-status label."
        }
        if ([string]::IsNullOrWhiteSpace([string]$vector.input_bytes.expression) -or
            [string]::IsNullOrWhiteSpace([string]$vector.input_bytes.interface)) {
            throw "Vector '$($vector.name)' lacks its symbolic expression or interface reference."
        }
    }

    Write-Output "P005_DRAFT_SPEC_BUILD=PASS LEAN=$expectedVersion VECTORS=$($vectors.Count) SHA256=$committedHash"
    Write-Output 'P101_RUST_CONFORMANCE=NOT_YET_ADMITTED'
}
finally {
    if (Test-Path -LiteralPath $tempArtifact -PathType Leaf) {
        Remove-Item -LiteralPath $tempArtifact -Force
    }
}
