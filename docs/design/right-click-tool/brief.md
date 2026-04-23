## 📝 Brief: RightClickTool | smith-windows

**Роль:** Senior Rust Engineer (Implementation)

**Цель:** Реализовать RightClickTool для выполнения правого клика по UI элементам через uiautomation crate.

---

## 🎯 Файл-структура

```
docs/design/right-click-tool/
├── specification.md    ✅ (создано)
├── contract.md         ✅ (создано)
├── test-plan.md        ✅ (создано)
└── brief.md            ✅ (создано)

src/
├── core/
│   └── right_click.rs          🆕 (создать)
└── runtime/backends/windows/
    └── right_click.rs          🆕 (создать)

examples/
└── right_click_example.rs      🆕 (создать)
```

---

## 🧱 Типы (src/core/right_click.rs)

### Импорты
```rust
use std::time::Duration;
use thiserror::Error;
use tokio_util::sync::CancellationToken;
use uiautomation::types::Point;
```

### Типы

#### `RightClickConfig`
```rust
#[derive(Debug, Clone)]
pub struct RightClickConfig {
    pub timeout: Duration,
    pub cancellation: CancellationToken,
}
```

#### `RightClickError`
```rust
#[derive(Error, Debug, Clone)]
pub enum RightClickError {
    #[error("Element not found")]
    ElementNotFound,
    
    #[error("Element is disabled")]
    ElementNotEnabled,
    
    #[error("Element is offscreen")]
    ElementOffscreen,
    
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

---

## 🔧 Валидация (src/core/right_click.rs)

### `validate_right_click_config()`
```rust
pub fn validate_right_click_config(config: &RightClickConfig) -> Result<(), RightClickError>
```

**Правила:**
- `timeout > 0` (не zero)
- `timeout <= Duration::from_secs(3600)` (не больше 1 часа)
- `cancellation` — любой валидный CancellationToken

**Возврат:** `Ok(())` или `Err(RightClickError::InvalidConfig)`

---

## 🌐 Backend Trait (src/core/right_click.rs)

### `RightClickBackend` trait
```rust
#[async_trait::async_trait(?Send)]
pub trait RightClickBackend {
    async fn right_click(&self, element: &uiautomation::UIElement) -> Result<(), RightClickError>;
}
```

**Примечание:** `?Send` — UIElement is !Send / !Sync

---

## 🪟 Windows Backend (src/runtime/backends/windows/right_click.rs)

### `RightClickBackendWindows`
```rust
pub struct RightClickBackendWindows;

impl RightClickBackendWindows {
    pub fn new() -> Self;
}

impl Default for RightClickBackendWindows;

#[async_trait::async_trait(?Send)]
impl RightClickBackend for RightClickBackendWindows {
    async fn right_click(&self, element: &uiautomation::UIElement) -> Result<(), RightClickError>;
}
```

**Реализация right_click():**
1. Проверка валидности элемента (get_control_type)
2. Проверка enabled (is_enabled())
3. Проверка offscreen (is_offscreen())
4. Получение координат центра элемента (bounding rectangle)
5. Выполнение правого клика через `uiautomation::inputs::Mouse::right_click(&Point)`
6. Обработка ошибок COM

**Важно:** COM-вызовы НЕ через `spawn_blocking` — UIElement is !Send, а методы uiautomation синхронные

---

## 🔌 Высокоуровневая функция (src/core/right_click.rs)

### `right_click_with_config()`
```rust
pub async fn right_click_with_config(
    element: &uiautomation::UIElement,
    config: &RightClickConfig,
) -> Result<(), RightClickError>
```

**Алгоритм:**
1. Валидация config → `validate_right_click_config()`
2. Создание backend → `RightClickBackendWindows::new()`
3. Вызов `backend.right_click(element).await`
4. Проверка cancellation после завершения
5. Обработка timeout через `tokio::time::timeout()`

**Важно:** Если UIElement is !Send, timeout wrapper может не работать — использовать ручную проверку cancellation

---

## 🧪 Мок Backend (src/core/right_click.rs)

### `MockRightClickBackend`
```rust
#[derive(Debug, Clone, Default)]
pub struct MockRightClickBackend {
    state: Arc<Mutex<MockRightClickState>>,
}

