# /plan: ToggleTool Implementation | smith-windows

**Specification:** `docs/design/toggle-tool/specification.md`
**Contract:** `docs/design/toggle-tool/contract.md`
**Test Plan:** `docs/design/toggle-tool/test-plan.md`
**Author:** smith-architect
**Date:** 2026-04-22
**Status:** draft → approved → code

---

### [File] → [Entities] → [cfg-flags] → [Tests] → [Validation]

---

### `src/core/toggle.rs` (CORE MODULE)

**Entities:**
- `ToggleConfig { timeout: Duration, cancellation: CancellationToken }`
- `ToggleError` enum (10 variants via thiserror)
- `ToggleBackend` trait (5 methods)
- `MockToggleBackend` + `MockToggleState` (для тестов)
- `validate_toggle_config()` function
- `toggle_element_with_config()` function
- `set_radio_with_config()` function
- `set_toggle_with_config()` function
- `is_checked_with_config()` function
- `is_selected_with_config()` function

**cfg-flags:**
- `#[cfg(test)] mod tests` — inline unit tests
- `#[cfg(target_os = "windows")]` — platform checks (в бэкендах)

**Tests:**
- `test_validate_toggle_config_valid` — timeout 5s
- `test_validate_toggle_config_zero_timeout` — 0s → InvalidConfig
- `test_validate_toggle_config_large_timeout` — 3601s → InvalidConfig
- `test_validate_toggle_config_one_hour_edge` — 3600s → OK
- `test_validate_toggle_config_negative_timeout` — -1s → InvalidConfig
- `test_mock_backend_creation` — call_count=0
- `test_mock_backend_reset` — state reset
- `test_mock_backend_with_state` — custom state
- `test_mock_backend_idempotent_error` — repeated error doesn't change state

**Validation:**
- `timeout > 0` AND `timeout <= 3600` (1 hour)
- `cancellation` must be valid `CancellationToken`
- No validation for `ToggleBackend` methods (element state checked in backend)

---

### `src/runtime/backends/windows/toggle.rs` (WINDOWS BACKEND)

**Entities:**
- `ToggleBackendWindows` struct (один экземпляр)
- Implementation of `ToggleBackend` trait using `uiautomation`

**Methods:**
- `toggle_element()` — `UITogglePattern.toggle()` or `UIValuePattern.set_value()`
- `set_radio()` — `UISelectionPattern.select()` or `UIValuePattern.set_value()`
- `set_toggle()` — `UIValuePattern.set_value()` or `UITogglePattern.toggle()`
- `is_checked()` — `UITogglePattern.current_state` or `UISelectionPattern.is_selected`
- `is_selected()` — `UISelectionPattern.is_selected` or `UITogglePattern.current_state`

**Helper Methods (internal):**
- `check_element_validity()` — validate element not null, enabled, onscreen
- `get_toggle_state()` — get current toggle state from element
- `has_toggle_pattern()` — check if element supports TogglePattern
- `has_value_pattern()` — check if element supports ValuePattern
- `has_legacy_ia()` — check if element supports LegacyIAccessible

**cfg-flags:**
- `#[cfg(target_os = "windows")]` — only for Windows
- `#[cfg(not(target_os = "windows"))]` — stub in `unsupported.rs`

**Tests:**
- Integration tests in `tests/integration/toggle_tool/`
  - `test_toggle_element_success` — успешно тоггл
  - `test_set_radio_selected` — radio button selected
  - `test_set_radio_deselected` — radio button deselected
  - `test_set_toggle_on` — toggle switch on
  - `test_set_toggle_off` — toggle switch off
  - `test_is_checked_true` — checkbox checked
  - `test_is_checked_false` — checkbox unchecked
  - `test_is_selected_true` — radio selected
  - `test_is_selected_false` — radio deselected
  - `test_element_not_enabled` — disabled element → ElementNotEnabled
  - `test_element_offscreen` — offscreen element → ElementOffscreen
  - `test_pattern_not_supported` — no toggle pattern → ElementNotSupported
  - `test_timeout` — превышение таймаута → Timeout
  - `test_cancellation` — отмена операции → Cancelled

**Validation:**
- `element.is_enabled()` → `ElementNotEnabled`
- `element.is_offscreen()` → `ElementOffscreen`
- `has_toggle_pattern()` OR `has_value_pattern()` OR `has_legacy_ia()` → `ElementNotSupported`

---

### `src/runtime/backends/windows/mod.rs` (EXPORT)

**Add:**
```rust
pub mod toggle;
pub use toggle::ToggleBackendWindows;
```

---

### `src/runtime/backends/unsupported.rs` (STUB)

**Add (for ToggleTool):**
- `ToggleBackendUnsupported` struct
- Implementation of `ToggleBackend` trait returning `ToggleError::UnsupportedPlatform`

---

### `src/core/mod.rs` (EXPORT)

**Add:**
```rust
pub mod toggle;
pub use toggle::{ToggleConfig, ToggleError, ToggleBackend, MockToggleBackend};
```

---

### `src/runtime/backends/mod.rs` (PLATFORM SELECTOR)

**Add:**
```rust
#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(not(target_os = "windows"))]
pub mod unsupported;
```

---

### `src/lib.rs` (LIB EXPORT)

**Add:**
```rust
pub use crate::core::toggle::{ToggleConfig, ToggleError, ToggleBackend, MockToggleBackend};
pub use crate::runtime::backends::windows::toggle::ToggleBackendWindows;
```

---

### Documentation

**Documents (already created):**
- `docs/design/toggle-tool/specification.md` — ✅
- `docs/design/toggle-tool/contract.md` — ✅
- `docs/design/toggle-tool/test-plan.md` — ✅
- `docs/design/toggle-tool/brief.md` — ✅

**MUST UPDATE:**
- `docs/design/toggle-tool/brief.md` — this plan (add implementation details)
- `README.md` — add ToggleTool to Key Features and Modules
- `CHANGELOG.md` — add ToggleTool entry in [Unreleased]
- `ARCHITECTURE.md` — add ToggleTool Architecture section
- `tools/bundle_context.rs` — add path `docs/design/toggle-tool/`

---

### Verification (AFTER CODE)

```bash
# Unit tests
cargo test --lib

# Integration tests
cargo test --test integration

# Linting
cargo clippy -- -D warnings

# Formatting
cargo fmt --all

# Compilation check
cargo check
```

---

### Notes

**COM Safety (CRITICAL):**
- `UIElement` is `!Send` and `!Sync` — cannot be moved between threads
- `UIAutomation` is `!Send` and `!Sync` — cannot be moved between threads
- ALL traits MUST use `#[async_trait::async_trait(?Send)]`
- NO `spawn_blocking` for UIA calls

**Timeout Handling:**
- Since we can't use `spawn_blocking`, timeout is checked via cancellation token
- The backend call itself is synchronous and does not block the async runtime

**Idempotency:**
- On error, element state is NOT changed
- Repeated calls with same data produce same result

---

### Date and Status

**Start:** 2026-04-22
**End:** [待填写]
**Status:** draft → approved → code
**ADR:** docs/adr/XXX-toggle-tool.md
