---

# 🏗️ smith-windows

**UI Automation API for Windows — Rust library based on `uiautomation` crate**

`smith-windows` — это библиотека на Rust для автоматизации Windows через UI Automation API. Проект является MVP-надстройкой над `uiautomation` crate (v0.24.4) для интеграции в экосистему `smith-core`.

## 📋 Table of Contents

- [Key Features](#-key-features)
- [Installation](#-installation)
- [Quick Start](#-quick-start)
- [Modules](#-modules)
- [Architecture](#-architecture)
- [Development](#-development)
- [License](#-license)

## 🚀 Key Features

- **ClipboardTool**: Работа с системным буфером обмена Windows (получение и установка текста)
- **ClickTool**: Клик по UI-элементам через UI Automation API (левый, правый, двойной клик)
- **RightClickTool**: Правый клик по UI-элементам через UI Automation API (обёртка вокруг ClickTool)
- **TypeTool**: Ввод текста в UI-элементы через clipboard
- **SetTextTool**: Программная установка текста через UI Automation API (не через печать клавиш!)
- **ReadTool**: Получение текстового содержимого из UI-элементов
- **InputTextTool**: Эмуляция ввода с клавиатуры (как будто человек нажимает клавиши) через Keyboard::send_keys()
- **InputTool**: Детекция hover и hotkey (Ctrl+Hover для захвата элементов)
- **WaitTool**: Ожидание появления/исчезновения элементов с таймаутом и периодическим опросом
- **InspectTool**: Интерактивный режим инспекции элементов (Ctrl+Hover для захвата)
- **ScreenshotTool**: Захват скриншотов экрана/окна/региона в PNG формате через GDI/GDI+
- **ScrollTool**: Прокрутка UI-элементов через UI Automation API (programmatic + fallback to synthetic)
- **SelectorStorage**: Сохранение/загрузка селекторов на диск (JSON)
- **AutomationSession**: Управление сессиями UI Automation
- **ToggleTool**: Управление состоянием переключателей (checkboxes, radio buttons, toggle switches)
- **FocusTool**: Активация окна перед взаимодействием с элементами (решение: "окно в фоне")
- **WindowControlTool**: Управление состоянием окон (maximize/restore/minimize) через UI Automation API
- **Idempotent Operations**: Повторные вызовы не ломают состояние
- **Zero Silent Failures**: Все ошибки явные через `Result`
- **COM Safety**: UIA operations run on single thread (STA affinity) with `!Send`/`!Sync` contract

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
smith-windows = { path = "../smith-windows" }
tokio = { version = "1", features = ["full"] }
thiserror = "1"
tracing = "0.1"
```

## 🚀 Quick Start

### Creating a Session

```rust
use smith_windows::launch_process;
use smith_windows::SessionConfig;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = SessionConfig::new("notepad.exe");
    let session = launch_process(config).await?;

    // Use the session...
    Ok(())
}
```

### Clipboard Operations

```rust
use smith_windows::{ClipboardConfig, SetTextParams, get_text_with_config, set_text_with_config, has_text_with_config};
use smith_windows::ClipboardError;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set text to clipboard
    let params = SetTextParams {
        text: "Hello, Clipboard!".to_string(),
    };
    let config = ClipboardConfig {
        timeout: std::time::Duration::from_secs(5),
        cancellation: tokio_util::sync::CancellationToken::new(),
    };
    set_text_with_config(&params, &config).await?;

    // Get text from clipboard
    let text = get_text_with_config(&config).await?;
    println!("Clipboard text: {}", text);

    // Check if clipboard contains text
    let has_text = has_text_with_config(&config).await?;
    println!("Has text: {}", has_text);

    Ok(())
}
```

### Recording and Saving Selectors

```rust
use smith_windows::selector_storage::{SelectorStorage, SelectorStorageConfig, RecordedSelector};
use smith_windows::SelectorRecorder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let recorder = SelectorRecorder::new();
    let config = SelectorStorageConfig::new();
    let storage = SelectorStorage::with_config(config);

    // Capture element under cursor
    let recorded = recorder.capture_element_under_cursor(&head_window).await?;

    // Save to storage
    storage.save_selector("my_selector", &recorded).await?;

    // Load from storage
    let loaded = storage.load_selector("my_selector").await?;

    Ok(())
}
```

### Clicking Elements (Left, Right, Double)

```rust
use smith_windows::{launch_process, ClickConfig, ClickTool, ClickType};
use smith_windows::ClickError;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = SessionConfig::new("notepad.exe");
    let session = launch_process(config).await?;

    // Left single click
    session.click().with_type(ClickType::LeftSingle).await_by_automation_id("ok_button").await?;

    // Right single click
    session.click().with_type(ClickType::RightSingle).await_by_automation_id("context_button").await?;

    // Left double click
    session.click().with_type(ClickType::LeftDouble).await_by_automation_id("file_item").await?;

    Ok(())
}
```

### Right Clicking Elements (Alternative API)

```rust
use smith_windows::{launch_process, RightClickConfig, RightClickTool};
use smith_windows::RightClickError;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = SessionConfig::new("notepad.exe");
    let session = launch_process(config).await?;

    // Right click element by automation ID
    session.right_click().await_by_automation_id("context_menu_button").await?;

    Ok(())
}
```

### Reading Text from Elements

```rust
use smith_windows::{launch_process, ReadConfig, ReadTool};
use smith_windows::ReadError;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = SessionConfig::new("notepad.exe");
    let session = launch_process(config).await?;

    // Get text from element by automation ID
    let text = session.read().await_by_automation_id("text_field").await?;

    Ok(())
}
```

### Taking Screenshots

```rust
use smith_windows::{launch_process, ScreenshotConfig, ScreenshotMode};
use smith_windows::ScreenshotError;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = SessionConfig::new("notepad.exe");
    let session = launch_process(config).await?;

    // Capture screenshot of the window
    let mode = ScreenshotMode::Window(session.main_element().clone());
    let image_data = session.screenshot().capture(&mode).await?;

    // Save to file
    std::fs::write("screenshot.png", image_data)?;

    Ok(())
}
```

### Scrolling Elements

```rust
use smith_windows::{launch_process, ScrollConfig, ScrollDirection};
use smith_windows::ScrollError;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = SessionConfig::new("notepad.exe");
    let session = launch_process(config).await?;

    // Scroll element down
    let config = ScrollConfig::new(Duration::from_secs(5));
    session.scroll().scroll_by_element(&element, &ScrollDirection::Down, &config).await?;

    Ok(())
}
```

### Toggling Elements

```rust
use smith_windows::{launch_process, ToggleConfig, set_toggle_with_config, toggle_element_with_config};
use smith_windows::ToggleError;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = SessionConfig::new("notepad.exe");
    let session = launch_process(config).await?;

    // Toggle checkbox
    let checkbox = session.find_by_automation_id("checkbox").await?;
    let config = ToggleConfig::new(Duration::from_secs(5));
    toggle_element_with_config(&checkbox, &config).await?;

    // Set radio button
    let radio = session.find_by_automation_id("radio_button").await?;
    set_radio_with_config(&radio, true, &config).await?;

    Ok(())
}
```

### Waiting for Elements

```rust
use smith_windows::{launch_process, WaitConfig, WaitMode};
use smith_windows::WaitError;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = SessionConfig::new("notepad.exe");
    let session = launch_process(config).await?;

    // Wait for element to appear
    let config = WaitConfig::new(Duration::from_secs(10), WaitMode::Visible);
    let element = session.wait().await_by_automation_id("target_element", &config).await?;

    Ok(())
}
```

### Inspecting Elements

```rust
use smith_windows::{launch_process, InspectConfig};
use smith_windows::InspectError;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = SessionConfig::new("notepad.exe");
    let session = launch_process(config).await?;

    // Get element info
    let element = session.find_by_automation_id("text_field").await?;
    let info = session.inspect().inspect_element(&element).await?;

    println!("Control Type: {:?}", info.control_type);
    println!("Name: {}", info.name);
    println!("Automation ID: {}", info.automation_id);

    Ok(())
}
```

### Recording Selectors

```rust
use smith_windows::SelectorRecorder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let recorder = SelectorRecorder::new();

    // Capture element under cursor
    let recorded = recorder.capture_element_under_cursor().await?;

    // Get selector string
    let selector = recorded.to_selector_string();
    println!("Selector: {}", selector);

    Ok(())
}
```

### Using Selector Storage

```rust
use smith_windows::selector_storage::{SelectorStorage, SelectorStorageConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = SelectorStorageConfig::new();
    let storage = SelectorStorage::with_config(config);

    // Save selector
    storage.save_selector("my_button", &recorded_selector).await?;

    // Load selector
    let loaded = storage.load_selector("my_button").await?;

    Ok(())
}
```

### Window Control

```rust
use smith_windows::{launch_process, WindowControlConfig, WindowControlAction};
use smith_windows::WindowControlError;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = SessionConfig::new("notepad.exe");
    let session = launch_process(config).await?;

    // Maximize window
    let config = WindowControlConfig::new(Duration::from_secs(5));
    session.window_control().window_control(&session.main_element(), &WindowControlAction::Maximize, &config).await?;

    Ok(())
}
```

### Focusing Window

```rust
use smith_windows::{launch_process, FocusConfig};
use smith_windows::FocusError;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = SessionConfig::new("notepad.exe");
    let session = launch_process(config).await?;

    // Focus window before interaction
    let config = FocusConfig::new(Duration::from_secs(5));
    session.focus().focus_element(&session.main_element(), &config).await?;

    Ok(())
}
```

## 📚 Modules

### Core Modules

| Module | Purpose |
|--------|---------|
| `clipboard` | System clipboard operations (get/set/check text) |
| `click` | Click operations on UI elements |
| `right_click` | Right click operations |
| `type` | Text input via clipboard |
| `set_text` | Programmatic text setting via UIA |
| `read` | Text reading from UI elements |
| `input_text` | Keyboard input emulation |
| `wait` | Element waiting with timeout |
| `inspect` | Element inspection |
| `screenshot` | Screenshot capture |
| `scroll` | Element scrolling |
| `toggle` | Toggle state control |
| `focus` | Window focus management |
| `window_control` | Window state control |
| `selector_storage` | Selector persistence |
| `automation_session` | Session management |

### Error Types

| Error | Module |
|-------|--------|
| `ClipboardError` | clipboard |
| `ClickError` | click |
| `RightClickError` | right_click |
| `TypeError` | type |
| `SetTextError` | set_text |
| `ReadError` | read |
| `InputTextError` | input_text |
| `WaitError` | wait |
| `InspectError` | inspect |
| `ScreenshotError` | screenshot |
| `ScrollError` | scroll |
| `ToggleError` | toggle |
| `FocusError` | focus |
| `WindowControlError` | window_control |
| `SelectorError` | selector |
| `StorageError` | selector_storage |
| `AutomationError` | automation_session |

## 🏗️ Architecture

See `ARCHITECTURE.md` for detailed architecture documentation.

### Key Principles

1. **Contracts First**: Behavior defined before implementation
2. **No Silent Failures**: All errors explicit (`Result`)
3. **Idempotency**: Repeated calls with same input → same state/output
4. **Modularity**: Clear boundaries, single responsibility
5. **Test-Driven Verification**: Tests define correctness
6. **UIAutomation-First**: All UI operations via `uiautomation` crate

### Platform Support

- **Windows**: Full support via `uiautomation` crate and `clipboard` crate
- **Non-Windows**: Stub operations return `OperationNotSupported` error

## 🧪 Development

### Running Tests

```bash
# All tests
cargo test

# Only clipboard tests
cargo test --lib -- clipboard

# Integration tests
cargo test --test clipboard_tool
```

### Linting

```bash
# Strict linting (warnings = errors)
cargo clippy -- -D warnings

# Standard linting
cargo clippy
```

### Formatting

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt --check
```

## 📄 License

This project is licensed under the MIT License - see the `LICENSE` file for details.

## 🙏 Acknowledgments

- Built on top of `uiautomation` crate (v0.24.4)
- Uses `clipboard` crate (v0.5) for clipboard operations
- Inspired by UI Automation patterns and best practices

---

**Author**: smith-planner  
**Date**: 2026-04-22  
**Module**: clipboard-tool  
**Status**: Implementation Complete
