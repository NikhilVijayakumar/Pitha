# Pīṭha — Component Model

## Purpose

The Component Model defines the logical building blocks that make up the Pīṭha platform.

Rather than organizing the architecture around Cargo crates, Pīṭha is organized around engineering capabilities. Each capability owns a distinct architectural responsibility and may be implemented by one or more crates.

This separation allows the platform to evolve by adding new implementations without changing the overall architecture.

---

## Component Architecture

Component Architecture defines the structural building blocks, their responsibilities, and how they compose into the platform. See [Component Model](component-model.md) for the detailed component definitions.

---

## Component Model

Pīṭha consists of five primary architectural components: Core, Foundation, Runtime, Storage, and Testing. Each component owns a distinct engineering concern and may be implemented by one or more crates. Components collaborate through explicit interfaces rather than direct implementation dependencies.

---

## Component Classification

Pīṭha consists of five primary architectural components.

| Component | Purpose |
|-----------|---------|
| Core | Shared platform primitives and contracts |
| Foundation | Common engineering infrastructure |
| Runtime | Application execution environments |
| Storage | Persistence abstractions and implementations |
| Testing | Engineering validation and evidence generation |

Each component owns a well-defined engineering concern and collaborates with other components through explicit interfaces.

---

## Core Component

### Role

The Core component provides the shared platform primitives used throughout Pīṭha.

It defines the common language used by every platform capability.

### Architectural Responsibility

- Shared platform types
- Common traits
- Platform contracts
- Shared error types
- Common result types
- Metadata
- Version information

### Representative Crates

```text
pitha-core
```

### Dependencies

**Depends On**

- Rust ecosystem

**Provides To**

- Foundation
- Runtime
- Storage
- Testing

---

## Foundation Component

### Role

The Foundation component provides the reusable engineering infrastructure shared by every application.

Foundation is runtime-independent and defines how applications are constructed and managed.

### Architectural Responsibility

- Application bootstrap
- State management
- Lifecycle coordination
- Configuration
- Logging
- Error handling

### Representative Crates

```text
pitha-foundation
```

### Dependencies

**Depends On**

- Core

**Provides To**

- Runtime
- Applications

---

## Runtime Component

### Role

The Runtime component provides execution environments for applications.

Each runtime adapts business logic to a specific transport or execution model while remaining independent of application logic.

### Architectural Responsibility

- Runtime lifecycle
- Request dispatch
- Startup and shutdown
- Transport coordination

### Representative Crates

```text
pitha-runtime-cli

pitha-runtime-mcp
```

Future runtimes may include:

```text
pitha-runtime-http

pitha-runtime-desktop

pitha-runtime-worker
```

### Dependencies

**Depends On**

- Foundation

**Provides To**

- Applications

---

## Storage Component

### Role

The Storage component provides persistence capabilities through common abstractions and interchangeable implementations.

Applications depend on storage contracts rather than concrete persistence technologies.

### Architectural Responsibility

- Storage contracts
- Persistence abstractions
- Local persistence implementations

### Representative Crates

```text
pitha-storage-core

pitha-storage-fs

pitha-storage-sqlite
```

Future implementations may include:

```text
pitha-storage-postgres

pitha-storage-s3
```

### Dependencies

**Depends On**

- Core

**Provides To**

- Foundation
- Applications

---

## Testing Component

### Role

The Testing component provides reusable engineering validation and standardized evidence generation.

Testing is treated as an engineering capability rather than solely a development activity.

### Architectural Responsibility

- Test infrastructure
- Runtime harnesses
- Mock infrastructure
- Fixtures
- Assertions
- Coverage
- Benchmarking
- Evidence generation
- Standardized reporting

### Representative Crates

```text
pitha-testing
```

### Dependencies

**Depends On**

- Core

May integrate with:

- Foundation
- Runtime
- Storage

**Provides To**

- Applications
- CI/CD Pipelines
- Quality Gates
- Auditing Systems

---

## Component Collaboration

The components collaborate through stable architectural boundaries.

```text
                    Applications
                           │
      ┌────────────┬────────┼────────┬────────────┐
      │            │        │        │            │
 Runtime      Foundation  Storage  Testing
      │            │        │        │
      └────────────┴────────┼────────┘
                           │
                         Core
                           │
                    Rust Ecosystem
```

Applications compose platform capabilities according to their requirements.

Components communicate through explicit contracts while maintaining clear ownership boundaries.

---

## Dependency Rules

Pīṭha enforces strict dependency direction.

- Applications depend on platform capabilities.
- Runtime depends on Foundation.
- Foundation depends on Core.
- Storage depends on Core.
- Testing depends on Core and may integrate with Foundation, Runtime, and Storage.
- Core depends only on the Rust ecosystem.

Circular dependencies are prohibited.

Business logic must never be implemented inside platform components.

---

## Extension Model

The architecture is designed for extension through composition.

New capabilities should be introduced by adding new implementations to an existing component rather than modifying existing architectural boundaries.

Examples include:

- Additional Runtime implementations
- Additional Storage implementations
- Additional Testing capabilities

This approach allows the platform to evolve while preserving architectural stability.

---

## Architectural Characteristics

The Component Model exhibits the following characteristics:

- Capability-oriented
- Modular
- Composable
- Layered
- Runtime independent
- Storage independent
- Testable by design
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

This architecture exists to define the logical building blocks that make up the Pīṭha platform.

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

The Component Model derives from the System Overview and defines the logical structure of the platform.

```text
Vision
    │
    ▼
System Overview
    │
    ▼
Component Model
    ├──► Crate Architecture
    ├──► Feature Technical Design
    └──► Engineering
```

The Component Model defines the architectural responsibilities and collaboration boundaries between platform capabilities.

Crate Architecture specifies how these components are realized as Cargo workspace crates.

Feature Technical Design defines the capabilities provided by each component.

Engineering documentation defines implementation standards for those capabilities.

**Non-contradiction rule:** No downstream document may introduce component responsibilities, dependency relationships, or collaboration patterns that contradict this Component Model.