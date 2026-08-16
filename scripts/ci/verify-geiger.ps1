[CmdletBinding()]
param()

$ErrorActionPreference = 'Stop'

& (Join-Path $PSScriptRoot 'check-source-policy.ps1')

$metadataText = (& cargo metadata --locked --no-deps --format-version 1 | Out-String)
if ($LASTEXITCODE -ne 0) {
    throw 'cargo metadata failed before cargo-geiger.'
}
$metadata = $metadataText | ConvertFrom-Json
$workspaceIds = [System.Collections.Generic.HashSet[string]]::new(
    [string[]] $metadata.workspace_members
)
$packages = @(
    $metadata.packages |
        Where-Object { $workspaceIds.Contains([string] $_.id) -and $_.name -like 'cj3-*' } |
        Sort-Object name
)

if ($packages.Count -eq 0) {
    throw 'No cj3-* workspace packages were found for cargo-geiger.'
}

foreach ($package in $packages) {
    $manifestPath = (Resolve-Path -LiteralPath $package.manifest_path).Path
    $reportText = (& cargo geiger --manifest-path $manifestPath --all-targets --all-features --locked --output-format Json | Out-String)
    if ($LASTEXITCODE -ne 0) {
        throw "cargo-geiger failed for $($package.name)."
    }
    $report = $reportText | ConvertFrom-Json
    $metrics = @($report.packages | Where-Object { $_.package.id.name -eq $package.name })
    if ($metrics.Count -ne 1) {
        throw "Expected exactly one cargo-geiger metric for $($package.name); found $($metrics.Count)."
    }

    $unsafeCount = 0
    foreach ($scope in @($metrics[0].unsafety.used, $metrics[0].unsafety.unused)) {
        foreach ($category in @('functions', 'exprs', 'item_impls', 'item_traits', 'methods')) {
            $unsafeCount += [int] $scope.$category.unsafe_
        }
    }

    if (-not $metrics[0].unsafety.forbids_unsafe) {
        throw "$($package.name) does not forbid unsafe code at every compiled entry point."
    }
    if ($unsafeCount -ne 0) {
        throw "$($package.name) contains $unsafeCount unsafe item(s)."
    }

    Write-Output "GEIGER_PACKAGE=$($package.name) UNSAFE_COUNT=0 FORBIDS_UNSAFE=true"
}

Write-Output "GEIGER_POLICY=PASS packages=$($packages.Count) unsafe_count=0"
