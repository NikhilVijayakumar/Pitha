# Pīṭha — Build Standards

## Purpose

This document defines the build lifecycle and quality requirements for the Pīṭha platform.

The purpose of these standards is to ensure that every platform crate is built, validated, tested, and packaged using a consistent engineering process.

Build standards are independent of CI/CD tooling and apply equally to local development and automated build pipelines.

---

## Build Principles

The build process follows several engineering principles.

- Every build must be deterministic.
- Every build must be reproducible.
- Every build must be fully automated.
- Build failures must stop the pipeline immediately.
- Every successful build produces verifiable engineering evidence.

---

## Build Lifecycle

Every build progresses through the following stages.

```text
Source

    │

Validation

    │

Quality Analysis

    │

Testing

    │

Packaging

    │

Artifact
```

Each stage must complete successfully before the next stage begins.

---

## Build Stages

### Stage 1 — Validation

Purpose

Verify that the source code is structurally correct.

Typical activities include:

- Dependency resolution
- Compilation checks
- Type validation
- Borrow checking

Failure at this stage prevents all subsequent stages.

---

### Stage 2 — Quality Analysis

Purpose

Verify that the code satisfies platform engineering standards.

Typical activities include:

- Static analysis
- Lint analysis
- Formatting validation
- Documentation validation

Quality violations block the build.

---

### Stage 3 — Testing

Purpose

Verify functional correctness.

Typical activities include:

- Unit testing
- Integration testing
- Documentation tests
- Property-based testing
- Platform test harness execution

Testing should produce standardized engineering evidence.

---

### Stage 4 — Packaging

Purpose

Produce distributable platform artifacts.

Typical activities include:

- Optimized compilation
- Binary generation
- Library packaging
- Artifact validation

Packaging occurs only after successful validation and testing.

---

## Standard Toolchain

The standard build toolchain consists of:

| Capability | Standard Tool |
|------------|---------------|
| Build System | Cargo |
| Compiler | rustc |
| Formatting | rustfmt |
| Static Analysis | Clippy |
| Testing | Cargo Test |
| Documentation | rustdoc |

Additional tooling may be introduced through Engineering approval.

---

## Quality Gates

Every build must satisfy the following quality gates.

### Validation successfully.
- Dependency resolution succeeds.
- Workspace integrity is maintained.

---

### Code Quality

- Formatting conforms to platform standards.
- Static analysis completes successfully.
- Lint violations are resolved.

Warnings should be treated as engineering issues.

---

### Testing pass.
- Test evidence is generated.
- No critical failures remain unresolved.

---

### Packaging

- Release artifacts are successfully produced.
- Packaging completes without build failures.

---

## Engineering Evidence

Every successful build should generate engineering evidence including:

- Build metadata
- Compilation results
- Static analysis results
- Test reports
- Coverage information (when applicable)
- Benchmark results (when applicable)
- Artifact metadata

Engineering evidence supports quality gates, auditing systems, and future engineering automation.

---

## Build Characteristics

The build process should exhibit the following characteristics.

- Deterministic
- Reproducible
- Automated
- Observable
- Incremental
- Platform-independent
- Auditable

---

## Continuous Integration

Continuous Integration systems should execute the complete build lifecycle.

Local development environments should be capable of executing the same workflow.

The build standard remains independent of any specific CI/CD platform.

---

## Future Evolution

The build process may evolve by introducing additional validation stages such as:

- Security scanning
- Dependency auditing
- License verification
- Performance regression testing
- Documentation quality analysis

Additional stages should extend the build lifecycle without changing its overall structure.

---

## Constraints

### Performance
- [C-001] Build pipelines must complete within 10 minutes for standard changes. (source: Architecture)

### Security
- [C-002] Build artifacts must not include secrets, credentials, or proprietary configuration. (source: Security Architecture)

### Compliance
- [C-003] Build evidence must be retained for 90 days to support audit requirements. (source: External Context)

---

## Engineering Principles

Engineering Principles define the implementation philosophy used throughout the Pīṭha platform, including deterministic behavior, fail early, observable systems, and stable contracts. They establish the decision hierarchy that governs all engineering choices.

Engineering Principles directly inform Build Standards by mandating deterministic and reproducible builds, fail-fast quality gates, and observable engineering evidence production. The build lifecycle stages enforce the principles of explicit over implicit and consistent error handling across the build pipeline.

See [Engineering Principles](engineering-principles.md) for the complete principles, decision hierarchy, and engineering characteristics.

---

## Technology Selection

Technology Selection defines the approved technologies used throughout the Pīṭha platform, including Rust, Cargo, Clippy, rustfmt, and the governance model for technology adoption. It establishes the standard build toolchain that this document builds upon.

Technology Selection directly determines the build toolchain approved in Build Standards. The standard toolchain (Cargo, rustc, rustfmt, Clippy, Cargo Test, rustdoc) flows from the technology choices established in Technology Selection. The dependency policy and versioning strategy must account for the approved technology stack.

See [Technology Selection](technology-selection.md) for the complete technology rationale, approved technologies, and governance model.

---

## Testing Standards

Testing Standards define the repository-wide testing strategy, test types, and coverage expectations for the Pīṭha platform. They establish unit testing, integration testing, end-to-end testing, property-based testing, and benchmarking as complementary verification levels.

Testing Standards constrain Build Standards by defining what the Testing stage (Stage 3) of the build lifecycle must verify. The test types, coverage expectations, and engineering evidence production requirements in Testing Standards determine which tests run during the build and what constitutes a passing build.

See [Testing Standards](testing-standards.md) for the complete testing strategy, test organization, coverage expectations, and engineering evidence requirements.

---

## Traceability

Build Standards derive from the Engineering Principles, Technology Selection, and Architecture.

```text
Vision
        │
        ▼
Constraints
        │
        ▼
Architecture
        │
        ▼
Engineering Principles
        │
        ├──► Technology Selection
        │
        ▼
Build Standards
        │
        ▼
Implementation
```

Engineering Principles define **how builds should behave**.

Technology Selection defines **which build technologies are approved**.

Build Standards define **how platform software is validated before release**.

Implementation realizes these standards through Cargo, CI/CD pipelines, and engineering tooling.

### Upstream Sources

- Architecture — system-wide design decisions and component boundaries
- Engineering Principles — implementation philosophy and decision hierarchy
- Technology Selection — approved technologies and governance model (where applicable)
- Constraints — non-functional requirements and engineering limitations

### Downstream Consumers

- Implementation — derives coding, building, and testing practices from this document
- Feature Technical Design — references engineering standards for technology conformance

**Non-contradiction rule:** Build pipelines shall not bypass mandatory validation stages, quality gates, or evidence generation defined in this document.