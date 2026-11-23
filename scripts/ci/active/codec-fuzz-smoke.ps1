[CmdletBinding()]
param()

# D6 gate `codec-fuzz-smoke` (ruling G0-C8d; P-007 resume ruling, clause 4.5).
#
# Runs every cargo-fuzz target under fuzz/ for a fixed number of runs from a fixed seed.
# The targets need a nightly toolchain; it is pinned by date in fuzz/rust-toolchain.toml
# and is used by this gate alone. The workflow job installs it and cargo-fuzz before it
# calls this script. This script installs nothing.

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

$repoRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot '..\..\..')).Path
$fuzzRoot = Join-Path $repoRoot 'fuzz'
$fuzzManifest = Join-Path $fuzzRoot 'Cargo.toml'

# Every target, with its bounded number of runs. A target under fuzz/ that is missing
# here, or one here that is missing there, fails the gate.
$targets = [ordered]@{
    'transaction_codec'  = 200000
    'header_codec'       = 200000
    'block_codec'        = 200000
    'sis_solution_codec' = 40000
    'ed25519_verify'     = 100000
    'length_limits'      = 300
}
$libFuzzerSeed = 20261122
# The driver version the workflow job installs. A different one fails the gate.
$requiredCargoFuzz = 'cargo-fuzz 0.13.2'

$toolchainFile = Get-Content -LiteralPath (Join-Path $fuzzRoot 'rust-toolchain.toml') -Raw
if ($toolchainFile -notmatch '(?m)^channel\s*=\s*"(nightly-\d{4}-\d{2}-\d{2})"\s*$') {
    throw 'fuzz/rust-toolchain.toml does not pin a nightly toolchain by date.'
}
$channel = $Matches[1]

Push-Location $repoRoot
try {
    # Presence is read from the list of installed toolchains. Asking the toolchain for
    # its version instead would let rustup install it on demand.
    $installed = @(& rustup toolchain list)
    if ($LASTEXITCODE -ne 0) {
        throw 'rustup toolchain list failed.'
    }
    if (-not ($installed | Where-Object { $_ -like "$channel-*" })) {
        throw "The pinned toolchain $channel is not installed. The workflow job installs it."
    }
    $nightly = (& rustc "+$channel" --version | Out-String).Trim()
    if ($LASTEXITCODE -ne 0) {
        throw "The pinned toolchain $channel does not run."
    }
    $cargoFuzz = (& cargo fuzz --version | Out-String).Trim()
    if ($LASTEXITCODE -ne 0) {
        throw 'cargo-fuzz is not installed. The workflow job installs it.'
    }
    if ($cargoFuzz -ne $requiredCargoFuzz) {
        throw "cargo-fuzz version mismatch. Required: $requiredCargoFuzz. Found: $cargoFuzz."
    }
    Write-Output "FUZZ_TOOLCHAIN=$nightly"
    Write-Output "FUZZ_DRIVER=$cargoFuzz"

    # The fuzz crate is outside the workspace, so the workspace's format job does not
    # see it. This check runs on the stable toolchain pinned in the repository root.
    & cargo fmt --manifest-path $fuzzManifest -- --check
    if ($LASTEXITCODE -ne 0) {
        throw 'The fuzz crate is not formatted.'
    }

    # The committed fuzz/Cargo.lock must be current: a fuzz build never rewrites it.
    & cargo "+$channel" metadata --locked --format-version 1 --manifest-path $fuzzManifest | Out-Null
    if ($LASTEXITCODE -ne 0) {
        throw 'fuzz/Cargo.lock is missing or out of date.'
    }

    $listed = @(
        & cargo "+$channel" fuzz list |
            ForEach-Object { $_.Trim() } |
            Where-Object { $_ -ne '' } |
            Sort-Object
    )
    if ($LASTEXITCODE -ne 0) {
        throw 'cargo fuzz list failed.'
    }
    $expected = @($targets.Keys | Sort-Object)
    if (($listed -join ',') -ne ($expected -join ',')) {
        throw "Fuzz target list mismatch. Under fuzz/: $($listed -join ', '). In this gate: $($expected -join ', ')."
    }

    $totalRuns = 0
    foreach ($target in $targets.Keys) {
        $runs = $targets[$target]
        $seeds = Join-Path $fuzzRoot "seeds/$target"
        if (-not (Test-Path -LiteralPath $seeds -PathType Container)) {
            throw "No committed seed inputs for fuzz target $target."
        }
        # libFuzzer writes new inputs to the first corpus directory. It is temporary, so
        # the committed seeds are read and never changed.
        $corpus = Join-Path ([IO.Path]::GetTempPath()) "cj3-fuzz-$target-$([Guid]::NewGuid().ToString('N'))"
        New-Item -ItemType Directory -Path $corpus | Out-Null
        try {
            & cargo "+$channel" fuzz run $target $corpus $seeds -- `
                "-runs=$runs" "-seed=$libFuzzerSeed" '-max_len=4096' '-timeout=60' `
                '-rss_limit_mb=4096' '-print_final_stats=1'
            if ($LASTEXITCODE -ne 0) {
                throw "Fuzz target failed: $target"
            }
        }
        finally {
            Remove-Item -LiteralPath $corpus -Recurse -Force
        }
        $totalRuns += $runs
        Write-Output "FUZZ_TARGET=$target RUNS=$runs RESULT=PASS"
    }

    Write-Output "CODEC_FUZZ_SMOKE=PASS TOOLCHAIN=$channel TARGETS=$($targets.Count) TOTAL_RUNS=$totalRuns SEED=$libFuzzerSeed"
}
finally {
    Pop-Location
}
