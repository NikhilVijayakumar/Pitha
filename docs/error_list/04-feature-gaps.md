# Feature Document Gaps

**Files:** `docs/raw/feature/*.md` (10 files)

**Note:** Replaces a prior v1 pass whose two headline "systemic, critical" findings are both false:
- F-SYS-001 claimed **all 10 files are missing `## Constraints`**. Verified false — grepped headings in all 10 files; every one has `## Constraints`.
- F-SYS-002 claimed **all 10 files use "Future Evolution" instead of "Future Extensions"**. Verified false — every file uses `## Future Extensions` already.

These two false findings accounted for 20 of v1's 38 claimed gaps. Findings below are re-derived from grepping actual headings in all 10 files plus reading 3 files in full (`logging.md`, `error-handling.md`, `configuration.md`).

---

## F-001 [Moderate] No file has an explicit Inputs/Outputs section

**Files:** all 10 feature docs (confirmed by heading grep across all 10)

**Problem:** The feature standard's optional `Inputs`/`Outputs` sections (`feat-sec-inputs-001`, `feat-sec-outputs-001`) aren't present under those names in any file. Docs instead use `## Dependencies` with `### Depends On` / `### Consumed By` subsections, which cover crate/feature-level dependencies but not necessarily data inputs/outputs (e.g. what data a feature accepts vs. what it produces).

**Fix:** Low priority since `Dependencies` already covers most of the traceability need. If stricter input/output data-contract documentation is wanted, add `## Inputs` / `## Outputs` subsections under each feature's `Capabilities` or `Dependencies` section rather than a whole new top-level section — avoid duplicating `Depends On`/`Consumed By`.

---

## F-002 [Low, spot-confirmed] Some Acceptance Criteria skip the "when" clause

**File:** `configuration.md`, lines 193–195 (AC-005), possibly others

**Problem:** Most ACs follow strict Given/When/Then. AC-005 reads "Given configuration validation failure, then the application does not start" — no explicit "when" clause. Live audit flags 7 of 10 files (`application-bootstrap`, `cli-runtime`, `configuration`, `lifecycle-management`, `mcp-runtime`, `state-management`, `storage-abstraction`) for `feat-sec-ac-002` ("Acceptance criteria are testable"); `error-handling.md`, `logging.md`, `testing-infrastructure.md` are correctly not flagged since their ACs are consistently Given/When/Then (verified: both `logging.md` and `error-handling.md` ACs follow the pattern with no exceptions).

**Fix:** Spot-check the 7 flagged files for ACs missing a "when" clause and normalize to Given/When/Then. Only `configuration.md` was directly verified this session — the other 6 weren't individually re-read, so treat them as likely-but-unconfirmed until checked.

---

## F-003 [Not a gap — false positive demonstrated] "Purpose"/"Non-Goals" raw findings are unreliable for this domain

Live audit flags `feat-sec-purpose-002` (Error, "Purpose states feature intent") and `feat-sec-ng-001/002` (Warning, "Non-goals section exists"/"explicitly stated") on **all 10 files**, including `logging.md` and `error-handling.md`, both of which were read in full and have clear, well-written Purpose and Non-Goals sections. This is tool noise, not a content gap — see CC-001 in [05-cross-cutting-issues.md](05-cross-cutting-issues.md). Don't action these two checks without a manual read of the specific file first.
