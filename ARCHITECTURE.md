# 🏗️ Architecture: smith-windows

## 🎯 Purpose
API for calling Windows automation tools via the `uiautomation` crate.
MVP for integration into smith-core. Windows-only, uiautomation-only.

## 🛠️ Tech Stack
- Language: Rust 1.95.0 (2024 edition)
- Async: `tokio`
- Errors: `thiserror` (lib), `anyhow` (apps)
- Serialization: `serde` + `serde_json`/`bincode`
- Testing: `#[test]`, `cargo clippy -- -D warnings`
- Logging: `tracing`
- **UIAutomation:** `uiautomation` crate 0.24.4

## 📦 Core Modules
- `src/core/` — Shared types, traits, error definitions, mocks, tests
- `src/runtime/` — Windows implementations via `uiautomation`

### Tool Modules
- **ClickTool**: Клик по UI-элементам (левый, правый, двойной клик)
- **TypeTool**: Ввод текста в UI-элементы через clipboard
- **SetTextTool**: Программная установка текста через UI Automation API
- **ReadTool**: Получение текстового содержимого из UI-элементов
- **ScreenshotTool**: Захват скриншотов через GDI/GDI+
- **InspectTool**: Интерактивная инспекция элементов (Ctrl+Hover)
- **ToggleTool**: Управление состоянием переключателей
- **ScrollTool**: Прокрутка UI-элементов (programmatic + synthetic)
- **ClipboardTool**: Работа с системным буфером обмена (get/set/check)
- **InputTool**: Детекция hover и hotkey
- **InputTextTool**: Эмуляция ввода с клавиатуры
- **FocusTool**: Активация окна перед взаимодействием
- **WaitTool**: Ожидание появления/исчезновения элементов
- **RightClickTool**: Правый клик через UI Automation
- **WindowControlTool**: Управление состоянием окон
- **AutomationSession**: Управление сессиями UI Automation
- **SelectorStorage**: Сохранение/загрузка селекторов (JSON)
- **SelectorRecorder**: Запись селекторов из UI

## 🔄 Communication & Flow
- Task lifecycle: `Created → Queued → Running → [Completed | Failed | Cancelled]`
- SessionBackend → SessionHandle → ClickBackend/TypeBackend/ReadBackend/InspectBackend/SetTextBackend/ScreenshotBackend/ScrollBackend/ToggleBackend/InputBackend/InputTextBackend/FocusBackend/WindowControlBackend/WaitBackend/RightClickBackend
- **COM Isolation**: All UIA operations must run on the same thread that created the `UIAutomation` instance
- **`?Send` Contract**: All async traits MUST use `#[async_trait::async_trait(?Send)]` to compile with `uiautomation` types
- **GDI/GDI+ Isolation**: Screenshot operations use `tokio::task::spawn_blocking` (no STA affinity required)

## 📐 Design Principles
1. **Contracts First:** Behavior defined before implementation
2. **No Silent Failures:** All errors explicit (`Result`), never `unwrap()`/`panic!`
3. **Idempotency:** Repeated calls with same input → same state/output
4. **Modularity:** Clear boundaries, single responsibility
5. **Test-Driven Verification:** Tests define correctness
6. **UIAutomation-First:** All UI operations via `uiautomation` crate

