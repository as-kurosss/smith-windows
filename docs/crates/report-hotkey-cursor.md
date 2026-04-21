# 📊 Отчёт по исследованию крейтов для глобальных хоткеев и мониторинга курсора

**Дата**: 21 апреля 2026  
**Проект**: smith-windows v0.1.0  
**Цель**: Реализация "Ctrl + Hover over element" для получения селектора UI элемента

---

## 📋 Резюме

Для реализации функциональности "Ctrl + hover over element" в `smith-windows` рекомендуется:

✅ **Использовать `uiautomation` crate** для всех операций:
- Получение координат элементов через `get_bounding_rectangle()`
- Перемещение курсора через `uiautomation::input::mouse_move()`
- Нажатие клавиш через `uiautomation::input::key_click()`

❌ **НЕ рекомендуется использовать**:
- `global-hotkey` — не async-совместим, требует Win32 event loop
- `windows-input` — низкоуровневый WinAPI, сложнее интегрировать
- `hotkey` — отсутствует документация, низкая стабильность

**Обоснование**:
- `smith-windows` построен на принципах **COM Isolation** и **UIAutomation-First**
- Все вызовы должны быть изолированы через `tokio::task::spawn_blocking`
- `uiautomation` предоставляет всю необходимую функциональность высокоуровневым API

---

## 🔍 Рассмотренные крейты

### 1. global-hotkey 0.7.0 ✅

| Характеристика | Значение |
|----------------|----------|
| **Версия** | 0.7.0 |
| **Лицензия** | Apache-2.0 OR MIT |
| **Платформы** | Windows, macOS, Linux (X11) |
| **Документация** | ~53.49% |
| **Async-совместимость** | ❌ Нет |
| **COM Safety** | ⚠️ Требует Win32 event loop |
| **Интеграция с tokio** | ❌ Требует обёртки в отдельном потоке |

**Ключевые компоненты**:
- `GlobalHotKeyManager` — управление хоткеями
- `HotKey` — определение горячей клавиши
- `GlobalHotKeyEvent::receiver()` — **синхронный** `crossbeam_channel`

**Проблемы для smith-windows**:
1. ❌ Не async-совместим — все вызовы синхронные
2. ❌ Требует активный Win32 message loop на том же потоке
3. ❌ Сложная интеграция с `tokio::task::spawn_blocking`

**Вывод**: **НЕ рекомендуется** для прямой интеграции в `smith-windows`

---

### 2. windows-input (windows crate) v0.58+ ✅

| Характеристика | Значение |
|----------------|----------|
| **Версия** | v0.58+ (modern windows-rs) |
| **Лицензия** | MIT |
| **Платформы** | Только Windows |
| **Async-совместимость** | ❌ Нет (синхронные WinAPI) |
| **COM Safety** | ⚠️ Требует spawn_blocking |
| **Интеграция с tokio** | ⚠️ Через `tokio::task::spawn_blocking` |

**Ключевые функции**:
- `SendInput`, `keybd_event`, `mouse_event` — симуляция ввода
- `GetCursorPos`, `SetCursorPos` — получение/установка курсора
- `RegisterHotKey`, `UnregisterHotKey` — глобальные хоткеи
- `SetWindowsHookEx`, `CallNextHookEx` — хуки сообщений

**Проблемы для smith-windows**:
1. ❌ Низкоуровневый WinAPI — требует управления окнами, сообщениями
2. ❌ `RegisterHotKey` требует `HWND` окна
3. ❌ Сложнее интегрировать в `spawn_blocking` из-за зависимости от контекста

**Вывод**: **НЕ рекомендуется** для основной функциональности, **возможно** для специфичных сценариев (глобальные хоткеи, низкоуровневый ввод)

---

### 3. hotkey 0.3.1 ⚠️

| Характеристика | Значение |
|----------------|----------|
| **Версия** | 0.3.1 |
| **Лицензия** | MIT |
| **Платформы** | Windows, macOS, Linux |
| **Документация** | 0% |
| **Async-совместимость** | ❌ Неизвестно |
| **COM Safety** | ⚠️ Требует исследование |

**Ключевые компоненты**:
- `Listener` — основной тип для управления хоткеями
- `ListenerID` — идентификатор зарегистрированного обработчика

**Проблемы для smith-windows**:
1. ❌ **0% документации** — невозможно определить безопасность вызовов
2. ❌ Отсутствие примеров и документации по `register_hotkey()`
3. ❌ Неизвестна async-совместимость

