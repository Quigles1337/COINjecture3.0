[CmdletBinding()]
param(
    [string]$LegacyRoot = 'C:\Users\LEET\COINjecture2.0-network',
    [ValidateRange(0, 1000)]
    [int]$Warmups = 2,
    [ValidateRange(1, 10000)]
    [int]$Samples = 9,
    [ValidateRange(1, 1000000)]
    [int]$CheckerRepetitions = 10000,
    [switch]$BootstrapLock
)

$ErrorActionPreference = 'Stop'

$expectedRevision = '58e0397fddd8e5ebd0d84fe00fbd022fae2b17ff'
$expectedHashes = @{
    'core\src\problem.rs' = 'EDB039CFBFBFE46AC39D4B0DEB0465CC779C04FE5B17C5C8C7DBB3509DCDBE27'
    'consensus\src\miner.rs' = '3AA12C026FF8CCEEFB00CC42B9D1EC8BE44CEB7B8D5366AA09EC04CACB014BC5'
    'consensus\src\problem_registry.rs' = '02D001745A17E0F48A8660BC64C515490B7ACC22845E6784872A7F199F9E7A4B'
    'Cargo.lock' = '9930A209663DD812D03DD654D5EA8F850152667DE455191B7C4645EB1CDB1BEA'
}

$scriptRoot = (Resolve-Path -LiteralPath $PSScriptRoot).Path
$repoRoot = (Resolve-Path -LiteralPath (Join-Path $scriptRoot '..\..')).Path
$legacyResolved = (Resolve-Path -LiteralPath $LegacyRoot).Path
$legacyRevision = (& git -C $legacyResolved rev-parse HEAD).Trim()
if ($LASTEXITCODE -ne 0 -or $legacyRevision -ne $expectedRevision) {
    throw "Legacy revision mismatch: expected $expectedRevision, observed $legacyRevision"
}

foreach ($entry in $expectedHashes.GetEnumerator()) {
    $sourcePath = Join-Path $legacyResolved $entry.Key
    $actual = (Get-FileHash -Algorithm SHA256 -LiteralPath $sourcePath).Hash
    if ($actual -ne $entry.Value) {
        throw "Legacy source hash mismatch for $($entry.Key): expected $($entry.Value), observed $actual"
    }
}

$legacyStatusBefore = @(& git -C $legacyResolved status --porcelain=v1 --untracked-files=all)
if ($LASTEXITCODE -ne 0) {
    throw "Unable to read legacy working-tree status (exit $LASTEXITCODE)"
}
if ($legacyStatusBefore.Count -ne 0) {
    throw 'Legacy checkout must be clean before read-only calibration.'
}

$outputRoot = Join-Path $scriptRoot 'evidence'
New-Item -ItemType Directory -Path $outputRoot -Force | Out-Null
$jsonlPath = Join-Path $outputRoot 'legacy-results.jsonl'
$markdownPath = Join-Path $outputRoot 'LEGACY-CALIBRATION.md'
$environmentPath = Join-Path $outputRoot 'ENVIRONMENT.md'
$committedLock = Join-Path $scriptRoot 'legacy-driver-Cargo.lock'

if (-not $BootstrapLock -and -not (Test-Path -LiteralPath $committedLock)) {
    throw 'The pinned legacy-driver-Cargo.lock is absent. Use -BootstrapLock once, inspect it, then commit it.'
}

$temporaryRoot = [IO.Path]::GetFullPath([IO.Path]::GetTempPath())
$temporaryPath = [IO.Path]::GetFullPath((Join-Path $temporaryRoot "cj3-p004-driver-$([guid]::NewGuid().ToString('N'))"))
if (-not $temporaryPath.StartsWith($temporaryRoot, [StringComparison]::OrdinalIgnoreCase)) {
    throw "Refusing temporary path outside the system temp root: $temporaryPath"
}

