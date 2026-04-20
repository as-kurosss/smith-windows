//! Unsupported platform backend for InspectTool
//!
//! This module provides stub implementations for non-Windows platforms.

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
