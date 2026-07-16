# Architecture Document Gaps

**Files:** `docs/raw/architecture/*.md` (9 files)

**Note:** Replaces a prior v1 pass that marked 7 of 9 files "PASS, well-structured" without apparent inspection. The live audit (documents_checked: 9, 213 raw findings, category score 71.3%) and direct reads surface real structural defects the v1 pass missed. Not every raw audit finding is trustworthy (see CC-001) — only items below were confirmed by reading the actual files.

---

## A-001 [Moderate] Duplicate "Purpose" subsections collide with document-level Purpose

**Files:** `component-model.md`, `data-flow.md`

**Problem:** Both docs give each subsection (per component / per flow) its own `### Purpose` heading, in addition to the document's own top-level `## Purpose`:

- `component-model.md`: `## Purpose` (line 3) plus `### Purpose` under Core, Foundation, Runtime, Storage, and Testing components (lines 31, 70, 106, 151, 196) — 5 duplicates.
- `data-flow.md`: `## Purpose` (line 3) plus `### Purpose` under Application Bootstrap Flow, State Flow, Storage Flow, and Testing Flow (lines 41, 73, 107, 143) — 4 duplicates.

Confirmed via `compile` diagnostics (`DuplicateSection`, type `purpose`) and by grepping headings directly.

**Impact:** The compiler classifies section *type* independent of heading depth, so every `### Purpose` collides with the document's semantic `purpose` slot. This doesn't corrupt the prose, but it means only one "Purpose" per doc is queryable/auditable by type — the other 4-5 are silently dropped from that semantic bucket.

**Fix:** Rename subsection headings to avoid the collision, e.g. `### Component Purpose` / `### Flow Purpose`, or `### Role` — keeping the top-level `## Purpose` as the only section of type `purpose`.

---

## A-002 [Moderate, needs config check] "Crate Architecture" required-section check fires on every architecture file, including crate-architecture.md itself

**Files:** all 9 architecture docs

**Problem:** Live audit's `arch-doc-rust-001` ("Crate Architecture required") fails on all 9 files — including `crate-architecture.md`, which is clearly the document meant to satisfy it (confirmed: its content is entirely about Cargo workspace layout, lines 1–20+ read directly).

**Likely cause:** Either (a) the check expects a literal heading `## Crate Architecture` that the file doesn't use (it opens with `## Purpose` / `## Workspace Organization` instead — confirmed by direct read), or (b) the check is scoped to run per-file across the whole domain rather than per-document, the same corpus-vs-per-document mismatch described in engineering (see E-002 in [03-engineering-gaps.md](03-engineering-gaps.md)).

**Fix:** If (a): add a heading alias so "Workspace Organization" or the doc's actual structure maps to the `crate_architecture` semantic type, or add a `## Crate Architecture` heading. If (b): fix the standard config to check this once across the domain, not once per file (see [06-structural-optimizations.md](06-structural-optimizations.md)).

---

## A-003 [Low confidence] trait-design.md flagged as not "technology-independent"

**File:** `trait-design.md`

Live audit fails `arch-sec-*-005`-style "technology-independent" checks on this file only (8 checks). Direct read shows the doc does contain Rust-specific code (`Arc`, `Mutex`, `RwLock`, a `SqliteStorage` example, lines 197–217) and is explicitly framed as "Pīṭha uses Rust traits" (line 5).

**Assessment:** This is likely **intentional** — Trait Design and Crate Architecture are the two architecture docs that are inherently Rust/Cargo-specific by design (confirmed by `arch-doc-rust-001` treating them as a distinct "rust doc" category). The technology-independence rule is probably meant for the other 7 docs (System Overview, Component Model, Communication, Data Flow, Security, Rationale, Constraints) and shouldn't apply here. Flagging for a human decision rather than recommending a content change — don't strip the Rust specifics from trait-design.md without confirming the standard's intent first.
