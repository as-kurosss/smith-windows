//! Right click tool implementation
//! Provides UI element right click functionality through UI Automation API.
//!
//! This is a convenience wrapper around ClickTool with ClickType::RightSingle.
//! All configuration and validation is delegated to ClickTool.

use std::time::Duration;
use tokio_util::sync::CancellationToken;

use crate::core::click::ClickError;

/// Configuration for right click operations
/// This is a wrapper around ClickConfig with click_type set to RightSingle
#[derive(Debug, Clone)]
pub struct RightClickConfig {
    /// Timeout for the right click operation
    pub timeout: Duration,
    /// Token for cancellation
    pub cancellation: CancellationToken,
}

/// Errors that can occur during right click operations
/// This is a type alias to ClickError
pub type RightClickError = ClickError;

/// Performs a right click operation with config validation and timeout handling
/// This is a convenience wrapper around click_with_config with RightSingle click type
pub async fn right_click_with_config(
    element: &uiautomation::UIElement,
    config: &RightClickConfig,
) -> Result<(), RightClickError> {
    // Convert to ClickConfig and delegate to ClickTool
    let click_config = crate::core::click::ClickConfig {
        click_type: crate::core::click::ClickType::RightSingle,
        timeout: config.timeout,
        cancellation: config.cancellation.clone(),
    };

    crate::core::click::click_with_config(element, &click_config).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_right_click_config_valid() {
        // Test that RightClickConfig can be created and used
        let cancellation = CancellationToken::new();
        let config = RightClickConfig {
            timeout: Duration::from_secs(5),
            cancellation,
        };

        // Verify config fields
        assert_eq!(config.timeout.as_secs(), 5);
    }

    #[test]
    fn test_right_click_error_type_alias() {
        // Test that RightClickError is the same as ClickError
        let error: RightClickError = ClickError::ElementNotFound;
        match error {
            ClickError::ElementNotFound => {}
            _ => panic!("Unexpected error type"),
        }
    }
}
