# Structural Optimizations

Prioritized recommendations from the verified audit (supersedes prior v1 list, which was built on several false premises — see CC-000).

---

| Priority | Item | Files | Effort | Fixes |
|----------|------|-------|--------|-------|
| 1 | Register the repo with samgraha as part of the normal workflow, and treat `documents_checked: 0` as a hard failure, not a passing 100% | tooling | Low | CC-003 — restores real audit results for the whole project |
| 2 | Add `status: pass/fail` to `audit()` findings; stop emitting one entry per check regardless of outcome | samgraha tool | Medium | CC-001 — makes the raw findings usable without manual cross-checking |
| 3 | Fix `score.overall`/`rating` to derive from `score.categories` instead of always showing 100/Excellent | samgraha tool | Low | CC-002 |
| 4 | Reconfigure engineering & architecture standards to check required-section coverage at domain level, not per-document | standard config | Medium | E-002, A-002 — single biggest driver of the 56.9%/71.3% category scores |
| 5 | Rename colliding `### Purpose` subheadings in `component-model.md` (5×) and `data-flow.md` (4×) | 2 files | Low | A-001 |
| 6 | Rename `## Vision` → `## Vision Statement` in `vision.md` | 1 file | Low | V-001 |
| 7 | Nest the 14 principle headings under `## Engineering Principles` in `engineering-principles.md` (currently empty per compiler) | 1 file | Low | E-001 |
| 8 | Decide whether `arch-doc-rust-001` ("Crate Architecture required") should scope to `crate-architecture.md` only, then fix heading/config accordingly | 1 file or standard config | Low–Medium | A-002 |
| 9 | Decide whether the "technology-independent" rule should exempt `trait-design.md`/`crate-architecture.md` as the intentionally Rust-specific docs | standard config | Low | A-003 |
| 10 | Spot-check the 6 unverified feature files (`application-bootstrap`, `cli-runtime`, `lifecycle-management`, `mcp-runtime`, `state-management`, `storage-abstraction`) for ACs missing a "when" clause, following the pattern found in `configuration.md` AC-005 | 6 files | Low | F-002 |

---

## Not recommended

- Don't act on v1's SO-003/SO-004/SO-005/SO-006 (add Constraints to features, rename "Future Evolution", rename "Core Principles", rename "Design Philosophy") — all target headings/sections that already exist correctly. Re-verify against the current file before touching any heading this list doesn't call out.
- Don't strip Rust-specific content from `trait-design.md` to satisfy "technology-independent" checks until item 9 above is resolved — it may be by design.
