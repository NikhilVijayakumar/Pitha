# Pīṭha — Trait Design

## Purpose

Pīṭha uses trait-based abstractions to define stable architectural contracts between platform components.

Rather than exposing concrete implementations, platform capabilities communicate through trait-based abstractions. This allows implementations to evolve independently while preserving stable interfaces for consuming applications.

Traits represent architectural contracts rather than implementation details.

---

## Component Architecture

Component Architecture defines the structural building blocks, their responsibilities, and how they compose into the platform. See [Component Model](component-model.md) for the detailed component definitions.

---

## Design Goals

The trait system is designed to achieve the following objectives.

- Decouple applications from platform implementations.
- Enable interchangeable implementations.
- Encourage composition over inheritance.
- Minimize coupling between platform components.
- Preserve compile-time type safety.
- Promote long-term API stability.

---

## Trait Categories

Pīṭha organizes traits into several architectural categories.

### Foundation Traits

Foundation traits define common engineering behavior shared across applications.

Typical responsibilities include:

- Application bootstrap
- Lifecycle management
- State access
- Configuration
- Logging

Foundation traits define **engineering contracts**, not business behavior.

---

### Runtime Traits

Runtime traits define execution contracts between applications and runtime environments.

Typical responsibilities include:

- Runtime startup
- Runtime shutdown
- Request dispatch
- Transport coordination
- Execution lifecycle

Runtime traits isolate applications from transport-specific implementations.

---

### Storage Traits

Storage traits define persistence contracts independent of storage technologies.

Typical responsibilities include:

- Reading data
- Writing data
- Transactions
- Migrations
- Repository operations

Applications depend only on storage contracts.

Concrete storage implementations satisfy those contracts.

---

### Testing Traits

Testing traits define reusable testing capabilities.

Typical responsibilities include:

- Test execution
- Runtime harnesses
- Mock behavior
- Evidence generation
- Reporting

Testing contracts standardize engineering validation across applications.

---

## Trait Relationships

Platform traits form a dependency hierarchy.

```text
Applications
        │
        ▼
Runtime Traits
        │
        ▼
Foundation Traits
        │
        ▼
Core Traits
```

Storage and Testing define independent contract families that integrate with Foundation while remaining isolated from application logic.

---

## Design Principles

### Abstraction Before Implementation

Applications depend on traits rather than concrete types.

Implementations satisfy contracts.

Applications consume contracts.

---

### Composition over Inheritance

Platform capabilities are composed through explicit trait implementations.

No inheritance hierarchy exists within the platform.

---

### Stable Contracts

Traits should evolve slowly.

Implementations may evolve independently without requiring changes to consuming applications.

---

### Minimal Interfaces

Traits should expose only the behavior required by consumers.

Avoid large, multi-purpose interfaces.

Favor focused contracts.

---

### Explicit Ownership

Traits should preserve explicit ownership semantics.

Ownership transfer should remain explicit.

Shared ownership should be minimized.

---

### Generic Composition

Applications compose platform capabilities through generic constraints or trait objects where appropriate.

Business logic remains independent of implementation details.

---

## Dependency Rules

Trait definitions follow strict dependency rules.

- Core traits have no platform dependencies.
- Foundation traits may depend on Core traits.
- Runtime traits may depend on Foundation traits.
- Storage traits depend only on Core contracts.
- Testing traits depend on Core and integrate with platform capabilities through published contracts.

Circular trait dependencies are prohibited.

---

## Implementation Guidelines

Trait implementations should:

- remain deterministic
- avoid unnecessary shared ownership
- preserve ownership semantics
- expose explicit error handling
- minimize public surface area

Shared ownership constructs should be used only where architectural requirements justify them.

---

## Example Composition

Applications compose platform capabilities through contracts.

```text
Application
    │
    ▼
Storage Contract ──► Storage Implementation
```

Concrete implementations are selected during application composition.

Applications remain independent of the underlying storage implementation.

---

## Architectural Characteristics

The trait architecture exhibits the following characteristics.

- Contract-driven
- Strongly typed
- Compile-time validated
- Implementation independent
- Composable
- Extensible
- Explicit ownership
- Stable APIs

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

This architecture exists to define the trait-based abstraction used to decouple implementation details from business logic.

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

## Traceability

Trait Design derives from the Component Model and Crate Architecture.

```text
Vision
        │
        ▼
Component Model
        │
        ▼
Crate Architecture
        │
        ▼
Trait Design
        │
        ▼
Engineering
        │
        ▼
Implementation
```

The Component Model defines platform capabilities.

The Crate Architecture defines workspace organization.

Trait Design defines the contracts between those capabilities.

Engineering documentation specifies implementation conventions for those contracts.

Implementation realizes the contracts in code.

**Non-contradiction rule:** No downstream document may introduce trait hierarchies, dependency relationships, or contract responsibilities that contradict this Trait Design.
