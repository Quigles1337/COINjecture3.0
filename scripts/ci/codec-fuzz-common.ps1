# Shared by the two fuzz gate handlers: `active/codec-fuzz-smoke.ps1` (D6) and
# `active/full-fuzz-corpus.ps1` (D6 Nightly). Dot-source it, then call
# Invoke-CodecFuzzGate.
#
# Both gates run the same cargo-fuzz targets on the same nightly toolchain, pinned by
# date in fuzz/rust-toolchain.toml (ruling G0-C8d; P-007 resume ruling, clause 4.5;
# Gate G0-F ruling, clause 2.3). The list of targets is here once, so the two gates
# cannot come to run different targets. They differ only in how long each target runs
# and from which seed. This script installs nothing: the workflow job installs the
# toolchain and cargo-fuzz before it calls a handler.

Set-StrictMode -Version Latest

# Every cargo-fuzz target under fuzz/. A target under fuzz/ that is missing here, or one
# here that is missing there, fails either gate.
$script:CodecFuzzTargets = @(
    'transaction_codec'
    'header_codec'
    'block_codec'
    'sis_solution_codec'
    'ed25519_verify'
    'length_limits'
)

# The driver version the workflow jobs install. A different one fails either gate.
$script:RequiredCargoFuzz = 'cargo-fuzz 0.13.2'

# The arithmetic back end every build of cj3-types uses (Gate G0-F ruling, clause 4.5c).
# The same flag as in .cargo/config.toml.
$script:SerialBackendFlags = '--cfg curve25519_dalek_backend="serial"'

