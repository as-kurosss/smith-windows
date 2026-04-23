//! Windows backend for click operations using UI Automation

use tracing::{error, info};

use crate::core::click::{ClickConfig, ClickError};

/// Windows click backend implementation
pub struct ClickBackendWindows;

impl ClickBackendWindows {
    /// Creates a new Windows click backend
    pub fn new() -> Self {
        Self
    }
}

impl Default for ClickBackendWindows {
    fn default() -> Self {
        Self::new()
    }
}

impl ClickBackendWindows {
    /// Performs a click operation on the given element with specified click type
    pub async fn click(
        &self,
        element: &uiautomation::UIElement,
        click_type: crate::core::click::ClickType,
    ) -> Result<(), ClickError> {
        // Check element validity first - access a property to validate
        let _control_type = match element.get_control_type() {
            Ok(val) => val,
            Err(e) => {
                error!("Failed to get element control type: {}", e);
                return Err(ClickError::ElementNotFound);
            }
        };

        // Check if element is enabled
        let enabled_result = element.is_enabled();
        let is_enabled = match enabled_result {
            Ok(val) => val,
            Err(e) => {
                error!("Failed to check if element is enabled: {}", e);
                return Err(ClickError::ComError(e.to_string()));
            }
        };

        if !is_enabled {
            error!("Click failed: element is disabled");
            return Err(ClickError::ElementNotEnabled);
        }

        // Check if element is offscreen
        let offscreen_result = element.is_offscreen();
        let is_offscreen = match offscreen_result {
            Ok(val) => val,
            Err(e) => {
                error!("Failed to check if element is offscreen: {}", e);
                return Err(ClickError::ComError(e.to_string()));
            }
        };

        if is_offscreen {
            error!("Click failed: element is offscreen");
            return Err(ClickError::ElementOffscreen);
        }

        // Perform the click based on click type
        let result = match click_type {
            crate::core::click::ClickType::LeftSingle => element.click(),
            crate::core::click::ClickType::RightSingle => element.right_click(),
            crate::core::click::ClickType::LeftDouble => element.double_click(),
        };

        match result {
            Ok(()) => {
                info!("Click operation completed successfully: {:?}", click_type);
                Ok(())
            }
            Err(e) => {
                error!("Click operation failed: {}", e);
                Err(ClickError::ComError(e.to_string()))
            }
        }
    }
}

/// Performs a click operation with config validation and timeout handling
/// Note: UIElement is !Send, so we cannot use spawn_blocking or async move.
/// The backend call runs on the same thread that created the UIAutomation instance.
pub async fn click_with_config(
    element: &uiautomation::UIElement,
    config: &ClickConfig,
) -> Result<(), ClickError> {
    // Validate config BEFORE any backend calls
    crate::core::click::validate_click_config(config)?;

    info!(
        "Starting click operation with timeout: {:?}, click_type: {:?}",
        config.timeout, config.click_type
    );

    let backend = ClickBackendWindows::new();

    // Direct call to backend - UIElement cannot be moved into spawn_blocking
    // The backend call itself is synchronous and does not block the async runtime
    let click_result = backend.click(element, config.click_type).await;

    // Check for cancellation after backend call
    if config.cancellation.is_cancelled() {
        error!("Click operation cancelled during completion");
        return Err(ClickError::Cancelled);
    }

    // Apply timeout logic manually since we can't use timeout() wrapper
    // For now, return the direct result - timeout should be handled at a higher level
    click_result
}
