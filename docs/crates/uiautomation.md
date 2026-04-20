# uiautomation 0.24.4

**Source**: [docs.rs](https://docs.rs/uiautomation/0.24.4/)

## 📚 Overview

`uiautomation` — Rust wrapper вокруг Windows UI Automation API.  
Предоставляетsafe абстракции для взаимодействия с элементами интерфейса.  
**Только Windows**, x86_64-pc-windows-msvc.

**Поддержка документации**: 73.02%

## 🔑 Key Types

| Type | Description |
|------|-------------|
| `UIAutomation` | Главный объект, предоставляющий доступ к элементам интерфейса |
| `UIElement` | Представление элемента UI (окна, кнопки и т.д.) |
| `UITreeWalker` | Для обхода иерархии элементов UI (родители, дети, братья/сёстры) |
| `UIMatcher` | Для построения условий поиска элементов |
| `Error`, `Result` | Обработка ошибок |
| `InvokePattern`, `ValuePattern`, `SelectionPattern`, `TogglePattern` | UI Automation-паттерны |

## 🔧 Key Methods

#### Модули:
- `core` — основные типы: `UIAutomation`, `UIElement`, `UITreeWalker`, `UIMatcher`
- `patterns` — поддержка UI Automation-паттернов
- `actions` — выполнение действий над элементами (клик, ввод текста)
- `events` — работа с событиями UI
- `filters` — строители фильтров для поиска элементов
- `types`/`variants` — определения типов и вариантов
- `input` — низкоуровневая работа с вводом
- `clipboard` — доступ к буферу обмена
- `processes` — информация о процессах

## ⚠️ COM Safety Rules (for smith-windows)

**Project-Specific Requirements:**
- **ALWAYS** use `tokio::task::spawn_blocking` for ALL UIA calls
- **NEVER** call `CoInitializeEx`, `CoUninitialize` directly
- **ALL** calls must be in STA (Single-Threaded Apartment) threads
- **AVOID** calling UIA directly from background threads without proper context
- **DO NOT** send `UIElement`, `UIAutomation` between threads (не `Send`/`Sync` по умолчанию)

**Rationale:**
The `uiautomation` crate is built on top of the `windows` crate which uses COM. COM requires proper initialization and thread affinity. The `uiautomation` crate manages this internally, but when used in async Rust with Tokio, all calls must be isolated to prevent COM apartment violations.

## 🎯 Usage Pattern

```rust
use uiautomation::{UIAutomation, UIElement, ControlType};

fn main() -> uiautomation::Result<()> {
    let automation = UIAutomation::new()?;
    let desktop = automation.desktop()?;
    
    let matcher = uiautomation::filters::ControlType(ControlType::Window)
        .and_name("Калькулятор");
    let calculator = desktop.find_child_by(&matcher)?;
    
    if let Some(calc) = calculator {
        let five = calc.find_child_by_name("Пять")?;
        if let Some(btn) = &five {
            btn.invoke()?;
        }
    }
    Ok(())
}
```

## 🔗 Additional Resources

- [docs.rs API](https://docs.rs/uiautomation/0.24.4/)
- [crates.io](https://crates.io/crates/uiautomation)
- [GitHub Repository](https://github.com/leexgone/uiautomation)
