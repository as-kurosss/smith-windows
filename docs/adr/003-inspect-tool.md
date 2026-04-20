# ADR 003: InspectTool Module

**Date:** 2026-04-21  
**Status:** [approved]  
**Author:** smith-architect  
**Module:** InspectTool

## Context

Проект smith-windows требует модуль для инспекции UI-элементов, который позволит:
- Запускать интерактивный режим инспекции
- При наведении курсора и нажатии Ctrl сохранять путь автоматизации от головного окна до целевого элемента
- Получать строковый селектор в формате `Window->Button->CheckBox{Name}`

## Decision

Добавить новый модуль `InspectTool` в структуру smith-windows с аналогичной архитектурой к `ClickTool` и `TypeTool`:

- **Файлы:**
  - `src/core/inspect.rs` — типы, трейты, валидация
  - `src/runtime/backends/windows/inspect.rs` — Windows-реализация
  - `tests/integration/inspect_tests.rs` — интеграционные тесты

- **Типы:**
  - `InspectConfig { timeout: Duration, cancellation: CancellationToken }`
  - `InspectError` — enum с `thiserror` (ElementNotFound, ElementNotEnabled, ElementOffscreen, Timeout, Cancelled, InvalidConfig, ComError, InvalidSelector)
  - `InspectBackend` — трейт с методом `inspect_path()`

- **Функции:**
  - `validate_inspect_config()` — валидация конфига (timeout bounds)
  - `validate_inspect_path()` — валидация иерархии (element в потомках head_window)
  - `get_inspect_path(head_window, element)` — построение строки-пути (deprecated, полная реализация в backend)
  - `inspect_path_with_config()` — основная функция с timeout/cancellation

- **Полный путь через UITreeWalker (реализовано):**
  - Получение `UITreeWalker` через `UIAutomation::create_tree_walker()`
  - Метод `get_parent()` для обхода от `element` к `head_window`
  - Сравнение элементов через `UIAutomation::compare_elements()` (так как UIElement не реализует PartialEq)
  - Проверка иерархии (element должен быть потомком head_window)
  - Максимальная глубина пути: 256 элементов
  - Путь форматируется как `Window->Button->CheckBox{Name}` (полная иерархия)

- **Особенности:**
  - COM-вызовы изолированы через `tokio::task::spawn_blocking`
  - Валидация иерархии через `UITreeWalker` и `compare_elements()`
  - Максимальная глубина пути: 256 элементов
  - Формат пути: `ElementControlType{Name}` или `ElementControlType`

## Consequences

### Positive
- Добавляется функционал интерактивной инспекции UI-элементов
- Единообразная архитектура с другими инструментами (ClickTool, TypeTool)
- Чёткие контракты и тесты
- COM-безопасность через `spawn_blocking`

### Negative
- Необходимость поддержки дополнительных файлов и тестов
- Зависимость от `uiautomation` crate (уровень доступа к `ControlType`, `Name`, `get_parent()`)

## Alternative Considered

### Прямой WinAPI вызов через GetForegroundWindow()
**Rejected:** Нарушает принцип UIAutomation-first и COM safety.

### Использование скриншота и OCR
**Rejected:** Слишком медленно и неточно для точного инспектирования элементов.

## Implementation Checklist

- [x] `docs/design/inspect-tool/specification.md` — создана (с полной реализацией через UITreeWalker)
- [x] `docs/design/inspect-tool/contract.md` — создана (с описанием полного пути)
- [x] `docs/design/inspect-tool/test-plan.md` — создана
- [x] `docs/design/inspect-tool/brief.md` — создана (с полной реализацией через UITreeWalker)
- [x] `src/core/inspect.rs` — создана с типами, трейтом, валидацией и unit тестами
- [x] `src/runtime/backends/windows/inspect.rs` — создана с полной реализацией через UITreeWalker
- [x] `src/runtime/backends/unsupported.rs` — создана для non-Windows платформ
- [x] `src/core/mod.rs` — добавлен `pub mod inspect`
- [x] `src/runtime/backends/windows/mod.rs` — добавлен `pub mod inspect` и экспорт `InspectBackendWindows`
- [x] `src/lib.rs` — добавлен re-export `InspectConfig`, `InspectError`, `InspectBackend`
- [x] `tests/inspect_tests.rs` — интеграционные тесты (6 юнит-тестов + 11 интеграционных)
- [x] `ARCHITECTURE.md` — обновлена структура проекта
- [x] `context_bundle.md` — обновлён с InspectTool документацией
- [x] `AGENTS.md` — обновлена
- [x] `CHANGELOG.md` — обновлён раздел Unreleased
- [x] `README.md` — обновлён с InspectTool
- [x] `cargo test` — все 34 теста проходят (включая 6 тестов InspectTool)
- [x] `cargo clippy -- -D warnings` — без предупреждений
- [x] `cargo fmt` — отформатировано

## References

- `docs/design/inspect-tool/` — полный набор документов
- `docs/design/click-tool/` — аналогичная архитектура для референса
- `docs/design/type-tool/` — аналогичная архитектура для референса
