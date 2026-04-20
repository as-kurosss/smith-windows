//! Windows backend for click operations using UI Automation

use tracing::{info, error};

use crate::core::click::{ClickConfig, ClickError};
use crate::core::click::validate_click_config;

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
    /// Performs a click operation on the given element
    pub async fn click(&self, element: &uiautomation::UIElement) -> Result<(), ClickError> {
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

        // Perform the click
        let result = element.click();
        
        match result {
            Ok(()) => {
                info!("Click operation completed successfully");
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
pub async fn click_with_config(
    element: &uiautomation::UIElement,
    config: &ClickConfig,
) -> Result<(), ClickError> {
    // Validate config BEFORE any backend calls
    validate_click_config(config)?;
    
    info!(
        "Starting click operation with timeout: {:?}",
        config.timeout
    );

    let backend = ClickBackendWindows::new();
    
    // Wrap with timeout and cancellation
    let click_future = async move {
        backend.click(element).await
    };

    // Wrap the future with timeout
    let result = tokio::time::timeout(config.timeout, click_future).await;
    
    match result {
        Ok(click_result) => {
            // Check for cancellation
            if config.cancellation.is_cancelled() {
                error!("Click operation cancelled during completion");
                return Err(ClickError::Cancelled);
            }
            click_result
        }
        Err(_) => {
            error!("Click operation timed out after {:?}", config.timeout);
            Err(ClickError::Timeout)
        }
    }
}
