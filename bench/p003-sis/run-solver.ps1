[CmdletBinding()]
param(
    [ValidateRange(1, 300)]
    [int] $TimeoutSeconds = 30
)

$ErrorActionPreference = 'Stop'
$repoRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot '..\..')).Path
Push-Location $repoRoot
try {
    & cargo build --release --locked -p cj3-solver-sis
    if ($LASTEXITCODE -ne 0) {
        throw 'Release external-solver build failed.'
    }
    $solver = (Resolve-Path -LiteralPath 'target\release\cj3-solver-sis.exe').Path
    $cases = @(
        [pscustomobject]@{ n = 8; m = 112; q = 67; beta_squared = 112; seeds = 5 },
        [pscustomobject]@{ n = 12; m = 192; q = 149; beta_squared = 192; seeds = 5 },
        [pscustomobject]@{ n = 16; m = 288; q = 257; beta_squared = 288; seeds = 3 },
        [pscustomobject]@{ n = 24; m = 480; q = 577; beta_squared = 480; seeds = 1 }
    )
    $sha256 = [System.Security.Cryptography.SHA256]::Create()
    try {
        foreach ($case in $cases) {
            foreach ($seedByte in 0..($case.seeds - 1)) {
                $seed = ('{0:x2}' -f $seedByte) * 32
                $startInfo = [System.Diagnostics.ProcessStartInfo]::new()
                $startInfo.FileName = $solver
                foreach ($argument in @(
                    [string] $case.n,
                    [string] $case.m,
                    [string] $case.q,
                    [string] $case.beta_squared,
                    $seed
                )) {
                    $startInfo.ArgumentList.Add($argument)
                }
                $startInfo.UseShellExecute = $false
                $startInfo.RedirectStandardOutput = $true
                $startInfo.RedirectStandardError = $true
                $startInfo.CreateNoWindow = $true
                $process = [System.Diagnostics.Process]::new()
                $process.StartInfo = $startInfo
                $clock = [System.Diagnostics.Stopwatch]::StartNew()
                [void] $process.Start()
                if (-not $process.WaitForExit($TimeoutSeconds * 1000)) {
                    $process.Kill($true)
                    $process.WaitForExit()
                    $clock.Stop()
                    [pscustomobject]@{
                        n = $case.n
                        m = $case.m
                        q = $case.q
                        beta_squared = $case.beta_squared
                        seed_byte = $seedByte
                        status = 'censored'
                        limit_ms = $TimeoutSeconds * 1000
                        elapsed_ns = $clock.ElapsedTicks * (1000000000 / [System.Diagnostics.Stopwatch]::Frequency)
                        output_sha256 = $null
                    } | ConvertTo-Json -Compress
                    continue
                }
                $clock.Stop()
                $stdout = $process.StandardOutput.ReadToEnd().Trim()
                $stderr = $process.StandardError.ReadToEnd().Trim()
                if ($process.ExitCode -ne 0) {
                    [pscustomobject]@{
                        n = $case.n
                        m = $case.m
                        q = $case.q
                        beta_squared = $case.beta_squared
                        seed_byte = $seedByte
                        status = 'completed-no-candidate'
                        limit_ms = $TimeoutSeconds * 1000
                        elapsed_ns = $clock.ElapsedTicks * (1000000000 / [System.Diagnostics.Stopwatch]::Frequency)
                        output_sha256 = $null
                        diagnostic = $stderr
                    } | ConvertTo-Json -Compress
                    continue
                }
                $digest = $sha256.ComputeHash([System.Text.Encoding]::UTF8.GetBytes($stdout))
                [pscustomobject]@{
                    n = $case.n
                    m = $case.m
                    q = $case.q
                    beta_squared = $case.beta_squared
                    seed_byte = $seedByte
                    status = 'solved-and-rechecked'
                    limit_ms = $TimeoutSeconds * 1000
                    elapsed_ns = $clock.ElapsedTicks * (1000000000 / [System.Diagnostics.Stopwatch]::Frequency)
                    output_sha256 = [Convert]::ToHexString($digest).ToLowerInvariant()
                    diagnostic = $null
                } | ConvertTo-Json -Compress
            }
        }
    }
    finally {
        $sha256.Dispose()
    }
}
finally {
    Pop-Location
}
