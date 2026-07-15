# Pīṭha (पीठ)

> **A modular Rust engineering platform for building consistent, maintainable, and auditable applications.**

Pīṭha (पीठ) is the foundational platform upon which applications are built.

In traditional Indian architecture (*Vastu Shastra*), a **Pīṭha** is the carefully prepared base that provides structural stability for everything constructed above it. Inspired by this principle, Pīṭha provides the common engineering foundation for Rust applications while allowing each application to focus exclusively on its domain logic.

Rather than reimplementing application bootstrap, state management, runtime infrastructure, storage, configuration, logging, and testing for every project, Pīṭha standardizes these capabilities into reusable, composable crates.

---

# Vision

Pīṭha enables teams to build Rust applications with a shared engineering architecture.

Applications built on Pīṭha should differ only in **what they do**, never in **how they are engineered**.

The platform provides:

* Consistent application architecture
* Standardized runtime infrastructure
* Reusable storage implementations
* Engineering-focused testing and evidence generation
* Clean separation between infrastructure and business logic

---

# Design Philosophy

Pīṭha follows four fundamental principles.

## Build Business Logic, Not Infrastructure

Applications should contain domain-specific functionality only.

Common engineering concerns belong to the platform.

---

## Composition over Inheritance

Rust favors composition.

Pīṭha provides reusable components that applications compose rather than inherit.

---

## Convention over Reinvention

Every application follows the same startup sequence, lifecycle, runtime model, storage architecture, and testing standards.

Developers learn one engineering model that applies across the ecosystem.

---

## Engineering by Evidence

Testing is treated as an engineering capability rather than merely a development activity.

Applications produce standardized evidence that can be consumed by quality gates, auditing systems, documentation pipelines, and engineering tooling.

---

# Architecture

```text
                    Application
              (Business / Domain Logic)
                         ▲
                         │
                 Runtime Layer
        CLI • MCP • HTTP • Desktop
                         ▲
                         │
              Foundation Layer
 App • State • Lifecycle • Config • Logging • Errors
                         ▲
                         │
             Storage & Testing
                         ▲
                         │
                Rust Ecosystem
 Tokio • Serde • Tracing • SQLx • etc.
```

Applications depend on Pīṭha.

Pīṭha depends on the Rust ecosystem.

Business logic never depends directly on infrastructure libraries.

---

# Workspace Structure

```text
pitha-platform/

Cargo.toml

crates/

├── pitha-core/
│
├── pitha-foundation/
│
├── pitha-runtime-cli/
│
├── pitha-runtime-mcp/
│
├── pitha-storage-core/
│
├── pitha-storage-fs/
│
├── pitha-storage-sqlite/
│
└── pitha-testing/
```

Each crate owns a single engineering responsibility.

---

# Platform Modules

## pitha-core

Shared platform primitives.

Responsibilities include:

* Common types
* Shared traits
* Platform errors
* Common result types
* Metadata
* Version information
* Internal platform contracts

This crate contains no runtime or application logic.

---

## pitha-foundation

The engineering foundation used by every application.

Responsibilities:

* Application bootstrap
* State management
* Lifecycle management
* Configuration
* Logging
* Error handling

Foundation defines how applications are constructed but remains independent of any execution environment.

---

## pitha-runtime-cli

Provides the Command Line runtime.

Responsibilities:

* Command execution
* Argument parsing
* Startup
* Shutdown
* Runtime lifecycle

Applications requiring a CLI simply compose this runtime.

---

## pitha-runtime-mcp

Provides the Model Context Protocol runtime.

Responsibilities:

* MCP server initialization
* Tool registration
* Request handling
* Runtime lifecycle

Applications expose MCP capabilities by composing this runtime.

---

## pitha-storage-core

Defines storage abstractions shared across implementations.

Responsibilities:

* Storage traits
* Common storage types
* Storage errors
* Platform contracts

Applications depend on storage abstractions rather than concrete implementations.

---

## pitha-storage-fs

Filesystem-based storage implementation.

Responsibilities:

* File operations
* Directory management
* Workspace management
* Local persistence

---

## pitha-storage-sqlite

SQLite-based storage implementation.

Responsibilities:

* Database lifecycle
* Migrations
* Transactions
* Query execution
* Local structured persistence

---

## pitha-testing

Testing is considered a first-class engineering capability.

Unlike traditional frameworks where testing primarily validates software behavior, Pīṭha treats testing as a source of engineering evidence.

Capabilities include:

* Unit testing support
* Integration testing support
* End-to-end testing support
* Test contexts
* Runtime harnesses
* Mock infrastructure
* Fixtures
* Assertions
* Coverage collection
* Benchmark support
* Evidence generation
* Standardized reporting

Testing infrastructure is designed to support deterministic quality gates, automated auditing, and engineering analysis.

---

# Layer Responsibilities

## Applications

Own:

* Business rules
* Domain models
* Workflows
* Product-specific services
* Application protocols

Applications should contain no reusable infrastructure.

---

## Runtime

Own:

* Application execution
* Request dispatch
* Runtime lifecycle
* Transport-specific behavior

Runtime crates never contain business logic.

---

## Foundation

Own:

* Application infrastructure
* Lifecycle
* State
* Configuration
* Logging
* Error management

Foundation is independent of any runtime.

---

## Storage

Own:

* Persistence
* Data access
* Workspace storage
* Local databases

Business logic should never depend on a specific storage implementation.

---

## Testing

Own:

* Engineering validation
* Standardized evidence generation
* Runtime testing
* Shared testing infrastructure

Testing provides reusable engineering capabilities across the platform.

---

# Dependency Direction

```text
Application
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

Storage and Testing are shared platform capabilities that integrate with Foundation and Runtime without introducing circular dependencies.

Every dependency points downward.

---

# Engineering Principles

Pīṭha follows these engineering principles throughout the platform:

* Single Responsibility
* Explicit Dependencies
* Composition over Inheritance
* Modular Design
* Runtime Independence
* Storage Independence
* Testability by Design
* Deterministic Engineering
* Evidence-Driven Quality
* Long-Term Maintainability

---

# Non-Goals

Pīṭha intentionally does **not** provide:

* Business workflows
* Domain models
* Product logic
* Application-specific storage schemas
* Product-specific protocols
* AI agents
* Business orchestration

These responsibilities belong to the consuming application.

---

# Example Applications

Applications built on Pīṭha may include:

* Knowledge Engineering Platforms
* Developer Tools
* CLI Applications
* MCP Servers
* Desktop Applications
* Future HTTP Services

Each application composes only the capabilities it requires.

---

# Long-Term Goal

Pīṭha aims to become the common engineering platform for the ecosystem.

By standardizing infrastructure, runtime architecture, storage, and testing, every application gains a consistent engineering foundation while remaining free to evolve its own domain logic independently.

The platform enables developers to spend their time building products—not rebuilding infrastructure.
