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
pub use crate::core::click::{ClickBackend, ClickConfig, ClickError};
pub use crate::core::inspect::{InspectBackend, InspectConfig, InspectError};
pub use crate::core::r#type::{TypeBackend, TypeConfig, TypeError};
