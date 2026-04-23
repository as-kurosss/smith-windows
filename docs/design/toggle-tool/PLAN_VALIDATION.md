# ToggleTool /plan Validation Report

**Date:** 2026-04-22
**Validator:** smith-planner
**Status:** ✅ APPROVED → awaiting code implementation

---

## 📋 Validation Checklist

### Project Rules Compliance

| Rule | Status | Evidence |
|------|--------|----------|
| Spec first → plan → code | ✅ | Plan based on specification.md, contract.md, test-plan.md |
| Modularity: one file = one responsibility | ✅ | `src/core/toggle.rs` + `src/runtime/backends/windows/toggle.rs` |
| No `unwrap()`/`expect()`/`panic!` | ✅ | Plan explicitly forbids these in src/ |
| Idempotency: same input → same state | ✅ | Plan specifies "state unchanged on error" for all errors |
| UIAutomation-first | ✅ | All operations use uiautomation crate patterns |
| COM Safety: `?Send` on all traits | ✅ | Plan requires `#[async_trait::async_trait(?Send)]` |
| Error handling via `thiserror` | ✅ | Plan specifies 10 `ToggleError` variants |
| Async via tokio | ✅ | Plan uses `Duration` timeout, `CancellationToken` |
| Tests: unit + integration | ✅ | Plan specifies inline unit tests + integration tests |
| cfg-flags: windows/non-windows | ✅ | Plan includes `#[cfg(target_os = "windows")]` checks |
| Validation in core module | ✅ | Plan specifies `validate_toggle_config()` in core |
| Backend pattern support | ✅ | Plan includes toggle/Value/LegacyIA pattern checks |
| No `spawn_blocking` for UIA | ✅ | Plan explicitly warns against this (UIElement is !Send) |

### Architecture Rules Compliance

| Rule | Status | Evidence |
|------|--------|----------|
| `UIElement` is `!Send`/`!Sync` | ✅ | Plan correctly uses `#[async_trait::async_trait(?Send)]` |
| COM operations on single thread | ✅ | Backend calls are direct, no thread hopping |
| Contract guarantees enforced | ✅ | Plan specifies atomicity: state unchanged on error |
| Platform stubs for non-Windows | ✅ | Plan includes `unsupported.rs` with stubs |

### Test Plan Completeness

| Test Type | Status | Evidence |
|-----------|--------|----------|
| Unit tests: positive | ✅ | 10+ test cases specified |
| Unit tests: negative | ✅ | 10+ test cases specified |
| Unit tests: boundary | ✅ | 0s, 3600s, 3601s timeout tests |
| Integration tests | ✅ | 14 test cases specified |
| Mock backend tests | ✅ | Creation, reset, state tests |
| Cancellation tests | ✅ | Token-based tests |
| Timeout tests | ✅ | Duration-based tests |
| Idempotency tests | ✅ | Repeated error call tests |

### Documentation Consistency

| Document | Status | Evidence |
|----------|--------|----------|
| specification.md | ✅ | Input/output, boundaries, success criteria |
| contract.md | ✅ | Requirements, guarantees, prohibitions |
| test-plan.md | ✅ | Positive/boundary/negative scenarios |
| brief.md | ✅ | Complete implementation instructions |
| plan.md | ✅ | This validation report |
| ARCHITECTURE.md | ✅ | ToggleTool section exists and aligns |

### Backend Pattern Compliance

| Pattern | Status | Evidence |
|---------|--------|----------|
| ClickTool pattern | ✅ | `ToggleBackend` mirrors `ClickBackend` |
| TypeTool pattern | ✅ | `ToggleConfig` mirrors `TypeConfig` |
| MockBackend pattern | ✅ | `MockToggleBackend` uses `Arc<Mutex<MockToggleState>>` |
| Error enum pattern | ✅ | 10 variants via `thiserror` |
| Validation function | ✅ | `validate_toggle_config()` |
| Async trait pattern | ✅ | `#[async_trait::async_trait(?Send)]` |

### Verification Commands

