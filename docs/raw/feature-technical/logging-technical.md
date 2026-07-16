# Pīṭha — Logging Technical

## Purpose

Technical implementation details for the Logging feature, covering structured event recording, span management, subscriber delivery, and filter configuration.

---

## Participating Components

| Component | Crate | Role | Dependencies |
|-----------|-------|------|--------------|
| Logging Initializer | pitha-logger | Sets up logging pipeline | Config |
| Logger | pitha-logger | Records structured events | Tracing |
| Span Manager | pitha-logger | Manages hierarchical spans | Tracing |
| Log Formatter | pitha-logger | Formats log output | None |
| Subscriber Manager | pitha-logger | Manages output subscribers | Tracing |
| Log Filter | pitha-logger | Filters log events | Config |

---

## Component Interactions

```text
Log Event
    │
    ├──► Logger
    │       ├──► Create Event
    │       └──► Attach Fields
    │
    ├──► Span Manager
    │       ├──► Current Span
    │       └──► Span Hierarchy
    │
    ├──► Log Filter
    │       ├──► Level Check
    │       └──► Target Check
    │
    ├──► Log Formatter
    │       ├──► JSON Format
    │       └──► Text Format
    │
    └──► Subscriber Manager
            ├──► Console Subscriber
            ├──► File Subscriber
            └──► Custom Subscriber
```

---

## Data Ownership

| Data | Owner | Lifecycle | Access |
|------|-------|-----------|--------|
| Log Event | Logger | Per-event | Write-once |
| Span Context | Span Manager | Per-span | Read-write |
| Filter Config | Log Filter | Static after init | Read-only |
| Formatted Output | Log Formatter | Per-event | Write-once |

---

## Engineering Constraints

- Deterministic pipeline: Event → Logger → Context → Filter → Format → Subscriber → Output
- Structured fields instead of plain text
- Non-blocking async delivery
- Multiple simultaneous subscribers supported
- Logging failures must not crash the application

---

## Security Considerations

- Secrets in log fields automatically redacted
- Log level does not expose internal implementation details
- File log output has restrictive permissions

---

## Traceability

```text
Feature: Logging
    │
    ├──► Architecture: Observability
    ├──► Architecture: Data Flow
    ├──► Engineering: Code Standards
    ├──► Security: Data Classification
    └──► Feature-Technical: This Document
```
