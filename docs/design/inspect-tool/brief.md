## 🤖 Инструкция для агента: InspectTool | smith-windows

**📁 Источники (читать в порядке приоритета):**
1. `docs/design/inspect-tool/specification.md` — вход/выход, границы, критерии, полная реализация
2. `docs/design/inspect-tool/contract.md` — требования, гарантии, запреты, сбои
3. `docs/design/inspect-tool/test-plan.md` — сценарии тестов, обязательные проверки
4. `AGENTS.md`, `ARCHITECTURE.md` — правила кода, стек, процесс

**🔗 Перекрёстные ссылки:**
- `docs/design/inspect-tool/specification.md` — текущий документ (для ссылок внутри проекта)
- `docs/design/click-tool/` — аналогичный модуль ClickTool (для референса)
- `docs/design/type-tool/` — аналогичный модуль TypeTool (для референса)

**🎯 Задача:**
Сгенерировать код реализации модуля InspectTool для Rust automation на Windows. Модуль должен предоставлять режим инспекции UI-элементов — запуск интерактивного режима, где при наведении курсора и нажатии Ctrl сохраняется путь автоматизации от головного окна до целевого элемента.

**Полный путь иерархии:** Модуль строит полный путь от `head_window` до `element` через `UITreeWalker` с использованием `UIAutomation::create_tree_walker()` и `get_parent()` для обхода дерева вверх.

**📋 Формат вывода (строго):**
```
src/core/inspect.rs → InspectConfig, InspectError, InspectBackend trait, validate_inspect_config() → [no cfg] → [core tests] → [валидация ДО бэкенда]
src/runtime/backends/windows/inspect.rs → InspectBackendImpl, get_inspect_path() → [cfg(windows)] → [integration tests] → [spawn_blocking]
tests/integration/inspect_tests.rs → интеграционные тесты всех сценариев → [cfg(test)] → [cargo test]
```

**✅ Обязательные элементы в плане:**
- `validate_inspect_config()` вынесена в `src/core/inspect.rs` (валидация ДО вызова бэкенда)
- Единый `InspectError` (`thiserror`) с точными именами из контракта: `ElementNotFound`, `ElementNotEnabled`, `ElementOffscreen`, `Timeout`, `Cancelled`, `InvalidConfig`, `ComError`, `InvalidSelector`
- Сигнатура с явным `InspectConfig { timeout: Duration, cancellation: CancellationToken }`
- Изоляция COM-вызовов через `tokio::task::spawn_blocking` в `src/runtime/backends/windows/inspect.rs`
- Трейт `InspectBackend` в core для абстракции реализации
- Тесты: `#[cfg(test)] mod tests` внутри `src/core/inspect.rs` + `tests/integration/inspect_tests.rs`
- `MockInspectBackend` с `Arc<Mutex<MockState>>` для проверки идемпотентности при `Err`
- Функция `get_inspect_path(head_window, element)` для построения строки-пути через `get_control_type()` и `get_name()`
- **Полный путь строится через UITreeWalker:**
  - Получение `UITreeWalker` через `UIAutomation::create_tree_walker()`
  - Метод `get_parent()` для обхода от `element` к `head_window`
  - Сравнение элементов через `UIAutomation::compare_elements()`
  - Проверка иерархии (element должен быть потомком head_window)
  - Максимальная глубина 256 элементов

**🚫 Запреты:**
- ❌ Не генерировать код на этапе плана (только план, генерация через `/plan` → `smith-coder`)
- ❌ Не использовать `unwrap()`/`panic!`/`expect()` даже в примерах
- ❌ Не создавать `src/core/inspect/tests.rs` (объединить в `mod tests` внутри `inspect.rs`)
- ❌ Не менять контракт без явного согласования (проверить `docs/design/inspect-tool/contract.md`)
- ❌ Не использовать прямые WinAPI вызовы (CoInitializeEx, CoCreateInstance, BSTR, VARIANT)
- ❌ Не использовать `GetForegroundWindow()` — использовать `is_enabled()`/`is_offscreen()` через `uiautomation`
- ❌ Не менять состояние UI (клик, ввод, фокус) в режиме инспекции
- ❌ Не пропускать иерархическую проверку через UITreeWalker

**🔄 Процесс:**
1. Создать `src/core/inspect.rs` с типами `InspectConfig`, `InspectError`, трейтом `InspectBackend`, функцией `validate_inspect_config()` и функцией `get_inspect_path()` для построения пути
2. Создать `src/runtime/backends/windows/inspect.rs` с реализацией `InspectBackendImpl::inspect_path()`, изолирующей COM-вызовы через `spawn_blocking`
3. Добавить тесты в `mod tests` внутри `src/core/inspect.rs` для валидации config и path
4. Добавить интеграционные тесты в `tests/integration/inspect_tests.rs` для всех сценариев
5. Обновить `src/core/mod.rs` и `src/runtime/backends/windows/mod.rs` для экспорта новых типов
6. Обновить `src/lib.rs` для re-export InspectTool
7. Запустить `cargo test && cargo clippy -- -D warnings` для проверки

**📝 Метаданные:**
- Автор: Архитектор smith-windows
- Дата: 2026-04-21
- Статус: `[draft]`
- Реализация: Полный путь иерархии через UITreeWalker (uiautomation v0.24.4)
