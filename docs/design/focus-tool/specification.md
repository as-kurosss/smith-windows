## 📐 Specification: FocusTool | smith-windows

**🎯 Purpose:** Activate a UI element's window before interaction to ensure input operations (text, click) target the correct window, not the currently focused one. Solves the problem: "When a script starts, the target window may be in the background; InputTextTool would type into the active window instead of the target."

**📥 Input:**
- `element: &UIElement` | valid UIA element with a window parent (must have `WindowPattern`) | element inside target application window
- `config: FocusConfig` | valid configuration with timeout and cancellation | `FocusConfig { timeout: Duration::from_secs(5), cancellation: CancellationToken }`

**📤 Output:**
- `Result<(), FocusError>` | success or specific error
- On success: window containing the element is activated (brought to foreground), element is validated (enabled, onscreen)
- On error: no state changes, logging records failure cause

**🔄 Validation steps:**
1. `validate_config()` - config timeout bounds, cancellation token validity
2. `validate_element_ready()` - check `is_enabled()` and `is_offscreen()`
3. `activate_window()` - get `WindowPattern`, call `WaitForInputIdle()` then activate

**⚠️ Boundaries:**
- Timeout: operation not completed within `config.timeout` → `FocusError::Timeout`
- Cancellation: `cancellation.is_cancelled()` during execution → `FocusError::Cancelled`
- Element not enabled: `is_enabled() == false` → `FocusError::ElementNotEnabled`
- Element offscreen: `is_offscreen() == true` → `FocusError::ElementOffscreen`
- Window pattern not available: element lacks `WindowPattern` → `FocusError::WindowPatternNotAvailable`
- Window already active: no error, return success (idempotent)
- Empty/zero timeout: `validate_config()` returns `FocusError::InvalidConfig`
- Timeout > 1 hour: `validate_config()` returns `FocusError::InvalidConfig`

**✅ Success criteria:**
- [ ] All boundary scenarios handled without panics or `unwrap()`
- [ ] State remains unchanged on error (idempotency)
- [ ] Logging via `tracing` records success/failure cause
- [ ] COM calls run on STA thread (no `spawn_blocking`, UIElement is `!Send`/`!Sync`)
- [ ] Validation occurs BEFORE backend call
- [ ] Window activation attempts without blocking (non-modal)

---
## 🗓️ Implementation plan (for `/plan`)
- [ ] Create `src/core/focus.rs`: `FocusConfig`, `FocusError`, validation functions, `FocusBackend` trait, `MockFocusBackend`
- [ ] Create `src/runtime/backends/windows/focus.rs`: Windows implementation via `uiautomation::patterns::UIWindowPattern`
- [ ] Export `FocusTool` from `src/lib.rs`
- [ ] Add unit tests: valid config, invalid config, element not enabled, offscreen, timeout, cancellation
- [ ] Add integration tests: end-to-end focus flow
- [ ] Update `README.md`, `CHANGELOG.md`, run `context_bundle`
- [ ] Checks: `cargo test`, `cargo clippy -- -D warnings`
