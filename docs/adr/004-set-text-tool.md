# ADR 004: SetTextTool Module

**Date:** 2026-04-22
**Status:** [approved]
**Author:** smith-architect
**Module:** SetTextTool

## Context

Проект smith-windows требует модуль для **программной установки текста** в UI-элементы через UI Automation API.

**Ключевое отличие от TypeTool:**
- `TypeTool` — имитирует нажатие клавиш клавиатуры (typing simulation)
- `SetTextTool` — устанавливает текст напрямую через `UIValuePattern.set_value()` (programmatic text setting)

**Сценарии использования SetTextTool:**
- Установка текста в поля ввода `Edit` без эмуляции клавиш
- Массовая настройка значений в UI без задержек на keyboard events
- Работа с полями, где keyboard events могут вызывать побочные эффекты (валидация, auto-complete)

## Decision

Добавить новый модуль `SetTextTool` в структуру smith-windows с аналогичной архитектурой к `ClickTool`, `TypeTool` и `InspectTool`:

- **Файлы:**
  - `src/core/set_text.rs` — типы, трейты, валидация
  - `src/runtime/backends/windows/set_text.rs` — Windows-реализация
  - `tests/integration/set_text_tests.rs` — интеграционные тесты

- **Типы:**
  - `SetTextConfig { timeout: Duration, cancellation: CancellationToken }`
  - `SetTextError` — enum с `thiserror`:
    - `InputSelectorError(String)` — неверный селектор
    - `ElementNotFoundError` — элемент не найден
    - `ElementNotEnabled` — элемент отключён
    - `ElementOffscreen` — элемент скрыт
    - `ElementReadOnly` — элемент только для чтения
    - `Timeout` — превышен таймаут
    - `Cancelled` — операция отменена
    - `InvalidConfig(String)` — неверный конфиг
    - `ComError(String)` — COM-ошибка
  - `SetTextBackend` — трейт с методом `set_text_with_config()`

- **Функции:**
  - `validate_set_text_config()` — валидация конфига (timeout bounds: 1ms-60s)
  - `validate_text_not_empty()` — валидация текста (не пустая строка)
  - `validate_text_length()` — валидация длины текста (max 32767 символов)
  - `set_text_with_config()` — основная функция с timeout/cancellation

- **Windows-реализация (через UIValuePattern):**
  - Получение элемента по селектору через `find_first()` или `find_last()`
  - Проверка `is_enabled()` и `is_offscreen()`
  - Получение `UIValuePattern` через `element.get_pattern::<UIValuePattern>()?`
  - Установка текста через `pattern.set_value(text)?`
  - COM-вызовы изолированы через `tokio::task::spawn_blocking`
  - Проверка на readonly: `pattern.is_read_only()` или обработка исключения

- **Особенности:**
  - Прямая установка текста через `UIValuePattern` (НЕ clipboard + paste!)
  - COM-безопасность через `spawn_blocking`
  - Поддержка Unicode (Rust `&str` → WinAPI `BSTR` → `set_value()`)
  - Идемпотентность: повторный вызов с теми же данными не меняет состояние

## Consequences

### Positive
- ✅ Добавляется функционал программной установки текста (быстрее keyboard typing)
- ✅ Единообразная архитектура с другими инструментами
- ✅ Чёткие контракты и тесты
- ✅ COM-безопасность через `spawn_blocking`
- ✅ Unicode-поддержка

### Negative
- ⚠️ Не все UI-элементы поддерживают `UIValuePattern` (требуется fallback)
- ⚠️ Метод `set_value()` может выбрасывать исключения для readonly элементов
- ⚠️ Необходимость поддержки дополнительных файлов и тестов

## Alternative Considered

### 1. Clipboard + Paste Simulation (как TypeTool)
**Rejected:** Это эмуляция ввода, а не программная установка. Медленно и может вызывать побочные эффекты.

### 2. SendMessage / WM_SETTEXT (WinAPI)
**Rejected:** Нарушает принцип UIAutomation-first. Прямой WinAPI вызовы должны быть изолированы в `uiautomation` crate.

### 3. Clipboard + ControlSend (UIAutomation)
**Rejected:** Сложная эмуляция, медленно, не идемпотентно.

## Implementation Checklist

- [x] `docs/design/set-text-tool/specification.md` — создана (с полной реализацией через UIValuePattern)
- [x] `docs/design/set-text-tool/contract.md` — создана (с описанием прямой установки)
- [x] `docs/design/set-text-tool/test-plan.md` — создана
- [x] `docs/design/set-text-tool/brief.md` — создана (с полной реализацией через UIValuePattern)
- [x] `src/core/set_text.rs` — создана с типами, трейтом, валидацией и unit тестами
- [x] `src/runtime/backends/windows/set_text.rs` — создана с полной реализацией через UIValuePattern
- [x] `src/runtime/backends/unsupported.rs` — создана для non-Windows платформ
- [x] `src/core/mod.rs` — добавлен `pub mod set_text`
- [x] `src/runtime/backends/windows/mod.rs` — добавлен `pub mod set_text` и экспорт `SetTextBackendWindows`
- [x] `src/lib.rs` — добавлен re-export `SetTextConfig`, `SetTextError`, `SetTextBackend`
- [x] `tests/set_text.rs` — интеграционные тесты (9 тестов с `serial_test`)
- [x] `ARCHITECTURE.md` — обновлена структура проекта (docs + src)
- [x] `context_bundle.md` — обновлён с SetTextTool документацией
- [x] `AGENTS.md` — обновлена
- [x] `cargo test` — все 9 интеграционных + 3 юнит-теста проходят
- [x] `cargo clippy -- -D warnings` — без предупреждений
- [x] `cargo fmt` — отформатировано

## References

- `docs/design/set-text-tool/` — полный набор документов
- `docs/design/click-tool/` — аналогичная архитектура для референса
- `docs/design/type-tool/` — аналогичная архитектура для референса (но с keyboard vs UIValuePattern различием)
- `docs/design/inspect-tool/` — аналогичная архитектура для референса
- [UIValuePattern на docs.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/winauto/uiauto-uivaluepattern)
