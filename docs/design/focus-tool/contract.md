## 📜 Contract: FocusTool | smith-windows

**🔹 Requirements (BEFORE call):**
- `element` must be a valid `UIElement` obtained via `uiautomation` crate
- `element` must have `WindowPattern` available (call `get_pattern::<UIWindowPattern>()`)
- `config.timeout` must be `> 0` and `<= Duration::from_secs(3600)` (1 hour)
- `config.cancellation` must be a valid `CancellationToken`
- Element must be `is_enabled() == true` and `is_offscreen() == false`

**🔸 Guarantees (AFTER):**
- If `Ok`: window containing element is activated (foreground), element validated (enabled, onscreen), no side effects beyond activation
- If `Err`: no state changes (idempotent), element window not modified, logging records specific failure reason

**🚫 Prohibitions:**
- DO NOT use `spawn_blocking` for UIA calls (UIElement is `!Send`/`!Sync`)
- DO NOT call `CoInitializeEx`, `CoCreateInstance`, `BSTR`, `VARIANT` directly in backends
- DO NOT use `GetForegroundWindow()` or similar Win32 APIs for window activation
- DO NOT use `unwrap()`, `expect()`, `panic!` anywhere in `src/`
- DO NOT modify element properties directly (focus only activates window)
- DO NOT block the async runtime (COM calls are synchronous)

**⚡ Failures:**
- `FocusError::Timeout`: operation exceeded `config.timeout`, window not activated
- `FocusError::Cancelled`: cancellation token fired, partial state reset
- `FocusError::ElementNotEnabled`: element is disabled, window not activated
- `FocusError::ElementOffscreen`: element is offscreen, window not activated
- `FocusError::WindowPatternNotAvailable`: element lacks `WindowPattern`, window not activated
- `FocusError::InvalidConfig`: validation failed (timeout bounds), no attempt made
- `FocusError::ComError`: COM/WinAPI error during window activation, logging records error

---
## 🗓️ For `/plan`: key validation checkpoints
- [ ] Input validation happens in `validate_config()` and `validate_element_ready()` BEFORE any backend calls
- [ ] Events/logs sent via `tracing::info!()` on success, `tracing::error!()` on failures
- [ ] Errors handled via `FocusError` enum (thiserror), not `panic!` or `unwrap()`
