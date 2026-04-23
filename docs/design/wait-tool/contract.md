## 📜 Contract: WaitTool | smith-windows

**🔹 Requirements (BEFORE call):**
- `WaitConfig` must have `timeout > 0`, `interval > 0`, `interval <= timeout`
- `UIAutomation` must be initialized successfully
- `root` element must be valid (desktop or window root)
- `selector` must be valid (AutomationId, Name, or ControlType)
- Cancellation token must be valid (CancellationToken is Clone + Send + Sync)

**🔸 Guarantees (AFTER):**
If `Ok(true)`: Element was found within timeout, COM state unchanged, no side effects
If `Ok(false)`: Element was NOT found within timeout, COM state unchanged, no side effects
If `Err(WaitError::InvalidConfig)`: No COM calls made, state unchanged
If `Err(WaitError::Cancelled)`: Operation was cancelled by token, state unchanged
If `Err(WaitError::ComError)`: COM error occurred, no state changes expected

**🚫 Prohibitions:**
- ❌ NO `unwrap()`, `expect()`, or `panic!` anywhere in library code
- ❌ NO global mutable state without Arc + Mutex
- ❌ NO changing contract without explicit agreement
- ❌ NO using `GetForegroundWindow()` or similar Win32 APIs
- ❌ NO using `spawn_blocking` for UIElement (it's !Send)
- ❌ NO clipboard or external process interactions

**⚡ Failures:**
- **Timeout**: Returns `Ok(false)` (not an error), operation completes naturally
- **Cancellation**: Returns `Err(WaitError::Cancelled)`, token checked before and during loop
- **Invalid config**: Returns `Err(WaitError::InvalidConfig)` before any COM calls
- **COM error**: Returns `Err(WaitError::ComError(e))`, error logged via tracing
- **Element not found**: Returns `Ok(false)` after timeout expires (normal behavior)

---
## 🗓️ For `/plan`: key validation checkpoints
- [x] Input validation happens in `validate_wait_config()` BEFORE any backend calls
- [x] Events/logs are sent via `tracing::info!()`, `tracing::error!()` on success/error
- [x] Errors handled via `thiserror`, not `panic!`
- [x] Mock backend uses `Arc<Mutex<MockState>>` for idempotency check
- [x] COM calls are synchronous (direct uiautomation calls), no spawn_blocking
