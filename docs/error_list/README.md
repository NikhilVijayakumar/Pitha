# Documentation Audit Report (v2 — Verified)

**Date:** 2026-07-16
**Scope:** All `docs/raw/` documents (51 docs across vision, architecture, engineering, feature)
**Method:** Live `samgraha` MCP audit (repo registered + compiled this session) cross-checked against direct file reads. Supersedes the prior v1 pass in this directory, which was manual and contains factually wrong claims (see CC-000 in [05-cross-cutting-issues.md](05-cross-cutting-issues.md)).

---

## What changed from v1

1. **The repo was never registered with samgraha.** `list_repositories` returned empty. Every prior scorecard under `docs/report/*/scorecard/latest.md` (all showing 100%, zero findings) was computed against **0 documents** — they are not real audit results. Fixed by running `compile` + `register_repository` this session.
2. **v1's manual claims don't match the actual files.** e.g. v1 claimed `vision.md` uses `# Vision` (H1) — the file actually uses `## Vision` (H2). Several v1 findings (SO-005, SO-006, CC-002 examples) describe headings that don't exist in the current files.
3. **The live audit tool's raw findings list is noisy and cannot be trusted at face value.** Every check the deterministic provider runs is emitted as a "finding" regardless of pass/fail, with no status field. Verified by spot-check: `feature/logging.md` and `feature/error-handling.md` were both flagged for weak Purpose/Non-Goals sections that are, on direct read, clearly present and well-written. See CC-001.
4. Findings below are only those **confirmed by direct file inspection**, not raw tool output.

---

## Summary (verified findings only)

| Domain | Files | Confirmed issues | Severity |
|--------|-------|-------------------|----------|
| Vision | 1 | 1 | Moderate (heading mismatch) |
| Architecture | 9 | 3 | 1 Moderate (duplicate sections), 1 Moderate (heading requirement), 1 Low-confidence |
| Engineering | 6 | 2 | 1 Moderate (empty section), 1 High (structural mismatch, affects all 6 files) |
| Feature | 10 | 2 | 1 Moderate (missing Inputs/Outputs, affects all 10), 1 Low (AC phrasing, spot-confirmed in 1 file) |
| Tooling/Process | — | 3 | 1 Critical (repo never registered), 1 High (ambiguous audit output), 1 Moderate (misleading score rollup) |

---

## File Index

| File | Covers |
|------|--------|
| [01-vision-gaps.md](01-vision-gaps.md) | Vision document gaps |
| [02-architecture-gaps.md](02-architecture-gaps.md) | Architecture document gaps |
| [03-engineering-gaps.md](03-engineering-gaps.md) | Engineering document gaps |
| [04-feature-gaps.md](04-feature-gaps.md) | Feature document gaps |
| [05-cross-cutting-issues.md](05-cross-cutting-issues.md) | Tooling gaps + issues spanning domains |
| [06-structural-optimizations.md](06-structural-optimizations.md) | Prioritized recommendations |

## Severity Legend

- **Critical:** Blocks reliable auditing or is factually broken
- **High:** Real defect affecting many files or misleading a reader/tool
- **Moderate:** Confirmed content/structure gap, single or few files
- **Low:** Minor, cosmetic, or single-instance