| Command | Status | Expected |
|---------|--------|----------|
| `cargo test --lib` | ✅ | Unit tests pass |
| `cargo test --test integration` | ✅ | Integration tests pass |
| `cargo clippy -- -D warnings` | ✅ | No warnings |
| `cargo fmt --all` | ✅ | Proper formatting |
| `cargo check` | ✅ | Compilation succeeds |

---

## 🔍 Key Design Decisions

### 1. COM Safety (CRITICAL)
**Decision:** Use `#[async_trait::async_trait(?Send)]` on all traits, no `spawn_blocking`

**Rationale:** `UIElement`, `UIAutomation`, `UITreeWalker` are `!Send` and `!Sync`. Attempting to move them across threads will cause compilation errors.

**Evidence:** Plan explicitly states:
- "ALL traits MUST use `#[async_trait::async_trait(?Send)]`"
- "NO `spawn_blocking` for UIA types"
- "Backend calls run on the same thread as UIAutomation instance"

### 2. Timeout Handling
**Decision:** Validate timeout at config level, not via `tokio::time::timeout`

**Rationale:** Since we cannot use `spawn_blocking`, we cannot wrap the backend call with `timeout()`. The timeout is validated during config validation.

**Evidence:** Plan specifies:
- `validate_toggle_config()` checks `timeout > 0 && timeout <= 3600`
- Cancellation checked via `CancellationToken::is_cancelled()`

### 3. Pattern Support
**Decision:** Support TogglePattern, ValuePattern, and LegacyIAccessible

**Rationale:** Different controls support different patterns. The implementation must check for support and use the appropriate pattern.

**Evidence:** Plan specifies:
- `has_toggle_pattern()` check
- `has_value_pattern()` check
- `has_legacy_ia()` check

### 4. Idempotency
**Decision:** On error, element state is NOT changed

**Rationale:** Idempotency ensures repeated calls with the same input produce the same result. On error, the element state should remain unchanged.

**Evidence:** Plan specifies:
- "On error, element state is NOT changed (atomicity)"
- "Repeated calls with same input produce same result"
- "No side effects (clipboard unchanged, no windows created)"

---

## 📊 Comparison with ClickTool/TypeTool

| Feature | ClickTool | TypeTool | ToggleTool |
|---------|-----------|----------|------------|
| Config struct | `ClickConfig` | `TypeConfig` | `ToggleConfig` |
| Error enum | `ClickError` (7 variants) | `TypeError` (7 variants) | `ToggleError` (10 variants) |
| Backend trait | `ClickBackend` | `TypeBackend` | `ToggleBackend` |
| Mock backend | `MockClickBackend` | `MockTypeBackend` | `MockToggleBackend` |
| Validation function | `validate_click_config()` | `validate_type_config()` | `validate_toggle_config()` |
| Async trait | `#[async_trait(?Send)]` | `#[async_trait(?Send)]` | `#[async_trait(?Send)]` |
| Unit tests | 7 tests | 5 tests | 8+ tests |
| Integration tests | 13 tests | 8 tests | 14 tests |

**Note:** ToggleTool has more error variants and more integration tests due to the complexity of toggle operations (multiple patterns, state checking, etc.)

---

## 🎯 Approval Recommendation

**Status:** ✅ **APPROVED** → Ready for code implementation

**Rationale:**
1. Plan fully complies with all project rules and architecture guidelines
2. Test plan is comprehensive and covers all scenarios
3. Documentation is consistent across all files
4. Pattern follows established ClickTool/TypeTool conventions
5. COM safety is properly addressed
6. All verification commands are specified

**Next Steps:**
1. Coder implements `src/core/toggle.rs`
2. Coder implements `src/runtime/backends/windows/toggle.rs`
3. Coder updates all mod files and lib exports
4. Coder writes all tests
5. Verification: `cargo test` + `cargo clippy -- -D warnings`
6. Architect signs off
7. ADR: `docs/adr/XXX-toggle-tool.md`

---

## 📝 Metadata

**Project:** smith-windows
**Module:** ToggleTool
**Validation Date:** 2026-04-22
**Validator:** smith-planner
**Status:** ✅ APPROVED → awaiting code implementation
**ADR:** docs/adr/XXX-toggle-tool.md (to be created after approval)
