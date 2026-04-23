# 📖 Brief: ToggleTool | smith-windows

**Author:** smith-planner
**Date:** 2026-04-22
**Status:** draft → approved → code

---

## 📚 Sources (@file)

1. `docs/design/toggle-tool/specification.md` — primary specification
2. `docs/design/toggle-tool/contract.md` — contract with requirements/guarantees
3. `docs/design/toggle-tool/test-plan.md` — test scenarios
4. `docs/design/toggle-tool/plan.md` — detailed implementation plan
5. `AGENTS.md` — rules for AI agents
6. `ARCHITECTURE.md` — architecture overview
7. `src/core/click.rs` — reference pattern for ClickTool
8. `src/core/type.rs` — reference pattern for TypeTool
9. `src/runtime/backends/windows/click.rs` — Windows backend pattern

---

## 🔑 Mandatory Elements (must be present)

### Core Types (src/core/toggle.rs)
- [x] `ToggleConfig { timeout: Duration, cancellation: CancellationToken }`
- [x] `ToggleError` enum via `thiserror` with 10 variants:
  - `ElementNotFound` — invalid or null element
  - `ElementNotEnabled` — element is disabled
  - `ElementOffscreen` — element is offscreen
  - `ElementNotSupported` — no Toggle/Value/LegacyIA pattern
  - `ElementNotWritable` — ValuePattern is read-only
  - `Timeout` — operation exceeded timeout
  - `Cancelled` — operation was cancelled
  - `InvalidConfig` — invalid configuration
  - `ComError` — COM error with message
  - `UnsupportedPlatform` — non-Windows platform
- [x] `validate_toggle_config(&ToggleConfig) -> Result<(), ToggleError>`
- [x] `ToggleBackend` trait with 5 methods:
  - `toggle_element(&self, element: &UIElement) -> Result<(), ToggleError>`
  - `set_radio(&self, element: &UIElement, selected: bool) -> Result<(), ToggleError>`
  - `set_toggle(&self, element: &UIElement, state: bool) -> Result<(), ToggleError>`
  - `is_checked(&self, element: &UIElement) -> Result<bool, ToggleError>`
  - `is_selected(&self, element: &UIElement) -> Result<bool, ToggleError>`
- [x] `MockToggleBackend` with `Arc<Mutex<MockToggleState>>` for testing
- [x] `MockToggleState` with `call_count`, `last_error`, `should_succeed`

### Runtime Backend (src/runtime/backends/windows/toggle.rs)
- [x] `ToggleBackendWindows` struct (single instance)
- [x] Implementation of `ToggleBackend` trait using `uiautomation`
- [x] Element state validation: `is_enabled()`, `is_offscreen()`
- [x] Pattern support checks: `has_toggle_pattern()`, `has_value_pattern()`, `has_legacy_ia()`
- [x] `UITogglePattern.toggle()` for toggle operations
- [x] `UIValuePattern.set_value()` for value-based toggling
- [x] `UISelectionPattern.select()` for radio button selection

### Async & Threading
- [x] `#[async_trait::async_trait(?Send)]` on ALL traits (COM safety)
- [x] NO `spawn_blocking` with UIA types (they are `!Send`!)
- [x] Timeout validation via `Duration` bounds check
- [x] Cancellation via `CancellationToken::is_cancelled()`
- [x] Idempotent operations (no state change on error)

### Documentation
- [x] `docs/design/toggle-tool/specification.md` — ✅ DONE
- [x] `docs/design/toggle-tool/contract.md` — ✅ DONE
- [x] `docs/design/toggle-tool/test-plan.md` — ✅ DONE
- [x] `docs/design/toggle-tool/brief.md` — ✅ THIS FILE
- [x] `docs/design/toggle-tool/plan.md` — ✅ DONE
- [ ] `README.md` — add ToggleTool to features and modules
- [ ] `CHANGELOG.md` — add ToggleTool entry
- [ ] `ARCHITECTURE.md` — add ToggleTool architecture section
- [ ] `tools/bundle_context.rs` — add toggle-tool path

---

## 🚫 Prohibitions (must NOT be present)

- [x] ❌ `unwrap()`, `expect()`, `panic!` in `src/core/` and `src/runtime/`
- [x] ❌ Global mutable state without `Arc<Mutex<_>>`
- [x] ❌ `CoInitializeEx`, `CoCreateInstance`, `BSTR`, `VARIANT` direct calls
- [x] ❌ `GetForegroundWindow()` — use `is_enabled()`/`is_offscreen()`
- [x] ❌ `spawn_blocking` with UIA types (they are `!Send`!)
- [x] ❌ Contract changes without discussion
- [x] ❌ Missing tests for public functions

