# Pīṭha — Configuration Technical

## Purpose

Technical implementation details for the Configuration feature, covering TOML parsing, layered resolution, typed access, and immutability guarantees.

---

## Participating Components

| Component | Crate | Role | Dependencies |
|-----------|-------|------|--------------|
| Configuration Loader | pitha-config | Loads config from sources | None |
| TOML Parser | pitha-config | Parses TOML format | toml crate |
| Configuration Validator | pitha-config | Validates against schema | None |
| Default Provider | pitha-config | Provides built-in defaults | None |
| Environment Resolver | pitha-config | Resolves env var overrides | None |
| Configuration Registry | pitha-config | Holds validated config | None |

---

## Component Interactions

```text
Config Request
    │
    ├──► Default Provider
    │       └──► Built-in Defaults
    │
    ├──► TOML Parser
    │       ├──► File Read
    │       └──► Parse TOML
    │
    ├──► Environment Resolver
    │       └──► Env Var Override
    │
    ├──► Configuration Validator
    │       ├──► Schema Check
    │       └──► Type Validation
    │
    └──► Configuration Registry
            └──► Immutable Config
```

---

## Data Ownership

| Data | Owner | Lifecycle | Access |
|------|-------|-----------|--------|
| Default Config | Default Provider | Static | Read-only |
| File Config | TOML Parser | Static after load | Read-only |
| Env Overrides | Environment Resolver | Static after resolve | Read-only |
| Validated Config | Configuration Registry | Application lifetime | Read-only |

---

## Engineering Constraints

- Layered resolution: Defaults → TOML → Env Vars (deterministic precedence)
- Strongly typed APIs (no string-based lookups)
- Invalid config prevents application startup
- Immutability after bootstrap (no runtime config changes)

---

## Security Considerations

- Config values not logged at INFO level or above
- Secrets marked with redaction flags
- File permissions validated before reading
- No config file symlinks followed

---

## Traceability

```text
Feature: Configuration
    │
    ├──► Architecture: Data Flow
    ├──► Architecture: Component Model
    ├──► Engineering: Code Standards
    ├──► Security: Data Classification
    └──► Feature-Technical: This Document
```
