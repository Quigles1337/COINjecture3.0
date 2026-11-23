[CmdletBinding()]
param()

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

$repoRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot '..\..')).Path
$sourceRoot = Join-Path $PSScriptRoot '..\..\crates'
$sourceFiles = @(Get-ChildItem -LiteralPath $sourceRoot -Recurse -File -Filter '*.rs')

if ($sourceFiles.Count -eq 0) {
    throw 'No Rust source entry points were found.'
}

$bannedPattern = '\bsolve_time\b|\bwork_score\b|\bself_reported\b|\breported_[A-Za-z0-9_]*\b'
$networkPattern = '\bmain(?:net|[-_ ]?network)\b'
$floatPattern = '\bf32\b|\bf64\b'
$failures = [System.Collections.Generic.List[string]]::new()

foreach ($file in $sourceFiles) {
    $content = Get-Content -Raw -LiteralPath $file.FullName
    if ($content -match $bannedPattern) {
        $failures.Add("banned consensus identifier: $($file.FullName)")
    }
    if ($content -match $networkPattern) {
        $failures.Add("forbidden production-network surface: $($file.FullName)")
    }
    if ($content -match $floatPattern) {
        $failures.Add("floating-point type forbidden in cj3-* crate: $($file.FullName)")
    }
}

$crateDirectories = @(Get-ChildItem -LiteralPath $sourceRoot -Directory -Filter 'cj3-*')
foreach ($crate in $crateDirectories) {
    $entryPoints = @(
        Get-ChildItem -LiteralPath (Join-Path $crate.FullName 'src') -File -ErrorAction Stop |
            Where-Object { $_.Name -in @('lib.rs', 'main.rs') }
    )
    if ($entryPoints.Count -eq 0) {
        $failures.Add("missing crate entry point: $($crate.FullName)")
        continue
    }
    foreach ($entryPoint in $entryPoints) {
        $content = Get-Content -Raw -LiteralPath $entryPoint.FullName
        if ($content -notmatch '#!\[forbid\(unsafe_code\)\]') {
            $failures.Add("missing forbid(unsafe_code): $($entryPoint.FullName)")
        }
    }
}

# ---------------------------------------------------------------------------------------
# Domain tags (Gate G0-F ruling, clause 4.5d).
#
# No Rust source outside crates/cj3-types names a domain tag constant. A tagged preimage
# is built in one place, cj3_types::hash, and every other crate calls it (Protocol Spec
# §1.2, §2).
#
# The rule reads the whole text of a file, comments included: a name in a comment is
# still a name, and reading everything is the stricter reading. It reaches every Rust
# source that git tracks or would track, in every workspace of the repository. The tests
# of cj3-types are inside crates/cj3-types, and the vector scripts are Python, so the
# ruling's two exemptions hold by where the rule looks.
#
# It matches the constants' names. It does not match the tag bytes typed out again.
# ---------------------------------------------------------------------------------------

$domainTagHome = 'crates/cj3-types/'
$domainTagSource = 'crates/cj3-types/src/domain.rs'
# The seven tags of Protocol Spec §1.2.
$ruledDomainTags = @('D_ADDR', 'D_TX', 'D_HDR', 'D_INST', 'D_SOL', 'D_BEACON', 'D_GENESIS')