## 📁 Project Structure
```
smith-windows/
├── docs/
│   ├── design/<module>/     # Working docs: specification.md, contract.md, test-plan.md, brief.md
│   │   └── inspect-tool/    # InspectTool: inspect mode with Ctrl+hover capture
│   │   └── click-tool/      # ClickTool: click on UI elements
│   │   └── type-tool/       # TypeTool: type text on UI elements
│   │   └── set-text-tool/   # SetTextTool: set text programmatically (NOT keyboard typing)
│   │   └── read-tool/       # ReadTool: read text from UI elements
│   │   └── screenshot-tool/ # ScreenshotTool: capture screenshots via GDI/GDI+
│   │   └── automation-session/  # AutomationSession: session management
│   │   └── toggle-tool/     # ToggleTool: control toggle state (checkboxes, radio buttons, toggle switches)
│   │   └── right-click-tool/ # RightClickTool: right click via UI Automation
│   │   └── scroll-tool/     # ScrollTool: scroll UI elements via UI Automation
│   │   └── clipboard-tool/  # ClipboardTool: system clipboard operations (get/set/check text)
│   ├── templates/           # Document templates: specification.md, contract.md, test-plan.md, brief.md
│   └── adr/                 # Architecture Decision Records
├── src/
│   ├── core/                # Traits, types, errors, mocks, tests
│   │   ├── click.rs         # ClickTool: config, error, backend trait
│   │   ├── type.rs          # TypeTool: config, error, backend trait
│   │   ├── set_text.rs      # SetTextTool: config, error, backend trait
│   │   ├── read.rs          # ReadTool: config, error, backend trait
│   │   ├── screenshot.rs    # ScreenshotTool: config, error, backend trait, mode enum
│   │   ├── inspect.rs       # InspectTool: config, error, backend trait
│   │   ├── toggle.rs        # ToggleTool: config, error, backend trait
│   │   ├── scroll.rs        # ScrollTool: config, error, backend trait
│   │   ├── clipboard.rs     # ClipboardTool: config, error, backend trait
│   │   ├── input.rs         # InputTool: hover and hotkey detection
│   │   ├── input_text.rs    # InputTextTool: keyboard emulation input
│   │   ├── focus.rs         # FocusTool: activate window before interaction
│   │   ├── automation_session.rs  # AutomationSession: session management
│   │   ├── right_click.rs   # RightClickTool: right click via UI Automation
│   │   ├── window_control.rs  # WindowControlTool: window state management
│   │   ├── wait.rs          # WaitTool: wait for element appearance/disappearance
│   │   ├── selector.rs      # Selector: element selection utilities
│   │   ├── selector_storage.rs  # SelectorStorage: save/load selectors to disk
│   │   └── selector_recorder.rs # SelectorRecorder: record selectors from UI
│   └── runtime/             # Windows implementations + unsupported stub
│       └── backends/
│           └── windows/
│               ├── click.rs     # ClickBackendWindows implementation
│               ├── type.rs      # TypeBackendWindows implementation
│               ├── set_text.rs  # SetTextBackendWindows implementation
│               ├── read.rs      # ReadBackendWindows implementation
│               ├── screenshot.rs # ScreenshotBackendWindows (GDI/GDI+ via spawn_blocking)
│               ├── inspect.rs   # InspectBackendWindows implementation
│               ├── toggle.rs    # ToggleBackendWindows implementation
│               ├── scroll.rs    # ScrollBackendWindows implementation
│               └── clipboard.rs # ClipboardBackendWindows (clipboard crate)
├── tests/                   # Integration tests
├── .qwen/agents/            # smith-architect, smith-planner, smith-coder, smith-crate-researcher, smith-debugger, smith-compliance
├── tools/                   # Development helper tools
│   └── bundle_context.rs    # Project context collector (for AI agents)
├── Cargo.toml
├── AGENTS.md
└── ARCHITECTURE.md
```

## 🔄 Workflow
1. Architect writes spec + contract + test plan → `docs/design/<module>/`
2. Planner creates `/plan` → architect approves
3. Coder generates code + tests → architect verifies via `cargo test` + `clippy`
4. Decision logged in `docs/adr/`

## 🚫 COM Safety & Thread Affinity

**Critical Rules for UIAutomation:**
- `UIElement`, `UIAutomation`, `UITreeWalker` are `!Send` and `!Sync` — they cannot be moved between threads
- All async futures containing UIA calls must run on the same thread that created the `UIAutomation` instance
- `tokio::task::spawn_blocking` is NOT applicable for passing UIA types — it will cause compilation errors
- All UIA-related traits MUST use `#[async_trait::async_trait(?Send)]` to compile with `uiautomation` types

**Why?**
The `uiautomation` crate is built on top of the `windows` crate which uses COM (Component Object Model). COM requires proper initialization and thread affinity. The `uiautomation::UIAutomation::new()` method automatically initializes COM with `COINIT_MULTITHREADED`, but the returned types (`UIElement`, `UIAutomation`, `UITreeWalker`) are not thread-safe and must be used on a single thread.

