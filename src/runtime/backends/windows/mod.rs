//! Windows backend module

pub mod automation_session;
pub mod click;
pub mod clipboard;
pub mod focus;
pub mod hotkey;
pub mod input;
pub mod input_text;
pub mod inspect;
pub mod read;
pub mod right_click;
pub mod screenshot;
pub mod scroll;
pub mod set_text;
pub mod toggle;
pub mod r#type;
pub mod wait;
pub mod window_control;

// Re-export for convenience
pub use crate::runtime::backends::windows::automation_session::SessionBackendWindows;
pub use crate::runtime::backends::windows::click::ClickBackendWindows;
pub use crate::runtime::backends::windows::clipboard::ClipboardBackendWindows;
pub use crate::runtime::backends::windows::focus::{focus_with_config, FocusBackendWindows};
pub use crate::runtime::backends::windows::input::InputBackendWindows;
pub use crate::runtime::backends::windows::input_text::InputTextBackendWindows;
pub use crate::runtime::backends::windows::inspect::InspectBackendWindows;
pub use crate::runtime::backends::windows::r#type::TypeBackendWindows;
pub use crate::runtime::backends::windows::read::ReadBackendWindows;
pub use crate::runtime::backends::windows::right_click::RightClickBackendWindows;
pub use crate::runtime::backends::windows::screenshot::ScreenshotBackendWindows;
pub use crate::runtime::backends::windows::scroll::ScrollBackendWindows;
pub use crate::runtime::backends::windows::set_text::SetTextBackendWindows;
pub use crate::runtime::backends::windows::toggle::ToggleBackendWindows;
pub use crate::runtime::backends::windows::wait::WaitBackendWindows;
pub use crate::runtime::backends::windows::window_control::{
    window_control_with_config, WindowControlBackendWindows,
};