function Invoke-CodecFuzzGate {
    <#
    .SYNOPSIS
    Runs every cargo-fuzz target once, each within its budget, and prints the gate's
    summary line. Call it without capturing its output, so the fuzzers' logs stream.

    .PARAMETER SummaryName
    The name that opens the summary line, for example CODEC_FUZZ_SMOKE.

    .PARAMETER Budget
    One entry per target: the number of runs (BudgetKind Runs) or of seconds
    (BudgetKind Seconds) that libFuzzer is given. Its keys must be exactly the shared
    target list.
    #>
    [CmdletBinding()]
    param(
        [Parameter(Mandatory = $true)]
        [ValidatePattern('^[A-Z][A-Z0-9_]*$')]
        [string] $SummaryName,

        [Parameter(Mandatory = $true)]
        [System.Collections.IDictionary] $Budget,

        [Parameter(Mandatory = $true)]
        [ValidateSet('Runs', 'Seconds')]
        [string] $BudgetKind,

        [Parameter(Mandatory = $true)]
        [ValidateRange(1, 2147483647)]
        [int] $Seed
    )

    $repoRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot '..\..')).Path
    $fuzzRoot = Join-Path $repoRoot 'fuzz'
    $fuzzManifest = Join-Path $fuzzRoot 'Cargo.toml'

    $budgeted = @($Budget.Keys | Sort-Object)
    $shared = @($script:CodecFuzzTargets | Sort-Object)
    if (($budgeted -join ',') -ne ($shared -join ',')) {
        throw "Budget mismatch. Shared target list: $($shared -join ', '). Budgeted: $($budgeted -join ', ')."
    }

    $toolchainFile = Get-Content -LiteralPath (Join-Path $fuzzRoot 'rust-toolchain.toml') -Raw
    if ($toolchainFile -notmatch '(?m)^channel\s*=\s*"(nightly-\d{4}-\d{2}-\d{2})"\s*$') {
        throw 'fuzz/rust-toolchain.toml does not pin a nightly toolchain by date.'
    }
    $channel = $Matches[1]

    Push-Location $repoRoot
    try {
        # Presence is read from the list of installed toolchains. Asking the toolchain
        # for its version instead would let rustup install it on demand.
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
        if ($cargoFuzz -ne $script:RequiredCargoFuzz) {
            throw "cargo-fuzz version mismatch. Required: $($script:RequiredCargoFuzz). Found: $cargoFuzz."
        }
        Write-Output "FUZZ_TOOLCHAIN=$nightly"
        Write-Output "FUZZ_DRIVER=$cargoFuzz"

        # The fuzz crate is outside the workspace, so the workspace's format job does
        # not see it. This check runs on the stable toolchain pinned in the repository
        # root.
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
        if (($listed -join ',') -ne ($shared -join ',')) {
            throw "Fuzz target list mismatch. Under fuzz/: $($listed -join ', '). In the shared list: $($shared -join ', ')."
        }

        # One arithmetic back end (Gate G0-F ruling, clause 4.5c). cargo-fuzz sets
        # RUSTFLAGS for its own flags, and a RUSTFLAGS variable replaces the flags of
        # .cargo/config.toml. cargo-fuzz appends the caller's RUSTFLAGS to its own, so
        # the setting is passed here. Without it cj3-types refuses to compile, so a fuzz
        # build that loses the setting fails; it does not fuzz another back end.
        if (Test-Path -LiteralPath Env:RUSTFLAGS) {
            throw 'RUSTFLAGS is set in this environment. The fuzz gates set it themselves.'
        }
        $env:RUSTFLAGS = $script:SerialBackendFlags
        Write-Output "FUZZ_RUSTFLAGS=$($env:RUSTFLAGS)"

        $total = 0
        foreach ($target in $script:CodecFuzzTargets) {
            $amount = [int] $Budget[$target]
            if ($amount -lt 1) {
                throw "Fuzz target $target has no budget."
            }
            $total += $amount
            if ($BudgetKind -eq 'Runs') {
                $budgetArgument = "-runs=$amount"
                $budgetLabel = "RUNS=$amount"
            }
            else {
                $budgetArgument = "-max_total_time=$amount"
                $budgetLabel = "SECONDS=$amount"
            }

            $seeds = Join-Path $fuzzRoot "seeds/$target"
            if (-not (Test-Path -LiteralPath $seeds -PathType Container)) {
                throw "No committed seed inputs for fuzz target $target."
            }
            # libFuzzer writes new inputs to the first corpus directory. It is temporary,
            # so the committed seeds are read and never changed.
            $corpus = Join-Path ([IO.Path]::GetTempPath()) "cj3-fuzz-$target-$([Guid]::NewGuid().ToString('N'))"
            New-Item -ItemType Directory -Path $corpus | Out-Null
            try {
                & cargo "+$channel" fuzz run $target $corpus $seeds -- `
                    $budgetArgument "-seed=$Seed" '-max_len=4096' '-timeout=60' `
                    '-rss_limit_mb=4096' '-print_final_stats=1'
                if ($LASTEXITCODE -ne 0) {
                    # A hosted runner is discarded after the job, so the failing inputs
                    # are printed here. `base64 -d` turns each back into the input.
                    $artifacts = Join-Path $fuzzRoot "artifacts/$target"
                    if (Test-Path -LiteralPath $artifacts -PathType Container) {
                        foreach ($artifact in @(Get-ChildItem -LiteralPath $artifacts -File)) {
                            $bytes = [IO.File]::ReadAllBytes($artifact.FullName)
                            Write-Output "FUZZ_ARTIFACT=$target/$($artifact.Name) BYTES=$($bytes.Length) BASE64=$([Convert]::ToBase64String($bytes))"
                        }
                    }
                    throw "Fuzz target failed: $target (seed $Seed, $budgetLabel)"
                }
            }
            finally {
                Remove-Item -LiteralPath $corpus -Recurse -Force
            }
            Write-Output "FUZZ_TARGET=$target $budgetLabel RESULT=PASS"
        }

        $totalLabel = if ($BudgetKind -eq 'Runs') { 'TOTAL_RUNS' } else { 'TOTAL_SECONDS' }
        Write-Output "$SummaryName=PASS TOOLCHAIN=$channel TARGETS=$($script:CodecFuzzTargets.Count) $totalLabel=$total SEED=$Seed"
    }
    finally {
        Remove-Item -LiteralPath Env:RUSTFLAGS -ErrorAction SilentlyContinue
        Pop-Location
    }
}