**Вывод**: **НЕ рекомендуется** из-за отсутствия документации

---

### 4. uiautomation 0.24.4 ✅ (РЕКОМЕНДУЕТСЯ)

| Характеристика | Значение |
|----------------|----------|
| **Версия** | 0.24.4 |
| **Лицензия** | MIT |
| **Платформы** | Только Windows |
| **Документация** | 73.02% |
| **Async-совместимость** | ✅ Через `spawn_blocking` |
| **COM Safety** | ✅ Управляет COM внутри |
| **Интеграция с tokio** | ✅ Полная поддержка |

**Ключевые функции для "Ctrl+Hover"**:

#### Модуль `input`:
```rust
// uiautomation::input
pub fn mouse_click(x: i32, y: i32) -> Result<(), String>;
pub fn mouse_move(x: i32, y: i32) -> Result<(), String>;
pub fn key_click(key: Key) -> Result<(), String>;
pub fn type_text(text: &str) -> Result<(), String>;
```

#### Модуль `types`:
```rust
// uiautomation::types
pub enum Key {
    ControlKey,
    ShiftKey,
    AltKey,
    // ...
}

pub enum ControlType {
    Button, Calendar, CheckBox, ComboBox, Edit, Hyperlink,
    Image, ListItem, List, Menu, MenuBar, MenuItem,
    ProgressBar, RadioButton, ScrollBar, Slider, Spinner,
    StatusBar, Tab, TabItem, Text, Toolbar, Window, Pane,
    // ...
}
```

**Преимущества для smith-windows**:
1. ✅ **Высокоуровневая абстракция** — не нужно управлять окнами, хуками
2. ✅ **COM Safety** — библиотека управляет инициализацией COM
3. ✅ **Async-совместимость** — все вызовы через `spawn_blocking`
4. ✅ **Встроенная интеграция** — уже используется в `smith-windows`
5. ✅ **Документация 73.02%** — достаточна для понимания

**Вывод**: **РЕКОМЕНДУЕТСЯ** для реализации "Ctrl+Hover" в `smith-windows`

---

## 🎯 Рекомендованный подход для "Ctrl+Hover"

### Архитектура

```
User Action (Ctrl + Hover) → InspectTool → InspectBackend → UIElement Selector Path
                                 ↓
                    uiautomation::input (mouse_move, key_click)
                                 ↓
                    UIElement.get_bounding_rectangle()
                                 ↓
                    build_inspect_path_with_walker()
```

### Реализация

#### 1. Создать модуль `src/core/hover.rs`

```rust
use std::time::Duration;
use thiserror::Error;
use tokio_util::sync::CancellationToken;

/// Configuration for hover operations
#[derive(Debug, Clone)]
pub struct HoverConfig {
    pub timeout: Duration,
    pub cancellation: CancellationToken,
}

/// Errors that can occur during hover operations
#[derive(Error, Debug, Clone)]
pub enum HoverError {
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

/// Trait for hover backend implementations
#[async_trait::async_trait(?Send)]
pub trait HoverBackend {
    async fn hover_with_ctrl(&self, element: &uiautomation::UIElement) -> Result<(), HoverError>;
}
```

#### 2. Windows-реализация `src/runtime/backends/windows/hover.rs`

```rust
use tracing::{error, info};
use crate::core::hover::{HoverBackend, HoverError};

pub struct HoverBackendWindows;

impl HoverBackendWindows {
    pub fn new() -> Self {
        Self
    }
}

impl Default for HoverBackendWindows {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait(?Send)]
impl HoverBackend for HoverBackendWindows {
    async fn hover_with_ctrl(&self, element: &uiautomation::UIElement) -> Result<(), HoverError> {
        // 1. Получить координаты центра элемента
        let rect = element.get_bounding_rectangle().map_err(|e| {
            error!("Failed to get bounding rectangle: {}", e);
            HoverError::ComError(e.to_string())
        })?;
        
        let center_x = rect.0 + rect.2 / 2;
        let center_y = rect.1 + rect.3 / 2;

        // 2. Нажать Ctrl
        tokio::task::spawn_blocking(|| {
            uiautomation::input::key_click(uiautomation::types::Key::ControlKey)
        }).await.map_err(|e| {
            error!("Failed to press Ctrl: {}", e);
            HoverError::ComError(e.to_string())
        })??;

        // 3. Переместить курсор к элементу
        tokio::task::spawn_blocking(move || {
            uiautomation::input::mouse_move(center_x, center_y)
        }).await.map_err(|e| {
            error!("Failed to move mouse: {}", e);
            HoverError::ComError(e.to_string())
        })??;

        // 4. Подождать (визуализация hover-эффекта)
        tokio::time::sleep(std::time::Duration::from_millis(150)).await;

        // 5. Отпустить Ctrl
        tokio::task::spawn_blocking(|| {
            uiautomation::input::key_click(uiautomation::types::Key::ControlKey)
        }).await.map_err(|e| {
            error!("Failed to release Ctrl: {}", e);
            HoverError::ComError(e.to_string())
        })??;

        info!("Hover with Ctrl completed successfully");
        Ok(())
    }
}
```

