//! Backends module — конкретные реализации для разных платформ

pub use crate::runtime::backends::windows::input_text::InputTextBackendWindows;
pub use crate::runtime::backends::windows::read::ReadBackendWindows;
pub use crate::runtime::backends::windows::screenshot::ScreenshotBackendWindows;
pub use crate::runtime::backends::windows::scroll::ScrollBackendWindows;
pub use crate::runtime::backends::windows::toggle::ToggleBackendWindows;
pub use crate::runtime::backends::windows::wait::WaitBackendWindows;
pub use crate::runtime::backends::windows::window_control::WindowControlBackendWindows;

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(not(target_os = "windows"))]
pub mod unsupported;

#[cfg(not(target_os = "windows"))]
pub use crate::runtime::backends::unsupported::ToggleBackendUnsupported;

#[cfg(not(target_os = "windows"))]
pub use crate::runtime::backends::unsupported::WindowControlBackendUnsupported;
