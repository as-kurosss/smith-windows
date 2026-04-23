## 🧪 Test Plan: FocusTool | smith-windows

**✅ Positive:** valid element + valid config → window activated, `Ok(())`
- Element is enabled, onscreen, has `WindowPattern`
- `config.timeout = Duration::from_secs(5)`
- Expected: window activated successfully

**🔄 Boundary:** 
- `config.timeout = Duration::from_secs(1)` (minimum valid) → `Ok(())`
- `config.timeout = Duration::from_secs(3600)` (maximum valid) → `Ok(())`
- Element already active window → `Ok(())` (idempotent, no error)
- `config.cancellation` already cancelled → `FocusError::Cancelled`

**❌ Negative:** 
- Empty/zero timeout → `FocusError::InvalidConfig`
- Timeout > 1 hour → `FocusError::InvalidConfig`
- Element disabled (`is_enabled() == false`) → `FocusError::ElementNotEnabled`
- Element offscreen (`is_offscreen() == true`) → `FocusError::ElementOffscreen`
- Element lacks `WindowPattern` → `FocusError::WindowPatternNotAvailable`
- `config.cancellation` fires during wait → `FocusError::Cancelled`
- Timeout occurs during activation → `FocusError::Timeout`

**🔍 Mandatory checks:**
- [ ] On `Err`, element window not activated, state unchanged
- [ ] No duplicate events/logs (single log per operation)
- [ ] No `unwrap()`, `panic!`, or blocking in async functions
- [ ] `MockFocusBackend` idempotency: repeated `Err` calls don't change state
- [ ] COM calls run on STA thread (compile-time check via `#[async_trait(?Send)]`)
- [ ] Validation occurs before backend call in `focus_with_config()`

---
## 🗓️ For `/plan`: tests as steps
- [ ] Create `src/core/focus.rs` with unit tests (`#[cfg(test)] mod tests`)
- [ ] Test `validate_config()`: valid, zero timeout, large timeout
- [ ] Test `validate_element_ready()`: enabled/onscreen, disabled, offscreen
- [ ] Test `MockFocusBackend`: creation, state, idempotency on error
- [ ] Create `tests/integration/focus.rs` with end-to-end tests
- [ ] Test timeout scenario with short timeout
- [ ] Test cancellation scenario with token
- [ ] Run `cargo test -- --nocapture`
