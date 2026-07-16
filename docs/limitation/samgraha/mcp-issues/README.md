# Samgraha MCP Issues — Functionality Fixes Needed in MCP Repo

These issues are with the Samgraha MCP tool itself (the binary, the audit engine, the compile system, the project plan system). They need to be fixed in the Samgraha MCP repository (`E:\Python\samgraha\`).

---

## MCP-001: Check-Scope False Positives

**Severity:** Critical
**Affects:** All domains (Architecture, Engineering, Implementation, Vision)
**Total False Positives:** ~163 across all domains
**Location:** `crates/audit/src/providers.rs` — `DeterministicAuditProvider::check_rule()`

### Problem

The deterministic audit engine evaluates section-presence checks at **document scope against every document** in a domain. A check requiring "Communication" section fires on `trait-design.md` (which should not contain communication paths) and records an Error finding.

### Root Cause

In `providers.rs:92-131`, the `section_presence` check iterates over ALL documents in the domain and creates a finding for every document that doesn't have the section:

```rust
"section_presence" => {
    let section_key = rule.scope.to_lowercase()...;
    documents
        .par_iter()
        .filter(|doc| {
            let count = doc.quality.per_type.get(&section_key)...;
            if count > 0 { return false; }  // Has section, skip
            // Doesn't have section — create finding
        })
        .map(|doc| AuditFinding { ... })
        .collect()
}
```

This creates findings for every document that doesn't have the section, even when the section exists in the correct document.

### Evidence

| Domain | Check | False Positives | Example |
|--------|-------|----------------|---------|
| Architecture | `arch-sec-comm-001` | 8 | Fires on all 9 docs, only `communication.md` has it |
| Architecture | `arch-doc-rust-001` | 8 | Fires on all 9 docs, only `crate-architecture.md` has it |
| Engineering | 20 section checks | 120+ | Each check fires on all 6 docs |
| Implementation | 3 optional section checks | 8 | Each check fires on both docs |
| Vision | 3 checks | 3 | Fires on all docs |

### Expected Behavior

Section-presence checks should evaluate at **collection scope** (does the section exist somewhere in the domain?) rather than **document scope** (does this specific document have the section?).

### Fix Required

Change the audit engine to evaluate section-presence checks at collection scope:

```rust
"section_presence" => {
    let section_key = rule.scope...;
    let has_anywhere = documents.iter().any(|doc| {
        doc.quality.per_type.get(&section_key).copied().unwrap_or(0) > 0
    });
    if !has_anywhere {
        // Section missing from entire domain — create one finding
        vec![finding(rule.id, "Missing section in domain")]
    } else {
        vec![]  // Section exists somewhere — pass
    }
}
```

---

## MCP-002: No Prerequisite Checking in Project Plan

**Severity:** Medium
**Affects:** Project plan system
**Location:** `crates/services/` — project plan execution

### Problem

The project plan system generates phased workflows without verifying that required upstream documentation exists. Phase 4 (Audit implementation + security) proceeds even if Feature Technical and Security docs don't exist.

### Evidence

- Project plan has 8 phases with sequential dependencies
- Phase 1 (Generate docs) doesn't create all required upstream docs
- Phase 4 (Audit implementation) assumes Feature Technical and Security docs exist
- Phase 8 (Final verify) assumes all domains can pass

### Expected Behavior

Each phase should verify that its required inputs exist before proceeding. If prerequisites are missing, the phase should either:
1. Generate the missing docs
2. Skip with a warning
3. Fail with a clear error message

### Fix Required

Add prerequisite checking to the project plan system. Each phase should declare its required inputs and verify they exist.

---

## MCP-003: No Score Threshold Configuration

**Severity:** Low
**Affects:** Project plan system
**Location:** `crates/services/` — project plan execution

### Problem

There is no way to configure acceptable score thresholds per domain. The project plan assumes all domains can reach 90, but some domains have structural limitations that prevent this.

### Evidence

- Architecture max achievable: ~88-89 (check-scope false positives)
- Engineering max achievable: ~88-91 (using `rust_dev` profile)
- Implementation max achievable: ~72-75 (check-scope false positives)
- Vision max achievable: ~90-91 (check-scope false positives)

### Expected Behavior

Users should be able to configure per-domain score thresholds in `samgraha.toml`:

```toml
[audit.gates]
architecture = 85
engineering = 85
implementation = 70
```

### Fix Required

Add score threshold configuration to `samgraha.toml` and the project plan system.

---

## MCP-004: No Graceful Degradation

**Severity:** Medium
**Affects:** Project plan system
**Location:** `crates/services/` — project plan execution

### Problem

When a domain cannot reach the target score, the workflow fails instead of documenting the limitation and proceeding.

### Evidence

- 3 of 5 audited domains fail (Architecture 87.91, Engineering 72.08, Implementation 71.82)
- Final verify phase cannot pass because multiple domains fail
- No mechanism to document limitations and proceed

### Expected Behavior

When a domain cannot reach target:
1. Document the limitation
2. Record the maximum achievable score
3. Proceed to the next phase with a warning
4. Include limitations in the final report

### Fix Required

Add graceful degradation to the project plan system. Allow domains to fail with documented limitations.

---

## MCP-005: Literal Keyword Matching

**Severity:** Low
**Affects:** All domains
**Location:** `crates/audit/src/providers.rs` — keyword matching logic

### Problem

The audit engine uses exact substring matching for keywords. "aspires" does not satisfy "aspiration". "pipelines" triggers "pip" absence check.

### Evidence

- Vision: "aspires" (verb) didn't satisfy "aspiration" (noun) keyword
- Vision: "pipelines" contained "pip" as substring, triggering absence check

### Expected Behavior

Keyword matching should use word-boundary matching, not substring matching. "aspires" should satisfy "aspiration" check. "pipelines" should not trigger "pip" check.

### Fix Required

Change keyword matching from substring to word-boundary matching:

```rust
// Instead of:
body_lower.contains(&keyword)

// Use:
body_lower.split_whitespace().any(|w| w == keyword)
// Or regex: regex::Regex::new(&format!(r"\b{}\b", keyword))
```

---

## MCP-006: Philosophy Domain Missing from Pipeline Registry

**Severity:** Medium
**Affects:** Philosophy domain (and any caller relying on pipeline-level scores)
**Location:** Pipeline registry (wherever `vision`/`architecture`/`engineering`/`feature`/`security`/`feature-technical` pipelines are registered)

### Problem

`samgraha.toml` lists `philosophy` as one of the 7 in-scope documentation domains, and a Philosophy standard is registered (31 rules, 6 sections — same shape as the other 6, confirmed via `list_standards`). But the pipeline registry has no `philosophy` entry. `audit(pipeline="philosophy")` errors with "Unknown pipeline 'philosophy'", while `audit(pipeline="vision"|"architecture"|"engineering"|"feature"|"security"|"feature-technical")` all work. This is a binary/registry gap, not a standards-data gap — the standard exists, the pipeline wiring to run it structurally doesn't.

### Evidence

- `list_standards` → `philosophy` present, 31 rules, 6 sections.
- `audit(domain="philosophy", providers=["deterministic"])` → works, live score 84.21%.
- `audit(pipeline="philosophy")` → `{"error": "Unknown pipeline 'philosophy'. Valid values: doc, build, security, consistency, coverage, architecture, dependency, documentation-structure, vision, design, readme, prototype, external-context, engineering, feature, feature-technical, feature-design, deterministic-runtime, external-context-ownership, implementation, help"}`.
- `docs/proposal.md` reported philosophy 100.0/COMPLETE — traced to a stale cached `get_summary_report` (all 7 domains written same ~1s batch, no live pipeline run backing it, since none exists).

### Expected Behavior

Every domain listed in `samgraha.toml`'s `domain` array should have a matching pipeline, so structural/category-level scoring is available consistently across all 7 domains — not just deterministic-provider section checks.

### Fix Required

Register a `philosophy` pipeline analogous to the existing `vision`/`feature` pipelines (category buckets + structural checks), or if philosophy is intentionally deterministic-only, have `get_summary_report`/`audit` surface that explicitly instead of silently falling back to a stale cached score.

---

## MCP-007: Semantic Review Never Executes

**Severity:** Critical
**Affects:** All domain audits
**Location:** `crates/audit/src/providers.rs` — semantic provider invocation

### Problem

All audit results return empty `semantic_review`. The LLM-based section/document review never runs. Only deterministic (structural) checks execute.

### Evidence

All 7 domain audits return:
```json
"semantic_review": {
    "instruction": "",
    "rubrics": {},
    "tasks": []
}
```

### Impact

- Audit scores only reflect structural presence, not content quality
- Writing guidance (tone, voice, structure) from standards is never checked
- Summary report returns `spec_score: null`, `standard_score: null`
- No section-level findings to identify which content needs improvement

### Fix Required

1. Verify semantic provider is invoked when `providers=["deterministic", "semantic"]`
2. Add error logging when semantic provider fails
3. Implement section-level rubric evaluation
4. Populate `semantic_review.tasks` with per-section findings
5. Update `get_summary_report` to incorporate semantic scores

### Independently verified (2026-07-16)

Re-ran on vision with all three provider combos (default/unset, `["deterministic"]`, `["deterministic","semantic"]`) — `tasks: []` every time regardless of flag. Also checked the downstream half of the pipeline this breaks:
- `get_audit_report(domain="vision", stage="section")` → `{"report": null}`
- `get_audit_report(domain="vision", stage="document")` → `{"report": null}`
- `store_section_report`/`store_document_report` exist and accept a `SemanticReport` JSON object, but since `audit()` never emits any `tasks` to judge, there is nothing for an agent to construct a report from — the intended agent-in-the-loop flow (audit emits rubric tasks → agent judges → agent stores report → summary rolls it up) is dead at step 1 for every domain, not just vision. This is why `get_summary_report`'s `standard_score`/`spec_score` were `null` for all 7 domains checked this session.

---

## Summary

| ID | Issue | Severity | Fix Location |
|----|-------|----------|-------------|
| MCP-001 | Check-scope false positives | Critical | `crates/audit/src/providers.rs` |
| MCP-002 | No prerequisite checking | Medium | `crates/services/` |
| MCP-003 | No score thresholds | Low | `crates/services/` |
| MCP-004 | No graceful degradation | Medium | `crates/services/` |
| MCP-005 | Literal keyword matching | Low | `crates/audit/src/providers.rs` |
| MCP-006 | Philosophy domain missing from pipeline registry | Medium | Pipeline registry |
| MCP-007 | Semantic review never executes | Critical | `crates/audit/src/providers.rs` |

**Total MCP issues: 7**
**Critical: 2, High: 0, Medium: 3, Low: 2**

---

## Notes

### What Was Removed from Previous Version

The previous version incorrectly categorized these as MCP issues:

- **No check exclusion mechanism** — Actually a configuration issue. The `rust_dev` profile already excludes React/JS checks.
- **No project profiles** — Actually already exists. The knowledge repo has `rust_dev`, `react_dev`, `electron_dev`, `fastapi_dev`, `base_dev` profiles.
- **React/JS checks on Rust project** — Actually a configuration issue. Pīṭha should use the `rust_dev` profile.
- **Standards don't adapt to project type** — Actually already supported. Different profiles have different standards.

These are configuration issues that can be fixed by updating Pīṭha's `samgraha.toml` to use the `rust_dev` profile.
