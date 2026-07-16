# Pīṭha — Technology Selection

## Purpose

This document defines the approved technologies used throughout the Pīṭha platform.

Technology selections are based on the Engineering Principles, Architectural Constraints, and long-term maintainability goals established by the platform.

Approved technologies should be used consistently across all platform crates unless an architectural review explicitly approves an exception.

---

## Selection Principles

Technology choices follow several principles.

- Prefer mature and well-maintained libraries.
- Minimize external dependencies.
- Favor compile-time validation over runtime validation.
- Prefer ecosystem standards over niche alternatives.
- Choose technologies with long-term maintenance prospects.
- Avoid introducing overlapping libraries that solve the same problem.

---

## Technology Domains

### Programming Language

### Approved Technology

**Rust**

### Selection Rationale

The rationale for each technology choice considers the following alternatives considered and trade-offs evaluated.

Rust provides:

- Memory safety
- Thread safety
- Zero-cost abstractions
- Strong ownership semantics
- Excellent tooling
- Cargo workspace support

These capabilities align directly with the architectural goals of Pīṭha.

### Alternatives Considered

- Go
- C++
- Zig

### Decision

Rust is a mandatory platform dependency.

---

### Async Runtime

### Approved Technology

**Tokio**

### Selection Rationale

Tokio provides:

- Mature asynchronous runtime
- High ecosystem compatibility
- Work-stealing scheduler
- Structured task execution
- Blocking task isolation

### Alternatives Considered

- async-std
- smol

### Decision

Tokio is the standard asynchronous runtime.

---

### Serialization

### Approved Technology

**Serde**

### Selection Rationale

Serde provides:

- Compile-time type safety
- Excellent ecosystem support
- Multi-format serialization
- Minimal runtime overhead

### Alternatives Considered

- Manual serialization

### Decision

Serde is the standard serialization framework.

---

### Configuration

### Approved Technology

**TOML**

### Selection Rationale

TOML provides:

- Human readability
- Native Cargo compatibility
- Strong Rust ecosystem support

### Alternatives Considered

- YAML
- JSON

### Decision

Configuration files use TOML unless another format is required by an external system.

---

### Persistence

### Approved Technologies

- SQLx
- SQLite

### Selection Rationale

SQLite provides lightweight embedded persistence.

SQLx provides:

- Compile-time query validation
- Async integration
- Strong typing
- Minimal abstraction

### Alternatives Considered

- Diesel
- rusqlite
- SeaORM

### Decision

SQLite is the default structured storage implementation.

SQLx is the standard database library.

---

### Observability

### Approved Technology

**Tracing**

### Selection Rationale

Tracing provides:

- Structured logging
- Async context propagation
- Span-based diagnostics
- Rich ecosystem integration

### Alternatives Considered

- log
- slog

### Decision

Tracing is the platform logging standard.

---

### Error Handling

### Approved Technologies

- thiserror
- anyhow

### Selection Rationale

Platform crates expose structured errors through `thiserror`.

Applications use `anyhow` for ergonomic error propagation.

This separation preserves stable APIs while simplifying application development.

### Alternatives Considered

- eyre

### Decision

Platform and application error handling remain intentionally separated.

---

### Testing

### Approved Technologies

- Rust Test Framework
- Proptest

### Selection Rationale

The standard Rust testing framework provides unit and integration testing.

Proptest enables property-based testing for platform contracts and implementations.

Together they support deterministic engineering evidence.

### Alternatives Considered

- quickcheck

### Decision

These technologies form the baseline testing stack.

---

## Technology Selection

Technology adoption follows these governance rules.

### Default First

Approved technologies should be used by default.

### Justified Exceptions

Alternative technologies require documented engineering justification.

### One Technology per Capability

The platform should avoid multiple libraries solving the same engineering problem.

For example:

- One async runtime
- One logging framework
- One serialization framework
- One primary database library

---

### Long-Term Stability

Technology choices prioritize ecosystem maturity and long-term maintainability over short-term popularity.

---

## Future Evolution

Technology selections may evolve when:

- security concerns arise
- ecosystem support changes significantly
- superior alternatives emerge
- architectural requirements change

Technology replacement should preserve platform stability whenever practical.

---

## Constraints

### Performance
- [C-001] New technology adoption requires engineering review and documented rationale. (source: Architecture)

### Security
- [C-002] Approved technologies must support the platform's safety, performance, and determinism requirements. (source: Security Architecture)
### Compliance

- [C-003] Technology versions must remain compatible across the workspace for the duration of a release cycle. (source: External Context regulatory requirements)

---

## Engineering Principles

Engineering Principles define the implementation philosophy used throughout the Pīṭha platform, including simplicity first, strong typing, fail early, deterministic behavior, and minimal public APIs. They establish the decision hierarchy that governs all engineering choices.

Engineering Principles directly inform Technology Selection by requiring technologies that support compile-time validation (Rust, SQLx, Serde), deterministic behavior (Tokio, Tracing), and strong typing (Rust's type system). Every technology choice must align with these principles or receive explicit architectural review.

See [Engineering Principles](engineering-principles.md) for the complete principles, decision hierarchy, and engineering characteristics.

---

## Build Standards

Build Standards define the build lifecycle, quality requirements, and supply chain dependency policy for the Pīṭha platform. They establish the standard build toolchain (Cargo, Clippy, rustfmt), quality gates, and the dependency supply chain policy that governs how technology choices are validated.

Build Standards constrain Technology Selection by defining the toolchain and validation stages that every approved technology must pass. The build lifecycle stages (validation, quality analysis, testing, packaging) depend on the specific technology stack established in this document.

See [Build Standards](build-standards.md) for the complete build lifecycle, versioning strategy, dependency policy, and quality gates.

---

## Testing Standards

Testing Standards define the repository-wide testing strategy, test types, and coverage expectations for the Pīṭha platform. They establish unit testing, integration testing, end-to-end testing, property-based testing, and benchmarking as complementary verification levels, along with the testing toolchain (Proptest, Mockall, Criterion).

Technology Selection directly determines the testing technologies approved in Testing Standards. The choice of Proptest for property-based testing, Mockall for mocking, and Criterion for benchmarking flows from the technology governance model established in this document.

See [Testing Standards](testing-standards.md) for the complete testing strategy, test organization, coverage expectations, and engineering evidence requirements.

---

## Traceability

Technology Selection derives from the Engineering Principles, Constraints, and Architecture.

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
        ▼
Technology Selection
        │
        ▼
Implementation
```

Architecture defines **what capabilities are required**.

Engineering Principles define **how engineering decisions are made**.

Technology Selection defines **which technologies implement those capabilities**.

Implementation realizes those technology choices.

### Upstream Sources

Technology Selection links to upstream domains:

- Architecture — system-wide design decisions and component boundaries
- Engineering Principles — implementation philosophy and decision hierarchy
- Technology Selection — approved technologies and governance model (where applicable)
- Constraints — non-functional requirements and engineering limitations

### Downstream Consumers

Technology Selection is applied to downstream implementation:

- Implementation — derives coding, building, and testing practices from this document
- Feature Technical Design — references engineering standards for technology conformance

**Non-contradiction rule:** Introducing a new technology that overlaps an approved technology requires an engineering review and corresponding updates to this document.