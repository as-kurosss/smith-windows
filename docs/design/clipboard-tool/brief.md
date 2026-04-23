# 📝 Brief: ClipboardTool Implementation

**Author:** smith-planner  
**Date:** 2026-04-22  
**Status:** draft → awaiting_approval  
**Module:** clipboard-tool  

---

## 📁 Sources (reading priority)

1. **This brief** — основная инструкция для кодера
2. `docs/design/clipboard-tool/specification.md` — спецификация
3. `docs/design/clipboard-tool/contract.md` — контракт
4. `docs/design/clipboard-tool/test-plan.md` — сценарии тестов
5. `src/core/click.rs` — аналогичная реализация (последовательная обработка)
6. `src/core/set_text.rs` — аналогичная реализация (валидация + timeout)
7. `src/runtime/backends/windows/click.rs` — Windows backend паттерн
8. `src/runtime/backends/windows/set_text.rs` — Windows backend паттерн

---

## 🔗 Cross-references

- **Parent module:** `src/core/clipboard.rs`
- **Backend module:** `src/runtime/backends/windows/clipboard.rs`
- **Integration tests:** `tests/integration/clipboard_tool.rs`
- **Related tools:** read-tool (получение текста), set_text-tool (установка текста)

---

## 🎯 Task

Реализовать ClipboardTool — инструмент для работы с системным буфером обмена Windows:
- Получение текста из буфера (`get_text`)
- Установка текста в буфер (`set_text`)
- Проверка наличия текста (`has_text`)

**Ключевые требования:**
- Windows only (non-Windows → `ClipboardError::OperationNotSupported`)
- Error handling через `thiserror` (не `anyhow`, не `panic!`)
- Idempotency: повторные вызовы не меняют состояние
- Input validation ДО backend call
- Timeout и cancellation support

---

## 📋 Output Format

**Файлы:**
1. `src/core/clipboard.rs` — трейты, типы, валидация, моки, тесты
2. `src/runtime/backends/windows/clipboard.rs` — Windows-реализация
3. `tests/integration/clipboard_tool.rs` — интеграционные тесты

**Обновления:**
- `src/core/mod.rs` — добавить `pub mod clipboard;`
- `src/runtime/backends/windows/mod.rs` — добавить `pub mod clipboard;` и re-export
- `Cargo.toml` — добавить `clipboard = "0.5"`
- `ARCHITECTURE.md` — добавить ClipboardTool в список инструментов

---

## ✅ Mandatory Elements

**В `src/core/clipboard.rs`:**
- Типы: `ClipboardAction`, `ClipboardConfig`, `SetTextParams`
- Ошибки: `ClipboardError` (thiserror)
- Функции:
  - `validate_clipboard_config(config: &ClipboardConfig) -> Result<(), ClipboardError>`
  - `get_text_with_config(config: &ClipboardConfig) -> Result<String, ClipboardError>`
  - `set_text_with_config(params: &SetTextParams, config: &ClipboardConfig) -> Result<(), ClipboardError>`
  - `has_text_with_config(config: &ClipboardConfig) -> Result<bool, ClipboardError>`
- Трейты:
  - `#[async_trait::async_trait(?Send)] pub trait ClipboardBackend`
- Моки:
  - `MockClipboardBackend` с `Arc<Mutex<MockClipboardState>>`
  - `MockClipboardState` для изоляции состояния

**В `src/runtime/backends/windows/clipboard.rs`:**
- `ClipboardBackendWindows` — Windows-реализация через clipboard crate
- Методы: `get_text`, `set_text`, `has_text`
- All errors → `ClipboardError` (не `unwrap()`, не `panic!`)
- Логирование: `tracing::{info, error}!`

**В `tests/integration/clipboard_tool.rs`:**
- Полный lifecycle: create → run → complete/error/cancel
- Тесты отмены через CancellationToken
- Тесты таймаута
- Тесты идемпотентности при повторных вызовах с `Err`

**Валидация (в `validate_clipboard_config()`):**
- `timeout > 0` и `timeout <= Duration::from_secs(3600)`
- Возврат `InvalidConfig` для невалидных значений

**Тесты (в `#[cfg(test)] mod tests` внутри `src/core/clipboard.rs`):**
- Base: валидные данные → успех
- Boundary: `timeout=0`, `timeout>1hr`, `text=""` → ожидаемые ошибки
- Negative: отмена, таймаут → ожидаемые ошибки
- Mock: создание, сброс, идемпотентность при `Err`

---

## 🚫 Prohibitions

**В коде:**
- ❌ `unwrap()`, `expect()`, `panic!` — только `Result`/`Option`
- ❌ Глобальные мутабельные состояния без `Arc` + явного контекста
- ❌ Прямой вызов COM API (`CoInitializeEx`, `CoCreateInstance`, `BSTR`, `VARIANT`)
- ❌ `GetForegroundWindow()` — не использовать

**В документации:**
- ❌ Расхождения между `brief.md`, `specification.md`, `contract.md` и `/plan`
- ❌ Отсутствие тест-планов для модулей
- ❌ Документы без явных критериев успеха

---

## 🔄 Process

1. **Создать `docs/design/clipboard-tool/`**
   - specification.md (уже создан)
   - contract.md (уже создан)
   - test-plan.md (уже создан)
   - brief.md (уже создан)

2. **Создать `src/core/clipboard.rs`**
   - Типы: `ClipboardAction`, `ClipboardConfig`, `SetTextParams`
   - Ошибки: `ClipboardError` (thiserror)
   - Функции: `validate_clipboard_config`, `get_text_with_config`, `set_text_with_config`, `has_text_with_config`
   - Трейты: `ClipboardBackend` + `MockClipboardBackend`
   - Тесты: base, boundary, negative

3. **Создать `src/runtime/backends/windows/clipboard.rs`**
   - `ClipboardBackendWindows` — Windows-реализация
   - Методы: `get_text`, `set_text`, `has_text`
   - All errors → `ClipboardError`
   - Логирование: `tracing::{info, error}!`

4. **Обновить модульные файлы**
   - `src/core/mod.rs` — добавить `pub mod clipboard;`
   - `src/runtime/backends/windows/mod.rs` — добавить `pub mod clipboard;` и re-export
   - `Cargo.toml` — добавить `clipboard = "0.5"`

5. **Создать интеграционные тесты**
   - `tests/integration/clipboard_tool.rs`
   - Полный lifecycle, отмена, таймаут, идемпотентность

6. **Обновить документацию**
   - `ARCHITECTURE.md` — добавить ClipboardTool в список инструментов

7. **Проверки**
   - `cargo test`
   - `cargo clippy -- -D warnings`
   - `cargo fmt --check`

---

## 📝 Metadata

| Поле | Значение |
|------|----------|
| **Author** | smith-planner |
| **Date** | 2026-04-22 |
| **Status** | draft → awaiting_approval |
| **Module** | clipboard-tool |
| **Platform** | Windows only |
| **Dependencies** | `clipboard = "0.5"` |

---

**✅ Check before submit:**
- [ ] Все файлы созданы/обновлены
- [ ] Все функции возвращают `Result<T, ClipboardError>`
- [ ] Нет `unwrap()`/`panic!`/`expect()` в `src/`
- [ ] Валидация вызывается ДО backend
- [ ] Тесты покрывают base/boundary/negative
- [ ] `cargo test` и `cargo clippy -- -D warnings` проходят
- [ ] Документация согласована

**📌 Note:** If any discrepancy between this brief and spec/contract/test-plan → **STOP** and ask architect for clarification.
