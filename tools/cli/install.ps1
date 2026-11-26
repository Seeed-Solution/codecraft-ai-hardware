param(
    [ValidatePattern('^(latest|\d+\.\d+\.\d+)$')][string]$Version = 'latest',
    [ValidatePattern('^[A-Za-z0-9_-]+/[A-Za-z0-9_.-]+$')][string]$Repository = 'Seeed-Solution/codecraft-ai-hardware',
    [switch]$FromSource
)
$ErrorActionPreference = 'Stop'
$work = Join-Path ([IO.Path]::GetTempPath()) ([Guid]::NewGuid().ToString())
$stage = $null
New-Item -ItemType Directory $work | Out-Null
try {
    if ($FromSource) {
        $repo = Resolve-Path (Join-Path $PSScriptRoot '../..')
        Push-Location $repo
        try {
            $compiler = rustc -vV
            if ($LASTEXITCODE -ne 0) { throw 'Cannot inspect Rust compiler' }
            $target = ($compiler | Select-String '^host: ').Line -replace '^host: ', ''
            if (-not $target) { throw 'Cannot determine Rust host target' }
            cargo build --release --locked --package codecraft --target-dir .codecraft/cargo --target $target
            if ($LASTEXITCODE -ne 0) { throw 'CodeCraft build failed' }
            $binary = Join-Path $repo ".codecraft/cargo/$target/release/codecraft.exe"
        } finally { Pop-Location }
    } else {
        $arch = if ($env:PROCESSOR_ARCHITEW6432) { $env:PROCESSOR_ARCHITEW6432 } else { $env:PROCESSOR_ARCHITECTURE }
        if ($env:OS -ne 'Windows_NT' -or $arch -ne 'AMD64') { throw 'No published binary for this host; use -FromSource explicitly.' }
        if ($Version -eq 'latest') {
            $release = Invoke-RestMethod -Uri "https://api.github.com/repos/$Repository/releases/latest"
            if ($release.draft -or $release.prerelease -or $release.tag_name -notmatch '^v(\d+\.\d+\.\d+)$') {
                throw 'Latest release must be a published stable vX.Y.Z tag.'
            }
            $Version = $release.tag_name.Substring(1)
        }
        Write-Output "Installing CodeCraft $Version from $Repository"
        $asset = 'codecraft-x86_64-pc-windows-msvc.exe'
        $base = "https://github.com/$Repository/releases/download/v$Version"
        foreach ($name in @($asset, 'SHA256SUMS')) {
            Invoke-WebRequest -UseBasicParsing -Uri "$base/$name" -OutFile (Join-Path $work $name)
        }
        $entries = @(Get-Content (Join-Path $work 'SHA256SUMS') | Where-Object { $_ -match "^[a-fA-F0-9]{64}\s+$([regex]::Escape($asset))$" })
        if ($entries.Count -ne 1) { throw 'Missing or ambiguous SHA256SUMS entry' }
        $expected = ($entries[0] -split '\s+')[0]
        $binary = Join-Path $work $asset
        if ((Get-FileHash $binary -Algorithm SHA256).Hash -ne $expected) { throw 'Checksum mismatch; existing installation preserved.' }
        $actualVersion = & $binary --version
        if ($LASTEXITCODE -ne 0 -or $actualVersion -ne "codecraft $Version") { throw 'Binary version mismatch; existing installation preserved.' }
    }
    $destination = if ($env:CODECRAFT_BIN_DIR) { $env:CODECRAFT_BIN_DIR } else { Join-Path $env:LOCALAPPDATA 'CodeCraft/bin' }
    New-Item -ItemType Directory -Force $destination | Out-Null
    $destination = (Resolve-Path $destination).Path
    $stage = Join-Path $destination ('.codecraft-' + [Guid]::NewGuid().ToString() + '.exe')
    Copy-Item $binary $stage
    $installed = Join-Path $destination 'codecraft.exe'
    # Windows PowerShell converts $null to an empty string for this .NET
    # string parameter. NullString passes an actual null backup path.
    if (Test-Path $installed) { [IO.File]::Replace($stage, $installed, [NullString]::Value) }
    else { [IO.File]::Move($stage, $installed) }
    $stage = $null
    Write-Output "Installed $installed. Add this directory to PATH."
} finally {
    if ($stage -and (Test-Path $stage)) { Remove-Item $stage -Force }
    Remove-Item $work -Recurse -Force
}