try {
    $temporarySource = Join-Path $temporaryPath 'src'
    New-Item -ItemType Directory -Path $temporarySource -Force | Out-Null

    $template = Get-Content -Raw -LiteralPath (Join-Path $scriptRoot 'legacy-driver\Cargo.toml.in')
    $harnessToml = (Join-Path $scriptRoot '').Replace('\', '/')
    $coreToml = (Join-Path $legacyResolved 'core').Replace('\', '/')
    $consensusToml = (Join-Path $legacyResolved 'consensus').Replace('\', '/')
    $manifest = $template.Replace('__CJ3_HARNESS_PATH__', $harnessToml)
    $manifest = $manifest.Replace('__CJ2_CORE_PATH__', $coreToml)
    $manifest = $manifest.Replace('__CJ2_CONSENSUS_PATH__', $consensusToml)
    Set-Content -LiteralPath (Join-Path $temporaryPath 'Cargo.toml') -Value $manifest -NoNewline
    Copy-Item -LiteralPath (Join-Path $scriptRoot 'legacy-driver\src\main.rs') -Destination (Join-Path $temporarySource 'main.rs')

    $lockedArguments = @()
    if (-not $BootstrapLock) {
        Copy-Item -LiteralPath $committedLock -Destination (Join-Path $temporaryPath 'Cargo.lock')
        $lockedArguments = @('--locked')
    }

    $driverTarget = Join-Path $repoRoot 'target\p004-legacy-driver'
    & cargo run --release --target-dir $driverTarget --manifest-path (Join-Path $temporaryPath 'Cargo.toml') @lockedArguments -- `
        --jsonl $jsonlPath --markdown $markdownPath --warmups $Warmups --samples $Samples `
        --checker-repetitions $CheckerRepetitions
    if ($LASTEXITCODE -ne 0) {
        throw "Legacy driver failed with exit code $LASTEXITCODE"
    }

    $legacyStatusAfter = @(& git -C $legacyResolved status --porcelain=v1 --untracked-files=all)
    if ($LASTEXITCODE -ne 0) {
        throw "Unable to re-read legacy working-tree status (exit $LASTEXITCODE)"
    }
    if ($legacyStatusAfter.Count -ne 0) {
        throw 'Legacy checkout changed during read-only calibration.'
    }
    foreach ($entry in $expectedHashes.GetEnumerator()) {
        $sourcePath = Join-Path $legacyResolved $entry.Key
        $actual = (Get-FileHash -Algorithm SHA256 -LiteralPath $sourcePath).Hash
        if ($actual -ne $entry.Value) {
            throw "Legacy source changed during calibration: $($entry.Key)"
        }
    }

    if ($BootstrapLock) {
        Copy-Item -LiteralPath (Join-Path $temporaryPath 'Cargo.lock') -Destination $committedLock -Force
    }

    $driverLockHash = (Get-FileHash -Algorithm SHA256 -LiteralPath (Join-Path $temporaryPath 'Cargo.lock')).Hash
    $jsonlHash = (Get-FileHash -Algorithm SHA256 -LiteralPath $jsonlPath).Hash
    $markdownHash = (Get-FileHash -Algorithm SHA256 -LiteralPath $markdownPath).Hash
    $rustcVersion = (& rustc -Vv) -join "`n"
    $cargoVersion = (& cargo -V).Trim()
    $processor = (Get-CimInstance -ClassName Win32_Processor | Select-Object -First 1 -ExpandProperty Name).Trim()
    $os = (Get-CimInstance -ClassName Win32_OperatingSystem | Select-Object -First 1)
    $rustcIndented = (($rustcVersion -split "`n") | ForEach-Object { "    $($_.TrimEnd())" }) -join "`n"
    $sourcePins = (($expectedHashes.GetEnumerator() | Sort-Object Key | ForEach-Object { "$($_.Key)=$($_.Value)" })) -join '; '
    $harnessHash = (Get-FileHash -Algorithm SHA256 -LiteralPath (Join-Path $scriptRoot 'src\lib.rs')).Hash
    $driverHash = (Get-FileHash -Algorithm SHA256 -LiteralPath (Join-Path $scriptRoot 'legacy-driver\src\main.rs')).Hash
    $runnerHash = (Get-FileHash -Algorithm SHA256 -LiteralPath $PSCommandPath).Hash
    $environment = @"
# P-004 legacy calibration environment

- Captured UTC: $([DateTime]::UtcNow.ToString('o'))
- Host OS: $($os.Caption) $($os.Version) $($os.OSArchitecture)
- Processor: $processor
- Logical processors: $([Environment]::ProcessorCount)
- Cargo: $cargoVersion
- Rust compiler:
$rustcIndented
- CJ3 frame/base revision: $(git -C $repoRoot rev-parse HEAD)
- CJ3 harness source SHA-256: $harnessHash
- Legacy driver source SHA-256: $driverHash
- Orchestration script SHA-256: $runnerHash
- Legacy source root: $legacyResolved
- Legacy source revision: $legacyRevision
- Driver warmups/samples/checker repetitions: $Warmups / $Samples / $CheckerRepetitions
- Provisional checker comparison: 15,000,000 ns (P-003 recommendation; not G0-ratified)
- Generated driver lock SHA-256: $driverLockHash
- Raw JSONL SHA-256: $jsonlHash
- Generated Markdown SHA-256: $markdownHash
- Legacy source SHA-256 pins: $sourcePins

The driver compiled the exact local 2.0 path dependencies in an isolated temporary
Cargo project. Only the three executable mining variants were timed. Descriptor-only
registry entries and the Custom payload were emitted as explicit unmeasured records.
Checker observations are per-operation integer averages from bounded timed batches;
the batch size is recorded above and in every JSONL row.
"@
    Set-Content -LiteralPath $environmentPath -Value $environment

    "P004_LEGACY_REVISION=$legacyRevision"
    "P004_DRIVER_LOCK_SHA256=$driverLockHash"
    "P004_JSONL_SHA256=$jsonlHash"
    "P004_MARKDOWN_SHA256=$markdownHash"
    "P004_ENVIRONMENT=$environmentPath"
}
finally {
    if (Test-Path -LiteralPath $temporaryPath) {
        $resolvedTemporary = [IO.Path]::GetFullPath((Resolve-Path -LiteralPath $temporaryPath).Path)
        $leaf = Split-Path -Leaf $resolvedTemporary
        if (-not $resolvedTemporary.StartsWith($temporaryRoot, [StringComparison]::OrdinalIgnoreCase) -or
            -not $leaf.StartsWith('cj3-p004-driver-', [StringComparison]::Ordinal)) {
            throw "Refusing to remove unverified temporary directory: $resolvedTemporary"
        }
        Remove-Item -LiteralPath $resolvedTemporary -Recurse -Force
    }
}
