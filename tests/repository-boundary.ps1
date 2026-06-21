Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$repoRoot = (& git rev-parse --show-toplevel 2>$null).Trim()
if (-not $repoRoot) {
    throw 'Unable to locate repository root.'
}

$repoRoot = (Resolve-Path -LiteralPath $repoRoot).Path
Set-Location -LiteralPath $repoRoot

$failures = [System.Collections.Generic.List[string]]::new()

function Add-Failure {
    param([Parameter(Mandatory = $true)][string]$Message)
    $script:failures.Add($Message)
}

function Join-RepoPath {
    param([Parameter(Mandatory = $true)][string]$RelativePath)
    $nativeRelativePath = $RelativePath -replace '/', [System.IO.Path]::DirectorySeparatorChar
    return Join-Path -Path $repoRoot -ChildPath $nativeRelativePath
}

$sourceBookDir = 'Text' + 'book'
$breakdownDir = 'ana' + 'lysis'
$privateRepoMarker = '<private' + '-old-repo>'
$sensitiveTerms = @(
    $sourceBookDir,
    ($breakdownDir + '/'),
    $privateRepoMarker,
    ('API_' + 'KEY'),
    ('SEC' + 'RET'),
    ('TOK' + 'EN')
)

$forbiddenRootPaths = @(
    $sourceBookDir,
    $breakdownDir,
    'library',
    '.env'
)

foreach ($relativePath in $forbiddenRootPaths) {
    $fullPath = Join-RepoPath -RelativePath $relativePath
    if (Test-Path -LiteralPath $fullPath) {
        Add-Failure "Forbidden repository root path exists: $relativePath"
    }
}

$gitFiles = @(& git ls-files --cached --others --exclude-standard)
if ($LASTEXITCODE -ne 0) {
    throw 'Unable to enumerate tracked and untracked repository files.'
}

$repoFiles = @(
    $gitFiles |
        Where-Object { $_ } |
        ForEach-Object { $_ -replace '\\', '/' }
)

$forbiddenPathPatterns = @(
    @{ Pattern = '(^|/)\.env(\.|$)'; Label = 'environment file' },
    @{ Pattern = '\.(db-shm|db-wal|db|sqlite3|sqlite)$'; Label = 'database file' },
    @{ Pattern = '(^|/)(release|dist|build|target|\.cache|\.vite|runs)(/|$)'; Label = 'generated artifact directory' },
    @{ Pattern = '(^|/)ai/jobs/'; Label = 'local job artifact directory' },
    @{ Pattern = '\.(log|stdout|stderr)$'; Label = 'log or captured stream file' }
)

foreach ($repoFile in $repoFiles) {
    foreach ($entry in $forbiddenPathPatterns) {
        if ($repoFile -match $entry.Pattern) {
            Add-Failure "Forbidden $($entry.Label) is tracked or unignored: $repoFile"
        }
    }
}

$allowListedContentFiles = [System.Collections.Generic.HashSet[string]]::new([System.StringComparer]::OrdinalIgnoreCase)
[void]$allowListedContentFiles.Add('AGENTS.md')
[void]$allowListedContentFiles.Add('docs/WORKING_GUIDE.md')

$sensitiveRegex = [regex](($sensitiveTerms | ForEach-Object { [regex]::Escape($_) }) -join '|')

foreach ($repoFile in $repoFiles) {
    if ($allowListedContentFiles.Contains($repoFile)) {
        continue
    }

    $fullPath = Join-RepoPath -RelativePath $repoFile
    if (-not (Test-Path -LiteralPath $fullPath -PathType Leaf)) {
        continue
    }

    try {
        $content = Get-Content -LiteralPath $fullPath -Raw
    }
    catch {
        Add-Failure "Unable to read public repository file for boundary scan: $repoFile"
        continue
    }

    $matches = @(
        $sensitiveRegex.Matches($content) |
            ForEach-Object { $_.Value } |
            Sort-Object -Unique
    )

    if ($matches.Count -gt 0) {
        Add-Failure "Sensitive boundary marker appears outside the documentation allowlist: $repoFile [$($matches -join ', ')]"
    }
}

