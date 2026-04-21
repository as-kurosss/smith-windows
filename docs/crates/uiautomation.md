# uiautomation 0.24.4

**Source**: [docs.rs](https://docs.rs/uiautomation/0.24.4/uiautomation/)

## 📚 Overview
Rust wrapper around Windows UI Automation API. Provides safe abstractions for UI element interaction. Built on top of `windows` crate, wraps COM interfaces `IUIAutomation`, `IUIAutomationElement`, `IUIAutomationTreeWalker`.

**Key features:**
- High-level abstractions for UI element manipulation
- Pattern-based interaction (Invoke, Value, Text, etc.)
- Support for caching to optimize performance
- Fluent matcher API for element search
- Full control over UI Automation tree navigation

## 🔑 Key Types

| Type | Description |
|------|-------------|
| `UIAutomation` | Main entry point for UI Automation. Creates automation instance and provides methods to get root element, elements by point/handle/focus. |
| `UIElement` | Represents a single UI element (window, button, edit, etc.). Core building block for all UI operations. |
| `UITreeWalker` | Provides navigation methods (parent, children, siblings) through UI Automation tree. |
| `UIMatcher` | Fluent API for building search conditions and finding elements. Supports timeout, depth limits, custom filters. |
| `UICondition` | Condition for filtering elements (AND, OR, NOT, property-based conditions). |
| `UIAutomation::new()` | Creates automation instance with automatic COM initialization (COINIT_MULTITHREADED). |
| `UIAutomation::new_direct()` | Creates instance without COM initialization (requires manual `CoInitializeEx`). |

## 🔧 Key Methods

### UIAutomation Methods

- `UIAutomation::new() -> Result<UIAutomation>` — create automation instance with COM init
- `UIAutomation::get_root_element(&self) -> Result<UIElement>` — get desktop/root element
- `UIAutomation::element_from_point(&self, point: POINT) -> Result<UIElement>` — get element at screen coordinates
- `UIAutomation::get_focused_element(&self) -> Result<UIElement>` — get element with keyboard focus
- `UIAutomation::element_from_handle(&self, hwnd: HANDLE) -> Result<UIElement>` — get element by window handle
- `UIAutomation::create_matcher(&self) -> UIMatcher` — create fluent matcher for element search

### UIElement Methods

**Navigation:**
- `UIElement::find_first(scope: TreeScope, condition: &UICondition) -> Result<UIElement>` — find first matching element
- `UIElement::find_all(scope: TreeScope, condition: &UICondition) -> Result<Vec<UIElement>>` — find all matching elements
- `UITreeWalker::get_parent(element: &UIElement) -> Result<UIElement>` — get parent element
- `UITreeWalker::get_first_child(element: &UIElement) -> Result<UIElement>` — get first child
- `UITreeWalker::get_next_sibling(element: &UIElement) -> Result<UIElement>` — get next sibling

**Properties:**
- `UIElement::get_name() -> Result<String>` — element name
- `UIElement::get_automation_id() -> Result<String>` — unique automation ID
- `UIElement::get_classname() -> Result<String>` — window class name
- `UIElement::get_control_type() -> Result<ControlType>` — element type
- `UIElement::is_enabled() -> Result<bool>` — is element enabled
- `UIElement::is_offscreen() -> Result<bool>` — is element hidden
- `UIElement::has_keyboard_focus() -> Result<bool>` — has keyboard focus
- `UIElement::get_bounding_rectangle() -> Result<Rect>` — element bounding rectangle

**Interaction:**
- `UIElement::click() -> Result<()>` — left click
- `UIElement::double_click() -> Result<()>` — double click
- `UIElement::right_click() -> Result<()>` — right click
- `UIElement::set_focus() -> Result<()>` — set keyboard focus
- `UIElement::send_keys(keys: &str, interval: u64) -> Result<()>` — send keystrokes
- `UIElement::send_text(text: &str, interval: u64) -> Result<()>` — send text

**Patterns:**
- `UIElement::get_pattern<T: UiPattern>(&self) -> Result<T>` — get control pattern (InvokePattern, ValuePattern, TextPattern, etc.)

### Control Patterns

| Pattern | Use Case |
|---------|----------|
| `InvokePattern` | Button clicks, menu items |
| `ValuePattern` | Text input fields |
| `TextPattern` | Rich text editing |
| `RangeValuePattern` | Sliders, spinners |
| `TogglePattern` | Checkboxes, toggle buttons |
| `WindowPattern` | Window operations (minimize, maximize, close) |
| `SelectionPattern` | List boxes, combo boxes |

## ⚠️ COM Safety Rules (for smith-windows)

**Project-Specific Requirements:**
- **DO NOT** use `tokio::task::spawn_blocking` for COM calls — `uiautomation` methods are synchronous and don't block async runtime
- **DO NOT** call `CoInitializeEx`, `CoUninitialize` directly — `UIAutomation::new()` handles COM initialization automatically
- **ALL** calls must be in STA (Single-Threaded Apartment) threads
- **NEVER** pass `UIElement` between threads (it is `!Send` and `!Sync`)

**Rationale:**
The `uiautomation` crate is built on top of the `windows` crate which uses COM. COM requires proper initialization and thread affinity. The `uiautomation` crate manages COM initialization internally when using `UIAutomation::new()`, which wraps `CoInitializeEx(nullptr, COINIT_MULTITHREADED)`. 

For `smith-windows`, the recommended pattern is:
- Use `UIAutomation::new()` which initializes COM automatically
- Call `uiautomation` methods directly (they are synchronous and don't block the async runtime)
- Avoid `spawn_blocking` unless doing CPU-intensive work with the `UIElement` returned (which is impossible due to `!Send`)

**Critical:**
- `UIElement` is **NOT Send** (`!Send`) — it cannot be sent between threads
- `UIElement` is **NOT Sync** (`!Sync`) — it cannot be shared between threads
- `UITreeWalker` is also `!Send`/`!Sync`
- `UIAutomation` is also `!Send`/`!Sync`
- All these types must be used exclusively within the same thread that initialized COM

## 🎯 Usage Pattern

### Getting Root Element
```rust
use uiautomation::core::UIAutomation;

let automation = UIAutomation::new()?;
let root = automation.get_root_element()?;
```

### Getting Element at Coordinates
```rust
use uiautomation::core::UIAutomation;
use windows::Win32::System::Threading::GetCursorPos; // or your own cursor pos function

let automation = UIAutomation::new()?;

// Get cursor position (via WinAPI in spawn_blocking)
let point = get_cursor_pos()?;  // POINT { x, y }

// Get element at that position
let element = automation.element_from_point(point)?;
```

### Getting Element Under Cursor
> **Note**: There is NO direct `element_from_cursor()` method. Use `element_from_point` with cursor coordinates.

```rust
// Step 1: Get cursor position (via WinAPI)
let pos = get_cursor_pos()?;  // POINT

// Step 2: Get element at that point
let automation = UIAutomation::new()?;
let element = automation.element_from_point(pos)?;
```

### Using UIMatcher (Recommended for smith-windows)
```rust
use uiautomation::core::{UIAutomation, ControlType};

let automation = UIAutomation::new()?;

// Find first button named "OK"
let ok_button = automation
    .create_matcher()
    .name("OK")
    .control_type(ControlType::Button)
    .timeout(3000)  // 3 seconds
    .find_first()?;

ok_button.click()?;

// Find all text fields in Notepad
let notepad = automation
    .create_matcher()
    .classname("Notepad")
    .find_first()?;

let text_fields = automation
    .create_matcher()
    .from(notepad)
    .control_type(ControlType::Edit)
    .find_all()?;
```

### Using Control Patterns
```rust
use uiautomation::patterns::{ValuePattern, InvokePattern};
use uiautomation::core::UIAutomation;

let automation = UIAutomation::new()?;
let root = automation.get_root_element()?;

// Find edit control and set value
let edit = root.find_first(
    TreeScope::Subtree,
    &UICondition::new().control_type(ControlType::Edit)?
)?;

let value: ValuePattern = edit.get_pattern()?;
value.set_value("Hello, world!")?;

// Find button and invoke
let button = root.find_first(
    TreeScope::Subtree,
    &UICondition::new().control_type(ControlType::Button)?
)?;

let invoke: InvokePattern = button.get_pattern()?;
invoke.invoke()?;
```

## 🔗 Additional Resources

- [docs.rs API](https://docs.rs/uiautomation/0.24.4/uiautomation/)
- [GitHub Repository](https://github.com/leexgone/uiautomation)
- [Cargo.toml on crates.io](https://crates.io/crates/uiautomation)
