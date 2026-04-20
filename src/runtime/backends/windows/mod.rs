//! Windows backend module

pub mod automation_session;
pub mod click;
pub mod r#type;

// Re-export for convenience
pub use crate::runtime::backends::windows::automation_session::SessionBackendWindows;
pub use crate::runtime::backends::windows::click::{ClickBackendWindows};
pub use crate::runtime::backends::windows::r#type::{TypeBackendWindows};
