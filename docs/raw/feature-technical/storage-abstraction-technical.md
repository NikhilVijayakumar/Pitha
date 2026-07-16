# Pīṭha — Storage Abstraction Technical

## Purpose

Technical implementation details for the Storage Abstraction feature, covering repository contracts, transaction management, and pluggable backend architecture.

---

## Participating Components

| Component | Crate | Role | Dependencies |
|-----------|-------|------|--------------|
| Storage Contracts | pitha-storage | Defines storage interfaces | None |
| Repository Contracts | pitha-storage | Defines CRUD interfaces | None |
| Transaction Manager | pitha-storage | Manages transactions | None |
| Migration Manager | pitha-storage | Handles schema migrations | None |
| Storage Provider | pitha-storage | Selects backend implementation | None |
| Filesystem Backend | pitha-storage-fs | File-based storage | std::fs |
| SQLite Backend | pitha-storage-sqlite | SQLite storage | rusqlite |

---

## Component Interactions

```text
Storage Request
    │
    ├──► Storage Provider
    │       └──► Backend Select
    │
    ├──► Repository Contract
    │       ├──► CRUD Operation
    │       └──► Query Execution
    │
    ├──► Transaction Manager (optional)
    │       ├──► Begin Transaction
    │       ├──► Execute Operations
    │       └──► Commit / Rollback
    │
    └──► Migration Manager (optional)
            ├──► Schema Check
            └──► Run Migrations
```

---

## Data Ownership

| Data | Owner | Lifecycle | Access |
|------|-------|-----------|--------|
| Storage Config | Storage Provider | Static after init | Read-only |
| Repository Data | Backend Implementation | External lifetime | Read-write |
| Transaction State | Transaction Manager | Per-transaction | Read-write |
| Migration State | Migration Manager | Per-migration | Read-write |

---

## Engineering Constraints

- Applications code against contracts, not implementations
- Backend selected during composition (dependency injection)
- Transaction contracts with commit/rollback semantics
- Typed storage errors regardless of backend

---

## Security Considerations

- Storage paths validated against traversal attacks
- SQLite connections use parameterized queries (no SQL injection)
- File permissions set restrictively on new files

---

## Traceability

```text
Feature: Storage Abstraction
    │
    ├──► Architecture: Component Model
    ├──► Architecture: Data Flow
    ├──► Engineering: Component Patterns
    ├──► Security: Threat Model
    └──► Feature-Technical: This Document
```
