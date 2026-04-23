# ADR 005: InputTextTool Module

**Date:** 2026-04-22
**Status:** [approved]
**Author:** smith-architect
**Module:** InputTextTool

## Context

Проект smith-windows требует модуль для **эмуляции ввода с клавиатуры** (как будто это делает человек) через UI Automation API.

**Ключевое отличие от SetTextTool:**
- `SetTextTool` — программная установка текста через `UIValuePattern.set_value()` (direct value setting)
- `InputTextTool` — эмуляция нажатия клавиш через `Keyboard::send_keys()` (human-like typing)

**Сценарии использования InputTextTool:**
- Ввод текста с учётом горячих клавиш и комбинаций (Ctrl+C, Ctrl+V, Alt+F4)
- Ввод специальных клавиш (Enter, Tab, Delete, стрелки, F1-F24)
- Работа с полями, где keyboard events вызывают побочные эффекты (валидация, auto-complete, auto-save)
- Тестирование keyboard-only functionality

## Decision

Добавить новый модуль `InputTextTool` в структуру smith-windows с аналогичной архитектурой к `ClickTool`, `TypeTool`, `SetTextTool` и `InspectTool`:

- **Файлы:**
  - `src/core/input_text.rs` — типы, трейты, валидация
  - `src/runtime/backends/windows/input_text.rs` — Windows-реализация
  - `src/runtime/backends/unsupported.rs` — stub для non-Windows платформ
  - `tests/input_text_tests.rs` — интеграционные тесты (в корне `tests/`)

- **Типы:**
  - `InputTextConfig { text: String, timeout: Duration, cancellation: CancellationToken }`
  - `InputTextError` — enum с `thiserror`:
    - `InputSelectorError(String)` — неверный селектор
    - `ElementNotFoundError` — элемент не найден
    - `ElementNotEnabled` — элемент отключён
    - `ElementOffscreen` — элемент скрыт
    - `ElementReadOnly` — элемент только для чтения
    - `Timeout` — превышен таймаут
    - `Cancelled` — операция отменена
    - `InvalidConfig(String)` — неверный конфиг
    - `ComError(String)` — COM-ошибка
    - `InputExecutionError(String)` — ошибка выполнения ввода
  - `InputTextBackend` — трейт с методом `input_text(element, keys)`
  - `MockInputTextBackend` — мок с `Arc<Mutex<MockInputTextState>>`

- **Функции:**
  - `validate_input_selector()` — валидация селектора (не пустая, max 2048 символов)
  - `validate_input_text_config()` — валидация конфига (timeout bounds: 1ms-1hour, text bounds: 1-65536 chars)
  - `validate_element_ready()` — проверка состояния элемента (enabled, offscreen)
  - `input_text_with_config()` — основная функция с timeout/cancellation

- **Синтаксис `uiautomation::inputs::Keyboard::send_keys()`:**
  - Обычный текст: `"Hello"` → `send_text("Hello")`
  - Специальные клавиши: `"{enter}"`, `"{tab}"`, `"{delete}"`, `"{backspace}"`, `"{left}"`, `"{right}"`
  - Модификаторы: `{ctrl}`, `{shift}`, `{alt}`, `{lctrl}`, `{rctrl}`, `{lshift}`, `{rshift}`, `{lalt}`, `{ralt}`
  - Комбинации: `"{ctrl}(c)"` = Ctrl+C, `"{ctrl}{shift}(v)"` = Ctrl+Shift+V
  - Повторы: `"{enter 3}"` = Enter 3 раза
  - Unicode: `"Привет, мир! 👋"` → вводится корректно
  - Комбинации с Unicode: `"{ctrl}(a){ctrl}(c)Привет {ctrl}(v)"` → копирование и вставка

- **Windows-реализация (через Keyboard::send_keys):**
  - Получение элемента по селектору через `find_first()` или `find_last()`
  - Проверка `is_enabled()` через UIAutomation API
  - Проверка `is_offscreen()` через UIAutomation API
  - Ввод текста через `Keyboard::send_keys(keys)?` (синхронная операция)
  - COM-безопасность: `Keyboard` не является `!Send`/`!Sync`, но вызовы синхронные
  - Проверка readonly: не реализована напрямую (попытка ввода может не сработать)

