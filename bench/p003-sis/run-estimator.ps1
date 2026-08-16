[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)]
    [string] $EstimatorCheckout
)

$ErrorActionPreference = 'Stop'
$expectedCommit = '3e48ef421ec256afddb3e7d2249a77eab6e9ba12'
$expectedSisLatticeHash = 'D68EC5D0F471CF4904126211D8B2579186FA6DCE645AC7339E95BD621A505BE1'
$runtimeImage = 'sagemath/sagemath@sha256:ec32d9752b3a11c628103ca6802db890b63cbe9bb480cfea02de09656ecc84a2'
$checkout = (Resolve-Path -LiteralPath $EstimatorCheckout).Path
$head = (& git -C $checkout rev-parse HEAD | Out-String).Trim()
if ($LASTEXITCODE -ne 0 -or $head -ne $expectedCommit) {
    throw "Estimator checkout must be exactly $expectedCommit; found '$head'."
}
$worktreeStatus = @(& git -C $checkout status --porcelain=v1 --untracked-files=all)
if ($LASTEXITCODE -ne 0) {
    throw 'Unable to inspect the estimator worktree.'
}
if ($worktreeStatus.Count -ne 0) {
    throw 'Estimator checkout must be clean, including untracked files.'
}
$sisLattice = Join-Path $checkout 'estimator\sis_lattice.py'
$sourceHash = (Get-FileHash -Algorithm SHA256 -LiteralPath $sisLattice).Hash
if ($sourceHash -ne $expectedSisLatticeHash) {
    throw "estimator/sis_lattice.py hash mismatch: $sourceHash"
}

$script = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot 'estimator.sage')).Path
& docker run --rm --network none --read-only `
    --env HOME=/tmp --env PYTHONDONTWRITEBYTECODE=1 `
    --env PYTHONPATH=/lattice-estimator --env XDG_CACHE_HOME=/tmp/cache `
    --tmpfs /tmp:rw,noexec,nosuid,size=64m `
    --mount "type=bind,source=$checkout,target=/lattice-estimator,readonly" `
    --mount "type=bind,source=$script,target=/work/estimator.sage,readonly" `
    --entrypoint /usr/bin/sage $runtimeImage -python /work/estimator.sage
if ($LASTEXITCODE -ne 0) {
    throw 'Pinned estimator execution failed.'
}
