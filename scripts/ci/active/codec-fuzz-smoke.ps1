[CmdletBinding()]
param()

# D6 gate `codec-fuzz-smoke` (ruling G0-C8d; P-007 resume ruling, clause 4.5).
#
# Runs every cargo-fuzz target under fuzz/ for a fixed number of runs from a fixed seed.
# It is a bounded smoke run. The targets, the pinned nightly toolchain and the checks
# around them are in ../codec-fuzz-common.ps1, which the D6 Nightly gate
# `full-fuzz-corpus` shares. This script installs nothing.

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

. (Join-Path $PSScriptRoot '..\codec-fuzz-common.ps1')

# The bounded number of runs of each target.
$runs = [ordered]@{
    'transaction_codec'  = 200000
    'header_codec'       = 200000
    'block_codec'        = 200000
    'sis_solution_codec' = 40000
    'ed25519_verify'     = 100000
    'length_limits'      = 300
}

Invoke-CodecFuzzGate -SummaryName 'CODEC_FUZZ_SMOKE' -Budget $runs -BudgetKind Runs -Seed 20261122
