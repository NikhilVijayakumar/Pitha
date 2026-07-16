# Pīṭha — Repository Structure

## Purpose

This document defines the standard repository organization used throughout the Pīṭha platform.

A consistent repository structure improves discoverability, simplifies onboarding, enables tooling automation, and ensures every platform crate follows the same engineering conventions.

The repository organization is independent of application functionality and remains stable as the platform evolves.

---

## Repository Organization

Pīṭha is organized as a Cargo workspace.

```text
pitha-platform/

├── Cargo.toml
├── crates/
├── docs/
├── examples/
├── scripts/
├── tools/
├── .github/
├── .gitignore
├── LICENSE
└── README.md
```

Each top-level directory has a single responsibility.

---

## Top-Level Directories

| Directory | Responsibility |
|------------|----------------|
| `crates/` | Platform crates |
| `docs/` | Project documentation |
| `examples/` | Example applications and usage |
| `scripts/` | Build and automation scripts |
| `tools/` | Internal engineering tools |
| `.github/` | CI/CD workflows and templates |

Additional directories should only be introduced when they represent a reusable repository capability.

---

## Workspace Organization

All platform crates reside within the `crates/` directory.

```text
crates/

    pitha-core/

    pitha-foundation/

    pitha-runtime-*/

    pitha-storage-*/

    pitha-testing/
```

The exact set of crates is defined by the Crate Architecture.

Repository organization should remain independent of future crate additions.

---

## Documentation Organization

Documentation follows the Saṃgraha documentation standard.

```text
docs/

    raw/

        vision/

        architecture/

        engineering/

        feature/

        implementation/
```

Generated documentation should remain separate from source documentation.

---

## Standard Crate Layout

Every crate follows the standard Cargo layout.

```text
crate-name/

├── Cargo.toml
├── src/
├── tests/
├── benches/
├── examples/
└── README.md
```

Optional directories may be omitted when unused.

---

## Source Organization

Source code should remain modular.

A typical crate layout is:

```text
src/

    lib.rs

    error.rs

    types.rs

    traits.rs

    internal/
```

The exact internal organization may vary depending on crate responsibilities.

Public APIs should remain intentionally small.

---

## Test Organization

Testing follows the standard Cargo conventions.

```text
tests/

    integration/

benches/

examples/
```

Unit tests should remain close to implementation.

Integration tests should reside within the `tests/` directory.

Benchmarks should reside within `benches/`.

---

## Naming Conventions

The repository follows consistent naming conventions.

| Element | Convention |
|----------|------------|
| Workspace | kebab-case |
| Crates | kebab-case |
| Modules | snake_case |
| Files | snake_case |
| Types | PascalCase |
| Traits | PascalCase |
| Functions | snake_case |
| Constants | SCREAMING_SNAKE_CASE |

Naming should remain descriptive and consistent across the platform.

---

## Repository Principles

The repository organization follows several engineering principles.

### Discoverability

Engineers should locate functionality without prior project knowledge.

---

### Consistency

Every crate follows the same organizational conventions.

---

### Modularity

Repository organization reflects architectural boundaries.

---

### Minimalism

Directories should exist only when they provide clear engineering value.

---

### Scalability

The repository structure should support future platform capabilities without reorganization.

---

## Constraints

### Performance
- [C-001] New directories require architectural justification and must not duplicate existing organization. (source: Architecture)

### Security
- [C-002] Crate boundaries must enforce the architectural component model dependency rules. (source: Security Architecture)

### Compliance
- [C-003] Repository layout must support independent crate compilation and testing. (source: External Context)

---

## Engineering Principles

Engineering Principles define the implementation philosophy used throughout the Pīṭha platform, including principles like simplicity first, explicit over implicit, strong typing, and stable contracts. They establish the decision hierarchy (Vision → Constraints → Architecture → Engineering Principles → Implementation) that guides all engineering decisions.

Engineering Principles directly constrain Repository Structure by mandating consistency, minimalism, and discoverability across the workspace. The repository organization must reflect the architectural boundaries and crate relationships that the principles enforce.

See [Engineering Principles](engineering-principles.md) for the complete principles, decision hierarchy, and engineering characteristics.

---

## Technology Selection

Technology Selection defines the approved technologies used throughout the Pīṭha platform, including Rust, Tokio, Serde, SQLx, Tracing, and the governance model for technology adoption. It establishes selection principles that favor ecosystem maturity, minimal dependencies, and long-term maintainability.

Technology Selection constrains Repository Structure by determining which tooling configurations (Cargo.toml, Clippy, rustfmt) must be present in the workspace and individual crates. The approved technology stack directly shapes the standard crate layout and documentation organization.

See [Technology Selection](technology-selection.md) for the complete technology rationale, approved technologies, and governance model.

---

## Build Standards

Build Standards define the build lifecycle, quality requirements, and engineering evidence production for the Pīṭha platform. They establish the versioning strategy, dependency policy, and the standard toolchain (Cargo, Clippy, rustfmt) that must be configured across the workspace.

Build Standards constrain Repository Structure by requiring specific directories (`tests/`, `benches/`, `examples/`) within each crate. The build lifecycle stages (validation, quality analysis, testing, packaging) depend on the repository organization being consistent and predictable.

See [Build Standards](build-standards.md) for the complete build lifecycle, versioning strategy, dependency policy, and quality gates.

---

## Testing Standards

Testing Standards define the repository-wide testing strategy, test types, and coverage expectations for the Pīṭha platform. They establish unit testing, integration testing, end-to-end testing, property-based testing, and benchmarking as complementary verification levels.

Testing Standards constrain Repository Structure by requiring standardized test directories (`tests/integration/`, `benches/`, `examples/`) within each crate. The test organization conventions ensure that the repository layout supports the full verification strategy defined in Testing Standards.

See [Testing Standards](testing-standards.md) for the complete testing strategy, test organization, coverage expectations, and engineering evidence requirements.

---

## Traceability

Repository Structure derives from the Crate Architecture and Engineering Principles.

```text
Vision
        │
        ▼
Architecture
        │
        ▼
Engineering Principles
        │
        ▼
Repository Structure
        │
        ▼
Implementation
```

The Crate Architecture defines **how platform capabilities are packaged**.

Repository Structure defines **how those packages are organized within the source repository**.

Implementation realizes this organization in the workspace.

### Upstream Sources

- Architecture — system-wide design decisions and component boundaries
- Engineering Principles — implementation philosophy and decision hierarchy
- Technology Selection — approved technologies and governance model (where applicable)
- Constraints — non-functional requirements and engineering limitations

### Downstream Consumers

- Implementation — derives coding, building, and testing practices from this document
- Feature Technical Design — references engineering standards for technology conformance

**Non-contradiction rule:** Repository organization shall not violate the workspace boundaries or architectural relationships defined by the Crate Architecture.