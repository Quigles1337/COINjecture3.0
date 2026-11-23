[CmdletBinding()]
param()

# D6 Nightly gate `full-fuzz-corpus` (Gate G0-F ruling, clauses 2.3 and 4.5a).
#
# The long fuzz run: the same cargo-fuzz targets as the D6 gate `codec-fuzz-smoke`, on
# the same nightly toolchain pinned by date in fuzz/rust-toolchain.toml. Both are in
# ../codec-fuzz-common.ps1. This script installs nothing.
#
# Two things differ from the smoke run, and both are operating choices, not protocol
# values:
#
# - Each target runs for a fixed time instead of a fixed number of runs.
# - The libFuzzer seed is the UTC date as yyyyMMdd, so each night explores from a new
#   seed and the log names it. To repeat a night, pass that seed to the same target on
#   the same toolchain.
#
# A long run that finds nothing is not evidence that no defect exists.

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

. (Join-Path $PSScriptRoot '..\codec-fuzz-common.ps1')

# Seconds of fuzzing per target: one hour in all.
$seconds = [ordered]@{
    'transaction_codec'  = 600
    'header_codec'       = 600
    'block_codec'        = 600
    'sis_solution_codec' = 600
    'ed25519_verify'     = 600
    'length_limits'      = 600
}

$seed = [int] [DateTime]::UtcNow.ToString('yyyyMMdd', [Globalization.CultureInfo]::InvariantCulture)

Invoke-CodecFuzzGate -SummaryName 'FULL_FUZZ_CORPUS' -Budget $seconds -BudgetKind Seconds -Seed $seed
