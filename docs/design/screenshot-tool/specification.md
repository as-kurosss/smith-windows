## 📐 Specification: ScreenshotTool | smith-windows

**🎯 Purpose:** Capture screenshots of the Windows desktop in PNG format via GDI/GDI+ API, supporting three modes: full screen, specific window, and custom region.

**📥 Input:**
- `mode: ScreenshotMode` | specifies target: `Screen`, `Window(UIElement)`, or `Region{x,y,width,height}` | `ScreenshotMode::Window(element)`
- `config: ScreenshotConfig` | timeout, format, quality settings | `ScreenshotConfig { timeout: Duration::from_secs(5) }`
- `cancellation: CancellationToken` | async cancellation token | created via `tokio_util::sync::CancellationToken`

**📤 Output:**
- `Result<Vec<u8>, ScreenshotError>` | PNG image bytes in memory | `Ok(vec![89, 80, 78, 71, ...])`
- On success: PNG image data in memory, no side effects
- On error: no memory allocated, error type indicates failure reason

**⚠️ Boundaries:**
- Invalid region: `width <= 0` or `height <= 0` → `ScreenshotError::InvalidRegion`
- Negative coordinates: `x < 0` or `y < 0` → `ScreenshotError::InvalidRegion`
- Timeout: `timeout <= Duration::ZERO` → `ScreenshotError::InvalidConfig`
- Large timeout: `timeout > 1 hour` → `ScreenshotError::InvalidConfig`
- Element not found: `UIElement` invalid → `ScreenshotError::ElementNotFound`
- COM/GDI error: WinAPI call failure → `ScreenshotError::CaptureFailed(String)`
- Cancellation: during capture → `ScreenshotError::Cancelled`
- Timeout during capture → `ScreenshotError::Timeout`

**✅ Success criteria:**
- [ ] All boundary cases handled without panics
- [ ] Memory safely allocated/freed on success/error
- [ ] PNG bytes conform to standard format (magic bytes `89 50 4E 47`)
- [ ] COM/GDI calls isolated via `tokio::task::spawn_blocking`
- [ ] Validation occurs BEFORE backend call

---
## 🗓️ Implementation plan (for `/plan`)
- [ ] Create/update file: `src/core/screenshot.rs`
- [ ] Implement types: `ScreenshotMode`, `ScreenshotConfig`, `ScreenshotError`, `ScreenshotBackend`
- [ ] Implement `validate_screenshot_config()` and `validate_screenshot_mode()` in core
- [ ] Implement `ScreenshotBackend::capture()` with GDI/GDI+ calls via `spawn_blocking`
- [ ] Add tests: positive (screen capture), boundary (zero timeout), negative (invalid region)
- [ ] Update docs: `ARCHITECTURE.md` add ScreenshotTool section
- [ ] Checks: `cargo test`, `cargo clippy -- -D warnings`
