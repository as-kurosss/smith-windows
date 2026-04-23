# 🤖 Instruction for Agent: SetTextTool | smith-windows

**📁 Sources (read in priority order):**
1. `docs/design/set-text-tool/specification.md` — input/output, boundaries, criteria
2. `docs/design/set-text-tool/contract.md` — requirements, guarantees, prohibitions, failures
3. `docs/design/set-text-tool/test-plan.md` — test scenarios, mandatory checks
4. `docs/adr/XXX-set-text-tool.md` — architectural decisions (if exists)
5. `AGENTS.md`, `ARCHITECTURE.md` — code rules, stack, process

**🔗 Cross-references:**
- `docs/design/click-tool/specification.md` — аналогичная структура для ClickTool
- `docs/design/type-tool/specification.md` — аналогичная структура для TypeTool
- `docs/design/set-text-tool/contract.md` — запрет clipboard + Ctrl+V (использовать element.set_value() через ValuePattern)

**🎯 Task:**
Реализовать модуль SetTextTool для установки текста UI-элемента через UI Automation API (не через clipboard + paste!). Требуется:
1. Создать `SetTextError` с точными вариантами из контракта
2. Создать `SetTextConfig { timeout: Duration, cancellation: CancellationToken }`
3. Создать `validate_set_text_config()` для валидации конфига ДО backend
4. Создать `SetTextBackend` трейт с методом `set_text(&self, element: &UIElement, text: &str) -> Result<(), SetTextError>`
5. Создать `MockSetTextBackend` с `Arc<Mutex<MockSetTextState>>` для юнит-тестов
6. Создать `set_text_with_config()` с обработкой timeout и cancellation
7. Реализовать `SetTextBackendWindows` через `element.get_pattern::<ValuePattern>()?.set_value(text)`
8. Добавить `mod tests` внутри `src/core/set_text.rs`
9. Добавить интеграционные тесты в `tests/integration/set_text_tests.rs`

**📋 Output format (strict):**
```
[File] → [Entities] → [cfg-flags] → [Tests] → [Validation]
```

**✅ Mandatory plan elements:**
- `validate_set_text_config()` moved to `src/core/set_text.rs` (валидация ДО backend вызова)
- Unified `SetTextError` (`thiserror`) с точными вариантами из контракта:
  - `ElementNotFound`, `ElementNotEnabled`, `ElementOffscreen`, `ElementNotWritable`, `Timeout`, `Cancelled`, `InvalidConfig`, `ComError`
- Signature: `SetTextConfig { timeout: Duration, cancellation: CancellationToken }`
- `element.set_value()` через `ValuePattern` (НЕ clipboard + paste!)
- `#[cfg(test)] mod tests` внутри `src/core/set_text.rs` + `tests/integration/set_text_tests.rs`
- `MockSetTextBackend` с `Arc<Mutex<MockSetTextState>>` для идемпотентности проверки `Err`

**🚫 Prohibitions:**
- Не генерировать код на этапе планирования (только `/plan` + `brief.md`)
- Не использовать `unwrap()`/`panic!`/`expect()` даже в примерах (только явная обработка `Result`)
- Не создавать `src/core/set_text/tests.rs` — объединять в `mod tests` внутри `src/core/set_text.rs`
- Не менять контракт без явного согласования (все ошибки из контракта обязательны)
- Не использовать clipboard + Ctrl+V — только `element.get_pattern::<ValuePattern>()?.set_value(text)`
- Не использовать `spawn_blocking` — `UIElement` это `!Send`/`!Sync`

**🔄 Process:**
1. Создать `src/core/set_text.rs` с типами ошибок, конфигом и трейтами (по аналогии с `click.rs`/`type.rs`)
2. Реализовать `validate_set_text_config()` и `set_text_with_config()` с timeout/cancellation
3. Реализовать `MockSetTextBackend` с `Arc<Mutex<MockSetTextState>>`
4. Создать `src/runtime/backends/windows/set_text.rs` с `SetTextBackendWindows`
5. Добавить экспорт в `src/core/mod.rs`, `src/lib.rs`, `src/runtime/backends/windows/mod.rs`
6. Добавить `mod tests` внутри `src/core/set_text.rs`
7. Добавить `tests/integration/set_text_tests.rs` для интеграционных тестов

**📝 Metadata:**
- Author: smith-planner
- Date: 2026-04-22
- Status: [draft]
