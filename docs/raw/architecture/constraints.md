# Pīṭha — Constraints

## Purpose

This document defines the architectural, engineering, and technology constraints that govern the evolution of the Pīṭha platform.

Constraints represent non-negotiable rules that preserve architectural integrity, maintain engineering consistency, and ensure long-term platform maintainability.

All downstream architecture, engineering decisions, and implementations must comply with these constraints.

---

## Component Architecture

Component Architecture defines the structural building blocks, their responsibilities, and how they compose into the platform. See [Component Model](component-model.md) for the detailed component definitions.

---

## Architectural Constraints define the structural rules of the platform.

Violating these constraints changes the architecture and therefore requires an architectural review.

---

### Single Responsibility

Every crate shall own exactly one engineering capability.

Responsibilities shall not be combined simply for convenience.

Large capabilities should be decomposed into smaller, focused crates.

---

### Directional Dependencies

Dependencies shall always follow the architectural hierarchy.

```text
Applications
        │
        ▼
Runtime
        │
        ▼
Foundation
        │
        ▼
Core
```

Storage and Testing integrate with platform capabilities through published contracts while preserving dependency direction.

Circular dependencies are prohibited.

---

### Layer Isolation

Each architectural layer owns its responsibilities exclusively.

- Applications own business capabilities.
- Runtime owns execution environments.
- Foundation owns engineering infrastructure.
- Storage owns persistence.
- Testing owns engineering validation.
- Core owns shared platform contracts.

Responsibilities shall not cross architectural boundaries.

---

### Contract-Based Integration

Components communicate exclusively through published contracts.

Concrete implementations shall never become architectural dependencies.

---

### Runtime Independence

Foundation shall remain independent of execution environments.

Transport-specific behavior belongs exclusively within Runtime components.

---

## Storage Independence

Applications shall depend on storage contracts rather than persistence technologies.

Storage implementations shall remain interchangeable.

---

### Platform Independence

Platform capabilities shall not contain application-specific logic.

Business behavior belongs exclusively to consuming applications.

---

## Engineering Constraints

Engineering constraints define how the platform is developed.

---

## Composition over Inheritance

Reusable behavior shall be composed through traits and explicit dependencies.

Inheritance-style architectures are prohibited.

---

### Explicit Dependencies

Every dependency shall be declared explicitly.

Hidden service discovery, global dependency injection containers, and implicit runtime wiring are prohibited.

---

### Deterministic Testing

Platform testing shall produce deterministic and repeatable results whenever technically possible.

Engineering evidence should be generated in standardized formats.

---

### Stable Public APIs

Public interfaces should evolve conservatively.

Breaking changes should occur only in major platform versions.

---

### Minimal Public Surface

Only intentionally supported APIs should be publicly exposed.

Internal implementation details should remain private.

---

### Long-Term Maintainability

Engineering decisions should prioritize clarity, modularity, and long-term stability over short-term convenience.

---

## Technology Constraints

Technology constraints define mandatory implementation technologies.

---

### Programming Language

The platform shall be implemented in Rust.

Rust's ownership and type systems are considered architectural requirements.

---

### Workspace Model

The platform shall be organized as a Cargo workspace.

Each crate shall compile independently.

Each crate shall define explicit dependencies.

---

### Memory Safety

Unsafe Rust is prohibited unless explicitly justified and documented.

Safe Rust is the default implementation strategy.

---

### Async Runtime

Tokio is the standard asynchronous runtime.

Blocking operations shall not execute on asynchronous executor threads.

---

### Dependency Management

External dependencies shall be minimized.

Every dependency should provide clear engineering value.

Unmaintained or unnecessary dependencies should be avoided.

---

## Evolution Constraints

The architecture should evolve through extension rather than modification.

Preferred evolution strategies include:

- Adding new Runtime implementations
- Adding new Storage implementations
- Adding new Testing capabilities
- Adding new platform crates

Existing architectural boundaries should remain stable whenever possible.

---

## Constraint Categories

| Category | Scope |
|-----------|-------|
| Architectural | Platform structure |
| Engineering | Development practices |
| Technology | Implementation platform |
| Evolution | Future platform growth |

Architectural constraints take precedence over Engineering constraints.

Engineering constraints take precedence over Technology choices.

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

This architecture exists to define the architectural constraints that govern platform evolution.

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

## Crate Architecture

Crate Architecture defines how platform capabilities are packaged within the Cargo workspace. See [Crate Architecture](crate-architecture.md) for the complete workspace organization and dependency rules.

---

## Trait Design

Trait Design defines the trait-based abstraction used to decouple implementation details from business logic. See [Trait Design](trait-design.md) for the complete trait contracts and generic constraints.

---

## Traceability

The Constraints derive directly from the Vision and govern all downstream documentation.

```text
Vision
        │
        ▼
Constraints
        │
        ├──► System Overview
        ├──► Component Model
        ├──► Crate Architecture
        ├──► Communication
        ├──► Data Flow
        ├──► Trait Design
        ├──► Engineering
        └──► Implementation
```

The Vision defines **why** the platform exists.

Constraints define **what cannot be violated**.

Architecture defines **how the platform is organized**.

Engineering defines **how the platform is implemented**.

Implementation realizes those decisions in code.

**Non-contradiction rule:** Any proposal that violates a documented constraint requires an explicit architectural review and corresponding updates to the Vision and Architecture before implementation.