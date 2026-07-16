# Pīṭha — Crate Architecture

## Purpose

This document describes how the logical platform components defined in the Component Model are realized as Cargo workspace crates.

While the Component Model defines **architectural responsibilities**, the Crate Architecture defines **physical packaging**, **workspace organization**, and **dependency boundaries**.

Each crate represents a deployable and reusable engineering capability with a clearly defined responsibility.

---

## Component Architecture

Component Architecture defines the structural building blocks, their responsibilities, and how they compose into the platform. See [Component Model](component-model.md) for the detailed component definitions.

---

## Workspace Organization

Pīṭha is organized as a Cargo workspace.

```text
pitha-platform/
│
├── Cargo.toml
│
└── crates/
    ├── pitha-core/
    ├── pitha-foundation/
    ├── pitha-runtime-cli/
    ├── pitha-runtime-mcp/
    ├── pitha-storage-core/
    ├── pitha-storage-fs/
    ├── pitha-storage-sqlite/
    └── pitha-testing/
```

Each crate is independently versioned within the workspace while participating in a single engineering platform.

---

## Crate Classification

The workspace is organized according to the platform components.

| Platform Component | Workspace Crates |
|--------------------|------------------|
| Core | `pitha-core` |
| Foundation | `pitha-foundation` |
| Runtime | `pitha-runtime-cli`, `pitha-runtime-mcp` |
| Storage | `pitha-storage-core`, `pitha-storage-fs`, `pitha-storage-sqlite` |
| Testing | `pitha-testing` |

Future capabilities extend existing groups rather than introducing new architectural layers.

---

## Dependency Architecture

Workspace dependencies follow strict one-way relationships.

```text
                    Applications
                           │
        ┌──────────┬────────┼────────┬──────────┐
        │          │        │        │          │
 Runtime      Foundation  Storage  Testing
        │          │        │        │
        └──────────┴────────┼────────┘
                           │
                         Core
```

Dependency direction always points toward lower-level platform capabilities.

---

## Crate Dependencies

## pitha-core

**Purpose**

Provides the shared platform primitives used throughout the workspace.

**Depends On**

- Rust ecosystem only

**Used By**

- Foundation
- Runtime
- Storage
- Testing

---

## pitha-foundation

**Purpose**

Provides reusable engineering infrastructure shared across applications.

**Depends On**

- pitha-core

**Used By**

- Runtime crates
- Applications

---

## Runtime Crates

Current runtime implementations include:

- pitha-runtime-cli
- pitha-runtime-mcp

**Depends On**

- pitha-foundation

**Used By**

- Applications

Future runtime crates include:

- pitha-runtime-http
- pitha-runtime-desktop
- pitha-runtime-worker

---

## Storage Crates

Storage consists of one abstraction crate and one or more implementations.

### pitha-storage-core

Defines storage contracts.

Depends On

- pitha-core

### Storage Implementations

Current implementations include:

- pitha-storage-fs
- pitha-storage-sqlite

Depends On

- pitha-storage-core

Future implementations may include:

- pitha-storage-postgres
- pitha-storage-s3

---

## pitha-testing

Provides reusable testing infrastructure.

Depends On

- pitha-core

May integrate with

- Foundation
- Runtime
- Storage

Used By

- Applications
- CI/CD Pipelines
- Engineering Tooling

---

## Workspace Rules

The workspace follows the following architectural rules.

### Single Responsibility

Every crate owns one engineering capability.

No crate should mix unrelated responsibilities.

---

### Stable Interfaces

Public APIs define crate boundaries.

Internal implementation details remain private.

---

### Explicit Dependencies

All dependencies must be explicitly declared.

Implicit coupling between crates is prohibited.

---

### Directional Dependencies

Dependencies always point toward lower-level platform crates.

Higher-level crates must never be referenced by lower-level crates.

---

### No Circular Dependencies

Circular crate dependencies are prohibited.

Cross-component communication must occur through contracts defined in lower layers.

---

### Extension Strategy

New capabilities should be introduced by creating new crates rather than expanding unrelated ones.

Examples include:

- New runtime implementations
- New storage providers
- Additional engineering capabilities

Existing dependency boundaries should remain unchanged.

---

## Architectural Characteristics

The workspace architecture exhibits the following characteristics.

