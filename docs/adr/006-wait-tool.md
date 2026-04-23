# ADR 006: WaitTool Module

**Date:** 2026-04-22
**Status:** [approved]
**Author:** smith-architect
**Module:** WaitTool

## Context

Проект smith-windows требует модуль для **ожидания появления или исчезновения элемента UI** через UI Automation API. Это базовая операция для UI automation, которая используется во всех других инструментах (Click, Type, InputText, SetText).

**Сценарии использования WaitTool:**
- Ожидание появления окна после запуска приложения
- Ожидание появления элемента после анимации или загрузки
- Ожидание исчезновения элемента после закрытия окна
- Синхронизация тестов с состоянием приложения
- Проверка существования элемента перед операцией (как precondition)

**Существующие решения:**
- `AutomationSession`: находит элементы через `find_all()` и фильтрацию (один раз, без ожидания)
- `ClickTool`, `TypeTool`, `InputTextTool`, `SetTextTool`: все получают элемент как входной параметр, предполагая его существование

**Проблема:** Эти инструменты требуют, чтобы элемент уже существовал. WaitTool решает задачу ожидания появления элемента с таймаутом.

## Decision

Добавить новый модуль `WaitTool` в структуру smith-windows как базовую операцию:

- **Файлы:**
  - `src/core/wait.rs` — типы, трейты, валидация
  - `src/runtime/backends/windows/wait.rs` — Windows-реализация
  - `src/runtime/backends/unsupported.rs` — stub для non-Windows платформ
  - `tests/wait_tests.rs` — интеграционные тесты (в корне `tests/`)

- **Типы:**
  - `WaitConfig { timeout: Duration, interval: Duration, wait_for: WaitMode, selector: Selector }`
  - `WaitMode` — enum: `Existence` (ждать появления) | `Absence` (ждать исчезновения)
  - `WaitError` — enum с `thiserror`:
    - `InvalidConfig(String)` — неверный конфиг (timeout/interval bounds)
    - `Timeout` — превышен таймаут
    - `Cancelled` — операция отменена
    - `ComError(String)` — COM-ошибка
  - `WaitBackend` — трейт с методом `wait_element(automation, root, selector) -> Result<bool, WaitError>`
  - `MockWaitBackend` — мок с `Arc<Mutex<MockWaitState>>`

- **Функции:**
  - `validate_wait_config()` — валидация конфига (timeout > 0, interval > 0, interval <= timeout)
  - `wait_element_with_config()` — основная функция с timeout/cancellation, возвращает `Result<bool, WaitError>`
  - `WaitBackendWindows::wait_element()` — Windows-реализация через `UIElement::find_first()`

- **Алгоритм:**
  ```
  loop:
    - Check cancellation token
    - Find element via find_first()
    - If wait_for = Existence and element found → return Ok(true)
    - If wait_for = Absence and element NOT found → return Ok(true)
    - Sleep for interval
    - Check timeout
    - Continue loop
  return Ok(false)  // timeout expired
  ```

- **Windows-реализация:**
  - Получение элемента через `UIAutomation::create_matcher().from(root).find_first()`
  - Matcher строится из `Selector` (AutomationId, Name, или ControlType)
  - Если `find_first()` возвращает `Ok(Some(element))`, элемент найден
  - Если `find_first()` возвращает `Ok(None)`, элемент не найден
  - Если `find_first()` возвращает `Err(e)`, COM error → `WaitError::ComError(e)`

- **Особенности:**
  - Возвращает `true` (успех) если условие выполнено, `false` (timeout) если превышен таймаут
  - Timeout не является ошибкой — это нормальное поведение (элемент не появился)
  - Timeout occurs when all interval checks fail, not when find_first() returns None
  - COM-безопасность: `UIElement` is `!Send`/`!Sync`, direct calls only (no spawn_blocking)
  - Идемпотентность: повторный вызов с теми же данными не меняет состояние

## Consequences

### Positive
- ✅ Добавляется базовая операция ожидания элемента (фундамент для всех инструментов)
- ✅ Единообразная архитектура с другими инструментами (ClickTool, TypeTool, InputTextTool)
- ✅ Чёткие контракты и тесты
- ✅ Поддержка как появления, так и исчезновения элемента
- ✅ Proper timeout and cancellation support

### Negative
- ⚠️ Дополнительная сложность конфигурации (timeout, interval, mode)
- ⚠️ Необходимость поддержки дополнительных файлов и тестов
- ⚠️ Потенциальная задержка при больших timeout/interval значениях

## Alternative Considered

### 1. Встроить ожидание в каждый инструмент (ClickTool, TypeTool и т.д.)
**Rejected:** Нарушает принцип DRY и модульности. Каждый инструмент будет дублировать логику ожидания.

### 2. Добавить `wait_element()` как метод `AutomationSession`
**Rejected:** `AutomationSession` отвечает за запуск и сессию приложения, а не за ожидание элементов. Разделение ответственности.

### 3. Использовать `tokio::time::timeout` + `find_first()` вне WaitTool
**Rejected:** Это не提供 periodic polling — только однократный вызов с timeout. WaitTool нужен именно для циклического опроса.

### 4. Просто возвращать `Option<UIElement>` из find_first() и делать loop в пользовательском коде
**Rejected:** Нарушает принцип "zero silent failures" — пользовательский loop может не проверять таймауты и отмены. WaitTool инкапсулирует эту логику.

## Implementation Checklist

- [x] `docs/design/wait-tool/specification.md` — создана (с полной спецификацией API)
- [x] `docs/design/wait-tool/contract.md` — создана (с описанием ожидания)
- [x] `docs/design/wait-tool/test-plan.md` — создана (с сценариями для всех режимов)
- [x] `docs/design/wait-tool/brief.md` — создана (с инструкциями для кодера)
- [x] `docs/adr/006-wait-tool.md` — создана (ADR с архитектурным решением)
- [ ] `src/core/wait.rs` — создана с типами, трейтом, валидацией и unit тестами
- [ ] `src/runtime/backends/windows/wait.rs` — создана с полной реализацией через find_first()
- [ ] `src/runtime/backends/windows/mod.rs` — добавлен `pub mod wait` и экспорт `WaitBackendWindows`
- [ ] `src/runtime/backends/mod.rs` — добавлен экспорт `WaitBackendWindows`
- [ ] `src/runtime/backends/unsupported.rs` — добавлен `WaitBackendUnsupported` stub
- [ ] `src/lib.rs` — добавлен re-export `WaitConfig`, `WaitError`, `WaitMode`, `WaitBackend`, `MockWaitBackend`
- [ ] `tests/wait_tests.rs` — интеграционные тесты (5-7 тестов с `serial_test`)
- [ ] `README.md` — обновлена таблица модулей и архитектура
- [ ] `CHANGELOG.md` — добавлен раздел WaitTool (ADR 006)
- [ ] `context_bundle.md` — обновлён с WaitTool документацией
- [ ] `cargo test` — все интеграционные + unit тесты проходят
- [ ] `cargo clippy -- -D warnings` — без ошибок

## References

- `docs/design/wait-tool/` — полный набор документов
- `docs/design/input-text-tool/` — аналогичная архитектура для референса
- `docs/design/click-tool/` — аналогичная архитектура для референса
- `docs/design/automation-session/` — UIAutomation initialization pattern
- [UIAutomation::create_matcher на docs.rs](https://docs.rs/uiautomation/latest/uiautomation/struct.UIAutomation.html#method.create_matcher)
- [UIElement::find_first на docs.rs](https://docs.rs/uiautomation/latest/uiautomation/struct.UIElement.html#method.find_first)
