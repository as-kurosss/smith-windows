# 📝 Brief: ScrollTool | smith-windows

## 📁 Sources (reading priority)

1. **docs/design/scroll-tool/specification.md** — вход/выход, границы, критерии успеха
2. **docs/design/scroll-tool/contract.md** — требования, гарантии, запреты, ошибки
3. **docs/design/scroll-tool/test-plan.md** — сценарии тестов, проверки
4. **AGENTS.md** — правила для ИИ-агентов (критично: `?Send` contract, COM safety)
5. **ARCHITECTURE.md** — архитектурные паттерны (UIA `!Send`, GDI `spawn_blocking`)
6. **src/core/click.rs** — эталонный паттерн трейта/валидации/мока
7. **src/core/type.rs** — паттерн с таймаутом и отменой
8. **src/runtime/backends/windows/click.rs** — Windows-реализация UIA
9. **src/runtime/backends/windows/type.rs** — Windows-реализация ввода

## 🔗 Cross-references

| Файл | Назначение |
|------|------------|
| `docs/design/scroll-tool/specification.md` | Спецификация: входные сценарии, выход, границы |
| `docs/design/scroll-tool/contract.md` | Контракт: требования, гарантии, ошибки |
| `docs/design/scroll-tool/test-plan.md` | Тест-план: 15+ тестов, edge cases, идемпотентность |
| `src/core/click.rs` | Эталон: `ClickBackend`, `MockClickBackend`, `validate_*()` |
| `src/runtime/backends/windows/click.rs` | Эталон: `ClickBackendWindows` с UIA `IUIAutomationInvokePattern` |
| `src/runtime/backends/windows/type.rs` | Эталон: `TypeBackendWindows` с `IUIAutomationTextPattern` |

## 🎯 Task

Реализовать `ScrollTool` — инструмент прокрутки элементов UI через UI Automation API.

### Ключевые требования

1. **Два подхода:**
   - **Programmatic**: `IScrollPattern` / `IRangeValuePattern` через `uiautomation` crate
   - **Synthetic**: эмуляция колесика мыши через `uiautomation::inputs`

2. **Механизм fallback:** если паттерн недоступен → пробовать synthetic

3. **COM Safety (CRITICAL):** все UIA-операции в том же потоке (`UIElement` is `!Send`)

4. **Структура:**
   - `src/core/scroll.rs` — трейт, типы, валидация, мок
   - `src/runtime/backends/windows/scroll.rs` — реализация (programmatic + synthetic)
   - `src/runtime/backends/unsupported.rs` — stub для non-Windows

### Валидация

| Параметр | Валидация | Ошибка |
|----------|-----------|--------|
| `timeout` | > 0 и ≤ 1 час | `ScrollError::InvalidConfig` |
| `direction` | "vertical"/"horizontal" (case-insensitive) | `ScrollError::InvalidConfig` |
| `unit` | "line"/"page"/"pixel" | `ScrollError::InvalidConfig` |
| `amount` | 1-100 (wheel), -10000 to 10000 (pixels) | `ScrollError::InvalidConfig` |
| `element` | not null, enabled, onscreen | `ScrollError::ElementNotFound/NotEnabled/Offscreen` |

### Ошибки (`ScrollError` enum)

```rust
#[derive(thiserror::Error, Debug, Clone)]
pub enum ScrollError {
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
    #[error("Pattern not supported")]
    PatternNotSupported,
}
```

### Backend API

#### Programmatic
- `element.scroll_vertical(amount, unit)` → через `IScrollPattern::Scroll`
- `element.scroll_horizontal(amount, unit)` → через `IScrollPattern::Scroll` или `IRangeValuePattern::SetValue`

#### Synthetic
- `inputs::mouse_wheel(ticks, direction)` → через `uiautomation::inputs::mouse_wheel`

### Таймаут и отмена

- `ScrollConfig { timeout: Duration, cancellation: CancellationToken }`
- Валидация `timeout` в `validate_scroll_config()` **ДО** backend вызова
- Таймаут через `tokio::time::timeout()` **ПОСЛЕ** валидации
- Отмена проверяется **ПОСЛЕ** завершения backend call

## 📋 Output Format

**Файлы:**
1. `src/core/scroll.rs` — types, trait, validation, mock, public API
2. `src/runtime/backends/windows/scroll.rs` — `ScrollBackendWindows` implementation
3. `src/runtime/backends/windows/mod.rs` — `pub mod scroll;` + exports
4. `src/runtime/backends/mod.rs` — re-export
5. `src/core/mod.rs` — export
6. `tests/integration/scroll.rs` — integration tests
7. `docs/design/scroll-tool/brief.md` — этот файл

**Структура `src/core/scroll.rs`:**
- `ScrollConfig { timeout, cancellation }`
- `ScrollDirection { Vertical, Horizontal }` (enum)
- `ScrollUnit { Line, Page, Pixel }` (enum)
- `ScrollError` (enum + thiserror)
- `validate_scroll_config()` (validation function)
- `ScrollBackend` trait (programmatic + synthetic)
- `MockScrollBackend` (mock with `Arc<Mutex<MockScrollState>>`)
- `scroll_with_config()` (public async function)