#[derive(Debug, Default)]
pub struct MockRightClickState {
    pub call_count: usize,
    pub last_error: Option<RightClickError>,
    pub should_succeed: bool,
}

impl MockRightClickBackend {
    pub fn new() -> Self;
    pub fn with_state(state: MockRightClickState) -> Self;
    pub fn get_state(&self) -> MutexGuard<MockRightClickState>;
    pub fn reset(&self);
}

#[async_trait::async_trait(?Send)]
impl RightClickBackend for MockRightClickBackend {
    async fn right_click(&self, _element: &uiautomation::UIElement) -> Result<(), RightClickError>;
}
```

**Логика:**
- `state.call_count++` при каждом вызове
- Если `state.should_succeed` → `Ok(())`
- Иначе → `Err(state.last_error.clone().unwrap_or(ElementNotFound))`

---

## 🧪 Тесты (src/core/right_click.rs)

### Unit tests (в `#[cfg(test)] mod tests`)

1. `test_validate_right_click_config_valid()`
2. `test_validate_right_click_config_zero_timeout()`
3. `test_validate_right_click_config_large_timeout()`
4. `test_validate_right_click_config_one_hour_timeout()`
5. `test_validate_right_click_config_over_one_hour_timeout()`
6. `test_mock_backend_creation()`
7. `test_mock_backend_with_state()`
8. `test_mock_backend_reset()`
9. `test_mock_backend_success()`
10. `test_mock_backend_failure_element_not_found()`
11. `test_mock_backend_failure_element_not_enabled()`
12. `test_mock_backend_failure_element_offscreen()`

---

## 📚 Пример (examples/right_click_example.rs)

```rust
//! Example: Right Click Tool Usage
//!
//! Run: cargo run --example right_click_example

use std::time::Duration;
use tracing_subscriber::EnvFilter;
use smith_windows::core::right_click::{RightClickConfig, RightClickError, MockRightClickBackend};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    println!("=== RightClickTool Example ===");

    // Example 1: Configuration validation
    example_configuration().await?;

    // Example 2: Mock backend
    example_mock_backend().await?;

    Ok(())
}

async fn example_configuration() -> Result<(), Box<dyn std::error::Error>> {
    let cancellation = tokio_util::sync::CancellationToken::new();
    let config = RightClickConfig {
        timeout: Duration::from_secs(5),
        cancellation,
    };

    match validate_right_click_config(&config) {
        Ok(()) => println!("✓ Valid configuration"),
        Err(e) => println!("✗ Error: {}", e),
    }
    Ok(())
}

async fn example_mock_backend() -> Result<(), Box<dyn std::error::Error>> {
    let backend = MockRightClickBackend::new();
    println!("Mock backend created (for testing without UI)");
    Ok(())
}
```

---

## 📦 Экспорт модулей

### src/core/mod.rs
```rust
pub mod right_click;
```

### src/runtime/backends/windows/mod.rs
```rust
pub mod right_click;

pub use crate::runtime::backends::windows::right_click::RightClickBackendWindows;
```

---

## ✅ Критерии успеха

- [ ] Все файлы созданы по структуре
- [ ] `RightClickConfig`, `RightClickError`, `RightClickBackend` реализованы
- [ ] `validate_right_click_config()` валидирует timeout (0 < t <= 3600)
- [ ] `RightClickBackendWindows::right_click()` использует `uiautomation::inputs::Mouse::right_click()`
- [ ] Мок backend для тестирования без UI
- [ ] `right_click_with_config()` с timeout и cancellation
- [ ] Все тесты проходят (`cargo test --lib right_click`)
- [ ] `cargo clippy -- -D warnings` без ошибок
- [ ] Пример компилируется и запускается

---

## ⚠️ Важные замечания

1. **COM Safety:** UIElement is !Send / !Sync — не использовать `spawn_blocking` для него
2. **Timeout handling:** Если timeout wrapper не работает из-за !Send, использовать ручную проверку `cancellation.is_cancelled()`
3. **Mouse right click:** Использовать `uiautomation::inputs::Mouse::new().right_click(&Point)` или `click_button(MouseButton::RIGHT)`
4. **No panics:** Только `Result`/`Option`, никаких `unwrap()`/`panic!`

---

**Стартуем реализацию после утверждения плана архитектором.**
