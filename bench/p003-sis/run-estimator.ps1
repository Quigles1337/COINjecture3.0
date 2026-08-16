[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)]
    [string] $EstimatorCheckout
)

$ErrorActionPreference = 'Stop'
$expectedCommit = '3e48ef421ec256afddb3e7d2249a77eab6e9ba12'
$expectedSisLatticeHash = 'D68EC5D0F471CF4904126211D8B2579186FA6DCE645AC7339E95BD621A505BE1'
$checkout = (Resolve-Path -LiteralPath $EstimatorCheckout).Path
$head = (& git -C $checkout rev-parse HEAD | Out-String).Trim()
if ($LASTEXITCODE -ne 0 -or $head -ne $expectedCommit) {
    throw "Estimator checkout must be exactly $expectedCommit; found '$head'."
}
$sisLattice = Join-Path $checkout 'estimator\sis_lattice.py'
$sourceHash = (Get-FileHash -Algorithm SHA256 -LiteralPath $sisLattice).Hash
if ($sourceHash -ne $expectedSisLatticeHash) {
    throw "estimator/sis_lattice.py hash mismatch: $sourceHash"
}

$image = "cj3-lattice-estimator:$($expectedCommit.Substring(0, 8))"
& docker build --tag $image --file (Join-Path $checkout 'docker\Dockerfile.dev') $checkout
if ($LASTEXITCODE -ne 0) {
    throw 'Pinned estimator image build failed.'
}
$script = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot 'estimator.sage')).Path
& docker run --rm --env PYTHONPATH=/lattice-estimator `
    --mount "type=bind,source=$script,target=/work/estimator.sage,readonly" `
    --entrypoint /usr/bin/sage $image -python /work/estimator.sage
if ($LASTEXITCODE -ne 0) {
    throw 'Pinned estimator execution failed.'
}