**Correct Pattern for UIA:**
```rust
// ✅ CORRECT: Direct call to backend - no spawn_blocking
#[async_trait::async_trait(?Send)]
pub trait ClickBackend {
    async fn click(&self, element: &uiautomation::UIElement) -> Result<(), ClickError>;
}

// ❌ WRONG: spawn_blocking with UIElement will NOT COMPILE
// let result = tokio::task::spawn_blocking(move || {
//     backend.click(element).await  // COMPILATION ERROR: UIElement is !Send
// }).await;
```

## 🖼️ GDI/GDI+ Safety for ScreenshotTool

**Isolation Strategy:**
- ScreenshotTool uses Windows GDI/GDI+ API (`GetDesktopWindow`, `GetWindowDC`, `CreateCompatibleDC`, `BitBlt`, `StretchBlt`, `CreateDIBSection`)
- GDI/GDI+ calls are isolated via `tokio::task::spawn_blocking` (no STA affinity required)
- No COM initialization needed for GDI/GDI+ in this usage pattern

**Pattern:**
```rust
// ✅ CORRECT: spawn_blocking for GDI/GDI+ calls
#[async_trait::async_trait(?Send)]
pub trait ScreenshotBackend {
    async fn capture(&self, mode: &ScreenshotMode) -> Result<Vec<u8>, ScreenshotError>;
}

// Implementation uses spawn_blocking for GDI calls:
let result = tokio::task::spawn_blocking(move || {
    // GDI/GDI+ calls here
    // No UIA types involved, safe to block
    capture_screenshot(mode)
}).await;
```

## 📝 ReadTool Architecture

**Purpose:** Reading text content from UI elements (text fields, labels, headers, etc.)

**Pattern:**
```rust
// ✅ CORRECT: Direct call to backend - no spawn_blocking (UIA types !Send)
#[async_trait::async_trait(?Send)]
pub trait ReadBackend {
    async fn read_text(&self, element: &uiautomation::UIElement) -> Result<String, ReadError>;
}

// Implementation uses direct UIA calls:
let text = element.get_name()?;  // Fallback for labels
let text = text_pattern.get_document_text()?;  // Primary for text fields
```

**Text Sources:**
1. **Primary:** `UITextPattern.get_document_text()` — for text input fields with TextPattern support
2. **Fallback:** `UIElement.get_name()` — for label controls without TextPattern

**Supported Elements:**
- Edit controls (text fields)
- Text controls (labels)
- Header controls
- Any element with TextPattern or Name property

**Constraints:**
- Element must be enabled (`is_enabled() == true`)
- Element must not be offscreen (`is_offscreen() == false`)
- Element must support text (TextPattern or Name)

**Output:**
- `String` with element text content
- Empty string `""` for elements without text content

**Error Handling:**
- `ElementNotFound` — invalid or deleted UIElement
- `ElementNotEnabled` — element is disabled
- `ElementOffscreen` — element is not visible on screen
- `ElementNotWritable` — element does not support text reading
- `Timeout` — operation not completed within timeout
- `Cancelled` — operation cancelled via CancellationToken
- `InvalidConfig(String)` — timeout <= 0 or timeout > 1 hour
- `ComError(String)` — UI Automation error with message

## 📝 ToggleTool Architecture

**Purpose:** Controlling toggle state of checkboxes, radio buttons, and toggle switches

**Pattern:**
```rust
// ✅ CORRECT: Direct call to backend - no spawn_blocking (UIA types !Send)
#[async_trait::async_trait(?Send)]
pub trait ToggleBackend {
    async fn toggle_element(&self, element: &uiautomation::UIElement) -> Result<(), ToggleError>;
    async fn set_radio(&self, element: &uiautomation::UIElement, selected: bool) -> Result<(), ToggleError>;
    async fn set_toggle(&self, element: &uiautomation::UIElement, state: bool) -> Result<(), ToggleError>;
    async fn is_checked(&self, element: &uiautomation::UIElement) -> Result<bool, ToggleError>;
    async fn is_selected(&self, element: &uiautomation::UIElement) -> Result<bool, ToggleError>;
}
```

