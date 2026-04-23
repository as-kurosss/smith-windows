# ADR 008: ReadTool Module

**Date:** 2026-04-22
**Status:** [approved]
**Author:** smith-architect
**Module:** ReadTool

## Context

Проект smith-windows требует модуль для **получения текстового содержимого из UI-элементов** (полей ввода, лейблов, заголовков и т.д.) через UI Automation API. Это основная функциональность для валидации состояния приложения, получения данных для тестирования и отладки UI automation сценариев.

**Сценарии использования ReadTool:**
- Валидация текста в полях ввода после операций (test assertions)
- Получение сообщений об ошибках из UI для логирования
- Считывание данных из полей (формы, таблицы) для дальнейшей обработки
- Debugging — проверка текущего текстового состояния элементов
- Regression testing — сравнение текстового содержимого "before/after" изменений
- Отчётность — текст из UI элементов в отчётах о тестировании

**Существующие решения:**
- `AutomationSession`: управление сессиями UI Automation, но НЕ чтение текста
- `ClickTool`, `TypeTool`, `InputTextTool`, `SetTextTool`, `WaitTool`, `InspectTool` — все работают с UI элементами, но НЕ читают их текст
- Отсутствует модуль для чтения "текстового контента" из элементов

**Проблема:** Нет способа получить текстовое содержимое UI элементов для валидации и отладки. UI Automation предоставляет доступ к элементам, но не к их текстовому содержимому напрямую.

## Decision

Добавить новый модуль `ReadTool` в структуру smith-windows как вспомогательную операцию:

- **Файлы:**
  - `src/core/read.rs` — типы, трейты, валидация, чтение текста
  - `src/runtime/backends/windows/read.rs` — Windows-реализация через UI Automation
  - `src/runtime/backends/unsupported.rs` — stub для non-Windows платформ
  - `tests/integration/read_tool.rs` — интеграционные тесты

- **Типы:**
  - `ReadConfig { timeout: Duration, cancellation: CancellationToken }`
  - `ReadError` — enum с `thiserror`:
    - `ElementNotFound` — неверный или удалённый UIElement
    - `ElementNotEnabled` — элемент отключён
    - `ElementOffscreen` — элемент не виден на экране
    - `ElementNotWritable` — элемент не поддерживает чтение текста
    - `Timeout` — превышен таймаут чтения
    - `Cancelled` — операция отменена
    - `InvalidConfig(String)` — неверный конфиг (timeout <= 0 или timeout > 1 час)
    - `ComError(String)` — UI Automation ошибка с сообщением
    - `UnsupportedPlatform` — stub для non-Windows
  - `ReadBackend` — трейт с методом `read_text(&element: &UIElement) -> Result<String, ReadError>`
  - `MockReadBackend` — мок с `Arc<Mutex<MockReadState>>`

- **Функции:**
  - `validate_read_config()` — валидация конфига (timeout > 0, timeout <= 1 hour)
  - `read_text_with_config()` — основная функция с timeout/cancellation, возвращает `Result<String, ReadError>`
  - `ReadBackendWindows::read_text()` — Windows-реализация через UI Automation API

- **Алгоритм чтения текста (Windows):**
  ```
  1. Validate element:
     - Check element is valid (get_control_type)
     - Check element is enabled (is_enabled)
     - Check element is not offscreen (is_offscreen)

  2. Try to read text using TextPattern:
     - Get UITextPattern from element
     - Call get_document_text() to retrieve text content

  3. Fallback to Name property:
     - If TextPattern not available, try element.get_name()
     - This supports label controls without TextPattern

  4. Return text:
     - Return Ok(text) on success
     - Return ElementNotWritable if no text support
  ```

- **Windows-реализация:**
  - Прямые вызовы UIA методов (НЕ `spawn_blocking`) — UIA типы `!Send`/`!Sync`
  - COM инициализация обрабатывается `uiautomation` crate
  - Ошибки: `GetLastError()` не требуется, ошибки UIA передаются напрямую
  - Поддержка двух источников текста: TextPattern (для полей ввода) и Name (для лейблов)

- **Особенности:**
  - Выход: `String` с текстовым содержимым элемента
  - Пустая строка `""` допустима (элемент без текста)
  - Timeout обрабатывается через `tokio::time::timeout()`
  - Cancellation через `CancellationToken::is_cancelled()`
  - Идемпотентность: повторный вызов с теми же данными не меняет состояние

