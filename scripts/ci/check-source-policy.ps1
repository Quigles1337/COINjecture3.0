[CmdletBinding()]
param()

$ErrorActionPreference = 'Stop'
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

if ($failures.Count -ne 0) {
    $failures | ForEach-Object { Write-Error $_ }
    throw "Source policy failed with $($failures.Count) violation(s)."
}

Write-Output "SOURCE_POLICY=PASS files=$($sourceFiles.Count) crates=$($crateDirectories.Count)"
