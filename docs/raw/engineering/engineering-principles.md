# Pīṭha — Engineering Principles

## Purpose

Engineering Principles define the implementation philosophy used throughout the Pīṭha platform.

While the Vision defines the platform goals and the Architecture defines the system structure, Engineering Principles establish the rules engineers follow when designing, implementing, reviewing, and evolving platform code.

These principles guide implementation decisions while remaining independent of individual features.

---

## Engineering Principles

### Simplicity First

Prefer the simplest solution that satisfies the architectural requirements.

Complexity must be justified by measurable engineering value.

---

### Explicit over Implicit

Platform behavior should always be explicit.

Avoid hidden dependencies, implicit initialization, global state, and surprising behavior.

---

### Composition over Duplication

Before introducing new infrastructure, evaluate whether an existing platform capability can be composed.

Duplicate engineering logic should be extracted into reusable platform components.

---

### Strong Typing

Leverage Rust's type system to encode invariants whenever practical.

Compile-time validation is preferred over runtime validation.

---

### Fail Early

Configuration errors, invalid states, and contract violations should be detected as early as possible.

Prefer compile-time failures over runtime failures.

---

### Deterministic Behavior

Platform behavior should be deterministic.

The same inputs should produce the same outputs whenever practical.

---

### Minimal Public APIs

Only expose APIs intended for external consumption.

Everything else should remain private.

---

### Stable Contracts

Public interfaces should evolve conservatively.

Breaking changes require architectural review.

---

### Consistent Error Handling

Errors should be strongly typed, contextual, and recoverable whenever appropriate.

Panics should represent programming errors rather than expected execution paths.

---

### Observable Systems

Platform capabilities should expose sufficient logging and diagnostics to understand application behavior without modifying source code.

---

### Testable Design

Every public capability should be testable in isolation.

Testing should be considered during design rather than after implementation.

---

### Evolution through Extension

Platform evolution should occur by introducing new implementations rather than modifying established abstractions.

---

## Decision Hierarchy

When engineering decisions conflict, the following precedence applies.

```text
Vision

↓

Constraints

↓

Architecture

↓

Engineering Principles

↓

Implementation
```

Engineering decisions shall never contradict higher-level documentation.

---

## Engineering Characteristics

The engineering philosophy emphasizes:

- Simplicity
- Explicitness
- Determinism
- Strong typing
- Maintainability
- Testability
- Extensibility
- Long-term stability

---

## Constraints

### Performance
- [C-001] Engineering decisions must be justifiable through the documented decision hierarchy. (source: Architecture)

### Security
- [C-002] Platform capabilities must remain safe by default without requiring security-conscious consumers. (source: Security Architecture)

### Compliance
- [C-003] Engineering evidence must be machine-readable to support automated quality gates. (source: External Context)

---

## Technology Selection

Technology Selection defines the approved technologies used throughout the Pīṭha platform, including selection rationale, alternatives considered, and the governance model for technology adoption. It covers the programming language (Rust), async runtime (Tokio), serialization (Serde), persistence (SQLx/SQLite), observability (Tracing), error handling (thiserror/anyhow), and testing tools (Proptest).

Technology Selection derives directly from Engineering Principles, translating the platform's preference for strong typing, compile-time validation, and deterministic behavior into concrete technology choices. It constrains all downstream engineering standards by establishing which libraries and frameworks are approved for use.

See [Technology Selection](technology-selection.md) for the complete technology rationale, approved technologies, and governance model.

---

## Build Standards

Build Standards define the build lifecycle, quality requirements, and engineering evidence production for the Pīṭha platform. They establish the versioning strategy, dependency policy, standard toolchain (Cargo, Clippy, rustfmt), and quality gates that every platform crate must satisfy.

Build Standards translate Engineering Principles like deterministic behavior, fail early, and observable systems into concrete build stages (validation, quality analysis, testing, packaging). The build lifecycle enforces that every successful build produces verifiable engineering evidence.

See [Build Standards](build-standards.md) for the complete build lifecycle, versioning strategy, dependency policy, and quality gates.

---

## Testing Standards

Testing Standards define the repository-wide testing strategy, test types, and coverage expectations for the Pīṭha platform. They establish unit testing, integration testing, end-to-end testing, property-based testing, and benchmarking as complementary verification levels, along with the testing toolchain (Cargo Test, Proptest, Mockall, cargo-tarpaulin, Criterion).

Testing Standards operationalize the Testable Design and Deterministic Behavior principles by requiring standardized engineering evidence production. The testing strategy ensures every platform capability is verifiable in isolation and in combination.

See [Testing Standards](testing-standards.md) for the complete testing strategy, test organization, coverage expectations, and engineering evidence requirements.

---

## Traceability

Engineering Principles derive from the Vision, Constraints, and Architecture.

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
        ├──► Code Standards
        ├──► Build Standards
        ├──► Testing Standards
        ├──► Repository Structure
        └──► Technology Selection
```

Engineering Principles define **how engineers make implementation decisions**.

They constrain all engineering standards while remaining independent of specific technologies and features.

### Upstream Sources

- Architecture — system-wide design decisions and component boundaries
- Engineering Principles — implementation philosophy and decision hierarchy
- Technology Selection — approved technologies and governance model (where applicable)
- Constraints — non-functional requirements and engineering limitations

### Downstream Consumers

- Implementation — derives coding, building, and testing practices from this document
- Feature Technical Design — references engineering standards for technology conformance

**Non-contradiction rule:** Engineering standards shall not introduce implementation practices that violate the Engineering Principles.