## Consequences

### Positive
- ✅ Добавляется возможность чтения текстового содержимого UI элементов (для валидации и отладки)
- ✅ Единообразная архитектура с другими инструментами (ClickTool, TypeTool)
- ✅ Чёткие контракты и тесты
- ✅ Поддержка TextPattern и Name fallback для максимальной совместимости
- ✅ Proper timeout and cancellation support
- ✅ Компактный API — одна функция `read_text()`

### Negative
- ⚠️ Зависимость от UI Automation API (только Windows, но это допустимо)
- ⚠️ Дополнительная сложность конфигурации (timeout, token отмены)
- ⚠️ Необходимость поддержки дополнительных файлов и тестов
- ⚠️ UIA calls must run on single thread (STA affinity) — нет `spawn_blocking`

## Alternative Considered

### 1. Использовать SetTextTool и передавать пустую строку
**Rejected:** SetTextTool предназначен для записи текста, не для чтения. Нарушение принципа единой ответственности. ReadTool должен быть отдельным модулем.

### 2. Встроить чтение текста в каждый инструмент как опциональную функцию
**Rejected:** Нарушает принцип DRY и модульности. Каждый инструмент будет дублировать логику чтения. ReadTool должен быть независимым модулем.

### 3. Использовать сторонний crate (например, `ui_automation`) для чтения текста
**Rejected:** `uiautomation` crate уже используется, добавление ещё одного crate нарушает принцип minimal dependencies. `uiautomation` предоставляет необходимые методы для чтения текста.

### 4. Встроить чтение текста в `AutomationSession` как метод `get_text()`
**Rejected:** `AutomationSession` отвечает за запуск и сессию приложения, а не за чтение текста. Разделение ответственности. ReadTool должен быть независимым инструментом.

## Implementation Checklist

- [x] `docs/design/read-tool/specification.md` — создана (с полной спецификацией API)
- [x] `docs/design/read-tool/contract.md` — создана (с описанием требований)
- [x] `docs/design/read-tool/test-plan.md` — создана (с сценариями для всех режимов)
- [x] `docs/design/read-tool/brief.md` — создана (с инструкциями для кодера)
- [x] `docs/adr/008-read-tool.md` — создана (ADR с архитектурным решением)
- [x] `README.md` — обновлена таблица модулей и добавлен пример ReadTool
- [x] `CHANGELOG.md` — добавлен раздел ReadTool (ADR 008)
- [x] `ARCHITECTURE.md` — добавлен раздел ReadTool и COM safety notes
- [x] `src/core/read.rs` — создана с типами, трейтом, валидацией и unit тестами
- [x] `src/runtime/backends/windows/read.rs` — создана с полной реализацией через UI Automation
- [x] `src/runtime/backends/windows/mod.rs` — добавлен `pub mod read` и экспорт `ReadBackendWindows`
- [x] `src/runtime/backends/mod.rs` — добавлен экспорт `ReadBackendWindows`
- [x] `src/core/mod.rs` — добавлен `pub mod read`
- [ ] `src/lib.rs` — добавлен re-export `ReadConfig`, `ReadError`, `ReadBackend`, `MockReadBackend`, `read_text_with_config()`
- [ ] `tests/integration/read_tool.rs` — интеграционные тесты (5-7 тестов)
- [ ] `context_bundle.md` — обновлён с ReadTool документацией
- [ ] `cargo test` — все интеграционные + unit тесты проходят
- [ ] `cargo clippy -- -D warnings` — без ошибок

## References

- `docs/design/read-tool/` — полный набор документов
- `docs/design/click-tool/` — аналогичная архитектура для референса
- `docs/design/automation-session/` — UIAutomation initialization pattern
- [UI Automation Text Pattern](https://learn.microsoft.com/en-us/windows/win32/winauto/uiauto-textpattern)
- [UI Automation Element Properties](https://learn.microsoft.com/en-us/windows/win32/winauto/uiauto-elementproperties)
- [UITextPattern.get_documentText()](https://learn.microsoft.com/en-us/windows/win32/api/uiautomationcore/nf-uiautomationcore-iuiautomationtextpattern-get_documenttext)
- [UIElement.get_name()](https://learn.microsoft.com/en-us/windows/win32/winauto/uiauto-uielement-properties)
