# Pīṭha — CLI Runtime Technical

## Purpose

Technical implementation details for the CLI Runtime feature, covering argument parsing, command dispatch, signal handling, and exit code management.

---

## Participating Components

| Component | Crate | Role | Dependencies |
|-----------|-------|------|--------------|
| Runtime Coordinator | pitha-cli | Orchestrates CLI lifecycle | Bootstrap, Lifecycle |
| Argument Parser | pitha-cli | Parses CLI arguments | None |
| Command Dispatcher | pitha-cli | Routes commands to handlers | Argument Parser |
| Signal Handler | pitha-cli | Handles OS signals | Runtime Coordinator |
| Exit Manager | pitha-cli | Manages exit codes | Runtime Coordinator |

---

## Component Interactions

```text
CLI Entry
    │
    ├──► Argument Parser
    │       └──► Parsed Args
    │
    ├──► Bootstrap Coordinator
    │       └──► Foundation Ready
    │
    ├──► Command Dispatcher
    │       ├──► Command Lookup
    │       ├──► Handler Execute
    │       └──► Result Return
    │
    ├──► Signal Handler
    │       ├──► SIGTERM → Graceful Shutdown
    │       ├──► SIGINT → Graceful Shutdown
    │       └──► SIGHUP → Reload Config
    │
    └──► Exit Manager
            └──► Process Exit Code
```

---

## Data Ownership

| Data | Owner | Lifecycle | Access |
|------|-------|-----------|--------|
| Parsed Arguments | Argument Parser | Per-invocation | Read-only |
| Command Registry | Command Dispatcher | Static after init | Read-only |
| Signal State | Signal Handler | Runtime lifetime | Read-write |
| Exit Code | Exit Manager | Per-invocation | Write-only |

---

## Engineering Constraints

- Foundation bootstrap must complete before any command runs
- Typed exit codes: 0 (success), 1 (error), 2 (usage error)
- Signal handling is runtime-specific ( Tokio signal handlers)
- Commands are registered at compile time via macro or static registry

---

## Security Considerations

- No shell injection via argument parsing
- Command dispatch validates handler exists before execution
- Exit codes do not leak internal error details

---

## Traceability

```text
Feature: CLI Runtime
    │
    ├──► Architecture: System Overview
    ├──► Architecture: Communication
    ├──► Engineering: Async Patterns
    ├──► Security: Threat Model
    └──► Feature-Technical: This Document
```
