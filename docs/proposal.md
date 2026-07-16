# Pitha — Project Plan

## Phase 1 — Documentation ✅ COMPLETE

Scope: 7 domains audited. Real scores from `bucket_scores`. See details below.

## Phase 2 — Implementation

Scope: Build Cargo workspace with 8 crates across 5 components. See `docs/phase2-implementation.md`.

| Phase | Component | Crates | Status |
|-------|-----------|--------|--------|
| 2.1 | Core | pitha-core | ⬜ Pending |
| 2.2 | Foundation | pitha-foundation | ⬜ Pending |
| 2.3 | Storage | pitha-storage-core, pitha-storage-fs | ⬜ Pending |
| 2.4 | Testing | pitha-testing | ⬜ Pending |
| 2.5 | CLI Runtime | pitha-runtime-cli | ⬜ Pending |
| 2.6 | MCP Runtime | pitha-runtime-mcp | ⬜ Pending |
| 2.7 | Integration | workspace, CI | ⬜ Pending |

---

# Phase 1 — Documentation Details

All 7 domains audited. Real audit scores from `bucket_scores` (not the misleading `overall: 100.0`). Domains below 90 are affected by check definition bugs documented in `docs/limitation/samgraha/standard-issues/`.

| # | Domain | Tier | Type | Required Sections | Real Score | Status |
|---|--------|------|------|-------------------|------------|--------|
| 1 | vision | 1 | audit existing | 5 | 90.49 | ✅ ≥90 |
| 2 | architecture | 2 | audit existing | 5 | 87.91 | ⚠️ STD-009 false positives |
| 3 | engineering | 2 | audit existing | 4 | 72.20 | ⚠️ STD-011 false positives |
| 4 | feature | 2 | audit existing | 3 | 90.77 | ✅ ≥90 |
| 5 | philosophy | 1 | generate new | 3 | 84.21 | ⚠️ STD-010 false positives |
| 6 | security | 2 | generate new | 3 | 91.01 | ✅ ≥90 |
| 7 | feature-technical | 3 | generate new | 4 | 64.61 | ⚠️ NEEDS_WORK — corrected 2026-07-16: prior "~96" was the *pipeline* score (structural/readiness check), not the `bucket_scores` domain-quality figure this table uses everywhere else. Live `bucket_scores.feature-technical_deterministic_section` = 64.61. |

### Score Notes

- **`overall: 100.0`** in audit output is misleading — it only checks section presence, not quality.
- **Real quality scores** come from `bucket_scores` (e.g., `architecture_deterministic_section`).
- **Architecture (87.91):** All 5 required sections present across 9 docs. Score deflated by collection-level checks (STD-009) firing 9× instead of 1×.
- **Philosophy (84.21):** All 3 required sections present (guiding_principles, values, tradeoffs). Score deflated by inverted check logic (STD-010).
- **Engineering (72.20):** All 4 required sections present across 6 docs. Score deflated by collection-level checks (STD-011) firing 6× instead of 1×, producing 106 false findings.

### Known Check Definition Issues

| ID | Domain | Issue | Impact |
|----|--------|-------|--------|
| STD-009 | architecture | Collection-level checks fire on all 9 docs | 87.91 instead of ~95+ |
| STD-010 | philosophy | Claimed: inverted logic tests absence instead of presence | 84.21 instead of ~95+ |
| STD-011 | engineering | Collection-level checks fire on all 6 docs | 72.20 instead of ~95+ |

**Re-verified 2026-07-16, corrections:**
- STD-009/STD-011's score numbers are confirmed live-accurate (87.89≈87.91, 72.20 exact). But their root cause is the *same* engine behavior already logged as **MCP-001** (Critical, `docs/limitation/samgraha/mcp-issues/`) — the deterministic engine iterates every document per-check with no collection-scope code path at all. STD-009/011's proposed fix ("add `scope: collection` to the standard YAML") won't take effect until MCP-001 is fixed engine-side; the standard-side `scope` field already exists per-rule (confirmed via `get_standard`) and is presumably already being ignored. Treat these as downstream symptoms of MCP-001, not independently fixable standard bugs.
- STD-010's "inverted logic" diagnosis is **not confirmed**. Pulled the actual rule definitions (`get_standard(domain=philosophy)`): `phil-doc-001`/`phil-sec-gp-001`/`phil-sec-to-001` are plain `section_presence` checks with correct `semantic_type` mappings matching the doc's real headings (Principles→guiding_principles, Values→values, Trade-offs→tradeoffs). Nothing inverted in the rule config. `philosophy.md` was re-read directly and does have all 3 sections with real content. The simpler, already-documented explanation is CC-001/MCP-001 (findings list emits one entry per evaluated check regardless of pass/fail, no `status` field) — not a logic inversion bug specific to philosophy.
- Engineering's "all sections present" is also not fully accurate: `build-standards.md` has no literal `## Build Standards` heading (it uses `## Build Lifecycle`/`## Build Stages`/`## Standard Toolchain` instead) — confirmed via `compile`, which emits a real `MissingSection` diagnostic (`type: build_standards`) for this file, distinct from the noisy per-check findings. This is a genuine content gap, not tool noise.

