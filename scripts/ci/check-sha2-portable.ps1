[CmdletBinding()]
param()

# Ruling "P-101 TO ITS FRAME", clauses 2.3 and 4.3b: cj3-types enables the force-soft
# feature of sha2, so that its portable code runs on every processor, and D6 fails if the
# resolved build does not have it.
#
# Without the feature, on x86 processors, sha2 chooses between its portable code and
# processor extensions when it runs. Clause 4.5c of the gate ruling pinned the curve
# arithmetic so that every node runs the same code whatever its processor, and its
# reason covers the hash as well.
#
# This check reads three things and judges each the same way:
#   1. what cj3-types asks for in its manifest;
#   2. what cargo resolved, for the workspace and for the fuzz workspace;
#   3. what a build of the workspace's tests actually compiled, unit by unit. A resolved
#      feature list is a union over a workspace, and cargo can compile one crate more
#      than once with different features. The compiled units are the build.
#
# The judgment: at least one sha2 must be found, and every sha2 found must have
# `force-soft` and must not have `force-soft-compact`. sha2 0.10.9 tests the second
# feature first (src/sha256.rs, src/sha512.rs), so with both enabled it would run another
# body of portable code and `force-soft` would have no effect.
#
# The check proves its judgment on planted records on every run, so that a pass is not a
# silent miss.

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

$crate = 'sha2'
$required = 'force-soft'
$overriding = 'force-soft-compact'

$repoRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot '..\..')).Path

function Get-Sha2Problems {
    <#
    .SYNOPSIS
    Judges a list of feature lists, one per sha2 found. Returns the problems; an empty
    result is a pass.
    #>
    param(
        [Parameter(Mandatory = $true)]
        [AllowEmptyCollection()]
        [object[]] $FeatureLists
    )

    $problems = @()
    if ($FeatureLists.Count -eq 0) {
        $problems += "no $crate was found, so nothing was judged"
    }
    foreach ($features in $FeatureLists) {
        $list = @($features)
        if ($list -cnotcontains $required) {
            $problems += "a $crate lacks the feature $required (it has: $($list -join ', '))"
        }
        if ($list -ccontains $overriding) {
            $problems += "a $crate has the feature $overriding, which overrides $required"
        }
    }
    return , $problems
}

function Assert-Sha2 {
    param(
        [Parameter(Mandatory = $true)] [string] $What,
        [Parameter(Mandatory = $true)] [AllowEmptyCollection()] [object[]] $FeatureLists
    )
    $problems = Get-Sha2Problems -FeatureLists $FeatureLists
    if ($problems.Count -ne 0) {
        throw "${What}: $($problems -join '; ')."
    }
}

function Get-CargoMetadata {
    param([string] $ManifestPath)
    $arguments = @('metadata', '--format-version', '1', '--locked')
    if ($ManifestPath) {
        $arguments += @('--manifest-path', $ManifestPath)
    }
    $text = (& cargo @arguments | Out-String)
    if ($LASTEXITCODE -ne 0) {
        throw "cargo metadata failed for $(if ($ManifestPath) { $ManifestPath } else { 'the workspace' })."
    }
    return ($text | ConvertFrom-Json -AsHashtable)
}

# --- The judgment, proved on planted records -------------------------------------------
# Each planted record is a list of feature lists, as if that many sha2 crates were found.
$planted = @(
    @{ Name = 'one sha2 with the feature'; Lists = @(, @('force-soft')); Passes = $true }
    @{ Name = 'one sha2 with the feature and an unrelated one'; Lists = @(, @('force-soft', 'std')); Passes = $true }
    @{ Name = 'no sha2 at all'; Lists = @(); Passes = $false }
    @{ Name = 'one sha2 with no features'; Lists = @(, @()); Passes = $false }
    @{ Name = 'one sha2 with an unrelated feature only'; Lists = @(, @('std')); Passes = $false }
    @{ Name = 'one sha2 with the overriding feature only'; Lists = @(, @('force-soft-compact')); Passes = $false }
    @{ Name = 'one sha2 with both features'; Lists = @(, @('force-soft', 'force-soft-compact')); Passes = $false }
    @{ Name = 'two sha2, one of them without the feature'; Lists = @(@('force-soft'), @('std')); Passes = $false }
)
$accepted = 0
$rejected = 0
foreach ($record in $planted) {
    $problems = Get-Sha2Problems -FeatureLists $record.Lists
    $passed = ($problems.Count -eq 0)
    if ($passed -ne $record.Passes) {
        throw "The judgment is wrong on the planted record '$($record.Name)': passed=$passed, expected=$($record.Passes)."
    }
    if ($passed) { $accepted++ } else { $rejected++ }
}
Write-Output "SHA2_PORTABLE_PLANTED=PASS accepted=$accepted rejected=$rejected"

