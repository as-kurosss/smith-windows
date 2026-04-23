## 🤖 Instruction for Agent: ReadTool | smith-windows

**📁 Sources (read in priority order):**
1. `docs/design/read-tool/specification.md` — input/output, boundaries, criteria
2. `docs/design/read-tool/contract.md` — requirements, guarantees, prohibitions, failures
3. `docs/design/read-tool/test-plan.md` — test scenarios, mandatory checks
4. `src/core/click.rs` — аналогичный паттерн для инструментов (ClickTool)
5. `src/core/set_text.rs` — аналогичный паттерн для инструментов (SetTextTool)
6. `src/core/type.rs` — аналогичный паттерн для инструментов (TypeTool)
7. `src/runtime/backends/windows/click.rs` — Windows-реализация backend
8. `src/runtime/backends/windows/set_text.rs` — Windows-реализация backend
9. `AGENTS.md`, `ARCHITECTURE.md` — code rules, stack, process

**🔗 Cross-references:**
- `docs/design/read-tool/specification.md` — обязательный входной/выходной формат
- `docs/design/read-tool/contract.md` — гарантии и запреты
- `docs/adr/XXX-read-tool.md` — архитектурное решение (создаётся после утверждения плана)
- `docs/design/click-tool/` — аналогичный модуль для сравнения паттернов

**🎯 Task:**
Реализовать модуль ReadTool для получения текстового содержимого из UI-элементов через UI Automation API.

**📋 Output format (strict):**
```
[File] → [Entities] → [cfg-flags] → [Tests] → [Validation]
```

**✅ Mandatory plan elements:**
- [ ] `validate_read_config()` в `src/core/read.rs` (валидация ДО backend call)
- [ ] Unified `ReadError` (`thiserror`) с точными именами из contract:
  - `ElementNotFound`
  - `ElementNotEnabled`
  - `ElementOffscreen`
  - `ElementNotWritable`
  - `Timeout`
  - `Cancelled`
  - `InvalidConfig`
  - `ComError`
- [ ] `ReadConfig { timeout: Duration, cancellation: CancellationToken }`
- [ ] `ReadBackend` trait с `async fn read_text(&self, element: &UIElement) -> Result<String, ReadError>`
- [ ] `ReadBackendWindows` — Windows-реализация через `uiautomation`
- [ ] `MockReadBackend` с `Arc<Mutex<MockReadState>>` для идемпотентности на `Err`
- [ ] `read_text_with_config()` с валидацией + обработкой timeout/cancellation
- [ ] COM calls: прямые вызовы UIA методов (НЕ spawn_blocking) — UIA types `!Send`/`!Sync`
- [ ] Тесты: `#[cfg(test)] mod tests` внутри `src/core/read.rs` + `tests/integration/read_tool.rs`
- [ ] Тесты идемпотентности: повторный вызов `Err` не меняет состояние backend

**🚫 Prohibitions:**
- ❌ Не генерировать код на стадии `/plan` — только документация и план
- ❌ Не использовать `unwrap()`/`panic!`/`expect()` даже в примерах
- ❌ Не создавать `src/core/read/tests.rs` — объединять в `mod tests` или move в `tests/`
- ❌ Не менять контракт без явного согласия архитектора
- ❌ Не использовать `GetForegroundWindow()` — использовать `is_enabled()`/`is_offscreen()` через `uiautomation`
- ❌ Не оборачивать UIA calls в `spawn_blocking` — UIA types `!Send`/`!Sync`

**🔄 Process:**
1. Прочитать все источники (specification, contract, test-plan, аналогичные модули)
2. Сформировать и утвердить `/plan` с архитектором
3. Создать `src/core/read.rs` — типы, трейты, валидация, моки, тесты
4. Создать `src/runtime/backends/windows/read.rs` — Windows-реализация backend
5. Обновить `mod.rs` файлы для экспорта новых модулей
6. Создать `tests/integration/read_tool.rs` — интеграционные тесты
7. Запустить `cargo test && cargo clippy -- -D warnings`
8. Создать `docs/adr/XXX-read-tool.md` — запись архитектурного решения
9. Обновить `ARCHITECTURE.md` — добавить ReadTool в список инструментов

**📝 Metadata:**
- Author: smith-architect
- Date: 2026-04-22
- Status: `[draft]`
