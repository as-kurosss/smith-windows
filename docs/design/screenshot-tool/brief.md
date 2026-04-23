## 🤖 Instruction for Agent: ScreenshotTool | smith-windows

**📁 Sources (read in priority order):**
1. `docs/design/screenshot-tool/specification.md` — input/output, boundaries, criteria
2. `docs/design/screenshot-tool/contract.md` — requirements, guarantees, prohibitions, failures
3. `docs/design/screenshot-tool/test-plan.md` — test scenarios, mandatory checks
4. `AGENTS.md`, `ARCHITECTURE.md` — code rules, stack, process

**🔗 Cross-references:**
- `docs/design/screenshot-tool/specification.md` — dependency description (if any)
- `docs/design/automation-session/` — for UIElement capture mode
- `docs/adr/XXX-screenshot-tool.md` — COM safety notes (GDI vs UIA)

**🎯 Task:**
Generate code implementation for ScreenshotTool module in Rust. Module must capture screenshots of Windows desktop in PNG format via GDI/GDI+ API, supporting three modes: full screen, specific window (UIElement), and custom region (x,y,width,height). Implement with full error handling, timeout, and cancellation support.

**📋 Output format (strict):**
```
src/core/screenshot.rs → ScreenshotMode, ScreenshotConfig, ScreenshotError, ScreenshotBackend trait, validate_screenshot_config(), validate_screenshot_mode() → [no cfg] → [core tests] → [validation BEFORE backend]
src/runtime/backends/windows/screenshot.rs → ScreenshotBackendImpl → [cfg(windows)] → [integration tests] → [spawn_blocking for GDI/GDI+]
tests/integration/screenshot_tests.rs → integration tests for all modes and error scenarios → [cfg(test)] → [cargo test]
```

**✅ Mandatory plan elements:**
- `validate_screenshot_config()` and `validate_screenshot_mode()` moved to `src/core/screenshot.rs` (validation BEFORE backend call)
- Unified `ScreenshotError` (`thiserror`) with exact names from contract: `InvalidRegion`, `InvalidConfig`, `ElementNotFound`, `Timeout`, `Cancelled`, `CaptureFailed(String)`, `UnsupportedPlatform`
- Signature with explicit `ScreenshotConfig { timeout: Duration, cancellation: CancellationToken }`
- GDI/GDI+ calls isolated via `tokio::task::spawn_blocking` in `src/runtime/backends/windows/screenshot.rs`
- Mode variants: `Screen`, `Window(UIElement)`, `Region{x: i32, y: i32, width: u32, height: u32}`
- Tests: `#[cfg(test)] mod tests` inside `src/core/screenshot.rs` + `tests/integration/screenshot_tests.rs`
- `MockScreenshotBackend` with `Arc<Mutex<MockState>>` for idempotency check on `Err`

**🚫 Prohibitions:**
- ❌ Don't generate code at plan stage (only plan, code generation via `/plan` → `smith-coder`)
- ❌ Don't use `unwrap()`/`panic!`/`expect()` even in examples
- ❌ Don't create `src/core/screenshot/tests.rs` (combine in `mod tests` inside `screenshot.rs`)
- ❌ Don't change contract without explicit agreement (check `docs/design/screenshot-tool/contract.md`)
- ❌ Don't use `GetForegroundWindow()` for window capture — use `UIElement` from automation session
- ❌ Don't use `GetPixel()` — use `BitBlt`/`StretchBlt` for efficient screen capture
- ❌ Don't call GDI/GDI+ directly without `spawn_blocking` isolation

**🔄 Process:**
1. Create `src/core/screenshot.rs` with types: `ScreenshotMode`, `ScreenshotConfig`, `ScreenshotError`, `ScreenshotBackend` trait, `validate_screenshot_config()`, `validate_screenshot_mode()`
2. Create `src/runtime/backends/windows/screenshot.rs` with `ScreenshotBackendImpl::capture()`, isolating GDI/GDI+ calls via `spawn_blocking`
3. Add unit tests in `mod tests` inside `src/core/screenshot.rs` for config/mode validation
4. Add integration tests in `tests/integration/screenshot_tests.rs` for all modes and error scenarios
5. Run `cargo test && cargo clippy -- -D warnings` for verification
6. Update `ARCHITECTURE.md` to include ScreenshotTool section

**📝 Metadata:**
- Author: smith-architect
- Date: 2026-04-22
- Status: `[draft]`
