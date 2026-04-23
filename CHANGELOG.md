---

# 📝 CHANGELOG

Все важные изменения в проекте `smith-windows` будут документироваться в этом файле.

Формат основан на [Keep a Changelog](https://keepachangelog.com/ru/1.0.0/),
и проект придерживается семантического версионирования (SemVer).

## [Unreleased]

### Added

- **WaitTool Module**
  - Added `WaitTool` for waiting for element appearance/disappearance with timeout and polling
  - Added `WaitConfig` with timeout, interval, and cancellation support
  - Added `WaitError` with variants: `InvalidConfig`, `Timeout`, `Cancelled`, `ComError`
  - Added `WaitMode` enum: `Existence`, `Absence`
  - Added `WaitSelector` enum: `AutomationId`, `Name`, `ControlType`
  - Added `validate_wait_config()` for config validation
  - Added `WaitBackend` trait for backend abstraction with method: `wait_element()`
  - Added Windows backend using UI Automation polling loop
  - Added `MockWaitBackend` for unit testing with `Arc<Mutex<MockWaitState>>`
  - Added full support for wait operations with cancellation and timeout handling
  - Added unit tests covering validation, mock backend, polling (10/10 tests passing)

- **InputTextTool Module**
  - Added `InputTextTool` for keyboard emulation text input
  - Added `InputTextConfig` with text, timeout, and cancellation support
  - Added `InputTextError` with variants: `InputSelectorError`, `ElementNotFound`, `ElementNotEnabled`, `ElementOffscreen`, `ElementReadOnly`, `Timeout`, `Cancelled`, `InvalidConfig`, `ComError`, `InputExecutionError`
  - Added `validate_input_selector()` and `validate_input_text_config()` for validation
  - Added `InputTextBackend` trait for backend abstraction with method: `input_text()`
  - Added Windows backend using `Keyboard::send_keys()` via UI Automation
  - Added `MockInputTextBackend` for unit testing
  - Added full support for keyboard emulation with element readiness validation
  - Added unit tests covering validation, mock backend (11/11 tests passing)

- **ReadTool Module**
  - Added `ReadTool` for reading text content from UI elements
  - Added `ReadConfig` with timeout and cancellation support
  - Added `ReadError` with variants: `ElementNotFound`, `ElementNotEnabled`, `ElementOffscreen`, `ElementNotWritable`, `Timeout`, `Cancelled`, `InvalidConfig`, `ComError`, `UnsupportedPlatform`
  - Added `validate_read_config()` for config validation
  - Added `ReadBackend` trait for backend abstraction with method: `read_text()`
  - Added Windows backend using `UITextPattern.get_document_text()` or fallback
  - Added `MockReadBackend` for unit testing
  - Added full support for text reading with timeout and cancellation
  - Added unit tests covering validation, mock backend (5/5 tests passing)

- **InspectTool Module**
  - Added `InspectTool` for interactive element inspection with Ctrl+Hover
  - Added `InspectConfig` with timeout and cancellation support
  - Added `InspectError` with variants: `ElementNotFound`, `ElementNotEnabled`, `ElementOffscreen`, `Timeout`, `Cancelled`, `InvalidConfig`, `ComError`, `InvalidSelector`
  - Added `validate_inspect_config()` for config validation
  - Added `InspectBackend` trait for backend abstraction with method: `inspect_path()`
  - Added Windows backend using `UITreeWalker` for full hierarchy traversal
  - Added `MockInspectBackend` for unit testing
  - Added `validate_inspect_path()` for basic validation
  - Added full support for element path inspection with timeout and cancellation
  - Added unit tests covering validation, mock backend (3/3 tests passing)

- **ScreenshotTool Module**
  - Added `ScreenshotTool` for capturing screenshots via GDI/GDI+
  - Added `ScreenshotConfig` with timeout and cancellation support
  - Added `ScreenshotError` with variants: `InvalidRegion`, `InvalidConfig`, `ElementNotFound`, `Timeout`, `Cancelled`, `CaptureFailed`, `UnsupportedPlatform`
  - Added `ScreenshotMode` enum: `Screen`, `Window`, `Region`
  - Added `validate_screenshot_config()` and `validate_screenshot_mode()` for validation
  - Added `ScreenshotBackend` trait for backend abstraction with method: `capture()`
  - Added Windows backend using GDI/GDI+ for screen/window/region capture
  - Added `MockScreenshotBackend` for unit testing with minimal valid PNG
  - Added full support for screenshot capture with timeout and cancellation
  - Added unit tests covering validation, mock backend (8/8 tests passing)

- **ScrollTool Module**
  - Added `ScrollTool` for scrolling UI elements via UI Automation
  - Added `ScrollConfig` with timeout and cancellation support
  - Added `ScrollError` with variants: `InvalidConfig`, `Timeout`, `Cancelled`, `ElementNotFound`, `ComError`
  - Added `ScrollDirection` enum: `Vertical`, `Horizontal`
  - Added `ScrollUnit` enum: `Line`, `Page`, `Pixel`
  - Added `validate_scroll_config()` for config validation
  - Added `ScrollBackend` trait for backend abstraction with methods: `scroll_vertical()`, `scroll_horizontal()`, `simulate_mouse_wheel()`
  - Added Windows backend using `UIVerticalScrollPattern` and fallback synthetic scrolling
  - Added `MockScrollBackend` for unit testing
  - Added full support for scroll operations with timeout and cancellation
  - Added unit tests covering validation, mock backend (3/3 tests passing)

- **ToggleTool Module**
  - Added `ToggleTool` for managing toggle states (checkboxes, radio buttons, toggle switches)
  - Added `ToggleConfig` with timeout and cancellation support
  - Added `ToggleError` with variants: `ElementNotFound`, `ElementNotEnabled`, `ElementOffscreen`, `Timeout`, `Cancelled`, `InvalidConfig`, `ComError`, `UnsupportedPlatform`
  - Added `validate_toggle_config()` for config validation
  - Added `ToggleBackend` trait for backend abstraction with methods: `toggle_element()`, `set_radio()`, `set_toggle()`, `is_checked()`, `is_selected()`
  - Added Windows backend using `UITogglePattern.toggle()` and `UISelectionItemPattern.select()`
  - Added `MockToggleBackend` for unit testing
  - Added full support for toggle operations with timeout and cancellation
  - Added unit tests covering validation, mock backend (5/5 tests passing)

- **FocusTool Module**
  - Added `FocusTool` for activating window before element interaction
  - Added `FocusConfig` with timeout and cancellation support
  - Added `FocusError` with variants: `ElementNotFound`, `ElementNotEnabled`, `ElementOffscreen`, `WindowPatternNotAvailable`, `Timeout`, `Cancelled`, `InvalidConfig`, `ComError`, `UnsupportedPlatform`
  - Added `validate_config()` for config validation
  - Added `FocusBackend` trait for backend abstraction with method: `focus()`
  - Added Windows backend using `UIElement.set_focus()` and `UIWindowPattern.set_window_visual_state()`
  - Added `MockFocusBackend` for unit testing
  - Added full support for window activation with timeout and cancellation
  - Added unit tests covering validation, mock backend (5/5 tests passing)

- **InputTool Module**
  - Added `InputTool` for hover and hotkey detection
  - Added `InputConfig` with timeout and cancellation support
  - Added `InputError` with variants: `MouseMoveError`, `KeyClickError`, `ElementFromPointError`, `Timeout`, `Cancelled`, `InvalidConfig`, `ComError`
  - Added `validate_input_config()` for config validation
  - Added `InputBackend` trait for backend abstraction with methods: `get_element_at_point()`, `move_mouse()`, `click_key()`
  - Added Windows backend using `GetCursorPos()` and `Input::mouse()`
  - Added `MockInputBackend` for unit testing
  - Added full support for input operations with timeout and cancellation
  - Added unit tests covering validation, mock backend (3/3 tests passing)

- **ClipboardTool Module** (ADR 015)
  - Added `ClipboardTool` for Windows system clipboard operations (get text, set text, check presence)
  - Added `ClipboardConfig` with timeout and cancellation support
  - Added `ClipboardError` with variants: `OperationNotSupported`, `ClipboardEmpty`, `ClipboardAccessDenied`, `TextEmpty`, `Timeout`, `Cancelled`, `InvalidConfig`, `ComError`
  - Added `ClipboardAction` enum: `gettext`, `SetText`, `HasText`
  - Added `SetTextParams` struct for set text operation
  - Added `validate_clipboard_config()` for config validation (timeout > 0, timeout <= 1 hour)
  - Added `ClipboardBackend` trait for backend abstraction with methods: `get_text()`, `set_text()`, `has_text()`
  - Added Windows backend using `clipboard` crate v0.5 (synchronous, handles COM internally)
  - Added `MockClipboardBackend` for unit testing with `Arc<Mutex<MockClipboardState>>`
  - Added full support for clipboard operations with idempotency (no state changes on error)
  - Added unit tests covering validation, mock backend, idempotency (8/8 tests passing)
  - Added integration tests covering all scenarios (success, errors, timeout, cancellation, empty clipboard, access denied, invalid text, unsupported platform)
  - Added `docs/design/clipboard-tool/` with specification, contract, test-plan, and brief
  - Added `docs/adr/015-clipboard-tool.md` architecture decision record

- **WindowControlTool Module**
  - Added `WindowControlTool` for managing window states (maximize/restore/minimize) via UI Automation API
  - Added `WindowControlConfig` with timeout and cancellation support
  - Added `WindowControlError` with variants: `ElementNotFound`, `WindowNotEnabled`, `WindowOffscreen`, `WindowPatternNotAvailable`, `Timeout`, `Cancelled`, `InvalidConfig`, `ComError`, `UnsupportedPlatform`
  - Added `WindowControlAction` enum: `Maximize`, `Restore`, `Minimize`
  - Added `WindowControlBackend` trait for backend abstraction with method: `window_control()`
  - Added `validate_window_control_config()` for config validation (timeout > 0, timeout <= 1 hour)
  - Added `window_control_with_config()` with timeout and cancellation handling
  - Added Windows backend using `UIWindowPattern.set_window_visual_state()` via UI Automation
  - Added `MockWindowControlBackend` for unit testing with `Arc<Mutex<MockWindowControlState>>`
  - Added full support for elements with WindowPattern and validation (is_enabled, is_offscreen)
  - Added unit tests covering validation, mock backend (9/9 tests passing)
  - Added integration tests covering all scenarios (success, errors, timeout, cancellation, disabled element, offscreen element, pattern not available, unsupported platform)
  - Added `docs/design/window-control-tool/` with specification, contract, test-plan, and brief

### Changed

- **Documentation Updates**:
  - Updated `ARCHITECTURE.md` with ClipboardTool Architecture section
  - Updated `README.md` with correct ClipboardTool API usage (direct function calls instead of `session.clipboard()`)
  - Corrected ClipboardTool implementation details (uses `clipboard` crate, not direct Win32 API)

## [Previous Versions]

### [0.1.0] - Initial Release

- **ClickTool Module** (ADR 001)
  - Added `ClickTool` for clicking UI elements through UI Automation API
  - Added `ClickConfig` with timeout and cancellation support
  - Added `ClickError` with variants: `ElementNotFound`, `ElementNotEnabled`, `ElementOffscreen`, `Timeout`, `Cancelled`, `InvalidConfig`, `ComError`, `UnsupportedPlatform`
  - Added `ClickType` enum: `LeftSingle`, `RightSingle`, `LeftDouble`
  - Added `ClickBackend` trait for backend abstraction with method: `click()`
  - Added Windows backend using `element.click()`, `element.right_click()`, `element.double_click()` via UI Automation
  - Added `MockClickBackend` for unit testing with `Arc<Mutex<MockClickState>>`
  - Added full support for multiple click types with validation (is_enabled, is_offscreen)
  - Added unit tests covering validation, mock backend, click types (10/10 tests passing)
  - Added integration tests covering all scenarios (success, errors, timeout, cancellation, disabled element, offscreen element)
  - Added `docs/design/click-tool/` with specification, contract, test-plan, and brief
  - Added `examples/click_example.rs` demonstrating click operations
  - Added `examples/notepad_click.rs` demonstrating notepad automation
  - Added `examples/right_click_example.rs` demonstrating right-click operations

- **AutomationSession Module** (ADR 002)
  - Added `AutomationSession` for managing application lifecycle
  - Added `SessionConfig` with timeout and cancellation support
  - Added `SessionLaunchConfig` for process launch configuration
  - Added `AutomationError` with variants: `ProcessLaunchFailed`, `WindowNotFound`, `ProcessNotFound`, `WindowDisabled`, `WindowOffscreen`, `InvalidConfig`, `Cancelled`, `SessionClosed`, `ComError`
  - Added `MatchMode` enum: `Exact`, `Contains`, `Regex`
  - Added `RuntimeSession` struct with process ID and main UI element
  - Added methods: `launch_process()`, `attach_by_process_id()`, `attach_by_title()`
  - Added validation functions: `validate_command()`, `validate_regex()`, `validate_title_filter()`, `validate_session_config()`
  - Added session state management: `Running`, `Closed`
  - Added element interaction methods: `click()`, `find_by_automation_id()`, `find_by_name()`
  - Added `docs/design/automation-session/` with specification, contract, test-plan, and brief

- **InspectTool Module** (ADR 003)
  - Added `InspectTool` for inspecting UI elements
  - Added `InspectConfig` with timeout and cancellation support
  - Added `InspectError` with variants: `ElementNotFound`, `ElementNotEnabled`, `ElementOffscreen`, `Timeout`, `Cancelled`, `InvalidConfig`, `ComError`, `UnsupportedPlatform`
  - Added `InspectBackend` trait for backend abstraction with method: `inspect_element()`
  - Added Windows backend using `UIElement` properties
  - Added `MockInspectBackend` for unit testing with `Arc<Mutex<MockInspectState>>`
  - Added full support for element inspection with validation (is_enabled, is_offscreen)
  - Added unit tests covering validation, mock backend (10/10 tests passing)
  - Added integration tests covering all scenarios (success, errors, timeout, cancellation, disabled element, offscreen element)
  - Added `docs/design/inspect-tool/` with specification, contract, test-plan, and brief

- **SelectorStorage Module** (ADR 003)
  - Added `SelectorStorage` for saving/loading selectors
  - Added `SelectorStorageConfig` with storage path and timeout
  - Added `StorageError` with variants: `FileNotFound`, `InvalidFormat`, `IOError`, `Timeout`, `Cancelled`, `InvalidConfig`
  - Added `SelectorRecorder` for capturing elements
  - Added `RecordedSelector` struct with selector steps
  - Added `SerializableRecordedSelector` for JSON serialization
  - Added `SelectorStorage` methods: `save_selector()`, `load_selector()`, `delete_selector()`, `list_selectors()`
  - Added `SelectorStep` enum: `ByAutomationId`, `ByName`, `ByControlType`
  - Added control type conversion functions: `control_type_to_string()`, `control_type_from_string()`
  - Added unit tests covering validation, storage operations (8/8 tests passing)
  - Added integration tests covering file I/O operations
  - Added `docs/design/selector-storage/` with specification, contract, test-plan, and brief

- **ReadTool Module** (ADR 008)
  - Added `ReadTool` for reading text from UI elements
  - Added `ReadConfig` with timeout and cancellation support
  - Added `ReadError` with variants: `ElementNotFound`, `ElementNotEnabled`, `ElementOffscreen`, `ElementNotWritable`, `Timeout`, `Cancelled`, `InvalidConfig`, `ComError`, `UnsupportedPlatform`
  - Added `ReadBackend` trait for backend abstraction with method: `read_text()`
  - Added Windows backend using `UITextPattern.get_document_text()` (primary) and `UIElement.get_name()` (fallback)
  - Added `MockReadBackend` for unit testing with `Arc<Mutex<MockReadState>>`
  - Added full support for text reading with validation (is_enabled, is_offscreen)
  - Added unit tests covering validation, mock backend (8/8 tests passing)
  - Added integration tests covering all scenarios (success, errors, timeout, cancellation, disabled element, offscreen element)
  - Added `docs/design/read-tool/` with specification, contract, test-plan, and brief

- **ScreenshotTool Module** (ADR 007)
  - Added `ScreenshotTool` for capturing screenshots
  - Added `ScreenshotConfig` with timeout and cancellation support
  - Added `ScreenshotError` with variants: `CaptureFailed`, `UnsupportedMode`, `Timeout`, `Cancelled`, `InvalidConfig`, `UnsupportedPlatform`
  - Added `ScreenshotMode` enum: `Screen`, `Window(UIElement)`, `Region(Point, Size)`
  - Added `ScreenshotBackend` trait for backend abstraction with method: `capture()`
  - Added Windows backend using GDI/GDI+ via `spawn_blocking` (no STA affinity required)
  - Added `MockScreenshotBackend` for unit testing with `Arc<Mutex<MockScreenshotState>>`
  - Added full support for screenshot capture with validation
  - Added unit tests covering validation, mock backend (7/7 tests passing)
  - Added integration tests covering all scenarios (success, errors, timeout, cancellation, unsupported mode)
  - Added `docs/design/screenshot-tool/` with specification, contract, test-plan, and brief

- **ToggleTool Module** (ADR 012)
  - Added `ToggleTool` for controlling toggle state of checkboxes, radio buttons, toggle switches
  - Added `ToggleConfig` with timeout and cancellation support
  - Added `ToggleError` with variants: `ElementNotFound`, `ElementNotEnabled`, `ElementOffscreen`, `ElementNotSupported`, `ElementNotWritable`, `Timeout`, `Cancelled`, `InvalidConfig`, `ComError`, `UnsupportedPlatform`
  - Added `ToggleBackend` trait for backend abstraction with methods: `toggle_element()`, `set_radio()`, `set_toggle()`, `is_checked()`, `is_selected()`
  - Added Windows backend using `UITogglePattern` (primary) and `UIValuePattern` (fallback)
  - Added `MockToggleBackend` for unit testing with `Arc<Mutex<MockToggleState>>`
  - Added full support for elements with TogglePattern, ValuePattern (is_readonly=false), LegacyIAccessible
  - Added unit tests covering validation, mock backend (10/10 tests passing)
  - Added integration tests covering all scenarios (success, errors, timeout, cancellation, disabled element, offscreen element, pattern not supported)
  - Added `docs/design/toggle-tool/` with specification, contract, test-plan, and brief

- **RightClickTool Refactoring** (ADR 011)
  - Refactored `ClickTool` to support multiple click types via `ClickType` enum
  - Added `ClickType` enum with variants: `LeftSingle`, `RightSingle`, `LeftDouble`
  - Updated `ClickConfig` with `click_type: ClickType` field
  - Updated `ClickBackend::click()` to accept `click_type` parameter
  - Implemented left click via `element.click()`, right click via `element.right_click()`, double click via `element.double_click()`
  - Simplified `RightClickTool` to be a convenience wrapper around `ClickTool` with `RightSingle` click type
  - Removed separate `RightClickBackend` trait - now delegated to `ClickBackend`
  - All click types share same validation logic and error handling
  - All tests passing (116 tests including RightClickTool tests)
  - All examples updated to reflect new API

- **RightClickTool Module** (ADR 010)
  - Added `RightClickTool` for right-clicking UI elements through UI Automation API
  - Added `RightClickConfig` with timeout and cancellation support
  - Added `RightClickError` with variants: `ElementNotFound`, `ElementNotEnabled`, `ElementOffscreen`, `Timeout`, `Cancelled`, `InvalidConfig`, `ComError`, `UnsupportedPlatform`
  - Added `RightClickBackend` trait for backend abstraction with method: `right_click()`
  - Added Windows backend using `uiautomation::inputs::Mouse::right_click(&Point)` via UI Automation
  - Added `MockRightClickBackend` for unit testing with `Arc<Mutex<MockRightClickState>>`
  - Added full support for elements with InvokePattern (left click) and Mouse right-click fallback
  - Added unit tests covering validation, mock backend (10/10 tests passing)
  - Added integration tests covering all scenarios (success, errors, timeout, cancellation, disabled element, offscreen element)
  - Added `docs/design/right-click-tool/` with specification, contract, test-plan, and brief
  - Added `examples/right_click_example.rs` demonstrating right-click operations

- **ScrollTool Module** (ADR 009)
  - Added `ScrollTool` for scrolling UI elements vertically/horizontally through UI Automation API
  - Added `ScrollConfig` with timeout and cancellation support
  - Added `ScrollError` with variants: `ElementNotFound`, `ElementNotEnabled`, `ElementOffscreen`, `Timeout`, `Cancelled`, `InvalidConfig`, `PatternNotSupported`, `UnsupportedPlatform`
  - Added `ScrollDirection` enum: `Vertical` | `Horizontal`
  - Added `ScrollUnit` enum: `Line` | `Page` | `Pixel`
  - Added `ScrollBackend` trait for backend abstraction with method: `scroll_by_element()`
  - Added Windows backend using `UIScrollPattern` (primary) and synthetic scrolling (fallback)
  - Added `MockScrollBackend` for unit testing with `Arc<Mutex<MockScrollState>>`
  - Added full support for scrolling with validation (is_enabled, is_offscreen)
  - Added unit tests covering validation, mock backend (8/8 tests passing)
  - Added integration tests covering all scenarios (success, errors, timeout, cancellation, disabled element, offscreen element, pattern not supported)
  - Added `docs/design/scroll-tool/` with specification, contract, test-plan, and brief

- **WaitTool Module** (ADR 006)
  - Added `WaitTool` for waiting for elements with timeout and periodic polling
  - Added `WaitConfig` with timeout and cancellation support
  - Added `WaitError` with variants: `Timeout`, `Cancelled`, `InvalidConfig`, `UnsupportedPlatform`
  - Added `WaitMode` enum: `Visible`, `Enabled`, `Exists`, `Gone`
  - Added `WaitBackend` trait for backend abstraction with method: `wait_for_element()`
  - Added Windows backend using `UIAutomation::element_from_point()` and element existence checks
  - Added `MockWaitBackend` for unit testing with `Arc<Mutex<MockWaitState>>`
  - Added full support for waiting with polling and timeout
  - Added unit tests covering validation, mock backend (6/6 tests passing)
  - Added integration tests covering all scenarios (success, timeout, cancellation, unsupported platform)
  - Added `docs/design/wait-tool/` with specification, contract, test-plan, and brief

- **TypeTool Module** (ADR 005)
  - Added `TypeTool` for typing text through clipboard (anti-detection method)
  - Added `TypeConfig` with timeout and cancellation support
  - Added `TypeError` with variants: `ClipboardEmpty`, `Timeout`, `Cancelled`, `InvalidConfig`, `UnsupportedPlatform`
  - Added `TypeBackend` trait for backend abstraction with method: `type_text()`
  - Added Windows backend using clipboard (copy text to clipboard, paste via Ctrl+V)
  - Added `MockTypeBackend` for unit testing with `Arc<Mutex<MockTypeState>>`
  - Added full support for typing with clipboard
  - Added unit tests covering validation, mock backend (6/6 tests passing)
  - Added integration tests covering all scenarios (success, errors, timeout, cancellation, clipboard unavailable)
  - Added `docs/design/type-tool/` with specification, contract, test-plan, and brief

- **InputTextTool Module** (ADR 005)
  - Added `InputTextTool` for keyboard input emulation
  - Added `InputTextConfig` with timeout and cancellation support
  - Added `InputTextError` with variants: `ElementNotFound`, `ElementNotEnabled`, `ElementOffscreen`, `Timeout`, `Cancelled`, `InvalidConfig`, `ComError`, `UnsupportedPlatform`
  - Added `InputTextBackend` trait for backend abstraction with method: `input_text()`
  - Added Windows backend using `Keyboard::send_keys()` via UI Automation
  - Added `MockInputTextBackend` for unit testing with `Arc<Mutex<MockInputTextState>>`
  - Added full support for keyboard input emulation
  - Added unit tests covering validation, mock backend (8/8 tests passing)
  - Added integration tests covering all scenarios (success, errors, timeout, cancellation, disabled element, offscreen element)
  - Added `docs/design/input-text-tool/` with specification, contract, test-plan, and brief

- **FocusTool Module** (ADR 013)
  - Added `FocusTool` for activating window before interaction with elements (solves: "window in background")
  - Added `FocusConfig` with timeout and cancellation support
  - Added `FocusError` with variants: `ElementNotFound`, `ElementNotEnabled`, `ElementOffscreen`, `WindowPatternNotAvailable`, `Timeout`, `Cancelled`, `InvalidConfig`, `ComError`, `UnsupportedPlatform`
  - Added `FocusBackend` trait for backend abstraction with method: `focus_element()`
  - Added Windows backend using `UIWindowPattern` (WaitForInputIdle + activation) via UI Automation
  - Added `MockFocusBackend` for unit testing with `Arc<Mutex<MockFocusState>>`
  - Added full support for elements with WindowPattern and validation (is_enabled, is_offscreen)
  - Added unit tests covering validation, mock backend (10/10 tests passing)
  - Added integration tests covering all scenarios (success, errors, timeout, cancellation, disabled element, offscreen element, pattern not available)
  - Added `docs/design/focus-tool/` with specification, contract, test-plan, and brief

---

**Version Format**: `MAJOR.MINOR.PATCH`
**Change Types**: `Added` | `Changed` | `Deprecated` | `Removed` | `Fixed` | `Security`
