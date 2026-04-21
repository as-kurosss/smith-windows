# uiautomation 0.24.4 — Дополнительные функции

**Source**: [docs.rs](https://docs.rs/uiautomation/0.24.4/)

## 📚 Overview

Этот документ содержит **дополнительную информацию** о `uiautomation` crate v0.24.4, которая может быть полезна для реализации "Ctrl+Hover" функциональности в `smith-windows`.

**Статус документации**: 73.02%

**Платформа**: Только Windows (x86_64-pc-windows-msvc)

## 🔑 Дополнительные модули и методы

### Модуль `input` — низкоуровневый ввод

```rust
// uiautomation::input
mod input {
    pub fn mouse_click(x: i32, y: i32) -> Result<(), String>;
    pub fn mouse_move(x: i32, y: i32) -> Result<(), String>;
    pub fn mouse_wheel(x: i32, y: i32, wheel_delta: i32) -> Result<(), String>;
    pub fn key_click(key: Key) -> Result<(), String>;
    pub fn type_text(text: &str) -> Result<(), String>;
}
```

**Использование для "Ctrl+Hover"**:
- `mouse_move(x, y)` — переместить курсор к координатам элемента
- `key_click(Key::ControlKey)` — нажать/отпустить Ctrl
- `mouse_click(x, y)` — кликнуть с зажатым Ctrl

### Модуль `actions` — действия над элементами

```rust
// uiautomation::actions
mod actions {
    pub fn click(element: &UIElement) -> Result<(), String>;
    pub fn right_click(element: &UIElement) -> Result<(), String>;
    pub fn hover(element: &UIElement) -> Result<(), String>;
    pub fn double_click(element: &UIElement) -> Result<(), String>;
    pub fn drag(element: &UIElement, to_x: i32, to_y: i32) -> Result<(), String>;
}
```

**Использование**:
- `hover(element)` — вызвать hover-эффект на элементе
- `click(element)` — стандартный клик

### Модуль `patterns` — UI Automation паттерны

```rust
// uiautomation::patterns
mod patterns {
    pub struct InvokePattern;
    impl InvokePattern {
        pub fn invoke(&self) -> Result<(), String>;
    }

    pub struct ValuePattern;
    impl ValuePattern {
        pub fn set_value(&self, value: &str) -> Result<(), String>;
        pub fn value(&self) -> Result<String, String>;
    }

    pub struct RangeValuePattern;
    impl RangeValuePattern {
        pub fn set_value(&self, value: f64) -> Result<(), String>;
        pub fn value(&self) -> Result<f64, String>;
    }

    pub struct SelectionPattern;
    impl SelectionPattern {
        pub fn selection(&self) -> Result<Vec<UIElement>, String>;
    }
}
```

### Модуль `types` — типы и константы

```rust
// uiautomation::types
mod types {
    pub enum ControlType {
        Button,
        Calendar,
        CheckBox,
        ComboBox,
        Edit,
        Hyperlink,
        Image,
        ListItem,
        List,
        Menu,
        MenuBar,
        MenuItem,
        ProgressBar,
        RadioButton,
        ScrollBar,
        Slider,
        Spinner,
        StatusBar,
        Tab,
        TabItem,
        Text,
        Toolbar,
        Window,
        Pane,
        Header,
        HeaderItem,
        Table,
        TitleBar,
        Separator,
        SemanticZoom,
        ButtonControlType,
        CalendarControlType,
        CheckBoxControlType,
        ComboBoxControlType,
        EditControlType,
        HyperlinkControlType,
        ImageControlType,
        ListItemControlType,
        ListControlType,
        MenuControlType,
        MenuBarControlType,
        MenuItemControlType,
        ProgressBarControlType,
        RadioButtonControlType,
        ScrollBarControlType,
        SliderControlType,
        SpinnerControlType,
        StatusBarControlType,
        TabControlType,
        TabItemControlType,
        TextControlType,
        ToolbarControlType,
        WindowControlType,
        PaneControlType,
        HeaderControlType,
        HeaderItemControlType,
        TableControlType,
        TitleBarControlType,
        SeparatorControlType,
        SemanticZoomControlType,
    }

    pub enum OrientationType {
        None,
        Horizontal,
        Vertical,
    }

    pub enum LayeredWindowType {
        None,
        Top,
        Bottom,
        Topmost,
        Bottommost,
    }
}
```

## 🔧 Ключевые методы для "Ctrl+Hover"

### Получение координат элемента:

```rust
use uiautomation::types::ControlType;

// Получить элемент
let element = ...; // UIElement

// Получить координаты (bounding rectangle)
let rect = element.get_bounding_rectangle()?; // Возвращает (i32, i32, i32, i32) = (x, y, width, height)

// Получить центр элемента
let center_x = rect.0 + rect.2 / 2;
let center_y = rect.1 + rect.3 / 2;

// Получить контрольный тип (для проверки)
let control_type = element.get_control_type()?; // Возвращает ControlType
```

### Двигать курсор к элементу с зажатым Ctrl:

```rust
use uiautomation::input;

// Нажать Ctrl
input::key_click(uiautomation::types::Key::ControlKey)?;

// Переместить курсор к центру элемента
input::mouse_move(center_x, center_y)?;

// Небольшая задержка для визуализации
std::thread::sleep(std::time::Duration::from_millis(100));

// Отпустить Ctrl
input::key_click(uiautomation::types::Key::ControlKey)?;
```

### Использовать hover-паттерн (если доступен):

```rust
use uiautomation::patterns::InvokePattern;

// Попытаться получить паттерн (некоторые элементы поддерживают hover)
if let Ok(pattern) = element.pattern::<InvokePattern>() {
    // Вызвать hover (если реализовано)
    // Заметка: InvokePattern не всегда доступен для всех элементов
}
```

### Ожидание появления элемента под курсором:

```rust
use std::time::Duration;
use uiautomation::UIAutomation;

let automation = UIAutomation::new()?;
let root = automation.desktop()?;

// Повторять поиск с задержкой
for _ in 0..10 {
    let matcher = uiautomation::filters::ControlType(ControlType::Window)
        .and_name("Ожидаемое окно");
    
    if let Ok(element) = root.find_child_by(&matcher) {
        if let Some(el) = element {
            // Элемент найден, получить координаты
            let rect = el.get_bounding_rectangle()?;
            let center_x = rect.0 + rect.2 / 2;
            let center_y = rect.1 + rect.3 / 2;
            
            // Выполнить hover
            input::mouse_move(center_x, center_y)?;
            break;
        }
    }
    
    tokio::time::sleep(Duration::from_millis(200)).await;
}
```

## ⚠️ COM Safety Rules (дополнительно)

**Project-Specific Requirements:**
- **ALL** вызовы `uiautomation` **ДОЛЖНЫ** быть изолированы через `tokio::task::spawn_blocking`
- **NEVER** передавать `UIElement`, `UIAutomation` между потоками (не `Send`/`Sync`)
- **NEVER** вызывать `CoInitializeEx`, `CoUninitialize` напрямую
- **AVOID** вызовы UIA из background threads без правильного контекста

**Rationale:**
`uiautomation` построен на `windows` crate, который использует COM. COM требует правильной инициализации и привязки к потоку (STA — Single-Threaded Apartment). Библиотека `uiautomation` управляет этим внутри, но при использовании с `tokio` все вызовы должны быть изолированы.

## 🎯 Пример: "Ctrl+Hover" через uiautomation

```rust
use std::time::Duration;
use tokio::time;
use uiautomation::{UIAutomation, UIElement, filters, types::ControlType, input};

async fn hover_with_ctrl(element: &UIElement) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Получить координаты центра элемента
    let rect = element.get_bounding_rectangle()?;
    let center_x = rect.0 + rect.2 / 2;
    let center_y = rect.1 + rect.3 / 2;

    tracing::info!("Moving cursor to ({}, {})", center_x, center_y);

    // 2. Нажать Ctrl
    tokio::task::spawn_blocking(|| {
        input::key_click(uiautomation::types::Key::ControlKey)
    }).await??;

    // 3. Переместить курсор к элементу
    tokio::task::spawn_blocking(|| {
        input::mouse_move(center_x, center_y)
    }).await??;

    // 4. Подождать (визуализация hover-эффекта)
    time::sleep(Duration::from_millis(150)).await;

    // 5. Отпустить Ctrl
    tokio::task::spawn_blocking(|| {
        input::key_click(uiautomation::types::Key::ControlKey)
    }).await??;

    Ok(())
}
```

## 🔗 Дополнительные ресурсы

- [docs.rs API](https://docs.rs/uiautomation/0.24.4/)
- [Crates.io](https://crates.io/crates/uiautomation)
- [GitHub Repository](https://github.com/leexgone/uiautomation)
- [UI Automation Specification](https://learn.microsoft.com/en-us/windows/win32/winauto/uiauto-uiautomationreference)

## 📋 Integration Notes for smith-windows

### ✅ Рекомендуется для "Ctrl+Hover"

Для реализации "Ctrl+Hover" в `smith-windows` **рекомендуется использовать `uiautomation` crate**, а не низкоуровневый WinAPI:

1. **`uiautomation::input`** предоставляет `mouse_move`, `mouse_click`, `key_click`
2. **Высокоуровневая абстракция** — не нужно управлять окнами, хуками, сообщениями
3. **COM Safety** — `uiautomation` управляет инициализацией COM сам
4. **Совместимость** — работает со всеми UI Automation-совместимыми приложениями

### 🚫 Что НЕ нужно делать

1. **НЕ использовать `global-hotkey`** — не async-совместим, требует event loop
2. **НЕ использовать `RegisterHotKey`** — требует `HWND`, сложно интегрировать в `spawn_blocking`
3. **НЕ использовать `SetWindowsHookEx`** — сложные COM-требования, проблемы с безопасностью
4. **НЕ использовать `GetCursorPos` напрямую** — `uiautomation` предоставляет `get_bounding_rectangle`

### 📝 Рекомендации по реализации

1. **Добавить новый модуль** `src/core/hover.rs` с трейтом `HoverBackend`
2. **Windows-реализация** в `src/runtime/backends/windows/hover.rs`:
   - Получить координаты через `get_bounding_rectangle()`
   - Использовать `uiautomation::input::mouse_move()` и `key_click()`
   - Обернуть всё в `spawn_blocking`
3. **Интеграция с InspectTool** — при нажатии Ctrl+Hover запустить инспекцию
4. **Документировать** в `docs/design/hover-tool/` по принципу "Contracts First"

### ⚠️ Важные моменты

1. **HIGH DPI**: `get_bounding_rectangle()` возвращает физические координаты, `mouse_move` корректно обрабатывает масштабирование
2. **Асинхронность**: все вызовы через `spawn_blocking`, а не напрямую
3. **Ошибки**: все методы возвращают `Result`, обрабатывать через `thiserror`
4. **Таймауты**: использовать `tokio::time::timeout` для избежания зависаний
5. **Отмена**: поддерживать `CancellationToken` для отмены операций
