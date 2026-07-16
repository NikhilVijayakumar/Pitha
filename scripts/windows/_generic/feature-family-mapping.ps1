[CmdletBinding()]
param(
    [Parameter(Mandatory=$true)] [string] $RepoRoot,
    [Parameter(Mandatory=$true)] [string] $RepoFingerprint,
    [Parameter(Mandatory=$true)] [string] $Out
)

# feature-family-mapping - Category C generic script
# Adapted for Pitha: feature docs in docs/raw/feature/ without number prefixes.

$DocsRoot = Join-Path (Join-Path $RepoRoot "docs") "raw"

if (-not (Test-Path -LiteralPath $DocsRoot -PathType Container)) {
    $result = @{
        check = "feature-family-mapping"
        domain = "_generic"
        category = "C"
        status = "error"
        metrics = @{ features_count = 0; refs_found = 0; refs_valid = 0; orphans = 0 }
        evidence = @("docs-root not found: $DocsRoot")
        executed_at = (Get-Date -AsUTC).ToString("yyyy-MM-ddTHH:mm:ssZ")
        repo_fingerprint = $RepoFingerprint
    }
    $result | ConvertTo-Json -Depth 10 | Set-Content -Path $Out -Encoding UTF8
    exit 1
}

$executedAt = [DateTime]::UtcNow.ToString("yyyy-MM-ddTHH:mm:ssZ")

$featureDir = Join-Path $DocsRoot "feature"
$features = @{}
$refsFound = 0
$refsValid = 0
$orphans = 0
$evidence = @()

if (Test-Path -LiteralPath $featureDir -PathType Container) {
    Get-ChildItem -Path $featureDir -Filter "*.md" -File | ForEach-Object {
        $base = $_.BaseName
        $features[$base] = $_.FullName
    }
}

$featuresCount = $features.Count

foreach ($name in $features.Keys) {
    $content = Get-Content -Path $features[$name] -Raw -ErrorAction SilentlyContinue
    if (-not $content) { continue }

    $refsFound++
    if ($content -match '(?m)^## Traceability') {
        $refsValid++
    } else {
        $orphans++
        $evidence += "Feature '$name' is missing a Traceability section"
    }

    $requiredSections = @("## Purpose", "## Scope")
    foreach ($section in $requiredSections) {
        if ($content -notmatch [regex]::Escape($section)) {
            $orphans++
            $evidence += "Feature '$name' is missing section: $section"
        }
    }
}

if ($featuresCount -eq 0) {
    $status = "not_applicable"
    $evidence = @("No feature documents found in docs-root/feature/")
} elseif ($orphans -gt 0) {
    $status = "fail"
} else {
    $status = "pass"
}

$result = @{
    check = "feature-family-mapping"
    domain = "_generic"
    category = "C"
    status = $status
    metrics = @{
        features_count = $featuresCount
        refs_found = $refsFound
        refs_valid = $refsValid
        orphans = $orphans
    }
    evidence = $evidence
    executed_at = $executedAt
    repo_fingerprint = $RepoFingerprint
}

$result | ConvertTo-Json -Depth 10 | Set-Content -Path $Out -Encoding UTF8
