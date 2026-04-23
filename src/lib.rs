//! smith-windows — библиотека для автоматизации Windows через UI Automation API
//!
//! # Overview
//!
//! Эта библиотека предоставляет кроссплатформенный API для взаимодействия с
//! Windows UI Automation API через `uiautomation` crate.

pub mod core;
pub mod runtime;

// Re-export core types for convenience
pub use crate::core::automation_session::{
    attach_by_process_id, attach_by_title, launch_process, validate_command, validate_regex,
    validate_session_config, validate_title_filter, AutomationError, MatchMode, RuntimeSession,
    SessionBackend, SessionConfig, SessionLaunchConfig, SessionState,
};
pub use crate::core::click::{
    validate_click_config, ClickBackend, ClickConfig, ClickError, MockClickBackend,
};
pub use crate::core::clipboard::{
    get_text_with_config, has_text_with_config, set_text_with_config, validate_clipboard_config,
    ClipboardAction, ClipboardBackend, ClipboardConfig, ClipboardError, MockClipboardBackend,
    SetTextParams,
};
pub use crate::core::focus::{
    focus_with_config, validate_config, FocusBackend, FocusConfig, FocusError, MockFocusBackend,
};
pub use crate::core::input::{
    validate_input_config, InputBackend, InputConfig, InputError, MockInputBackend,
};
pub use crate::core::input_text::{
    validate_input_text_config, InputTextBackend, InputTextConfig, InputTextError,
    MockInputTextBackend,
};
pub use crate::core::inspect::{
    validate_inspect_config, InspectBackend, InspectConfig, InspectError, MockInspectBackend,
};
pub use crate::core::r#type::{
    validate_type_config, MockTypeBackend, TypeBackend, TypeConfig, TypeError,
};
pub use crate::core::read::{
    read_text_with_config, validate_read_config, MockReadBackend, ReadBackend, ReadConfig,
    ReadError,
};
pub use crate::core::screenshot::{
    screenshot_with_config, validate_screenshot_config, validate_screenshot_mode,
    MockScreenshotBackend, ScreenshotBackend, ScreenshotConfig, ScreenshotError, ScreenshotMode,
};
pub use crate::core::selector::{RecordedSelector, Selector, SelectorError, SelectorStep};
pub use crate::core::selector_storage::{
    control_type_from_string, control_type_to_string, SelectorStorage, SelectorStorageConfig,
    SerializableRecordedSelector, SerializableSelectorStep, StorageError,
};
pub use crate::core::set_text::{
    validate_set_text_config, MockSetTextBackend, SetTextBackend, SetTextConfig, SetTextError,
};
pub use crate::core::toggle::{
    is_checked_with_config, is_selected_with_config, set_radio_with_config, set_toggle_with_config,
    toggle_element_with_config, validate_toggle_config, MockToggleBackend, ToggleBackend,
    ToggleConfig, ToggleError,
};
pub use crate::core::wait::{
    validate_wait_config, MockWaitBackend, WaitBackend, WaitConfig, WaitError, WaitMode,
    WaitSelector,
};
pub use crate::core::window_control::{
    validate_window_control_config, MockWindowControlBackend, WindowControlAction,
    WindowControlBackend, WindowControlConfig, WindowControlError,
};
pub use crate::runtime::backends::windows::focus::FocusBackendWindows;
pub use crate::runtime::backends::windows::input::{
    get_cursor_position, get_element_under_ctrl_hotkey, get_element_under_cursor,
};
