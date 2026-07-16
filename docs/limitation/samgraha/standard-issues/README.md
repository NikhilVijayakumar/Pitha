# Samgraha Standard Issues — Data Fixes Needed in Standards Repo

These issues are with the documentation standards themselves (the rules, the check definitions, the section requirements). They need to be fixed in the Samgraha knowledge-hub repository (`E:\Python\Kriti\samgraha\system\`), not the MCP implementation.

---

## STD-008: Feature-Technical Standard Lacks 1:1 Mapping Requirement with Feature Documents

**Severity:** Medium
**Domain:** Feature-Technical
**Affects:** Feature-Technical documentation quality
**Location:** `system/rust_dev/documentation-standards/11-feature-technical-standards.md`

### Problem

The Feature-Technical standard does not require a 1:1 correspondence between feature documents and feature-technical documents. This allows feature-technical docs to be written without proper alignment to their corresponding feature doc.

### Evidence

- Before fix: Single `feature-technical/feature-technical.md` covering all features
- After fix: 10 individual feature-technical docs, each matching one feature doc
- Each feature doc must have exactly one corresponding feature-technical doc

### Fix Required

Add explicit mapping requirement to the Feature-Technical standard:

```markdown
## Mapping Requirement

Each feature document MUST have exactly one corresponding feature-technical document.
The feature-technical document name must match the feature document name with `-technical` suffix.

Example:
- Feature: `application-bootstrap.md`
- Feature-Technical: `application-bootstrap-technical.md`
```

---

## STD-009: Architecture Collection-Level Checks Fire on Individual Documents

**Severity:** High → **Downgraded, see correction**
**Domain:** Architecture
**Affects:** Audit score deflation (87.91 instead of ~95+)
**Location:** `system/rust_dev/documentation-standards/05-architecture.yaml` and `audit/deterministic/section/05-architecture/*.yaml`

### Problem

Architecture audit checks are defined at document scope but evaluate collection-level requirements. Every check fires against all 9 architecture documents instead of evaluating the collection as a whole. The standard states: "Architecture quality is evaluated across the complete documentation collection."

### Evidence

All 9 architecture documents have the 5 required sections plus optional sections. The 87.91 score (re-verified live 2026-07-16: 87.89, matches) is from 106+ inflated findings, not missing content.

### Fix Required

Update check definitions to use collection scope, or add a `scope: collection` flag to section-level checks.

### Correction (2026-07-16)

Score is confirmed accurate. But `get_standard(domain=architecture)` already shows a per-rule `scope` field — the standard-side config this fix targets may already exist and simply be ignored by the engine. `docs/limitation/samgraha/mcp-issues/README.md` MCP-001 (Critical) independently diagnosed the same symptom from the engine side: `providers.rs` iterates every document per-check with **no collection-scope code path at all**. Until MCP-001 is fixed, changing this standard's YAML `scope` value won't change engine behavior. Treat STD-009 as a downstream symptom of MCP-001, not an independently-fixable standard bug — don't schedule standard-repo work here before the engine fix lands.

---

## STD-010: Philosophy Checks Use Inverted Logic

**Severity:** High → **Unconfirmed, see correction**
**Domain:** Philosophy
**Affects:** Audit score deflation (84.21 instead of ~95+)
**Location:** `system/rust_dev/documentation-standards/02-philosophy.yaml` and `audit/deterministic/document/02-philosophy.yaml`

### Problem

Philosophy audit checks test for the *absence* of sections rather than their *presence*. The document has all 3 required sections (guiding_principles, values, tradeoffs) but checks report them as errors.

### Evidence

Knowledge DB confirms all required sections exist with correct semantic types and content. Checks `phil-001`, `phil-002`, `phil-003` report errors when sections are present.

### Fix Required

Verify check definitions test `exists: true` for required semantic types and correct any inverted conditions.

### Correction (2026-07-16)

Score (84.21) and section-presence facts are confirmed — `philosophy.md` read directly, all 3 sections present with real content. But the "inverted logic" diagnosis does not hold up: pulled the actual rule set via `get_standard(domain=philosophy, version=v1)`. The real check IDs are `phil-doc-001`, `phil-sec-gp-001`, `phil-sec-to-001` (not `phil-001/002/003` as stated here — those IDs don't exist in the standard). Each is a plain `section_presence` evidence-type check with a correctly-mapped `semantic_type` (`guiding_principles`→Principles, `values`→Values, `tradeoffs`→Trade-offs) — nothing inverted in the rule definition. The simpler explanation, already documented as **CC-001** (`docs/error_list/05-cross-cutting-issues.md`) and **MCP-001**: the engine's `findings` array includes one entry per evaluated check regardless of pass/fail, with no `status` field, so a passing check and a failing check produce visually identical "positive-sounding" finding text. Recommend re-labeling STD-010 as **not a standard-data bug** — it's the same MCP-001 output-ambiguity issue, not literal inverted check logic. Needs someone with access to the engine's actual pass/fail evaluation (not just the rule definition) to confirm either way before any standard-repo fix is attempted.

---

## STD-011: Engineering Collection-Level Checks Fire on Individual Documents

**Severity:** High → **Downgraded, see correction**
**Domain:** Engineering
**Affects:** Audit score deflation (72.20 instead of ~95+)
**Location:** `system/rust_dev/documentation-standards/07-engineering.yaml` and `audit/deterministic/section/07-engineering/*.yaml`

### Problem

Engineering audit checks are defined at document scope but evaluate collection-level requirements. Every check fires against all 6 engineering documents, producing 106 findings for documents that all have the required sections. The standard states: "Engineering quality is evaluated across the complete Engineering Documentation collection."

### Evidence

All 6 engineering documents have the 4 required sections plus optional sections. Content is detailed and project-specific. The 72.20 score (re-verified live 2026-07-16, exact match) is entirely from inflated findings.

### Fix Required

Update check definitions to use collection scope, or add a `scope: collection` flag to section-level checks.

### Correction (2026-07-16)

Score confirmed accurate, and same MCP-001 dependency as STD-009 applies (see above) — this is a downstream symptom, not independently fixable via standard YAML alone until the engine gains a collection-scope code path.

**Also: "all 6 documents have the 4 required sections" is not fully accurate.** `build-standards.md` has no literal `## Build Standards` heading — it's organized under `## Build Lifecycle`, `## Build Stages`, `## Standard Toolchain`, `## Quality Gates` instead. Confirmed via `compile`, which emitted a real `MissingSection` diagnostic (`type: build_standards`) for this file — this is a distinct, genuine content gap (real missing canonical heading), separate from the collection-scope noise this entry describes. Don't let the STD-011 fix suppress this one.

---

## Summary

| ID | Issue | Severity | Domain | Fix Location |
|----|-------|----------|--------|-------------|
| STD-008 | Feature-technical lacks 1:1 mapping requirement | Medium | feature-technical | `11-feature-technical-standards.md` |
| STD-009 | Architecture collection-level checks fire on all docs | High | architecture | `05-architecture.yaml` |
| STD-010 | Philosophy checks use inverted logic | High | philosophy | `02-philosophy.yaml` |
| STD-011 | Engineering collection-level checks fire on all docs | High | engineering | `07-engineering.yaml` |

**Total Standard issues: 4**
**Critical: 0, High: 3, Medium: 1, Low: 0**

---

## Notes

### Previously Resolved Issues

The following issues were identified, fixed, and removed:

- **STD-001**: Architecture collection-level checks — fixed by restructuring checks
- **STD-002**: Implementation optional sections — fixed by adding optional flag
- **STD-003**: Implementation upstream requirements — fixed by making dependencies optional
- **STD-004**: Vision literal keyword matching — fixed by adding word-boundary matching
- **STD-005**: External-context not applicable — resolved by removing domain (not needed for this project)
- **STD-006**: Design not needed — resolved by removing domain (not needed for this phase)
- **STD-007**: Feature-design not needed — resolved by removing domain (not needed for this phase)
