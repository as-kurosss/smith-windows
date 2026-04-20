## 🤖 Инструкция для агента: ClickTool | smith-windows

**📁 Источники (читать в порядке приоритета):**
1. `docs/design/click-tool/specification.md` — вход/выход, границы, критерии
2. `docs/design/click-tool/contract.md` — требования, гарантии, запреты, сбои
3. `docs/design/click-tool/test-plan.md` — сценарии тестов, обязательные проверки
4. `AGENTS.md`, `ARCHITECTURE.md` — правила кода, стек, процесс

**🔗 Перекрёстные ссылки:**
- `docs/design/click-tool/brief.md` — текущий документ (для ссылок внутри проекта)
- `docs/design/type-tool/` — аналогичный модуль TypeTool (если требуется рефакторинг)

**🎯 Задача:**
Сгенерировать код реализации модуля ClickTool для Rust automation на Windows. Модуль должен выполнять клик по UI-элементу через uiautomation crate 0.24.4 с полной обработкой ошибок, timeout и отмены.

**📋 Формат вывода (строго):**
```
src/core/click.rs → ClickConfig, ClickError, ClickBackend trait, validate_click_config() → [no cfg] → [core tests] → [валидация ДО бэкенда]
src/runtime/backends/windows/click.rs → ClickBackendImpl → [cfg(windows)] → [integration tests] → [spawn_blocking]
tests/integration/click_tests.rs → интеграционные тесты всех сценариев → [cfg(test)] → [cargo test]
```

**✅ Обязательные элементы в плане:**
- `validate_click_config()` вынесена в `src/core/click.rs` (валидация ДО вызова бэкенда)
- Единый `ClickError` (`thiserror`) с точными именами из контракта: `ElementNotFound`, `ElementNotEnabled`, `ElementOffscreen`, `Timeout`, `Cancelled`, `InvalidConfig`, `ComError`
- Сигнатура с явным `ClickConfig { timeout: Duration, cancellation: CancellationToken }`
- Изоляция COM-вызовов через `tokio::task::spawn_blocking` в `src/runtime/backends/windows/click.rs`
- Трейт `ClickBackend` в core для абстракции реализации
- Тесты: `#[cfg(test)] mod tests` внутри `src/core/click.rs` + `tests/integration/click_tests.rs`
- `MockClickBackend` с `Arc<Mutex<MockState>>` для проверки идемпотентности при `Err`

**🚫 Запреты:**
- ❌ Не генерировать код на этапе плана (только план, генерация через `/plan` → `smith-coder`)
- ❌ Не использовать `unwrap()`/`panic!`/`expect()` даже в примерах
- ❌ Не создавать `src/core/click/tests.rs` (объединить в `mod tests` внутри `click.rs`)
- ❌ Не менять контракт без явного согласования (проверить `docs/design/click-tool/contract.md`)
- ❌ Не использовать прямые WinAPI вызовы (CoInitializeEx, CoCreateInstance, BSTR, VARIANT)
- ❌ Не использовать `GetForegroundWindow()` — использовать `is_enabled()`/`is_offscreen()` через `uiautomation`

**🔄 Процесс:**
1. Создать `src/core/click.rs` с типами `ClickConfig`, `ClickError`, трейтом `ClickBackend` и функцией `validate_click_config()`
2. Создать `src/runtime/backends/windows/click.rs` с реализацией `ClickBackendImpl::click()`, изолирующей COM-вызовы
3. Добавить тесты в `mod tests` внутри `src/core/click.rs` для валидации config
4. Добавить интеграционные тесты в `tests/integration/click_tests.rs` для всех сценариев
5. Запустить `cargo test && cargo clippy -- -D warnings` для проверки

**📝 Метаданные:**
- Автор: Архитектор smith-windows
- Дата: 2026-04-19
- Статус: `[draft]`
