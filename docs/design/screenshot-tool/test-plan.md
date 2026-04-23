## 🧪 Test Plan: ScreenshotTool | smith-windows

**✅ Positive:**
- `ScreenshotMode::Screen` with valid config → `Ok(Vec<u8>)` with PNG magic bytes (`89 50 4E 47 0D 0A 1A 0A`)
- `ScreenshotMode::Window(ui_element)` with valid element → `Ok(Vec<u8>)` with PNG data
- `ScreenshotMode::Region{x: 0, y: 0, width: 100, height: 100}` → `Ok(Vec<u8>)` with partial screen PNG
- Timeout = 5s, fast capture → `Ok(Vec<u8>)` within timeout
- Timeout = 1ms, slow capture → `ScreenshotError::Timeout`

**🔄 Boundary:**
- `timeout = Duration::ZERO` → `ScreenshotError::InvalidConfig` (validation)
- `timeout = Duration::from_millis(1)` → `Ok(())` or `Timeout` depending on speed
- `timeout > 1 hour` (`Duration::from_secs(3601)`) → `ScreenshotError::InvalidConfig`
- `Region{x: -1, y: 0, width: 100, height: 100}` → `ScreenshotError::InvalidRegion`
- `Region{x: 0, y: 0, width: 0, height: 100}` → `ScreenshotError::InvalidRegion`
- `Region{x: 0, y: 0, width: 100, height: 0}` → `ScreenshotError::InvalidRegion`
- Invalid `UIElement` (nil/invalid handle) → `ScreenshotError::ElementNotFound`
- Maximum valid region (desktop size) → `Ok(Vec<u8>)` with full desktop PNG

**❌ Negative:**
- `ScreenshotMode::Window(invalid_element)` → `ScreenshotError::ElementNotFound`
- `ScreenshotMode::Region{x: 10000, y: 10000, width: 100, height: 100}` (off-screen) → `ScreenshotError::CaptureFailed`
- Cancellation during capture (`cancellation.cancel()`) → `ScreenshotError::Cancelled`
- Timeout during capture → `ScreenshotError::Timeout`
- GDI error (simulate via mock) → `ScreenshotError::CaptureFailed(String)`
- Non-Windows platform → `ScreenshotError::UnsupportedPlatform`

**🔍 Mandatory checks:**
- [ ] On `Err`, no memory allocated (checked via repeat call with same data → same error)
- [ ] No duplicate events/logs (checked via `cargo test -- --nocapture`)
- [ ] No `unwrap()`/`panic!`/blocking in async (cargo clippy -- -D warnings)
- [ ] `validate_screenshot_config()` and `validate_screenshot_mode()` called BEFORE backend (unit test)
- [ ] PNG output has correct magic bytes (test verifies first 8 bytes)
- [ ] Cancellation checked via `cancellation.is_cancelled()` before and during capture
- [ ] GDI calls isolated via `tokio::task::spawn_blocking` in backend
- [ ] Platform stub returns `UnsupportedPlatform` on non-Windows

---
## 🗓️ For `/plan`: tests as steps
- [ ] Create `mod tests` inside `src/core/screenshot.rs` and `tests/integration/screenshot_tests.rs`
- [ ] Implement `test_capture_screen`: `ScreenshotMode::Screen` → `Ok(Vec<u8>)` with PNG bytes
- [ ] Implement `test_capture_window`: `ScreenshotMode::Window(element)` → `Ok(Vec<u8>)`
- [ ] Implement `test_capture_region`: `ScreenshotMode::Region{...}` → `Ok(Vec<u8>)`
- [ ] Implement `test_capture_invalid_config_zero`: `timeout = 0` → `ScreenshotError::InvalidConfig`
- [ ] Implement `test_capture_invalid_config_large`: `timeout > 1h` → `ScreenshotError::InvalidConfig`
- [ ] Implement `test_capture_invalid_region_negative`: `x < 0` → `ScreenshotError::InvalidRegion`
- [ ] Implement `test_capture_invalid_region_zero`: `width = 0` → `ScreenshotError::InvalidRegion`
- [ ] Implement `test_capture_element_not_found`: invalid element → `ScreenshotError::ElementNotFound`
- [ ] Implement `test_capture_cancelled`: cancellation during capture → `ScreenshotError::Cancelled`
- [ ] Implement `test_capture_timeout`: timeout during capture → `ScreenshotError::Timeout`
- [ ] Implement `test_png_magic_bytes`: verify first 8 bytes of captured PNG
- [ ] Run `cargo test -- --nocapture` for verification
- [ ] Run `cargo clippy -- -D warnings` for code quality