- **Особенности:**
  - Посимвольная эмуляция ввода через `Keyboard::send_keys()` (human-like typing)
  - COM-безопасность: все вызовы синхронные и не блокируют async runtime
  - Поддержка Unicode (Rust `&str` → WinAPI `BSTR` → `send_keys()`)
  - Идемпотентность: повторный вызов с теми же данными не меняет состояние
  - Зависимость от `SetTextTool` для валидации состояния элемента

## Consequences

### Positive
- ✅ Добавляется функционал эмуляции ввода с клавиатуры (комбинации клавиш, специальные клавиши)
- ✅ Единообразная архитектура с другими инструментами
- ✅ Чёткие контракты и тесты
- ✅ Поддержка сложных комбинаций клавиш
- ✅ Unicode-поддержка

### Negative
- ⚠️ Медленнее программной установки текста (каждая клавиша отдельно)
- ⚠️ Может вызывать побочные эффекты в приложении (валидация, auto-save)
- ⚠️ Необходимость поддержки дополнительных файлов и тестов
- ⚠️ Нет прямой проверки readonly через `Keyboard::send_keys()`

## Alternative Considered

### 1. Clipboard + Ctrl+V Simulation
**Rejected:** Это эмуляция ввода через буфер обмена, а не прямой ввод клавиш. Требует дополнительных шагов (копирование, вставка) и может вызывать побочные эффекты.

### 2. TypeTool (уже существует)
**Rejected:** `TypeTool` использует clipboard + paste для ввода, а не эмуляцию клавиш. Для ввода с клавиатуры нужен отдельный модуль.

### 3. WinAPI keybd_event / SendInput
**Rejected:** Нарушает принцип UIAutomation-first. Прямой WinAPI вызовы должны быть изолированы в `uiautomation` crate.

### 4. UIAutomation Mouse + Click (клавиатурные элементы)
**Rejected:** Невозможно эмулировать ввод через клики на виртуальную клавиатуру. Требует специальных приложений.

## Implementation Checklist

- [x] `docs/design/input-text-tool/specification.md` — создана (с полной спецификацией API)
- [x] `docs/design/input-text-tool/contract.md` — создана (с описанием эмуляции ввода)
- [x] `docs/design/input-text-tool/test-plan.md` — создана (с сценариями для всех комбинаций)
- [x] `docs/design/input-text-tool/brief.md` — создана (с инструкциями для кодера)
- [x] `src/core/input_text.rs` — создана с типами, трейтом, валидацией и unit тестами
- [x] `src/runtime/backends/windows/input_text.rs` — создана с полной реализацией через Keyboard::send_keys()
- [x] `src/runtime/backends/windows/mod.rs` — добавлен `pub mod input_text` и экспорт `InputTextBackendWindows`
- [x] `src/runtime/backends/mod.rs` — добавлен экспорт `InputTextBackendWindows`
- [x] `src/runtime/backends/unsupported.rs` — добавлен `InputTextBackendUnsupported` stub
- [x] `src/lib.rs` — добавлен re-export `InputTextConfig`, `InputTextError`, `InputTextBackend`, `MockInputTextBackend`
- [x] `tests/input_text_tests.rs` — интеграционные тесты (11 тестов с `serial_test`)
- [x] `README.md` — обновлена таблица модулей и архитектура
- [x] `CHANGELOG.md` — добавлен раздел InputTextTool (ADR 005)
- [x] `context_bundle.md` — обновлён с InputTextTool документацией
- [x] `cargo test` — все 11 интеграционных + 12 юнит-тестов проходят
- [x] `cargo clippy -- -D warnings` — без ошибок

## References

- `docs/design/input-text-tool/` — полный набор документов
- `docs/design/set-text-tool/` — аналогичная архитектура для референса (но с programmatic vs keyboard typing различием)
- `docs/design/click-tool/` — аналогичная архитектура для референса
- `docs/design/type-tool/` — аналогичная архитектура для референса (но с clipboard vs keyboard typing различием)
- `docs/design/inspect-tool/` — аналогичная архитектура для референса
- [Keyboard::send_keys на docs.rs](https://docs.rs/uiautomation/latest/uiautomation/inputs/struct.Keyboard.html#method.send_keys)
- [Keyboard::send_text на docs.rs](https://docs.rs/uiautomation/latest/uiautomation/inputs/struct.Keyboard.html#method.send_text)
