# uiautomation 0.24.4

**Source**: [docs.rs](https://docs.rs/uiautomation/0.24.4/uiautomation/)

## 📚 Overview

Rust wrapper around Windows UI Automation API. Provides safe abstractions for UI element interaction through COM interfaces. The crate is built on top of the `windows` crate and requires Windows 10/11.

**Key characteristics for smith-windows:**
- All types (`UIAutomation`, `UIElement`, `UITreeWalker`) are **`!Send` and `!Sync`**
- Cannot use `spawn_blocking` - all calls must run on the same thread (STA)
- UIA methods are **synchronous** - they don't block the async runtime
- All async operations in smith-windows must use `#[async_trait::async_trait(?Send)]`

## 🔑 Key Types

| Type | Description |
|------|-------------|
| `UIAutomation` | Main entry point for UIA - creates matchers, tree walkers, compares elements |
| `UIElement` | Represents a UI element (window, button, text) - all properties and methods |
| `UITreeWalker` | Navigates the element tree (parent, children, siblings) |
| `UIMatcher` | Configuration for element search (name, control_type, class, automation_id) |
| `ControlType` | Enum of UI control types (Button, Edit, Window, Text, etc.) |
| `SearchScope` | Defines search范围 (Children, Descendants, Element, Subtree, etc.) |
| `Error`, `Result` | Error types from UIA operations |

## 🔧 Key Methods

### UIElement Methods

| Method | Returns | Description |
|--------|---------|-------------|
| `get_control_type()` | `Result<ControlType>` | Get element's control type |
| `get_name()` | `Result<String>` | Get element's name/title |
| `get_classname()` | `Result<String>` | Get element's window class name |
| `get_automation_id()` | `Result<String>` | Get element's automation ID |
| `is_enabled()` | `Result<bool>` | Check if element is enabled |
| `is_offscreen()` | `Result<bool>` | Check if element is offscreen |
| `get_process_id()` | `Result<u32>` | Get process ID owning this element |
| `get_parent()` | `Result<UIElement>` | Get parent element |
| `click()` | `Result<()>` | Click the element |
| `set_focus()` | `Result<()>` | Set focus to the element |
| `get_runtime_id()` | `Result<Vec<i32>>` | Get unique runtime ID |

### UIAutomation Methods

| Method | Returns | Description |
|--------|---------|-------------|
| `new()` | `Result<UIAutomation>` | Create new UIAutomation instance |
| `get_root_element()` | `Result<UIElement>` | Get desktop root element |
| `element_from_point(Point)` | `Result<UIElement>` | Get element at screen coordinates |
| `compare_elements(&UIElement, &UIElement)` | `Result<bool>` | Compare two elements for equality |
| `create_matcher()` | `MatcherBuilder` | Create matcher for element search |
| `create_tree_walker()` | `Result<UITreeWalker>` | Create tree walker for navigation |

### MatcherBuilder Methods (via `create_matcher()`)

| Method | Description |
|--------|-------------|
| `from(UIElement)` | Set search root element |
| `control_type(ControlType)` | Filter by control type |
| `name(&str)` | Filter by element name |
| `class(&str)` | Filter by window class |
| `automation_id(&str)` | Filter by automation ID |
| `enabled(bool)` | Filter by enabled state |
| `offscreen(bool)` | Filter by offscreen state |
| `framework_id(&str)` | Filter by framework ID |
| `timeout(u32)` | Set search timeout in ms |
| `find_first()` | `Result<UIElement>` - Find first matching element |
| `find_all()` | `Result<Vec<UIElement>>` - Find all matching elements |
| `find_first_opt()` | `Result<Option<UIElement>>` - Return None if not found |

### UITreeWalker Methods

| Method | Returns | Description |
|--------|---------|-------------|
| `get_parent(&UIElement)` | `Result<UIElement>` | Get parent element |
| `get_first_child(&UIElement)` | `Result<UIElement>` | Get first child |
| `get_last_child(&UIElement)` | `Result<UIElement>` | Get last child |
| `get_next_sibling(&UIElement)` | `Result<UIElement>` | Get next sibling |
| `get_previous_sibling(&UIElement)` | `Result<UIElement>` | Get previous sibling |

## ⚠️ COM Safety Rules (for smith-windows)

**Project-Specific Requirements:**
- **ALWAYS** use `#[async_trait::async_trait(?Send)]` for all traits using UIA types
- **NEVER** use `tokio::task::spawn_blocking` - UIElement is `!Send` and will fail to compile
- **ALL** UIA calls must run on the same thread that created UIAutomation (STA affinity)
- **DO NOT** pass UIElement across async boundaries with `async move` - use references instead
- **ALL** error handling must be explicit via `match` or `?` operator

**Rationale:**
The `uiautomation` crate wraps Windows COM interfaces. COM requires proper apartment threading - UIAutomation creates elements with STA affinity, and those elements cannot be moved between threads. Attempting to use `spawn_blocking` or `Send` futures with UIA types will result in compilation errors.

## 🎯 Usage Pattern for WaitTool

### Finding Elements - Option vs Result

The `uiautomation` crate has **TWO patterns** for element search:

#### Pattern 1: MatcherBuilder (Recommended for smith-windows)
```rust
use uiautomation::UIAutomation;

let automation = UIAutomation::new()?;

// Returns Result<UIElement> - Err if not found
let element = automation
    .create_matcher()
    .from(root)
    .control_type(ControlType::Button)
    .name("OK")
    .timeout(2000)  // 2 second timeout
    .find_first()?; // Returns Result<UIElement, Error>

// Alternative: Returns Result<Option<UIElement>>
let maybe_element = automation
    .create_matcher()
    .from(root)
    .automation_id("cancel_button")
    .find_first_opt()?; // Returns Result<Option<UIElement>, Error>

match maybe_element {
    Some(el) => println!("Found: {}", el.get_name()?), // Handle found element
    None => println!("Element not found"),              // Handle not found
}
```

#### Pattern 2: Direct UIElement Search (Legacy)
```rust
use uiautomation::{UIAutomation, SearchScope, UIMatcher, ControlType};

let automation = UIAutomation::new()?;
let root = automation.get_root_element()?;

// Create matcher with UIMatcher
let matcher = UIMatcher::new()
    .name("Button")?
    .control_type(ControlType::Button)?;

// Returns Result<UIElement> - Err if not found
let element = root.find_first(SearchScope::Descendants, &matcher)?;
```

### Check if Element Exists (Pattern for WaitTool)

**Method 1: Using `find_first_opt()` (RECOMMENDED)**
```rust
use uiautomation::UIAutomation;

async fn element_exists(
    automation: &UIAutomation,
    root: &uiautomation::UIElement,
    automation_id: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    // Returns Result<Option<UIElement>>
    let maybe_element = automation
        .create_matcher()
        .from(root.clone())
        .automation_id(automation_id)
        .timeout(1000)  // 1 second timeout for each check
        .find_first_opt()?;  // Returns Option<UIElement>
    
    Ok(maybe_element.is_some())
}
```

**Method 2: Try/Catch with find_first()**
```rust
use uiautomation::UIAutomation;

async fn element_exists_catch(
    automation: &UIAutomation,
    root: &uiautomation::UIElement,
    automation_id: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    let result = automation
        .create_matcher()
        .from(root.clone())
        .automation_id(automation_id)
        .timeout(1000)
        .find_first();
    
    match result {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),  // Treat any error as "not found"
    }
}
```

**Method 3: Check via Element Properties (for known elements)**
```rust
// If you already have a UIElement reference, check validity:
let is_enabled = element.is_enabled().unwrap_or(false);
let is_offscreen = element.is_offscreen().unwrap_or(true);

// If these fail, the element may be invalid/disposed
```

### Complete WaitTool Implementation Pattern

```rust
use std::time::{Duration, Instant};
use uiautomation::UIAutomation;

#[async_trait::async_trait(?Send)]
pub trait WaitBackend {
    async fn wait_for_element(
        &self,
        automation: &UIAutomation,
        root: &uiautomation::UIElement,
        automation_id: &str,
        timeout: Duration,
        interval: Duration,
    ) -> Result<bool, WaitError>;
}

#[async_trait::async_trait(?Send)]
impl WaitBackend for WaitBackendWindows {
    async fn wait_for_element(
        &self,
        automation: &UIAutomation,
        root: &uiautomation::UIElement,
        automation_id: &str,
        timeout: Duration,
        interval: Duration,
    ) -> Result<bool, WaitError> {
        let start = Instant::now();
        
        loop {
            // Check timeout
            if start.elapsed() >= timeout {
                return Ok(false);  // Timeout reached
            }
            
            // Check cancellation
            if self.config.cancellation.is_cancelled() {
                return Err(WaitError::Cancelled);
            }
            
            // Try to find element
            match automation
                .create_matcher()
                .from(root.clone())
                .automation_id(automation_id)
                .timeout(1000)  // 1 second per check
                .find_first_opt()
            {
                Ok(Some(_element)) => return Ok(true),  // Found!
                Ok(None) => {}  // Not found yet, continue polling
                Err(_) => {}    // Error treating as "not found"
            }
            
            // Wait before next poll
            tokio::time::sleep(interval).await;
        }
    }
}
```

### Error Handling Patterns

| Method | Returns | Error When | How to Handle |
|--------|---------|------------|----------------|
| `find_first()` | `Result<UIElement>` | Not found, timeout, COM error | Match on `Err` to handle |
| `find_first_opt()` | `Result<Option<UIElement>>` | COM error only | Match on `Option` to check existence |
| `find_all()` | `Result<Vec<UIElement>>` | Not found, timeout, COM error | Check `Vec::is_empty()` |
| `get_name()` | `Result<String>` | Element invalid, COM error | `unwrap_or_default()` or match |
| `is_enabled()` | `Result<bool>` | Element invalid, COM error | `unwrap_or(false)` for safety |
| `is_offscreen()` | `Result<bool>` | Element invalid, COM error | `unwrap_or(true)` for safety |

## 🔗 Additional Resources

- [docs.rs API](https://docs.rs/uiautomation/0.24.4/uiautomation/)
- [Cargo.toml on crates.io](https://crates.io/crates/uiautomation)
- **Source**: [leexgone/uiautomation on GitHub](https://github.com/leexgone/uiautomation)
