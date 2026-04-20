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
    SessionConfig, SessionLaunchConfig, AutomationError, SessionState, MatchMode, RuntimeSession,
    SessionBackend, launch_process, attach_by_title, attach_by_process_id,
    validate_session_config, validate_title_filter, validate_regex, validate_command,
};
pub use crate::core::click::{ClickConfig, ClickError, ClickBackend};
pub use crate::core::r#type::{TypeConfig, TypeError, TypeBackend};
