# Pitha — Phase 2: Implementation

Scope: Build the Cargo workspace with 8 crates implementing 10 features across 5 components. Implementation follows dependency direction: Core → Foundation → Storage → Testing → Runtime.

## Implementation Order

Implementation follows the dependency architecture defined in `docs/raw/architecture/component-model.md`.

| Phase | Component | Crates | Dependencies | Features |
|-------|-----------|--------|--------------|----------|
| 2.1 | Core | pitha-core | Rust ecosystem | Shared types, traits, error types |
| 2.2 | Foundation | pitha-foundation | Core | Bootstrap, config, logger, state, lifecycle, error handling |
| 2.3 | Storage | pitha-storage-core, pitha-storage-fs | Core | Storage contracts, filesystem implementation |
| 2.4 | Testing | pitha-testing | Core | Test infrastructure, evidence generation |
| 2.5 | Runtime | pitha-runtime-cli | Foundation | CLI runtime |
| 2.6 | Runtime | pitha-runtime-mcp | Foundation | MCP runtime |
| 2.7 | Integration | workspace integration | All | Cargo workspace, CI, docs |

## Phase 2.1 — Core

**Crate:** `pitha-core`
**Dependencies:** Rust ecosystem only
**Provides to:** Foundation, Storage, Testing, Runtime

### Tasks

- [ ] Create `crates/pitha-core/Cargo.toml`
- [ ] Define shared platform types (`src/lib.rs`)
- [ ] Define common traits (`src/traits.rs`)
- [ ] Define platform contracts (`src/contracts.rs`)
- [ ] Define shared error types (`src/error.rs`)
- [ ] Define common result types (`src/result.rs`)
- [ ] Define metadata types (`src/metadata.rs`)
- [ ] Define version information (`src/version.rs`)
- [ ] Add unit tests
- [ ] Verify `cargo check` passes

### Success Criteria

- All types are `pub` and documented
- Error types use `thiserror`
- No dependencies beyond Rust stdlib
- `cargo test` passes

---

## Phase 2.2 — Foundation

**Crate:** `pitha-foundation`
**Dependencies:** pitha-core
**Provides to:** Runtime, Applications

### Tasks

- [ ] Create `crates/pitha-foundation/Cargo.toml`
- [ ] Implement application bootstrap (`src/bootstrap/mod.rs`)
- [ ] Implement configuration management (`src/config/mod.rs`)
- [ ] Implement logging infrastructure (`src/logging/mod.rs`)
- [ ] Implement state management (`src/state/mod.rs`)
- [ ] Implement lifecycle coordination (`src/lifecycle/mod.rs`)
- [ ] Implement error handling (`src/error/mod.rs`)
- [ ] Add integration tests
- [ ] Verify `cargo check` passes

### Success Criteria

- Bootstrap sequence: Config → Logger → State → Lifecycle
- Configuration supports TOML + environment variables
- Logging uses `tracing`
- State registry is type-safe
- Lifecycle signals are typed
- Error types propagate correctly
- `cargo test` passes

---

## Phase 2.3 — Storage

**Crates:** `pitha-storage-core`, `pitha-storage-fs`
**Dependencies:** pitha-core, pitha-storage-core
**Provides to:** Foundation, Applications

### Tasks

- [ ] Create `crates/pitha-storage-core/Cargo.toml`
- [ ] Define storage contracts (`src/lib.rs`)
- [ ] Define persistence abstractions (`src/traits.rs`)
- [ ] Create `crates/pitha-storage-fs/Cargo.toml`
- [ ] Implement filesystem storage (`src/lib.rs`)
- [ ] Add integration tests
- [ ] Verify `cargo check` passes

### Success Criteria

- Storage traits are generic and composable
- Filesystem implementation is deterministic
- Error types are well-defined
- `cargo test` passes

---

## Phase 2.4 — Testing

**Crate:** `pitha-testing`
**Dependencies:** pitha-core, may integrate with Foundation, Storage
**Provides to:** Applications, CI/CD, Quality Gates

### Tasks

- [ ] Create `crates/pitha-testing/Cargo.toml`
- [ ] Implement test infrastructure (`src/lib.rs`)
- [ ] Implement runtime harnesses (`src/harness.rs`)
- [ ] Implement mock infrastructure (`src/mock.rs`)
- [ ] Implement fixtures (`src/fixtures.rs`)
- [ ] Implement evidence generation (`src/evidence.rs`)
- [ ] Add integration tests
- [ ] Verify `cargo check` passes

### Success Criteria

- Test infrastructure is reusable
- Evidence generation produces standardized output
- Mock infrastructure supports trait-based mocking
- `cargo test` passes

---

## Phase 2.5 — CLI Runtime

**Crate:** `pitha-runtime-cli`
**Dependencies:** pitha-foundation
**Provides to:** Applications

### Tasks

- [ ] Create `crates/pitha-runtime-cli/Cargo.toml`
- [ ] Implement CLI runtime lifecycle (`src/lib.rs`)
- [ ] Implement request dispatch (`src/dispatch.rs`)
- [ ] Implement startup/shutdown (`src/lifecycle.rs`)
- [ ] Add integration tests
- [ ] Verify `cargo check` passes

### Success Criteria

- CLI runtime manages application lifecycle
- Request dispatch is deterministic
- Startup/shutdown follows dependency order
- `cargo test` passes

---

## Phase 2.6 — MCP Runtime

**Crate:** `pitha-runtime-mcp`
**Dependencies:** pitha-foundation
**Provides to:** Applications

### Tasks

- [ ] Create `crates/pitha-runtime-mcp/Cargo.toml`
- [ ] Implement MCP runtime lifecycle (`src/lib.rs`)
- [ ] Implement request dispatch (`src/dispatch.rs`)
- [ ] Implement startup/shutdown (`src/lifecycle.rs`)
- [ ] Add integration tests
- [ ] Verify `cargo check` passes

### Success Criteria

- MCP runtime manages application lifecycle
- Request dispatch is deterministic
- Startup/shutdown follows dependency order
- `cargo test` passes

---

## Phase 2.7 — Integration

**Scope:** Workspace-wide integration

### Tasks

- [ ] Create root `Cargo.toml` workspace configuration
- [ ] Configure workspace dependencies
- [ ] Add workspace-level tests
- [ ] Add CI configuration (GitHub Actions)
- [ ] Add README.md
- [ ] Verify `cargo build --workspace` passes
- [ ] Verify `cargo test --workspace` passes
- [ ] Verify `cargo clippy --workspace` passes
- [ ] Verify `cargo fmt --check` passes

### Success Criteria

- All 8 crates compile together
- All tests pass
- No clippy warnings
- Code formatting is consistent
- CI pipeline is functional

---

## Summary

| Metric | Value |
|--------|-------|
| Total crates | 8 |
| Total features | 10 |
| Total components | 5 |
| Implementation phases | 7 |
| Estimated tasks | ~50 |
| Dependencies | Core → Foundation → Runtime, Core → Storage, Core → Testing |
