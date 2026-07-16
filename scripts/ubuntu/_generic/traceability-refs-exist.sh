#!/usr/bin/env bash
set -euo pipefail

# traceability-refs-exist - Category C generic script
# Adapted for Pitha: derives DocsRoot from RepoRoot (docs/raw),
# parses text-diagram Traceability format (not table format).

REPO_ROOT=""
FINGERPRINT=""
OUT=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --repo-root) REPO_ROOT="$2"; shift 2 ;;
    --repo-fingerprint) FINGERPRINT="$2"; shift 2 ;;
    --out) OUT="$2"; shift 2 ;;
    *) echo "Unknown arg: $1" >&2; exit 2 ;;
  esac
done

if [[ -z "$REPO_ROOT" || -z "$FINGERPRINT" || -z "$OUT" ]]; then
  echo "Usage: $0 --repo-root <path> --repo-fingerprint <value> --out <path>" >&2
  exit 1
fi

DOCS_ROOT="$REPO_ROOT/docs/raw"

if [[ ! -d "$DOCS_ROOT" ]]; then
  echo '{"check":"traceability-refs-exist","domain":"_generic","category":"C","status":"error","metrics":{"domains_checked":0,"refs_found":0,"refs_valid":0,"refs_missing":0},"evidence":["docs-root not found: '"$DOCS_ROOT"'"],"executed_at":"'"$(date -u +%Y-%m-%dT%H:%M:%SZ)"'","repo_fingerprint":"'"$FINGERPRINT"'"}' > "$OUT"
  exit 1
fi

EXECUTED_AT="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
DOMAINS_CHECKED=0
REFS_FOUND=0
REFS_VALID=0
REFS_MISSING=0
EVIDENCE=()

find "$DOCS_ROOT" -name '*.md' -type f | while read -r docfile; do
  if ! grep -q '^## Traceability' "$docfile" 2>/dev/null; then
    continue
  fi

  DOMAINS_CHECKED=$((DOMAINS_CHECKED + 1))
  docname="$(basename "$docfile" .md)"

  in_traceability=0
  traceability_content=""
  while IFS= read -r line; do
    if [[ "$line" == ^##\ * && "$in_traceability" -eq 1 ]]; then
      break
    fi
    if [[ "$line" == ^##\ Traceability ]]; then
      in_traceability=1
      continue
    fi
    if [[ "$in_traceability" -eq 1 ]]; then
      traceability_content+="$line"$'\n'
    fi
  done < "$docfile"

  # Extract document names from text diagrams
  while IFS= read -r ref; do
    [[ -z "$ref" ]] && continue
    REFS_FOUND=$((REFS_FOUND + 1))
    normalized="$(echo "$ref" | tr '[:upper:]' '[:lower:]' | tr ' ' '-' | tr -cd 'a-z0-9-')"

    found=0
    if [[ -d "$DOCS_ROOT/$normalized" ]] 2>/dev/null; then
      found=1
    else
      while IFS= read -r f; do
        fbase="$(basename "$f" .md)"
        fnorm="$(echo "$fbase" | tr '[:upper:]' '[:lower:]' | tr ' ' '-' | tr -cd 'a-z0-9-')"
        if [[ "$fnorm" == "$normalized" ]] || [[ "${fnorm//-/}" == "${normalized//-/}" ]]; then
          found=1
          break
        fi
      done < <(find "$DOCS_ROOT" -name '*.md' -type f 2>/dev/null)
    fi

    if [[ "$found" -eq 1 ]]; then
      REFS_VALID=$((REFS_VALID + 1))
    else
      REFS_MISSING=$((REFS_MISSING + 1))
      EVIDENCE+=("$docname: Referenced '$ref' has no matching document in docs-root")
    fi
  done < <(echo "$traceability_content" | grep -oE '[A-Z][a-zA-Z]+([ ][A-Z][a-zA-Z]+)*(-[A-Z][a-zA-Z]+)*' | sort -u)
done

if [[ "$DOMAINS_CHECKED" -eq 0 ]]; then
  STATUS="not_applicable"
  EVIDENCE=("No domains with Traceability sections found in docs-root")
elif [[ "$REFS_MISSING" -gt 0 ]]; then
  STATUS="fail"
else
  STATUS="pass"
fi

EVIDENCE_JSON="["
for i in "${!EVIDENCE[@]}"; do
  [[ $i -gt 0 ]] && EVIDENCE_JSON+=","
  escaped="$(echo "${EVIDENCE[$i]}" | sed 's/\\/\\\\/g; s/"/\\"/g')"
  EVIDENCE_JSON+="\"$escaped\""
done
EVIDENCE_JSON+="]"

cat > "$OUT" <<ENDJSON
{
  "check": "traceability-refs-exist",
  "domain": "_generic",
  "category": "C",
  "status": "$STATUS",
  "metrics": {
    "domains_checked": $DOMAINS_CHECKED,
    "refs_found": $REFS_FOUND,
    "refs_valid": $REFS_VALID,
    "refs_missing": $REFS_MISSING
  },
  "evidence": $EVIDENCE_JSON,
  "executed_at": "$EXECUTED_AT",
  "repo_fingerprint": "$FINGERPRINT"
}
ENDJSON
