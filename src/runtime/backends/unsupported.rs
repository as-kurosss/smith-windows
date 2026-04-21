//! Unsupported platform backend for InspectTool and InputTool
//!
//! This module provides stub implementations for non-Windows platforms.

use crate::core::input::{InputBackend, InputError};
use crate::core::inspect::{InspectBackend, InspectError};

/// Unsupported inspect backend implementation
pub struct InspectBackendUnsupported;

impl InspectBackendUnsupported {
    /// Creates a new unsupported backend
    pub fn new() -> Self {
        Self
    }
}

impl Default for InspectBackendUnsupported {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait(?Send)]
impl InspectBackend for InspectBackendUnsupported {
    async fn inspect_path(
        &self,
        _head_window: &uiautomation::UIElement,
        _element: &uiautomation::UIElement,
    ) -> Result<String, InspectError> {
        Err(InspectError::InvalidSelector)
    }
}

/// Unsupported input backend implementation
pub struct InputBackendUnsupported;

impl InputBackendUnsupported {
    /// Creates a new unsupported backend
    pub fn new() -> Self {
        Self
    }
}

impl Default for InputBackendUnsupported {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait(?Send)]
impl InputBackend for InputBackendUnsupported {
    async fn get_element_at_point(
        &self,
        _x: i32,
        _y: i32,
    ) -> Result<uiautomation::UIElement, InputError> {
        Err(InputError::ElementFromPointError(
            "Unsupported platform".to_string(),
        ))
    }

    async fn move_mouse(&self, _x: i32, _y: i32) -> Result<(), InputError> {
        Err(InputError::MouseMoveError(
            "Unsupported platform".to_string(),
        ))
    }

    async fn click_key(&self, _key: &str) -> Result<(), InputError> {
        Err(InputError::KeyClickError(
            "Unsupported platform".to_string(),
        ))
    }
}
