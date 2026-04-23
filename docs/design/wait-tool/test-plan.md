## 🧪 Test Plan: WaitTool | smith-windows

**✅ Positive:**
- Element exists from start → WaitConfig { timeout: 5s, interval: 100ms } → `Ok(true)` within 100ms
- Element appears after delay → WaitConfig { timeout: 2s, interval: 200ms } → `Ok(true)` after delay
- Wait for absence: element initially exists then disappears → WaitConfig { wait_for: Absence } → `Ok(true)` after disappearance
- Timeout with valid config → WaitConfig { timeout: 100ms, interval: 10ms } → `Ok(false)` after timeout

**🔄 Boundary:**
- `interval = 1ms` (minimum practical) → `Ok(true)` if element exists
- `timeout = 1ms` (minimum valid) → `Ok(false)` (too short for meaningful wait)
- `timeout == interval` → Exactly one check, returns `Ok(false)` if not found
- `timeout = 3600s` (1 hour, max valid) → Valid config, waits up to 1 hour
- `interval > timeout` → `Err(WaitError::InvalidConfig)`
- `timeout = 0` → `Err(WaitError::InvalidConfig)`
- `interval = 0` → `Err(WaitError::InvalidConfig)`

**❌ Negative:**
- Invalid selector (empty string) → `Err(WaitError::InvalidConfig)` or `Ok(false)` (element not found)
- Cancellation during wait → `Err(WaitError::Cancelled)`
- COM error during find_first() → `Err(WaitError::ComError(e))`
- Offscreen element → `Ok(false)` (element exists but is offscreen, or find_first returns null)
- Disabled element → `Ok(false)` (element exists but is disabled, or find_first returns null)

**🔍 Mandatory checks:**
- [x] On `Err`, state unchanged (no COM calls made for InvalidConfig)
- [x] No duplicate events/logs (each operation logs once: start, success/error)
- [x] No `unwrap()`, `panic!`, or blocking in async (use `?` and explicit match)
- [x] Mock backend state is idempotent on `Err` (repeated calls don't change state)
- [x] Cancellation token is checked before and after each find_first() call
- [x] UIElement is !Send, backend uses direct calls (no spawn_blocking)

---
## 🗓️ For `/plan`: tests as steps
- [x] Create `tests/wait_tests.rs` with `#[serial_test::serial]` attribute
- [x] Implement `#[cfg(test)] mod tests` in `src/core/wait.rs` (8 unit tests)
- [x] Test base: element exists, element absent, timeout
- [x] Test boundary: min/max timeout, zero interval, interval > timeout
- [x] Test negative: cancellation, COM error, invalid config
- [x] Run `cargo test -- wait` (verify all tests pass)
- [x] Run `cargo test -- wait --nocapture` (verify logs)
