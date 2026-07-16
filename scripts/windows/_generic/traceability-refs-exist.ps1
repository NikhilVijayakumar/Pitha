[CmdletBinding()]
param(
    [Parameter(Mandatory=$true)] [string] $RepoRoot,
    [Parameter(Mandatory=$true)] [string] $RepoFingerprint,
    [Parameter(Mandatory=$true)] [string] $Out
)

# traceability-refs-exist - Category C generic script
# Adapted for Pitha: derives DocsRoot from RepoRoot (docs/raw),
# parses text-diagram Traceability format (not table format).

$DocsRoot = Join-Path (Join-Path $RepoRoot "docs") "raw"

if (-not (Test-Path -LiteralPath $DocsRoot -PathType Container)) {
    $result = @{
        check = "traceability-refs-exist"
        domain = "_generic"
        category = "C"
        status = "error"
        metrics = @{ domains_checked = 0; refs_found = 0; refs_valid = 0; refs_missing = 0 }
        evidence = @("docs-root not found: $DocsRoot")
        executed_at = [DateTime]::UtcNow.ToString("yyyy-MM-ddTHH:mm:ssZ")
        repo_fingerprint = $RepoFingerprint
    }
    $result | ConvertTo-Json -Depth 10 | Set-Content -Path $Out -Encoding UTF8
    exit 1
}

$executedAt = [DateTime]::UtcNow.ToString("yyyy-MM-ddTHH:mm:ssZ")
$domainsChecked = 0
$refsFound = 0
$refsValid = 0
$refsMissing = 0
$evidence = @()

$domainDirs = @{}
Get-ChildItem -Path $DocsRoot -Directory -ErrorAction SilentlyContinue | ForEach-Object {
    $domainDirs[$_.Name.ToLower()] = $_.FullName
}

$mdFiles = Get-ChildItem -Path $DocsRoot -Filter "*.md" -File -Recurse

foreach ($docfile in $mdFiles) {
    $content = Get-Content -Path $docfile.FullName -Raw -ErrorAction SilentlyContinue
    if (-not $content) { continue }

    if ($content -notmatch '(?m)^## Traceability') { continue }

    $domainsChecked++
    $docname = $docfile.BaseName

    $inTraceability = $false
    $traceabilityLines = @()
    foreach ($line in ($content -split "`n")) {
        if ($line -match '(?m)^## ' -and $inTraceability) { break }
        if ($line -match '(?m)^## Traceability') { $inTraceability = $true; continue }
        if ($inTraceability) { $traceabilityLines += $line }
    }
    $traceabilityContent = $traceabilityLines -join "`n"

    $namePattern = '(?m)^\s*([A-Z][a-zA-Z]+(?:\s+[A-Z][a-zA-Z]+)*(?:-[A-Z][a-zA-Z]+)*)\s*$'
    $nameMatches = [regex]::Matches($traceabilityContent, $namePattern)

    $arrowPattern = '---[>]?\s*([A-Z][a-zA-Z]+(?:\s+[A-Z][a-zA-Z]+)*(?:-[A-Z][a-zA-Z]+)*)'
    $arrowMatches = [regex]::Matches($traceabilityContent, $arrowPattern)

    $inlinePattern = '(?:from|to|in|of|derives?\s+from|references?)\s+(?:the\s+)?([A-Z][a-zA-Z]+(?:\s+[A-Z][a-zA-Z]+)*)'
    $inlineMatches = [regex]::Matches($traceabilityContent, $inlinePattern)

    $allRefs = @()
    foreach ($m in $nameMatches) { $allRefs += $m.Groups[1].Value }
    foreach ($m in $arrowMatches) { $allRefs += $m.Groups[1].Value }
    foreach ($m in $inlineMatches) { $allRefs += $m.Groups[1].Value }

    $allRefs = $allRefs | Select-Object -Unique

    foreach ($ref in $allRefs) {
        $refsFound++
        $normalized = $ref.ToLower() -replace '\s+', '-' -replace '[^a-z0-9\-]', ''

        $found = $false
        foreach ($dirName in $domainDirs.Keys) {
            if ($dirName -eq $normalized -or $dirName -replace '-', '' -eq $normalized -replace '-', '') {
                $found = $true
                break
            }
        }

        if (-not $found) {
            $allFiles = Get-ChildItem -Path $DocsRoot -Filter "*.md" -File -Recurse -ErrorAction SilentlyContinue
            foreach ($f in $allFiles) {
                $fBase = $f.BaseName.ToLower() -replace '\s+', '-' -replace '[^a-z0-9\-]', ''
                if ($fBase -eq $normalized -or $fBase -replace '-', '' -eq $normalized -replace '-', '') {
                    $found = $true
                    break
                }
            }
        }

        if ($found) {
            $refsValid++
        } else {
            $refsMissing++
            $evidence += "${docname}: Referenced '${ref}' has no matching document in docs-root"
        }
    }
}

if ($domainsChecked -eq 0) {
    $status = "not_applicable"
    $evidence = @("No domains with Traceability sections found in docs-root")
} elseif ($refsMissing -gt 0) {
    $status = "fail"
} else {
    $status = "pass"
}

$result = @{
    check = "traceability-refs-exist"
    domain = "_generic"
    category = "C"
    status = $status
    metrics = @{
        domains_checked = $domainsChecked
        refs_found = $refsFound
        refs_valid = $refsValid
        refs_missing = $refsMissing
    }
    evidence = $evidence
    executed_at = $executedAt
    repo_fingerprint = $RepoFingerprint
}

$result | ConvertTo-Json -Depth 10 | Set-Content -Path $Out -Encoding UTF8