function Find-DomainTagUse {
    # Returns the paths, among $RelativePaths under $Root, that are outside the home of
    # the domain tags and name one of $TagNames as a whole word, in the case given.
    param(
        [Parameter(Mandatory = $true)] [string] $Root,
        [Parameter(Mandatory = $true)] [string[]] $RelativePaths,
        [Parameter(Mandatory = $true)] [string[]] $TagNames,
        [Parameter(Mandatory = $true)] [string] $HomePrefix
    )

    $pattern = '\b(?:' + (($TagNames | ForEach-Object { [regex]::Escape($_) }) -join '|') + ')\b'
    $found = [System.Collections.Generic.List[string]]::new()
    foreach ($relativePath in $RelativePaths) {
        $normalized = $relativePath.Replace('\', '/')
        if ($normalized.StartsWith($HomePrefix, [StringComparison]::Ordinal)) {
            continue
        }
        $fullPath = Join-Path $Root $normalized
        if (-not (Test-Path -LiteralPath $fullPath -PathType Leaf)) {
            continue
        }
        $text = Get-Content -Raw -LiteralPath $fullPath
        if ($null -ne $text -and $text -cmatch $pattern) {
            $found.Add("$normalized names $($Matches[0])")
        }
    }
    $found.ToArray()
}

# The names are the seven ruled ones, and with them every constant that the domain
# module declares as a tag, so a tag added there later is covered without an edit here.
$domainText = Get-Content -Raw -LiteralPath (Join-Path $repoRoot $domainTagSource)
$declaredDomainTags = @(
    [regex]::Matches($domainText, '(?m)^\s*pub const ([A-Z][A-Z0-9_]*): \[u8; TAG_LEN\]') |
        ForEach-Object { $_.Groups[1].Value }
)
foreach ($ruled in $ruledDomainTags) {
    if ($declaredDomainTags -cnotcontains $ruled) {
        $failures.Add("domain tag $ruled is not declared in $domainTagSource")
    }
}
$domainTagNames = @(($ruledDomainTags + $declaredDomainTags) | Sort-Object -Unique)

$trackedText = (& git -C $repoRoot -c core.quotepath=off ls-files --cached --others --exclude-standard -- '*.rs' | Out-String)
if ($LASTEXITCODE -ne 0) {
    throw 'git ls-files failed. The domain-tag rule reads the list of Rust sources from git.'
}
$rustSources = @($trackedText -split "`r?`n" | Where-Object { $_ -ne '' })
$outsideHome = @($rustSources | Where-Object { -not $_.StartsWith($domainTagHome, [StringComparison]::Ordinal) })
if ($rustSources -cnotcontains $domainTagSource -or $outsideHome.Count -eq 0) {
    throw 'The domain-tag rule found no Rust sources to read. That is a failure of the gate, not a pass.'
}

$domainTagUses = @(Find-DomainTagUse -Root $repoRoot -RelativePaths $rustSources -TagNames $domainTagNames -HomePrefix $domainTagHome)
foreach ($use in $domainTagUses) {
    $failures.Add("domain tag constant named outside crates/cj3-types: $use")
}

# The proof that the rule can fail. It plants sources in a temporary tree and runs the
# same function over them. A rule that flags nothing there has stopped working, and that
# fails the gate.
$canaryRoot = Join-Path ([IO.Path]::GetTempPath()) "cj3-source-policy-$([Guid]::NewGuid().ToString('N'))"
$canaryFiles = [ordered]@{
    # Must be flagged.
    'crates/cj3-consensus/src/lib.rs'       = 'use cj3_types::domain::D_TX;'
    'fuzz/fuzz_targets/planted.rs'          = '// builds H(D_HDR || header) itself'
    'crates/cj3-types-extra/src/lib.rs'     = 'const COPY: [u8; 16] = cj3_types::domain::D_SOL;'
    'bench/p004-admission/src/planted.rs'   = 'let tag = D_GENESIS;'
    # Must not be flagged.
    'crates/cj3-types/tests/vectors.rs'     = 'assert_eq!(domain::D_ADDR, RULED);'
    'crates/cj3-types/src/hash.rs'          = 'use crate::domain::{D_ADDR, D_BEACON};'
    'crates/cj3-net/src/lib.rs'             = 'let d_tx = 1; const MAX_D_TX_COUNT: u8 = 2; const D_TXN: u8 = 3;'
}
$canaryExpected = @(
    'bench/p004-admission/src/planted.rs names D_GENESIS'
    'crates/cj3-consensus/src/lib.rs names D_TX'
    'crates/cj3-types-extra/src/lib.rs names D_SOL'
    'fuzz/fuzz_targets/planted.rs names D_HDR'
)
try {
    foreach ($relativePath in $canaryFiles.Keys) {
        $planted = Join-Path $canaryRoot $relativePath
        New-Item -ItemType Directory -Path (Split-Path -Parent $planted) -Force | Out-Null
        Set-Content -LiteralPath $planted -Value $canaryFiles[$relativePath] -NoNewline
    }
    $canaryFound = @(
        Find-DomainTagUse -Root $canaryRoot -RelativePaths @($canaryFiles.Keys) -TagNames $domainTagNames -HomePrefix $domainTagHome |
            Sort-Object
    )
}
finally {
    if (Test-Path -LiteralPath $canaryRoot) {
        Remove-Item -LiteralPath $canaryRoot -Recurse -Force
    }
}
if (($canaryFound -join '|') -cne ($canaryExpected -join '|')) {
    throw "The domain-tag rule did not behave on its planted sources. Expected: $($canaryExpected -join '; '). Found: $($canaryFound -join '; ')."
}

if ($failures.Count -ne 0) {
    $failures | ForEach-Object { Write-Error $_ }
    throw "Source policy failed with $($failures.Count) violation(s)."
}

Write-Output "DOMAIN_TAG_POLICY=PASS tags=$($domainTagNames.Count) read_outside_cj3_types=$($outsideHome.Count) planted_flagged=$($canaryExpected.Count) planted_passed=$($canaryFiles.Count - $canaryExpected.Count)"
Write-Output "SOURCE_POLICY=PASS files=$($sourceFiles.Count) crates=$($crateDirectories.Count)"
