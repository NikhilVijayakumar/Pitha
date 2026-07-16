# Engineering Document Gaps

**Files:** `docs/raw/engineering/*.md` (6 files)

**Note:** Replaces a prior v1 pass that claimed `engineering-principles.md` uses a heading `# Core Principles` — the file actually uses `## Engineering Principles` (line 13, confirmed by direct read). That v1 finding (E-EP-001) is false and is dropped here.

---

## E-001 [Moderate] "Engineering Principles" section is empty — content lives under sibling subheadings instead

**File:** `engineering-principles.md`, line 13

**Problem:** `## Engineering Principles` (line 13) is immediately followed by `## Simplicity First` (line 15) with nothing in between. Confirmed by direct read and by `compile` diagnostics (`EmptySection`, type `guiding_principles`, Info). All 14 actual principles (Simplicity First, Explicit over Implicit, Composition over Duplication, Strong Typing, Fail Early, Deterministic Behavior, Minimal Public APIs, Stable Contracts, Consistent Error Handling, Observable Systems, Testable Design, Evolution through Extension) are written as same-level `##` siblings rather than nested under the "Engineering Principles" heading.

**Fix:** Either demote the 14 principle headings to `###` so they nest under `## Engineering Principles`, or add a short introductory paragraph directly under the `## Engineering Principles` heading before the first sibling section. The content itself is fine — this is purely a heading-nesting issue that makes the compiler see an empty section.

---

## E-002 [High] Standard requires every engineering doc to carry all 4 required sections; docs are organized one-topic-per-file

**Files:** all 6 engineering docs

**Problem:** The engineering standard's required sections (`Engineering Principles` / `guiding_principles`, `Technology Selection` / `rationale`, `Build Standards`, `Testing Standards`) are checked **per document**, but the actual docs are deliberately split by topic — `build-standards.md` only covers build, `testing-standards.md` only covers testing, etc. Confirmed via `compile` diagnostics: every one of the 6 files gets 3–4 `MissingSection` warnings for the topics owned by the *other* 5 files.

**Example:** `build-standards.md` gets flagged as missing "Engineering Principles", "Technology Selection", and "Testing Standards" — sections that correctly live in their own dedicated files instead.

**Impact:** This is the single largest source of noise in the engineering domain's audit score (56.9%) and is structural, not a content quality problem — the actual content is present, just spread across files as designed.

**Fix — pick one:**
1. **Reconfigure the standard** to check required-section coverage at the domain/corpus level (four sections must exist *somewhere* across the 6 files) rather than per-document. This matches how the docs are actually organized and is the lower-effort fix.
2. **Consolidate** the 6 topic files into one comprehensive `engineering.md` with all 4 sections, if the standard's per-document model is intentional and shouldn't change.

Recommend option 1 — splitting engineering concerns into separate topic files is reasonable documentation practice and shouldn't be penalized.

---

## E-003 [Info, no action] "Repository Principles" section in repository-structure.md

A prior v1 pass flagged this as possibly duplicating `engineering-principles.md`. Not independently re-verified this session; low priority, no live-audit signal supports it as a real defect. Leave as a manual follow-up if content overlap is suspected, not a confirmed gap.
