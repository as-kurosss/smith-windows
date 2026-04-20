# 📝 CHANGELOG

Все важные изменения в проекте `smith-windows` будут документироваться в этом файле.

Формат основан на [Keep a Changelog](https://keepachangelog.com/ru/1.0.0/),
и проект придерживается семантического версионирования (SemVer).

## [Unreleased]

### Added

- **InspectTool** (`docs/design/inspect-tool/`)
  - `specification.md` — функциональная спецификация InspectTool
  - `contract.md` — контракт: требования, гарантии, запреты, сбои
  - `test-plan.md` — план тестов: сценарии, граничные случаи, обязательные проверки
  - `brief.md` — инструкция для разработчика: файлы, типы, структура

### Changed

- **ARCHITECTURE.md**
  - Обновлена структура проекта для включения InspectTool
  - Добавлены ссылки на `docs/design/inspect-tool/`
  - Обновлены примеры структуры исходных файлов

- **README.md**
  - Удалён раздел "InspectTool Limitations" — теперь InspectTool поддерживает полный путь иерархии
  - Обновлены примеры использования InspectTool для отражения полной функциональности

- **docs/design/inspect-tool/**
  - `specification.md` — обновлён для отражения полной реализации через UITreeWalker
  - `contract.md` — обновлён для отражения полной валидации иерархии
  - `test-plan.md` — обновлён для отражения полной реализации и всех тестовых сценариев
  - `brief.md` — обновлён для отражения полного построения пути через UITreeWalker

- **tools/bundle_context.rs**
  - Добавлены пути к `README.md` и `CHANGELOG.md` для включения в контекстный бандл

### Technical Notes

#### InspectTool Implementation (Complete)

**Полная реализация:**
InspectTool теперь поддерживает **полный путь иерархии** от `head_window` до `element` через `UITreeWalker`.

**Что работает:**
- Полный путь иерархии строится через `UITreeWalker` из `UIAutomation::create_tree_walker()`
- Метод `get_parent()` позволяет пройти от элемента к `head_window`, собирая все промежуточные элементы
- Сравнение элементов через `UIAutomation::compare_elements()` (так как `UIElement` не реализует `PartialEq`)
- Иерархическая проверка: подтверждение, что `element` является потомком `head_window`
- Максимальная глубина пути: 256 элементов (защита от бесконечных циклов)
- Валидация элементов: `is_enabled()`, `is_offscreen()`
- Получение свойств элемента: `ControlType`, `Name`
- Обработка всех ошибок через `InspectError`

**Ключевые API `uiautomation` v0.24.4:**
- `UIAutomation::create_tree_walker()` → возвращает `UITreeWalker` для обхода дерева
- `UITreeWalker::get_parent(&element)` → возвращает родительский элемент или `Null` (для head_window)
- `UIAutomation::compare_elements(&e1, &e2)` → сравнивает два элемента на идентичность

**Формат пути:**
`Window->Button->CheckBox{Name}` (полная иерархия от head_window до element)

## [0.1.0] - Initial Release

### Added

- **ClickTool** — модуль клика по UI-элементам
- **TypeTool** — модуль ввода текста в UI-элементы
- **AutomationSession** — модуль управления сессиями UI Automation
- **Documentation templates** — шаблоны для спецификаций, контрактов и тест-планов
- **AI Agents configuration** — конфигурация для smith-architect, smith-planner, smith-coder, smith-debugger, smith-compliance

### Technical Details

- Rust 1.95.0, edition 2021
- Async runtime: `tokio`
- UIAutomation crate: v0.24.4
- Error handling: `thiserror`
- Logging: `tracing`
- Platform: Windows 10/11 only