Push-Location $repoRoot
try {
    # --- 1. What cj3-types asks for ----------------------------------------------------
    $workspace = Get-CargoMetadata -ManifestPath $null
    $types = @($workspace.packages | Where-Object { $_.name -eq 'cj3-types' })
    if ($types.Count -ne 1) {
        throw "Expected exactly one cj3-types package in the workspace, found $($types.Count)."
    }
    $requests = @($types[0].dependencies | Where-Object { $_.name -eq $crate })
    if ($requests.Count -ne 1) {
        throw "Expected exactly one $crate dependency in cj3-types, found $($requests.Count)."
    }
    Assert-Sha2 -What "cj3-types' manifest" -FeatureLists @(, @($requests[0].features))
    Write-Output "SHA2_PORTABLE_REQUEST=PASS crate=cj3-types dependency=$crate features=$(@($requests[0].features) -join ',')"

    # --- 2. What cargo resolved, for each workspace ------------------------------------
    $sha2Ids = @()
    foreach ($entry in @(
            @{ Label = 'workspace'; Manifest = $null; Metadata = $workspace }
            @{ Label = 'fuzz'; Manifest = 'fuzz/Cargo.toml'; Metadata = $null }
        )) {
        $metadata = if ($null -ne $entry.Metadata) { $entry.Metadata } else { Get-CargoMetadata -ManifestPath $entry.Manifest }
        $packages = @($metadata.packages | Where-Object { $_.name -eq $crate })
        $ids = @($packages | ForEach-Object { $_.id })
        $nodes = @($metadata.resolve.nodes | Where-Object { $ids -ccontains $_.id })
        if ($nodes.Count -ne $packages.Count) {
            throw "The $($entry.Label) resolution does not list every $crate package it names."
        }
        $lists = @($nodes | ForEach-Object { , @($_.features) })
        Assert-Sha2 -What "The $($entry.Label) resolution" -FeatureLists $lists
        $versions = @($packages | ForEach-Object { $_.version }) -join ','
        $shown = @($nodes | ForEach-Object { @($_.features) -join ',' }) -join ' | '
        Write-Output "SHA2_PORTABLE_RESOLVED=PASS scope=$($entry.Label) $crate=$versions features=$shown"
        if ($entry.Label -eq 'workspace') {
            $sha2Ids = $ids
        }
    }

    # --- 3. What a build of the workspace's tests compiled ------------------------------
    # The same build as the next step of the D6 job, stopped before the tests run. cargo
    # writes one JSON record per compiled unit to its output; progress and compiler
    # messages for people go to the error stream, which is left alone.
    $records = @(& cargo test --workspace --all-targets --all-features --locked --no-run --message-format=json)
    if ($LASTEXITCODE -ne 0) {
        throw 'The workspace tests did not build.'
    }
    $units = @()
    foreach ($line in $records) {
        $text = [string] $line
        if (-not $text.StartsWith('{') -or -not $text.Contains('"compiler-artifact"')) {
            continue
        }
        $record = $text | ConvertFrom-Json -AsHashtable
        if ($record.reason -ne 'compiler-artifact') {
            continue
        }
        if ($sha2Ids -ccontains $record.package_id) {
            $units += , @($record.features)
        }
    }
    Assert-Sha2 -What 'The build of the workspace tests' -FeatureLists $units
    $shown = @($units | ForEach-Object { @($_) -join ',' }) -join ' | '
    Write-Output "SHA2_PORTABLE_COMPILED=PASS scope=workspace units=$($units.Count) features=$shown"
}
finally {
    Pop-Location
}

Write-Output "SHA2_PORTABLE=PASS crate=$crate feature=$required"