- Cargo workspace oriented
- Modular
- Capability-driven
- Layered
- Explicit dependency management
- Independently evolvable crates
- Long-term maintainability

---

## Observability

Platform observability follows architectural principles defined in the Communication architecture.

Observability concerns include:

- Structured logging across component boundaries
- Distributed tracing for cross-component operations
- Performance metrics for architectural interactions
- Error propagation visibility
- State change auditing

Observability is implemented through published contracts rather than implementation-specific mechanisms.

---

## Operational Readiness

Platform operational readiness follows architectural principles defined in the Communication architecture.

| Concern | Requirement | Tier |
|---------|------------|------|
| Health Checks | Liveness and readiness probes per component | All tiers |
| Resource Limits | Backpressure at component boundaries to prevent cascade failure | Molecular, Organ |
| Graceful Shutdown | Sequencing follows dependency direction (organs before molecules before atoms) | All tiers |
| Circuit Breakers | Configurable failure thresholds for cross-component calls | Cross-Tier |
| Degradation | Fallback contract behavior when components fail | All tiers |
| Resource Cleanup | Ownership-based deallocation across component boundaries | Molecular, Organ |
| State Consistency | Typed lifecycle signals during operational transitions | All tiers |
| Error Recovery | Contract-level retry semantics with rollback without data loss | All tiers |
| Monitoring | Published observability contracts for alerting integration | All tiers |
| Deployment Readiness | Component contracts validated before startup | All tiers |

Operational readiness is validated through engineering evidence and automated testing.

---

## Rationale

This architecture exists to define how platform capabilities are packaged within the Cargo workspace.

The approach follows the engineering principles defined in the Engineering Principles document and is consistent with the platform's architectural characteristics.

---

## Constraints

This architecture operates within the following constraints:

- Must follow dependency direction defined in the Component Model
- Must preserve ownership semantics across component boundaries
- Must maintain architectural independence from implementation technologies
- Must support compile-time validation where possible

---

## Composition Patterns

Platform composition follows established patterns organized by architectural tier.

| Tier | Components | Composition Pattern | Mechanism |
|------|-----------|---------------------|-----------|
| Atomic | Core | Trait contracts define hierarchy interfaces | Barrel re-exports expose public API |
| Molecular | Foundation | Generic parameters enable compile-time composition | Module re-exports document public surface |
| Organ | Runtime, Storage, Testing | Trait objects enable slot-based runtime composition | Adapter patterns compose implementations |
| Cross-Tier | Applications | Dependency injection replaces service discovery | Applications compose only required tiers |

New capabilities are introduced by adding implementations, not modifying existing compositions.

Composition patterns are validated through architectural review and automated testing.

---

## System Overview

System Overview defines the high-level system description and structural approach. See [System Overview](system-overview.md) for the complete platform organization and architectural goals.

---

## Component Model

Component Model defines the logical building blocks that make up the Pīṭha platform. See [Component Model](component-model.md) for the complete component responsibilities and collaboration patterns.

---

## Communication

Communication defines how platform components collaborate while preserving clear architectural boundaries. See [Communication](communication.md) for the complete communication patterns and mechanisms.

---

## Data Flow

Data Flow defines how data moves through the system and who owns it. See [Data Flow](data-flow.md) for the complete data paths and ownership boundaries.

---

## Security

Security defines the architectural security posture, boundaries, and threat model. See [Security](security.md) for the complete trust boundaries and security controls.

---

## Trait Design

Trait Design defines the trait-based abstraction used to decouple implementation details from business logic. See [Trait Design](trait-design.md) for the complete trait contracts and generic constraints.

---

## Traceability

The Crate Architecture derives from the Component Model and defines how platform capabilities are packaged within the Cargo workspace.

```text
Vision
        │
        ▼
System Overview
        │
        ▼
Component Model
        │
        ▼
Crate Architecture
        │
        ▼
Engineering
        │
        ▼
Implementation
```

The Component Model defines **logical platform capabilities**.

The Crate Architecture defines **physical workspace organization**.

Engineering documentation defines **implementation standards**.

Implementation documentation records **the realized system**.

**Non-contradiction rule:** No downstream document may introduce crate responsibilities, dependency relationships, workspace organization, or packaging decisions that contradict this Crate Architecture.