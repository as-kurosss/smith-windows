# 🤖 Instruction for Agent: FocusTool | smith-windows

**📁 Sources (read in priority order):**
1. `docs/design/focus-tool/specification.md` — input/output, boundaries, criteria
2. `docs/design/focus-tool/contract.md` — requirements, guarantees, prohibitions, failures
3. `docs/design/focus-tool/test-plan.md` — test scenarios, mandatory checks
4. `src/core/click.rs` — similar pattern for backend traits and mocks
5. `src/core/input_text.rs` — similar pattern for element validation and timeout handling
6. `docs/adr/013-focus-tool.md` — architectural decisions
7. `AGENTS.md`, `ARCHITECTURE.md` — code rules, stack, process

**🔗 Cross-references:**
- `docs/design/input-text-tool/brief.md` — input text operations require window activation
- `docs/design/click-tool/brief.md` — click operations may need focused window
- `docs/design/automation-session/brief.md` — session handle provides UIAutomation instance

**🎯 Task:**
Generate production code for FocusTool module, implementing window activation via UI Automation API to ensure target window is foreground before text/click operations.

**📋 Output format (strict):**
```
src/core/focus.rs → FocusConfig, FocusError, validate_config(), validate_element_ready(), validate_window_pattern_available(), FocusBackend trait, MockFocusBackend, focus_with_config() → [cfg(test)] mod tests → tests: valid config, invalid config, element checks, window pattern, mock idempotency, cancellation, timeout
src/runtime/backends/windows/focus.rs → FocusBackendWindows → uiautomation::patterns::UIWindowPattern.wait_for_input_idle() + set_window_visual_state() + focus() → #[cfg(target_os = "windows")]
src/runtime/backends/unsupported.rs → focus_with_config stub → #[cfg(not(target_os = "windows"))]
src/core/mod.rs → pub mod focus; (добавить)
src/runtime/backends/windows/mod.rs → pub mod focus; + pub use focus::{FocusBackendWindows, focus_with_config}; (добавить)
src/lib.rs → pub use crate::core::focus::{FocusBackend, FocusConfig, FocusError, MockFocusBackend, focus_with_config}; + pub use crate::runtime::backends::windows::focus::FocusBackendWindows; (добавить)
tests/integration/focus.rs → integration tests: end-to-end focus flow with real window
```

**✅ Mandatory plan elements:**
- `validate_config()` moved to `src/core/focus.rs` (validation BEFORE backend call)
- Unified `FocusError` (`thiserror`) with exact names: `ElementNotFound`, `ElementNotEnabled`, `ElementOffscreen`, `WindowPatternNotAvailable`, `Timeout`, `Cancelled`, `InvalidConfig(String)`, `ComError(String)`, `UnsupportedPlatform`
- Signature: `focus_with_config(element: &UIElement, config: &FocusConfig) -> Result<(), FocusError>`
- COM calls run on STA thread (NO `spawn_blocking`, UIElement is `!Send`/`!Sync`)
- Use `#[async_trait::async_trait(?Send)]` for `FocusBackend` trait
- Tests: `#[cfg(test)] mod tests` inside `src/core/focus.rs` + `tests/integration/focus.rs`
- `MockFocusBackend` with `Arc<Mutex<MockFocusState>>` for idempotency check on `Err`
- Window activation via `WindowPattern.wait_for_input_idle()` + `set_window_visual_state(WindowVisualState::Normal)` + `focus()`

**🚫 Prohibitions:**
- Don't generate code at plan stage
- Don't use `unwrap()`/`panic!`/`expect()` even in examples
- Don't create `src/core/focus/tests.rs` (combine in `mod tests` or move to `tests/`)
- Don't change contract without explicit agreement
- Don't use `GetForegroundWindow()` or Win32 APIs for window activation
- Don't use `spawn_blocking` for UIA calls (compilation will fail due to `!Send`/`!Sync`)

**🔄 Process:**
1. Read templates and existing modules (click.rs, input_text.rs)
2. Create `src/core/focus.rs` with types, validation, trait, mock
3. Create `src/runtime/backends/windows/focus.rs` with Windows implementation
4. Add unit tests for validation, config, mock backend
5. Add integration tests for end-to-end flow
6. Export from `src/lib.rs` and `src/runtime/backends/windows/mod.rs`
7. Run `cargo test` and `cargo clippy -- -D warnings`
8. Update `README.md`, `CHANGELOG.md`, `context_bundle.md`

**📝 Metadata:**
- Author: smith-planner
- Date: 2026-04-22
- Status: awaiting_approval