#### 3. Интеграция с InspectTool

```rust
// В src/core/inspect.rs
pub async fn inspect_with_hover(
    element: &uiautomation::UIElement,
    config: &InspectConfig,
) -> Result<String, InspectError> {
    // 1. Выполнить hover с Ctrl
    let hover_backend = crate::runtime::backends::windows::hover::HoverBackendWindows::new();
    hover_backend.hover_with_ctrl(element).await?;
    
    // 2. Получить путь элемента (как в текущей реализации)
    let head_window = ...; // Получить активное окно
    let path = crate::runtime::backends::windows::inspect::build_inspect_path_with_walker(
        head_window, element
    )?;
    
    Ok(path)
}
```

---

## ⚠️ Правила безопасности

### ❌ Запрещено
1. **Прямой вызов `CoInitializeEx`**, `CoCreateInstance` — библиотека управляет сама
2. **Передача `UIElement`**, `UIAutomation` между потоками — не `Send`/`Sync`
3. **Использование `global-hotkey`** — не async-совместим
4. **Использование `RegisterHotKey`** — требует `HWND`, сложная интеграция
5. **Прямой вызов WinAPI** без `spawn_blocking` — нарушение COM Safety

### ✅ Обязательно
1. **Все вызовы WinAPI** → `tokio::task::spawn_blocking`
2. **Все ошибки** → явные `Result<T, Error>` через `thiserror`
3. **Проверка идемпотентности** — повторные вызовы не должны менять состояние при ошибках
4. **Поддержка отмены** — использовать `CancellationToken`
5. **TIMEOUT** — всегда использовать `tokio::time::timeout`

---

## 📝 Дополнительные ресурсы

### Созданные документы
- `docs/crates/global-hotkey.md` — полное описание `global-hotkey`
- `docs/crates/windows-input.md` — полное описание `windows-input`
- `docs/crates/uiautomation-advanced.md` — дополнительные методы `uiautomation`
- `docs/crates/uiautomation.md` — основное описание `uiautomation`

### Полезные ссылки
- [uiautomation docs.rs](https://docs.rs/uiautomation/0.24.4/)
- [global-hotkey docs.rs](https://docs.rs/global-hotkey/0.7.0/global_hotkey/)
- [windows-rs docs](https://microsoft.github.io/windows-rs/)
- [UI Automation Specification](https://learn.microsoft.com/en-us/windows/win32/winauto/uiauto-uiautomationreference)

---

## 🎓 Выводы

| Вопрос | Ответ |
|--------|-------|
| **Какой крейт использовать для "Ctrl+Hover"?** | `uiautomation` crate (уже используется в проекте) |
| **Какие функции использовать?** | `uiautomation::input::mouse_move()`, `key_click()` |
| **Как получить координаты элемента?** | `element.get_bounding_rectangle()` |
| **Как изолировать вызовы?** | Через `tokio::task::spawn_blocking` |
| **Можно ли использовать `global-hotkey`?** | Нет, не async-совместим |
| **Можно ли использовать `windows-input`?** | Только для специфичных сценариев, не рекомендуется |

---

**Рекомендация**: Продолжить развитие `smith-windows` на основе `uiautomation` crate, добавив новый модуль `hover-tool` с полной поддержкой "Ctrl+Hover" функциональности через `uiautomation::input`.

**План реализации**:
1. `docs/design/hover-tool/specification.md` — спецификация
2. `docs/design/hover-tool/contract.md` — контракт
3. `docs/design/hover-tool/test-plan.md` — тест-план
4. `docs/design/hover-tool/brief.md` — инструкция для кодера
5. Реализация в `src/core/hover.rs` + `src/runtime/backends/windows/hover.rs`