**Toggle Patterns:**
1. **Primary:** `UITogglePattern.toggle()` — for elements with TogglePattern support (checkboxes, toggle switches)
2. **Fallback:** `UIValuePattern.set_value()` — for ValuePattern with is_readonly=false (radio buttons)
3. **Legacy:** `LegacyIAccessible` — for older controls without modern patterns

**Supported Elements:**
- CheckBox controls (with TogglePattern)
- RadioButton controls (with SelectionPattern or LegacyIAccessible)
- ToggleSwitch controls (with TogglePattern)
- Any element with TogglePattern, ValuePattern (read-write), or LegacyIAccessible

**Constraints:**
- Element must be enabled (`is_enabled() == true`)
- Element must not be offscreen (`is_offscreen() == false`)
- Element must support toggle (TogglePattern, ValuePattern with is_readonly=false, or LegacyIAccessible)

**Output:**
- `()` (unit) on success - state changed to toggled/on/off
- `Err(ToggleError)` on failure - state unchanged

**Error Handling:**
- `ElementNotFound` — invalid or deleted UIElement
- `ElementNotEnabled` — element is disabled
- `ElementOffscreen` — element is not visible on screen
- `ElementNotSupported` — element does not support toggle patterns
- `ElementNotWritable` — ValuePattern exists but is_readonly=true
- `Timeout` — operation not completed within timeout
- `Cancelled` — operation cancelled via CancellationToken
- `InvalidConfig(String)` — timeout <= 0 or timeout > 1 hour
- `ComError(String)` — UI Automation error with message

## 📝 ClipboardTool Architecture

**Purpose:** System clipboard operations (get text, set text, check presence) via the `clipboard` crate

**Key Characteristics:**
- Uses `clipboard` crate v0.5 (handles COM internally)
- Synchronous API — no `spawn_blocking` required
- Windows-only (non-Windows returns `OperationNotSupported`)
- Idempotent: errors do not change system state

**Pattern:**
```rust
// ✅ CORRECT: Direct call to backend - no spawn_blocking (clipboard crate is sync)
#[async_trait::async_trait(?Send)]
pub trait ClipboardBackend {
    async fn get_text(&self) -> Result<String, ClipboardError>;
    async fn set_text(&self, text: &str) -> Result<(), ClipboardError>;
    async fn has_text(&self) -> Result<bool, ClipboardError>;
}

// Implementation uses clipboard crate directly (COM handled internally):
let text = clipboard::get_text()?;  // Returns Option<String>
clipboard::set_text("Hello")?;       // Returns Result<(), ClipboardError>
let has_text = clipboard::has_text()?;  // Returns Result<bool, ClipboardError>
```

**Supported Operations:**
1. **get_text()**: Retrieves text from clipboard (returns `String` or `ClipboardEmpty` error)
2. **set_text(text)**: Sets text to clipboard (returns `()` on success or `ClipboardAccessDenied` error)
3. **has_text()**: Checks if clipboard contains text (returns `bool`)

**Constraints:**
- Timeout: must be > 0 and <= 1 hour (validated in `validate_clipboard_config`)
- Text for set_text: must be non-empty (validated before backend call)
- Platform: Windows only (non-Windows returns `OperationNotSupported`)

**Output:**
- `get_text()`: `String` with clipboard content
- `set_text()`: `()` (unit) on success
- `has_text()`: `bool` indicating presence of text

**Error Handling:**
- `OperationNotSupported` — non-Windows platform
- `ClipboardEmpty` — clipboard contains no text (for get_text)
- `ClipboardAccessDenied` — clipboard is locked by another process
- `TextEmpty` — empty text provided for set_text
- `Timeout` — operation not completed within timeout
- `Cancelled` — operation cancelled via CancellationToken
- `InvalidConfig(String)` — timeout <= 0 or timeout > 1 hour
- `ComError(String)` — COM error with message

## 📚 Additional Resources

- **Agents**: `AGENTS.md` — rules for AI agents
- **Agents**: `.qwen/agents/` — AI agents configuration
- **Context Bundle**: `tools/bundle_context.rs` — tool for collecting canonical documentation
- **Templates**: `docs/templates/` — document templates

**Note**: Before developing a new module, run context bundle:
```bash
cargo run --bin bundle_context
```
