//! Windows backend for right click operations using UI Automation
//!
//! This is a convenience wrapper around ClickBackend with ClickType::RightSingle.

use crate::core::right_click::{RightClickConfig, RightClickError};

/// Windows right click backend implementation
/// This is a wrapper around ClickBackendWindows
pub struct RightClickBackendWindows;

impl RightClickBackendWindows {
    /// Creates a new Windows right click backend
    pub fn new() -> Self {
        Self
    }
}

impl Default for RightClickBackendWindows {
    fn default() -> Self {
        Self::new()
    }
}

impl RightClickBackendWindows {
    /// Performs a right click operation on the given element
    /// This is a wrapper around ClickBackendWindows::click with RightSingle
    pub async fn right_click(
        &self,
        element: &uiautomation::UIElement,
    ) -> Result<(), RightClickError> {
        let click_backend = crate::runtime::backends::windows::click::ClickBackendWindows::new();
        click_backend
            .click(element, crate::core::click::ClickType::RightSingle)
            .await
    }
}

/// Performs a right click operation with config validation and timeout handling
/// This is a wrapper around click_with_config with RightSingle click type
pub async fn right_click_with_config(
    element: &uiautomation::UIElement,
    config: &RightClickConfig,
) -> Result<(), RightClickError> {
    crate::core::right_click::right_click_with_config(element, config).await
}
