# Cross-Cutting Issues

Issues spanning multiple documents/domains, plus tooling/process gaps discovered while running this audit.

---

## CC-000 [Critical] Prior v1 error_list contains fabricated findings

**Scope:** This directory, prior version.

**Problem:** The pre-existing `docs/error_list/*.md` (written before this session) made specific factual claims about file contents that don't match the actual files:
- Claimed `vision.md` uses `# Vision`/`# Problem` (H1) — actual: `## Vision`/`## Problem` (H2).
- Claimed `vision.md` was missing "Guiding Principles" — actual: present, correctly named, line 147.
- Claimed `engineering-principles.md` used heading `# Core Principles` — actual: `## Engineering Principles`, line 13.
- Claimed **all 10** feature docs were missing `## Constraints` — actual: all 10 have it.
- Claimed **all 10** feature docs used "Future Evolution" instead of "Future Extensions" — actual: all 10 already use "Future Extensions".
- Claimed 7 of 9 architecture files were clean "PASS" with only `system-overview.md` needing work — actual: live audit + direct reads show real duplicate-section defects in `component-model.md` and `data-flow.md` that v1 missed entirely.

**Impact:** Anyone acting on v1 would have made incorrect edits (e.g., renaming headings that were already correct) and missed the real defects. Root cause unknown — likely written without reading the files, or against a stale/different copy.

**Resolution:** v1 findings are superseded by the corrected 01–04 files in this directory, which were checked against the current file contents directly.

---

## CC-001 [High] `samgraha` audit()'s raw findings list can't be trusted without cross-checking file content

**Scope:** All domains, live `mcp__samgraha__audit` calls this session.

**Problem:** The `findings` array returned by `audit()` includes an entry for essentially every check the deterministic provider evaluates, whether it passed or failed — there's no `status`/`pass` field, and message text ("Purpose section exists", "Non-goals are explicitly stated") reads the same regardless of outcome. Verified false-positive: `feature/logging.md` and `feature/error-handling.md` are both flagged for weak Purpose and Non-Goals sections, but both were read in full and have clear, complete sections satisfying exactly what the check names describe.

**Impact:** An agent (or person) consuming `audit()` output programmatically and treating every entry as an actionable defect will over-report gaps that don't exist — this is very likely what produced the original v1 error_list's inflated/wrong findings, and it's what this session avoided only by manually re-reading files instead of trusting the tool output at face value.

**Recommendation:** Add an explicit `status: pass|fail` (or omit passing checks from `findings` entirely) to `audit()`'s response so severity reflects an actual failure, not a check's configured severity-if-it-were-to-fail.

---

## CC-002 [Moderate] `score.overall`/`rating` is disconnected from `score.categories`

**Scope:** All 4 domain audits this session.

**Problem:** Every `audit()` call returned `"overall": 100, "rating": "Excellent"` in the `score` object regardless of the actual category score — architecture scored 71.3%, engineering 56.9%, feature 78%, vision 100% (the true value in `categories`). The `overall`/`rating` fields never moved off 100/Excellent even when `documents_passed: 0`.

**Impact:** A reader who looks only at the top-line `overall`/`rating` (as the pre-existing `docs/report/*/scorecard/latest.md` files do — see CC-003) sees "100% Excellent" for a domain that's actually at 57%. This is the same failure mode that let the stale scorecards go unnoticed.

**Recommendation:** Compute `overall` from `categories` (e.g. mean or worst-category), or drop the field if it isn't meant to be domain-specific.

---

## CC-003 [Critical] Repository was never registered — all pre-existing scorecards are computed against zero documents

**Scope:** `docs/report/{vision,architecture,engineering,feature}/scorecard/latest.{md,json}` and their `history/` entries.

**Problem:** `mcp__samgraha__list_repositories` returned empty at the start of this session — "Pitha" was never registered despite `samgraha.toml` and prior scorecard output existing. Re-running the same 4 domain audits against an unregistered repo returns `documents_checked: 0` for every domain yet still reports `"overall": 100, "rating": "Excellent"` (compounding CC-002). This means the existing `latest.md` scorecards (all showing 100%, empty Findings tables, timestamps `1784148590`–`1784183284`) were never real — they audited nothing.

**Fix applied this session:** Ran `compile` (produced `.samgraha/manifest.json`) then `register_repository` with that manifest. Live audits after registration correctly report `documents_checked: 1/9/6/10` per domain with real category scores.

**Recommendation:** Anyone relying on `docs/report/*/scorecard/latest.md` going forward should re-run the audit after confirming `list_repositories` shows "Pitha" registered — otherwise the report is stale/meaningless. Consider having `compile`/`audit` fail loudly (not silently return 0-document "Excellent" results) when the target repo isn't registered.

---

## CC-004 [Moderate] Engineering & Architecture standards check required sections per-document; docs are organized per-topic

Covered in detail in [03-engineering-gaps.md](03-engineering-gaps.md) (E-002) and [02-architecture-gaps.md](02-architecture-gaps.md) (A-002). Flagged here because it's the single biggest driver of both domains' depressed scores (56.9% engineering, 71.3% architecture) and is a standard-configuration issue, not a content-quality one.

---

## CC-005 [Low] `proposal.md` in `docs/raw/` isn't a recognized document type

**File:** `docs/proposal.md` (not under `docs/raw/`, so it's actually outside the audited root — re-checked this session: `samgraha.toml`'s `root_dir = "docs/raw"` doesn't include it). Original v1 claim that it's inside `docs/raw/` and being compiled as `generic` was not reproducible — the file lives at `docs/proposal.md`, one level up. No action needed; not in scope of the documentation audit.
