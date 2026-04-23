## 📐 Specification: WaitTool | smith-windows

**🎯 Purpose:** Ожидание появления или исчезновения элемента UI по селектору с периодическим опросом и таймаутом. Возвращает `true` (элемент найден) или `false` (элемент не найден в течение таймаута).

**📥 Input:**
- `config` (WaitConfig) | timeout > 0, interval > 0, selector (AutomationId/Name/ControlType) | WaitConfig { timeout: 5s, interval: 200ms, wait_for: Existence, selector: AutomationId("myButton") }
- `automation` (UIAutomation) | initialized COM instance | UIAutomation::new()?
- `root` (UIElement) | root element (desktop) | automation.get_root_element()?

**📤 Output:**
- `Result<bool, WaitError>` | true = element found, false = timeout | Ok(true) if element exists, Ok(false) if not found within timeout
- On error: WaitError | what DOES NOT change: COM state, element state

**⚠️ Boundaries:**
- What if timeout = 0? → InvalidConfig error
- What if interval = 0? → InvalidConfig error  
- What if timeout < interval? → InvalidConfig error (practical consideration)
- What if element never appears? → Returns Ok(false) after timeout
- What if cancelled? → Returns Err(WaitError::Cancelled)
- What if COM error during find? → Returns Err(WaitError::ComError(e))

**✅ Success criteria:**
- [x] All scenarios from "Boundaries" handled without panics
- [x] State doesn't break on error (idempotent)
- [x] Log/metric records result or failure cause via tracing
- [x] Proper cancellation support with CancellationToken
- [x] Thread-safe: UIElement is !Send, use direct calls (no spawn_blocking)

---
## 🗓️ Implementation plan (for `/plan`)
- [x] Create `docs/design/wait-tool/` with spec, contract, test-plan, brief
- [x] Create ADR 006 for WaitTool architectural decisions
- [ ] Create file: `src/core/wait.rs` (types: WaitConfig, WaitError, WaitMode; validation; trait WaitBackend; MockWaitBackend)
- [ ] Create file: `src/runtime/backends/windows/wait.rs` (Windows implementation via UIElement::find_first())
- [ ] Create file: `src/runtime/backends/unsupported.rs` update (add WaitBackendUnsupported stub)
- [ ] Update `src/runtime/backends/windows/mod.rs` (add pub mod wait + re-exports)
- [ ] Update `src/lib.rs` (add WaitTool re-exports)
- [ ] Create `tests/wait_tests.rs` (integration tests with serial_test)
- [ ] Update `README.md` (add WaitTool to Key Features, Modules)
- [ ] Update `CHANGELOG.md` (add WaitTool section)
- [ ] Run `cargo run --bin bundle_context` (update context_bundle.md)
- [ ] Validate: `cargo test`, `cargo clippy -- -D warnings`