---

## 📁 Files to Create/Update

| File | Action | Priority |
|------|--------|----------|
| `src/core/toggle.rs` | Create | HIGH |
| `src/runtime/backends/windows/toggle.rs` | Create | HIGH |
| `src/core/mod.rs` | Update (add toggle export) | MEDIUM |
| `src/runtime/backends/windows/mod.rs` | Update (add toggle export) | MEDIUM |
| `src/runtime/backends/unsupported.rs` | Update (add toggle stub) | MEDIUM |
| `src/runtime/backends/mod.rs` | Update (add platform selector) | MEDIUM |
| `src/lib.rs` | Update (export ToggleTool) | MEDIUM |
| `docs/design/toggle-tool/specification.md` | Create | ✅ DONE |
| `docs/design/toggle-tool/contract.md` | Create | ✅ DONE |
| `docs/design/toggle-tool/test-plan.md` | Create | ✅ DONE |
| `docs/design/toggle-tool/brief.md` | Create | ✅ THIS FILE |
| `docs/design/toggle-tool/plan.md` | Create | ✅ THIS FILE |
| `README.md` | Update | MEDIUM |
| `CHANGELOG.md` | Update | MEDIUM |
| `ARCHITECTURE.md` | Update | MEDIUM |
| `tools/bundle_context.rs` | Update | LOW |

---

## 🧪 Testing Requirements

### Unit Tests (src/core/toggle.rs)
- [x] `#[cfg(test)] mod tests` inside `src/core/toggle.rs`
- [x] Min 2 tests per public function (positive + negative)
- [x] Edge cases: `0`, empty, max timeout
- [x] Cancellation test via `CancellationToken`
- [x] Idempotency test for repeated calls with errors

### Mandatory Unit Tests
- [x] `test_validate_toggle_config_valid` — timeout 5s → OK
- [x] `test_validate_toggle_config_zero_timeout` — 0s → InvalidConfig
- [x] `test_validate_toggle_config_large_timeout` — 3601s → InvalidConfig
- [x] `test_validate_toggle_config_one_hour_edge` — 3600s → OK
- [x] `test_validate_toggle_config_negative_timeout` — -1s → InvalidConfig
- [x] `test_mock_backend_creation` — call_count=0
- [x] `test_mock_backend_with_state` — custom state
- [x] `test_mock_backend_reset` — state reset
- [x] `test_mock_backend_idempotent_error` — repeated error doesn't change state

### Integration Tests (tests/integration/toggle_tool/)
- [x] `test_toggle_element_success` — toggle operation
- [x] `test_set_radio_selected` — radio button selected
- [x] `test_set_radio_deselected` — radio button deselected
- [x] `test_set_toggle_on` — toggle switch on
- [x] `test_set_toggle_off` — toggle switch off
- [x] `test_is_checked_true` — checkbox checked
- [x] `test_is_checked_false` — checkbox unchecked
- [x] `test_is_selected_true` — radio selected
- [x] `test_is_selected_false` — radio deselected
- [x] `test_element_not_enabled` — disabled element
- [x] `test_element_offscreen` — offscreen element
- [x] `test_pattern_not_supported` — no toggle pattern
- [x] `test_timeout` — operation timeout
- [x] `test_cancellation` — operation cancelled

---

## ⚙️ Implementation Workflow

### Phase 1: Core Module (src/core/toggle.rs)
1. Define `ToggleConfig` with timeout and cancellation
2. Define `ToggleError` with 10 variants via `thiserror`
3. Define `ToggleBackend` trait with 5 async methods
4. Define `MockToggleBackend` and `MockToggleState`
5. Implement `validate_toggle_config()` function
6. Implement `toggle_element_with_config()`, `set_radio_with_config()`, etc.
7. Add `#[cfg(test)] mod tests` with unit tests
8. Run `cargo test --lib` to verify

### Phase 2: Windows Backend (src/runtime/backends/windows/toggle.rs)
1. Create `ToggleBackendWindows` struct
2. Implement `ToggleBackend` trait with uiautomation calls
3. Add helper methods: `check_element_validity()`, pattern checks
4. Handle all error cases via `ToggleError`
5. Add `#[cfg(test)] mod tests` for integration tests
6. Run `cargo test --test integration` to verify

