## 🗓️ /plan: ClipboardTool Implementation

**Author:** smith-planner  
**Date:** 2026-04-22  
**Status:** awaiting_approval  
**Module:** clipboard-tool  

---

### [Файл] → [Сущности] → [cfg-флаги] → [Тесты] → [Валидация]

---

### 📁 Файлы

**Создать:**
1. `src/core/clipboard.rs` — трейты, типы, валидация, моки, тесты
2. `src/runtime/backends/windows/clipboard.rs` — Windows-реализация
3. `docs/design/clipboard-tool/specification.md` — спецификация (уже создан)
4. `docs/design/clipboard-tool/contract.md` — контракт (уже создан)
5. `docs/design/clipboard-tool/test-plan.md` — сценарии тестов (уже создан)
6. `docs/design/clipboard-tool/plan.md` — этот документ (уже создан)

**Обновить:**
7. `src/core/mod.rs` — добавить `pub mod clipboard;`
8. `src/runtime/backends/windows/mod.rs` — добавить `pub mod clipboard;` и re-export
9. `Cargo.toml` — добавить `clipboard = "0.5"`
10. `ARCHITECTURE.md` — добавить ClipboardTool в список инструментов

---

### 📋 Сущности

**Типы:**
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClipboardAction {
    GetText,
    SetText,
    HasText,
}

#[derive(Debug, Clone)]
pub struct ClipboardConfig {
    pub timeout: Duration,
    pub cancellation: CancellationToken,
}

#[derive(Debug, Clone)]
pub struct SetTextParams {
    pub text: String,
}
```

**Ошибки (`thiserror`):**
```rust
#[derive(Error, Debug, Clone)]
pub enum ClipboardError {
    #[error("Operation not supported")]
    OperationNotSupported,
    #[error("Clipboard is empty")]
    ClipboardEmpty,
    #[error("Clipboard access denied")]
    ClipboardAccessDenied,
    #[error("Text is empty")]
    TextEmpty,
    #[error("Operation timed out")]
    Timeout,
    #[error("Operation was cancelled")]
    Cancelled,
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
    #[error("COM error: {0}")]
    ComError(String),
}
```

**Трейты:**
```rust
#[async_trait::async_trait(?Send)]
pub trait ClipboardBackend {
    async fn get_text(&self) -> Result<String, ClipboardError>;
    async fn set_text(&self, text: &str) -> Result<(), ClipboardError>;
    async fn has_text(&self) -> Result<bool, ClipboardError>;
}
```

**Моки:**
```rust
#[derive(Debug, Clone, Default)]
pub struct MockClipboardBackend {
    state: std::sync::Arc<std::sync::Mutex<MockClipboardState>>,
}

#[derive(Debug, Default)]
pub struct MockClipboardState {
    pub call_count: usize,
    pub last_error: Option<ClipboardError>,
    pub should_succeed: bool,
    pub clipboard_has_text: bool,
    pub clipboard_text: Option<String>,
}

impl MockClipboardBackend {
    pub fn new() -> Self;
    pub fn with_state(state: MockClipboardState) -> Self;
    pub fn get_state(&self) -> std::sync::MutexGuard<'_, MockClipboardState>;
    pub fn reset(&self);
}
```

**Функции:**
```rust
pub fn validate_clipboard_config(config: &ClipboardConfig) -> Result<(), ClipboardError>;

pub async fn get_text_with_config(config: &ClipboardConfig) -> Result<String, ClipboardError>;

pub async fn set_text_with_config(params: &SetTextParams, config: &ClipboardConfig) -> Result<(), ClipboardError>;

pub async fn has_text_with_config(config: &ClipboardConfig) -> Result<bool, ClipboardError>;
```

---

### 🛠️ cfg-флаги

```rust
#[cfg(target_os = "windows")]
// Windows-реализация через clipboard crate

#[cfg(not(target_os = "windows"))]
// Stub: вернуть ClipboardError::OperationNotSupported
```

**В Cargo.toml добавить:**
```toml
clipboard = "0.5"
```

---

### 🧪 Тесты

**В `src/core/clipboard.rs` (`#[cfg(test)] mod tests`):**

**Валидация:**
- `test_validate_clipboard_config_valid` — timeout 5s → Ok
- `test_validate_clipboard_config_zero_timeout` — Duration::ZERO → Err::InvalidConfig
- `test_validate_clipboard_config_large_timeout` — 3601s → Err::InvalidConfig

**Мок backend:**
- `test_mock_backend_creation` — call_count == 0
- `test_mock_backend_with_state` — should_succeed
- `test_mock_backend_reset` — сброс состояния
- `test_mock_backend_idempotent_error` — повторный вызов Err не меняет состояние

**Отмена и таймаут:**
- `test_get_text_cancellation_before` — отмена до операции
- `test_get_text_timeout` — Duration::ZERO → InvalidConfig
- `test_set_text_text_empty` — пустой текст → InvalidConfig

**В `tests/integration/clipboard_tool.rs`:**
- Полный lifecycle: create → run → complete/error/cancel
- Тесты отмены через CancellationToken
- Тесты таймаута
- Тесты идемпотентности при повторных вызовах с `Err`

---

### ✅ Валидация

**В `validate_clipboard_config()`:**
1. `timeout > 0` — иначе `InvalidConfig("timeout must be > 0")`
2. `timeout <= Duration::from_secs(3600)` — иначе `InvalidConfig("timeout must be <= 1 hour")`

**В `set_text_with_config()`:**
3. `params.text.is_empty()` — иначе `InvalidConfig("text cannot be empty")`

**Все проверки ДО backend call.**

---

### 🚫 Запреты

- ❌ `unwrap()`, `expect()`, `panic!` — только `Result`/`Option`
- ❌ `CoInitializeEx`, `CoCreateInstance`, `BSTR`, `VARIANT`
- ❌ `GetForegroundWindow()`
- ✅ `clipboard` crate — синхронный, **можно использовать без `spawn_blocking`**
- ✅ Все ошибки через `ClipboardError`, idempotency при `Err`

---

### 📝 Проверки после реализации

```bash
cargo test --lib -- clipboard
cargo test --test clipboard_tool
cargo clippy -- -D warnings
cargo fmt --check
```

---

**✅ Status: draft → awaiting_approval**  
**📌 Note:** After architect approval, run `cargo run --bin bundle_context` before coding.