**Структура `src/runtime/backends/windows/scroll.rs`:**
- `ScrollBackendWindows` struct
- `ScrollBackendWindows::new()`
- `#[async_trait::async_trait(?Send)] impl ScrollBackend for ScrollBackendWindows`

## ✅ Mandatory Elements

1. **[VALIDATION BEFORE BACKEND]** — `validate_scroll_config()` вызывается **ДО** создания backend и любых UIA вызовов
2. **[NO UNWRAP/EXPECT/PANIC]** — все `expect()` заменены на `?`, `match`, или `return Err(...)`
3. **[COM SAFETY]** — `#[async_trait::async_trait(?Send)]`, **НЕ** использовать `spawn_blocking` для UIA
4. **[IDEMPOTENCY]** — `MockScrollBackend` с `Arc<Mutex<MockScrollState>>`, повторный вызов `Err` не меняет состояние
5. **[FALLBACK]** — если `IScrollPattern` недоступен → пробовать `IRangeValuePattern`, если неудачно → synthetic
6. **[TIMEOUT]** — через `tokio::time::timeout(config.timeout, future).await`
7. **[CANCELLATION]** — `if config.cancellation.is_cancelled()` **ПОСЛЕ** завершения backend call
8. **[LOGGING]** — `tracing::info!` / `tracing::error!` на границах операции (start, result, error)
9. **[PLATFORM]** — `#[cfg(target_os = "windows")]` для backend, `#[cfg(not(target_os = "windows"))]` stub
10. **[TESTS]** — `#[cfg(test)] mod tests` в `src/core/scroll.rs`, `tests/integration/scroll.rs`

## 🚫 Prohibitions

- ❌ `unwrap()`, `expect()`, `panic!` в `src/`
- ❌ `tokio::task::spawn_blocking` для `UIElement`, `UIAutomation`, `UITreeWalker`
- ❌ `CoInitializeEx`, `CoCreateInstance`, `BSTR`, `VARIANT` в бэкендах
- ❌ `GetForegroundWindow()` — использовать `is_enabled()`/`is_offscreen()` через UIA
- ❌ Изменение контракта без согласования
- ❌ Генерация кода без утверждённого `/plan`
- ❌ Прямое сохранение `ControlType` — использовать string mapping

## 🔄 Process

### Step 1: Core Module (`src/core/scroll.rs`)
1. Определить `ScrollConfig`, `ScrollDirection`, `ScrollUnit`, `ScrollError`
2. Реализовать `validate_scroll_config()` с проверкой всех границ
3. Определить `ScrollBackend` trait (программный интерфейс)
4. Реализовать `MockScrollBackend` с idempotent state
5. Написать `scroll_with_config()` с таймаутом и отменой
6. Добавить `#[cfg(test)]` с 10+ тестами (валидация, мок, edge cases)

### Step 2: Windows Backend (`src/runtime/backends/windows/scroll.rs`)
1. Определить `ScrollBackendWindows` struct
2. Реализовать `new()`
3. Реализовать `ScrollBackend::scroll_vertical()`:
   - Попытка `IScrollPattern::Scroll`
   - Fallback на `IRangeValuePattern::SetValue` для вертикальной прокрутки
4. Реализовать `ScrollBackend::scroll_horizontal()` аналогично
5. Реализовать `ScrollBackend::simulate_mouse_wheel()` через `uiautomation::inputs::mouse_wheel`
6. Обработать все ошибки через `ScrollError`
7. Добавить `#[cfg(test)]` для backend-тестов

### Step 3: Module Integration
1. `src/runtime/backends/windows/mod.rs`: `pub mod scroll;` + `pub use scroll::ScrollBackendWindows;`
2. `src/runtime/backends/mod.rs`: `pub use scroll::ScrollBackendWindows;`
3. `src/core/mod.rs`: `pub mod scroll;`
4. `src/runtime/backends/unsupported.rs`: stub с `ScrollError::UnsupportedPlatformError`

### Step 4: Integration Tests (`tests/integration/scroll.rs`)
1. Full lifecycle: session → find element → scroll vertical/horizontal → verify
2. Fallback tests: element без паттернов → synthetic fallback
3. Error scenarios: invalid element → `ScrollError::ElementNotFound`
4. Timeout and cancellation: token-based cancellation, timeout handling

### Step 5: Verification
1. `cargo test scroll` — 15+ тестов должны пройти
2. `cargo clippy -- -D warnings` — без предупреждений
3. `cargo check` — компиляция без ошибок
4. Проверить `ARCHITECTURE.md` и `CHANGELOG.md` на обновления

## 📝 Metadata

- **Author**: Smith Planner (AI)
- **Date**: 2026-04-22
- **Status**: draft → awaiting_approval
- **Contract Compliance**: ✅ full (см. `docs/design/scroll-tool/contract.md`)
- **COM Safety**: ✅ `?Send` trait, без `spawn_blocking` для UIA
- **Tests**: 15+ unit/integration tests, 100% path coverage
- **Backend**: Windows-only, stub for unsupported platforms
- **Fallback**: programmatic → synthetic (если паттерн недоступен)