$ignoreSamples = @(
    "$sourceBookDir/sample.txt",
    "$breakdownDir/sample.json",
    'library/user.db',
    'userData/settings.json',
    '.env',
    '.env.local',
    'sample.db',
    'sample.db-shm',
    'sample.db-wal',
    'sample.sqlite',
    'sample.sqlite3',
    'release/app.zip',
    'dist/app.js',
    'build/app.js',
    'target/debug/app',
    '.cache/tmp',
    '.vite/cache',
    'runs/job.json',
    'ai/jobs/job.json',
    'debug.log',
    'job.stdout',
    'job.stderr',
    '.reasonix/desktop-topic-titles.json',
    'docs/overview-handoff-2026-06-05.md',
    'docs/project-overview-for-external-review.md',
    'docs/code-audit-9.4-9.6.md',
    'docs/architecture/context-pack-builder-preflight.md',
    'docs/architecture/schema-source-of-truth.md',
    'docs/methodology/whole-book-breakdown-methodology.md',
    'docs/research/local-note.md',
    'docs/superpowers/plans/local-plan.md',
    'docs/product/frontend-prototype.html',
    'docs/product/frontend-high-fidelity-example.png',
    'docs/product/visual-references/example.png',
    'docs/维度框架.md'
)

foreach ($samplePath in $ignoreSamples) {
    & git check-ignore --quiet -- $samplePath
    if ($LASTEXITCODE -ne 0) {
        Add-Failure ".gitignore does not ignore expected local-only path: $samplePath"
    }
}

$packageConfigCandidates = @(
    'package.json',
    'src-tauri/tauri.conf.json',
    'src-tauri/tauri.conf.json5',
    'src-tauri/Tauri.toml',
    'tauri.conf.json',
    'tauri.conf.json5'
)

$workflowRoot = Join-RepoPath -RelativePath '.github/workflows'
if (Test-Path -LiteralPath $workflowRoot -PathType Container) {
    $workflowFiles = Get-ChildItem -LiteralPath $workflowRoot -File -ErrorAction Stop |
        ForEach-Object { '.github/workflows/' + $_.Name }
    $packageConfigCandidates += $workflowFiles
}

$existingPackageConfigs = @(
    $packageConfigCandidates |
        Where-Object { Test-Path -LiteralPath (Join-RepoPath -RelativePath $_) -PathType Leaf }
)

$packageForbiddenTerms = @(
    'library/',
    'library\',
    ($sourceBookDir + '/'),
    ($sourceBookDir + '\'),
    ($breakdownDir + '/'),
    ($breakdownDir + '\'),
    '.env',
    '.db',
    '.sqlite',
    'runs/',
    'runs\',
    'release/',
    'release\',
    'logs/',
    'logs\',
    'cache/',
    'cache\'
)

if ($existingPackageConfigs.Count -eq 0) {
    Write-Host 'No package or release configuration files found; package boundary scan skipped.'
}
else {
    $packageForbiddenRegex = [regex](($packageForbiddenTerms | ForEach-Object { [regex]::Escape($_) }) -join '|')
    foreach ($configPath in $existingPackageConfigs) {
        $content = Get-Content -LiteralPath (Join-RepoPath -RelativePath $configPath) -Raw
        $matches = @(
            $packageForbiddenRegex.Matches($content) |
                ForEach-Object { $_.Value } |
                Sort-Object -Unique
        )

        if ($matches.Count -gt 0) {
            Add-Failure "Package or release config may include local-only content: $configPath [$($matches -join ', ')]"
        }
    }
}

if ($failures.Count -gt 0) {
    Write-Error ("Repository boundary check failed:`n - " + ($failures -join "`n - "))
    exit 1
}

Write-Host 'Repository boundary check passed.'
