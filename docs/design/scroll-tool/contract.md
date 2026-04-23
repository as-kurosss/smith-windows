## 📜 Contract: ScrollTool | smith-windows

**🔹 Requirements (BEFORE call):**
1. `element` must be a valid UIAutomation element (not null)
2. `config` must have `timeout` > 0 and <= 1 hour
3. `direction` must be "vertical" or "horizontal" (case-insensitive)
4. `unit` must be "line", "page", or "pixel"
5. `amount` must be in valid range (1-100 for wheel, -10000 to 10000 for pixels)

**🔸 Guarantees (AFTER):**
- If `Ok`: element scroll position changed OR mouse wheel event sent
- If `Err`: no system state changed, error type returned
- Logging records operation start, progress, and result

**🚫 Prohibitions:**
1. No `unwrap()`, `expect()`, `panic!` in library code
2. No direct COM calls (`CoInitializeEx`, `CoCreateInstance`, `BSTR`, `VARIANT`)
3. No `GetForegroundWindow()` - use `is_enabled()`/`is_offscreen()` via UIA
4. No global mutable state without `Arc` + explicit context
5. No changing contract without discussion

**⚡ Failures:**
- **Timeout**: Returns `ScrollError::Timeout`, no state changed
- **Cancellation**: Returns `ScrollError::Cancelled`, no state changed
- **Invalid config**: Returns `ScrollError::InvalidConfig`, no state changed
- **Element not found**: Returns `ScrollError::ElementNotFound`, no state changed
- **Element not enabled**: Returns `ScrollError::ElementNotEnabled`, no state changed
- **Element offscreen**: Returns `ScrollError::ElementOffscreen`, no state changed
- **Pattern not supported**: Returns `ScrollError::PatternNotSupported`, tries synthetic fallback
- **COM error**: Returns `ScrollError::ComError`, no state changed

---
## 🗓️ For `/plan`: key validation checkpoints

- [x] Input validation in `src/core/scroll.rs` before backend calls
- [x] Events/logs sent via `tracing::info!`/`tracing::error!` at operation boundaries
- [x] Errors handled via `ScrollError` enum, never `panic!`
- [x] Both programmatic (UIA patterns) and synthetic (mouse wheel) methods implemented
- [x] Fallback from programmatic to synthetic if pattern unavailable
