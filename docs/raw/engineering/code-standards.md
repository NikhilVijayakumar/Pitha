# Pīṭha — Code Standards

## Purpose

This document defines the coding standards used throughout the Pīṭha platform.

The purpose of these standards is to ensure that every platform crate is readable, maintainable, consistent, and aligned with the engineering philosophy of Pīṭha.

These standards apply to all production code unless explicitly documented otherwise.

---

## Coding Principles

The following principles guide all implementation decisions.

### Readability First

Code should be easy to understand.

Clarity is preferred over cleverness.

---

### Explicit over Implicit

Initialization, ownership, dependencies, and behavior should remain explicit.

Hidden behavior should be avoided.

---

### Strong Typing

Use Rust's type system to encode invariants whenever practical.

Compile-time validation is preferred over runtime validation.

---

### Small, Focused Components

Functions, modules, and types should have a single responsibility.

Large implementations should be decomposed into smaller units.

---

### Public API Minimalism

Expose only intentionally supported APIs.

Everything else should remain private.

---

## API Design Standards

Public APIs should follow consistent design principles.

- Traits define contracts.
- Constructors establish valid state.
- Builders are preferred for complex initialization.
- Public interfaces should remain stable.
- Generic abstractions should be introduced only when they improve reuse.

Breaking changes require architectural review.

---

## Naming Standards

The platform follows the Rust API Guidelines.

| Element | Convention |
|----------|------------|
| Crates | kebab-case |
| Modules | snake_case |
| Files | snake_case |
| Functions | snake_case |
| Types | PascalCase |
| Traits | PascalCase |
| Constants | SCREAMING_SNAKE_CASE |
| Static Variables | SCREAMING_SNAKE_CASE |
| Generic Parameters | UpperCamelCase (`T`, `E`, `S`) |

Names should describe intent rather than implementation.

---

## Module Organization

Modules should remain focused and cohesive.

- One module should represent one logical responsibility.
- Public modules expose only stable APIs.
- Internal implementation details remain private.
- Circular module dependencies are prohibited.

---

## Error Handling

Errors should be explicit and strongly typed.

Platform crates use typed error definitions.

Applications may use ergonomic error propagation where appropriate.

Engineering rules include:

- Do not panic for recoverable errors.
- Return `Result` for expected failures.
- Panic only for programming errors.
- Provide meaningful error context.

---

## Ownership and Borrowing

Ownership should remain explicit.

Engineering guidelines include:

- Prefer ownership when transferring responsibility.
- Borrow when ownership transfer is unnecessary.
- Avoid unnecessary cloning.
- Shared ownership should require architectural justification.
- Interior mutability should remain localized.

Rust's ownership model should be leveraged rather than bypassed.

---

## Async Programming

Asynchronous programming should follow platform standards.

- Async is used for I/O-bound operations.
- Blocking work should not execute on async executors.
- Long-running CPU work should be isolated.
- Runtime-specific async behavior belongs in Runtime crates.

---

## Documentation Standards

Public APIs should be documented.

Documentation should explain:

- Purpose
- Parameters
- Return values
- Errors
- Examples (when appropriate)

Comments should explain **why**, not **what**.

---

## Linting Standards

Static analysis follows workspace-wide standards.

The standard linting configuration includes:

- Rust compiler warnings
- Clippy
- Rustfmt

Warnings should be treated as engineering issues.

Intentional exceptions should be documented.

---

## Unsafe Code

Safe Rust is the default implementation strategy.

Unsafe code requires:

- Architectural justification
- Isolated implementation
- Documented invariants
- `SAFETY` explanations
- Dedicated tests
- Engineering review

The `#![forbid(unsafe_code)]` attribute should be applied wherever practical.

---

## Code Characteristics

Platform code should exhibit the following characteristics.

- Readable
- Explicit
- Strongly typed
- Modular
- Testable
- Deterministic
- Well documented
- Maintainable

---

## Constraints

### Performance
- [C-001] Production code must compile without warnings under the standard Clippy configuration. (source: Architecture)

### Security
- [C-002] Public API surface must not expose unsafe abstractions without explicit safety justification. (source: Security Architecture)

### Compliance
- [C-003] Code must conform to the Rust API Guidelines for public interface design. (source: External Context)

---

## Engineering Principles

Engineering Principles define the implementation philosophy used throughout the Pīṭha platform, including readability first, explicit over implicit, strong typing, public API minimalism, and stable contracts. They establish the decision hierarchy that governs all engineering choices.

Engineering Principles directly constrain Code Standards by mandating that source code must be readable, explicit, strongly typed, and minimally exposed. The coding principles (readability, explicitness, strong typing, small focused components) are direct implementations of the broader Engineering Principles.

See [Engineering Principles](engineering-principles.md) for the complete principles, decision hierarchy, and engineering characteristics.

---

## Technology Selection

Technology Selection defines the approved technologies used throughout the Pīṭha platform, including Rust, Tokio, Serde, thiserror/anyhow, and the governance model for technology adoption. It establishes which libraries and frameworks are approved for use across platform crates.

Technology Selection constrains Code Standards by determining which language features, crate patterns, and ecosystem conventions apply. The choice of thiserror for platform error types, anyhow for application error propagation, and Tokio for async programming directly shapes the coding patterns defined in this document.

See [Technology Selection](technology-selection.md) for the complete technology rationale, approved technologies, and governance model.

---

## Build Standards

Build Standards define the build lifecycle, quality requirements, and engineering evidence production for the Pīṭha platform. They establish the standard toolchain (Cargo, Clippy, rustfmt), quality gates, and the validation stages that code must pass.

Build Standards constrain Code Standards by defining the static analysis, formatting, and quality validation that source code must satisfy. The linting standards (Clippy, rustfmt) and quality gates in Build Standards enforce the coding practices established in this document.

See [Build Standards](build-standards.md) for the complete build lifecycle, versioning strategy, dependency policy, and quality gates.

---

## Testing Standards

Testing Standards define the repository-wide testing strategy, test types, and coverage expectations for the Pīṭha platform. They establish unit testing, integration testing, end-to-end testing, property-based testing, and benchmarking as complementary verification levels.

Code Standards and Testing Standards are closely coupled: the coding principles (small focused components, public API minimalism, testable design) directly support the testing strategy by ensuring code is modular and verifiable. The test organization conventions in Testing Standards depend on the module organization patterns defined in this document.

See [Testing Standards](testing-standards.md) for the complete testing strategy, test organization, coverage expectations, and engineering evidence requirements.

---

## Traceability

Code Standards derive from the Engineering Principles, Technology Selection, and Build Standards.

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
        ├──► Build Standards
        │
        ▼
Code Standards
        │
        ▼
Implementation
```

Engineering Principles define **how engineers make implementation decisions**.

Technology Selection defines **which technologies are used**.

Build Standards define **how code is validated**.

Code Standards define **how production code is written**.

Implementation realizes these standards throughout the platform.

### Upstream Sources

- Architecture — system-wide design decisions and component boundaries
- Engineering Principles — implementation philosophy and decision hierarchy
- Technology Selection — approved technologies and governance model (where applicable)
- Constraints — non-functional requirements and engineering limitations

### Downstream Consumers

- Implementation — derives coding, building, and testing practices from this document
- Feature Technical Design — references engineering standards for technology conformance

**Non-contradiction rule:** Source code shall conform to the coding practices, API design rules, and ownership principles defined in this document.