All three are documented in `docs/limitation/samgraha/standard-issues/`; STD-009/011's duplication with MCP-001 and STD-010's downgrade are noted there too.

## After Implementation (deferred)

| Domain | Tier | Required Sections | Reason |
|--------|------|-------------------|--------|
| qa | 6 | 4 | Needs implemented code to write accurate test docs |
| build | 7 | 3 | Needs actual build system to document accurately |
| readme | 8 | 12 | Needs complete project to write accurate readme |

## Removed From This Phase

| Domain | Reason |
|--------|--------|
| design | Not needed for this project |
| feature-design | Not needed for this project |
| prototype | Not needed for this project |
| external-context | No custom library dependencies in this repo |
| product-guide | Can be added later |
| implementation | Excluded in samgraha.toml domain_exclusion |

---

## Detailed Required Sections

### 1. vision (Tier 1) — 5 required
- Purpose
- Vision Statement
- Problem
- Solution
- Target Audience

Optional: Platform Pillars, Philosophy, Guiding Principles, Success Criteria, Traceability, Systems Vision

### 2. architecture (Tier 2) — 5 required
- System Overview
- Component Model
- Communication Paths
- Data Flow
- Security Considerations

Optional: Purpose, Rationale, Constraints, Traceability, Operational Readiness, Observability, Layered Architecture, Component Architecture, Crate Architecture, Trait Design

### 3. engineering (Tier 2) — 4 required
- Guiding Principles
- Rationale (Technology Selection)
- Build Standards
- Testing Standards

Optional: Purpose, Code Standards, Constraints, Traceability, Security Standards, Versioning, Async Patterns, Response Models, Migration Strategy, Hook Conventions, Component Patterns, State Management, Ownership Patterns, Unsafe Guidelines

### 4. feature (Tier 2) — 3 required
- Purpose
- Functional Requirements
- Acceptance Criteria

Optional: Business Rules, Constraints, Dependencies, Future Extensions, Inputs, Outputs, Non Goals, Traceability, Observability, Stakeholders, Success Criteria, Systems Feature Definition

### 5. philosophy (Tier 1) — 3 required
- Principles
- Values
- Trade-offs

Optional: Purpose, Backend Philosophy, Systems Philosophy

### 6. security (Tier 2) — 3 required
- Threat Model
- Data Classification
- Security Principles

Optional: Purpose, Compliance, Incident Response, Constraints, Traceability, API Auth Patterns, Systems Security

### 7. feature-technical (Tier 3) — 4 required
- Purpose
- Participating Components
- Component Interactions
- Data Ownership

Optional: Feature Specification, Component Responsibilities, Runtime Behavior, Communication Paths, Integration Points, External Dependencies, Runtime Constraints, Architectural Constraints, Security Considerations, Performance Considerations, Failure Handling, Extension Points, Traceability, Data Governance, Observability, Versioning, Layer Implementation, Crate Implementation, Error Implementation

---

## Summary

| Metric | Value |
|--------|-------|
| Total domains completed | 7/7 audited (not all passing) |
| Domains ≥90 threshold | 3/7 (vision 90.49, feature 90.77, security 91.01) |
| Domains <90 | 4/7 (architecture 87.91, engineering 72.20, philosophy 84.21, feature-technical 64.61) |
| All required sections present | 6/7 — engineering's `build-standards.md` genuinely missing `## Build Standards` heading (real `MissingSection`, not tool noise) |
| Average real quality score | 83.03 (recomputed after correcting feature-technical from ~96 to 64.61) |
| Total required sections | 27 |
| Total docs created/audited | 39 |
| Knowledge DB sections | 979 |
| Known standard issues | 4 (STD-008/009/010/011) — see corrections above; STD-009/011 likely duplicate MCP-001, STD-010 unconfirmed |
