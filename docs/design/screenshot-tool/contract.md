## 📜 Contract: ScreenshotTool | smith-windows

**🔹 Requirements (BEFORE call):**
- `mode` must be valid: `Region` with `x >= 0`, `y >= 0`, `width > 0`, `height > 0`; `Window` with valid `UIElement`
- `config` must be valid: `timeout > Duration::ZERO` and `timeout <= 1 hour`
- `cancellation` must be an initialized token
- Platform: Windows only (non-Windows returns `ScreenshotError::UnsupportedPlatform`)

**🔸 Guarantees (AFTER):**
- If `Ok(Vec<u8>)`: PNG image captured, memory contains valid PNG data, no side effects
- If `Err(ScreenshotError::InvalidRegion)`: no memory allocated, error detected during validation
- If `Err(ScreenshotError::InvalidConfig)`: no memory allocated, error detected during validation
- If `Err(ScreenshotError::ElementNotFound)`: no memory allocated, window element invalid
- If `Err(ScreenshotError::Timeout)`: capture stopped, no memory allocated
- If `Err(ScreenshotError::Cancelled)`: capture stopped, no memory allocated
- If `Err(ScreenshotError::CaptureFailed(String))`: no memory allocated, WinAPI failure logged
- If `Err(ScreenshotError::UnsupportedPlatform)`: stub returned, no system calls made

**🚫 Prohibitions:**
- ❌ Use of `unwrap()`, `expect()`, `panic!` anywhere in implementation
- ❌ Global mutable state without `Arc` + `Mutex`/`RwLock`
- ❌ Direct GDI/GDI+ calls without `tokio::task::spawn_blocking`
- ❌ Modification of `ScreenshotMode` or `ScreenshotConfig` inside function
- ❌ Reading pixels from screen via `GetPixel()` (inefficient, use `BitBlt`/`StretchBlt`)
- ❌ Using `GetForegroundWindow()` for window capture (use UIElement from automation session)

**⚡ Failures:**
- **Timeout**: via `tokio::time::timeout()` → `ScreenshotError::Timeout`, state unchanged
- **Cancellation**: check `cancellation.is_cancelled()` before and during capture → `ScreenshotError::Cancelled`
- **Invalid region/config**: validation in `validate_screenshot_*()` → `ScreenshotError::InvalidRegion`/`InvalidConfig` BEFORE backend
- **GDI/GDI+ errors**: `GetLastError()` → `ScreenshotError::CaptureFailed(String)`
- **Unsupported platform**: stub returns `ScreenshotError::UnsupportedPlatform` via `#[cfg(not(target_os = "windows"))]`

---
## 🗓️ For `/plan`: key validation checkpoints
- [ ] Input validation in `src/core/screenshot.rs`: `validate_screenshot_config()`, `validate_screenshot_mode()`
- [ ] Events/logs via `tracing::info!()`/`tracing::error!()` at start/completion
- [ ] Errors via `ScreenshotError` (`thiserror`), never `panic!`
- [ ] GDI/GDI+ calls in `src/runtime/backends/windows/screenshot.rs` via `tokio::task::spawn_blocking`
- [ ] Platform check: `#[cfg(target_os = "windows")]` for implementation, `#[cfg(not(target_os = "windows"))]` for stub