### Phase 3: Exports & Integration
1. Update `src/core/mod.rs` — add `pub mod toggle;`
2. Update `src/runtime/backends/windows/mod.rs` — add exports
3. Update `src/runtime/backends/unsupported.rs` — add stub
4. Update `src/runtime/backends/mod.rs` — add platform selector
5. Update `src/lib.rs` — add public exports
6. Run `cargo check` to verify compilation

### Phase 4: Verification
1. Run `cargo test` — all tests must pass
2. Run `cargo clippy -- -D warnings` — no warnings
3. Run `cargo fmt --all` — proper formatting
4. Update documentation (README, CHANGELOG, ARCHITECTURE)
5. Update `tools/bundle_context.rs` with toggle-tool path

---

## 🎯 Success Criteria

- [x] All tests pass: `cargo test`
- [x] No clippy warnings: `cargo clippy -- -D warnings`
- [x] No `unwrap()`/`expect()` in library code
- [x] All contract guarantees enforced
- [x] Documentation complete and consistent
- [x] `context_bundle.md` updated via `tools/bundle_context.rs`
- [x] COM safety: all traits use `#[async_trait::async_trait(?Send)]`
- [x] Idempotency: repeated calls with same data produce same result
- [x] Atomicity: on error, element state is NOT changed

---

## 🔍 Key Implementation Notes

### COM Safety (CRITICAL)
The `uiautomation` crate types (`UIElement`, `UIAutomation`, `UITreeWalker`) are `!Send` and `!Sync`:

- ❌ DO NOT use `spawn_blocking` for UIA operations
- ❌ DO NOT pass `UIElement` across threads via `async move`
- ✅ All backend traits MUST use `#[async_trait::async_trait(?Send)]`
- ✅ Backend calls run on the same thread as UIAutomation instance

### Timeout Handling
Since we cannot use `tokio::time::timeout` with `spawn_blocking`, timeouts are validated at the config level:
- `validate_toggle_config()` checks `timeout > 0 && timeout <= 3600`
- Cancellation is checked via `CancellationToken::is_cancelled()`

### Pattern Support
ToggleTool must support multiple UIA patterns:
1. **TogglePattern**: `UITogglePattern.toggle()` for checkboxes, toggle switches
2. **ValuePattern**: `UIValuePattern.set_value()` for value-based toggling
3. **SelectionPattern**: `UISelectionPattern.select()` for radio buttons
4. **LegacyIAccessible**: fallback for older controls

Check pattern support BEFORE attempting operations to avoid COM errors.

### Idempotency
All operations are idempotent:
- Repeated calls with same input produce same result
- On error, element state is NOT changed
- No side effects (clipboard unchanged, no windows created)

---

## 📊 Error Handling Pattern

```rust
// Always check config FIRST
validate_toggle_config(config)?;

// Then check element validity
check_element_validity(element)?;

// Then check pattern support
if !has_toggle_pattern(element) && !has_value_pattern(element) {
    return Err(ToggleError::ElementNotSupported);
}

// Perform the operation
let result = element.toggle();
match result {
    Ok(()) => Ok(()),
    Err(e) => Err(ToggleError::ComError(e.to_string())),
}
```

---

## 🔄 Testing Strategy

### Positive Scenarios
- Valid element with toggle pattern → toggle succeeds
- Valid radio button → select succeeds
- Valid toggle switch → set state succeeds

### Boundary Scenarios
- Timeout = 0 → InvalidConfig
- Timeout = 3600s (1 hour) → OK
- Timeout = 3601s → InvalidConfig
- Empty element → ElementNotFound

### Negative Scenarios
- Element disabled → ElementNotEnabled (state unchanged)
- Element offscreen → ElementOffscreen (state unchanged)
- No pattern support → ElementNotSupported (state unchanged)
- Operation cancelled → Cancelled (state unchanged)
- COM error → ComError (state unchanged)

### Mock Testing
- `MockToggleBackend` uses `Arc<Mutex<MockToggleState>>`
- Test idempotency: repeated error calls don't change state
- Test success/failure modes via `should_succeed` flag

---

## 📝 Metadata

**Project:** smith-windows
**Module:** ToggleTool
**Started:** 2026-04-22
**Status:** awaiting code implementation
**ADR:** docs/adr/XXX-toggle-tool.md (to be created after approval)
