## 🤖 Instruction for Agent: WaitTool | smith-windows

**📁 Sources (read in priority order):**
1. `docs/design/wait-tool/specification.md` — input/output, boundaries, criteria
2. `docs/design/wait-tool/contract.md` — requirements, guarantees, prohibitions, failures
3. `docs/design/wait-tool/test-plan.md` — test scenarios, mandatory checks
4. `docs/design/automation-session/specification.md` — UIAutomation session pattern (referenced)
5. `docs/adr/005-input-text-tool.md` — similar async tool with timeout/cancellation
6. `AGENTS.md`, `ARCHITECTURE.md` — code rules, stack, process

**🔗 Cross-references:**
- `docs/design/input-text-tool/brief.md` — similar async pattern with timeout
- `docs/design/automation-session/brief.md` — UIAutomation initialization pattern
- `src/runtime/backends/windows/automation_session.rs` — UIAutomation example
- `src/runtime/backends/windows/click.rs` — COM error handling pattern

**🎯 Task:**
Implement WaitTool for smith-windows: an async tool that waits for an element to appear or disappear by periodically polling UI Automation, returning `true` if the condition is met within the timeout, `false` otherwise.

**📋 Output format (strict):**
```
[File] → [Entities] → [cfg-flags] → [Tests] → [Validation]
src/core/wait.rs → WaitConfig, WaitError, WaitMode, WaitBackend trait, MockWaitBackend, validate_wait_config(), wait_with_config() → #[cfg(test)] mod tests (8 unit tests) → cargo test -- wait && cargo clippy -- -D warnings
src/runtime/backends/windows/wait.rs → WaitBackendWindows, wait_element_with_config() → cfg(target_os = "windows") → tests in src/core/wait.rs + tests/wait_tests.rs → cargo test -- wait
src/runtime/backends/unsupported.rs → WaitBackendUnsupported stub → cfg(not(target_os = "windows")) → tests in src/core/wait.rs → cargo test -- wait
tests/wait_tests.rs → 5-7 integration tests with serial_test → no cfg flags → cargo test -- wait
```

**✅ Mandatory plan elements:**
- `WaitMode` enum: `Existence` (wait for element to appear) | `Absence` (wait for element to disappear)
- `WaitError` with exact variants: `InvalidConfig`, `Timeout`, `Cancelled`, `ComError(String)`
- `WaitConfig { timeout: Duration, interval: Duration, wait_for: WaitMode, selector: Selector }`
- `validate_wait_config()` function: check timeout > 0, interval > 0, interval <= timeout
- `WaitBackend` trait: `async fn wait_element(&self, automation: &UIAutomation, root: &UIElement, selector: &Selector) -> Result<bool, WaitError>`
- `MockWaitBackend` with `Arc<Mutex<MockWaitState>>` for idempotency check
- `wait_with_config()` function: validates config, then calls backend in loop with interval sleep
- Windows backend uses `UIElement::find_first()` with matcher from selector
- No `spawn_blocking`: UIElement is `!Send`, calls must be synchronous

**🚫 Prohibitions:**
- Don't generate code at plan stage — plan only, then wait for approval
- Don't use `unwrap()`/`panic!`/`expect()` even in examples
- Don't create `src/core/wait/tests.rs` — combine in `mod tests` inside wait.rs or move to tests/
- Don't change contract without explicit agreement from architect
- Don't use `GetForegroundWindow()` or any Win32 APIs directly
- Don't use `spawn_blocking` for UIElement (it's `!Send` and `!Sync`)

**🔄 Process:**
1. Read all documents: specification.md → contract.md → test-plan.md → ADR 005
2. Review existing patterns: automation_session.rs, click.rs, input_text.rs
3. Create `src/core/wait.rs` with types, validation, trait, mock, unit tests
4. Create `src/runtime/backends/windows/wait.rs` with Windows implementation
5. Update `src/runtime/backends/windows/mod.rs` with `pub mod wait` and exports
6. Update `src/runtime/backends/unsupported.rs` with stub
7. Update `src/lib.rs` with WaitTool re-exports
8. Create `tests/wait_tests.rs` with integration tests
9. Update `README.md` and `CHANGELOG.md`
10. Run `cargo run --bin bundle_context` to update context_bundle.md
11. Validate: `cargo test -- wait` and `cargo clippy -- -D warnings`

**📝 Metadata:**
- Author: smith-architect
- Date: 2026-04-22
- Status: [draft] → [approved] → [implementation]
- Dependencies: uiautomation 0.24.4, tokio 1.x, thiserror 1.x
