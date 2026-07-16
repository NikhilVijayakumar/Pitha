# Pīṭha — System Overview

## Purpose

Pīṭha is a modular Rust engineering platform organized as a Cargo workspace. It provides reusable, composable engineering capabilities that eliminate the need for applications to repeatedly implement common infrastructure such as application bootstrap, state management, lifecycle coordination, runtime execution, storage, configuration, logging, error handling, and testing.

Applications built on Pīṭha focus exclusively on domain logic while the platform provides a consistent engineering foundation through reusable platform capabilities.

Pīṭha follows a capability-oriented architecture where each engineering capability is implemented as one or more independent crates with clearly defined responsibilities and dependency boundaries.

---

## Component Architecture

Component Architecture defines the structural building blocks, their responsibilities, and how they compose into the platform. See [Component Model](component-model.md) for the detailed component definitions.

---

## System Overview

Pīṭha is a modular Rust engineering platform organized as a Cargo workspace. It provides reusable, composable engineering capabilities organized into five architectural components: Core, Foundation, Runtime, Storage, and Testing. Applications focus on domain logic while the platform provides a consistent engineering foundation.

---

## Architectural Goals

The architecture of Pīṭha is designed to:

- Standardize engineering infrastructure across applications.
- Promote composition through reusable platform capabilities.
- Maintain clear separation between business logic and engineering concerns.
- Support multiple execution environments without changing application logic.
- Enable interchangeable infrastructure implementations through stable abstractions.
- Produce standardized engineering evidence that supports quality gates and auditing.
- Provide a platform that evolves independently from the applications built upon it.

---

## Platform Organization

Pīṭha is organized as a collection of engineering capabilities rather than a monolithic framework.

Each capability is implemented as one or more Cargo crates that own a single engineering responsibility.

Applications compose only the capabilities they require.

This approach enables independent evolution of platform components while maintaining a consistent engineering model across the ecosystem.

---

## Platform Capabilities

Pīṭha currently provides four major capability groups.

## Foundation

Provides runtime-independent engineering infrastructure.

Responsibilities include:

- Application bootstrap
- State management
- Lifecycle management
- Configuration
- Logging
- Error handling

---

## Runtime

Provides execution environments for applications.

Current runtimes include:

- CLI
- Model Context Protocol (MCP)

Future runtimes may include:

- HTTP
- Desktop
- Background Worker

Runtime capabilities coordinate application execution but never contain business logic.

---

## Storage

Provides persistence abstractions and implementations.

Current storage capabilities include:

- Storage contracts
- Filesystem storage
- SQLite storage

Applications depend on storage abstractions rather than implementation details.

---

## Testing

Provides standardized engineering validation and evidence generation.

Capabilities include:

- Unit testing support
- Integration testing support
- End-to-end testing support
- Benchmark support
- Coverage collection
- Runtime harnesses
- Evidence generation

Testing is treated as an engineering capability rather than solely a development activity.

---

## Layer Responsibilities

| Layer | Responsibility |
|---------|---------------|
| Application | Business logic, domain models, workflows |
| Runtime | Execution environment and transport coordination |
| Foundation | Shared engineering infrastructure |
| Core | Shared platform primitives and contracts |
| Rust Ecosystem | External libraries and community crates |

Storage and Testing are cross-cutting platform capabilities that integrate with Foundation and Runtime while remaining independent of application logic.

---

## Dependency Direction

Pīṭha enforces one-way dependency flow.

- Applications depend on platform capabilities.
- Platform capabilities never depend on applications.
- Runtime depends on Foundation.
- Foundation depends on Core.
- Storage depends on Core and integrates with Foundation.
- Testing depends on platform capabilities but never on applications.
- Core depends only on the Rust ecosystem.

Circular dependencies are not permitted.

---

## System Structure

```text
                    Applications
                         │
        ┌──────────┬─────┼─────┬──────────┐
        │          │     │     │          │
    Runtime   Foundation Storage Testing
        │          │      │      │
        └──────────┴──────┼──────┘
                           │
                         Core
                           │
                    Rust Ecosystem
         Tokio • Serde • Tracing • SQLx • ...
```

Applications compose platform capabilities according to their requirements.

Platform capabilities collaborate through stable contracts while maintaining clear ownership boundaries.

---

## Architectural Characteristics

Pīṭha exhibits the following architectural characteristics:

- Modular
- Layered
- Capability-oriented
- Composable
- Runtime independent
- Storage independent
- Testable by design
- Workspace-oriented
- Explicit dependency management
- Long-term maintainability

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

This architecture exists to define the high-level system description and structural approach.

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

Architecture derives from the Vision (Tier 0) and provides the structural blueprint for the remainder of the documentation hierarchy.

```text
Tier 0 : Vision
        │
        ▼
Tier 1 : Architecture
        ├──► Feature Design
        └──► Engineering
                └──► Implementation
```

The Architecture defines **how** the platform is organized.

Feature documents define **what** capabilities exist.

Engineering documents define **how** those capabilities are implemented.

Implementation documents describe **what has been built**.

**Non-contradiction rule:** No downstream document may introduce component boundaries, dependency relationships, ownership assignments, or communication paths that contradict this System Overview.