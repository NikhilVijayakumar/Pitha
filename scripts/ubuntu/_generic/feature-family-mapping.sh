#!/usr/bin/env bash
set -euo pipefail

# feature-family-mapping - Category C generic script
# Adapted for Pitha: feature docs in docs/raw/feature/ without number prefixes.

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
  echo '{"check":"feature-family-mapping","domain":"_generic","category":"C","status":"error","metrics":{"features_count":0,"refs_found":0,"refs_valid":0,"orphans":0},"evidence":["docs-root not found: '"$DOCS_ROOT"'"],"executed_at":"'"$(date -u +%Y-%m-%dT%H:%M:%SZ)"'","repo_fingerprint":"'"$FINGERPRINT"'"}' > "$OUT"
  exit 1
fi

EXECUTED_AT="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
FEATURE_DIR="$DOCS_ROOT/feature"

declare -A FEATURES=()
REFS_FOUND=0
REFS_VALID=0
ORPHANS=0
EVIDENCE=()

if [[ -d "$FEATURE_DIR" ]]; then
  while IFS= read -r f; do
    base="$(basename "$f" .md)"
    FEATURES[$base]="$f"
  done < <(find "$FEATURE_DIR" -maxdepth 1 -name '*.md' -type f 2>/dev/null)
fi

FEATURES_COUNT=${#FEATURES[@]}

for name in "${!FEATURES[@]}"; do
  content=$(cat "${FEATURES[$name]}" 2>/dev/null || true)
  [[ -z "$content" ]] && continue

  REFS_FOUND=$((REFS_FOUND + 1))
  if echo "$content" | grep -q '^## Traceability'; then
    REFS_VALID=$((REFS_VALID + 1))
  else
    ORPHANS=$((ORPHANS + 1))
    EVIDENCE+=("Feature '$name' is missing a Traceability section")
  fi

  for section in "## Purpose" "## Scope"; do
    if ! echo "$content" | grep -qF "$section"; then
      ORPHANS=$((ORPHANS + 1))
      EVIDENCE+=("Feature '$name' is missing section: $section")
    fi
  done
done

if [[ "$FEATURES_COUNT" -eq 0 ]]; then
  STATUS="not_applicable"
  EVIDENCE=("No feature documents found in docs-root/feature/")
elif [[ "$ORPHANS" -gt 0 ]]; then
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
  "check": "feature-family-mapping",
  "domain": "_generic",
  "category": "C",
  "status": "$STATUS",
  "metrics": {
    "features_count": $FEATURES_COUNT,
    "refs_found": $REFS_FOUND,
    "refs_valid": $REFS_VALID,
    "orphans": $ORPHANS
  },
  "evidence": $EVIDENCE_JSON,
  "executed_at": "$EXECUTED_AT",
  "repo_fingerprint": "$FINGERPRINT"
}
ENDJSON
