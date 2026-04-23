//! Windows set text implementation via uiautomation

use tracing::{error, info};

use crate::core::set_text::{SetTextBackend, SetTextError};

/// Windows set text backend implementation
pub struct SetTextBackendWindows;

impl SetTextBackendWindows {
    /// Creates a new Windows set text backend
    pub fn new() -> Self {
        Self
    }
}

impl Default for SetTextBackendWindows {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait(?Send)]
impl SetTextBackend for SetTextBackendWindows {
    async fn set_text(
        &self,
        element: &uiautomation::UIElement,
        text: &str,
    ) -> Result<(), SetTextError> {
        // Check if element is enabled
        let enabled_result = element.is_enabled();
        let is_enabled = match enabled_result {
            Ok(val) => val,
            Err(e) => {
                error!("Failed to check if element is enabled: {}", e);
                return Err(SetTextError::ComError(e.to_string()));
            }
        };

        if !is_enabled {
            error!("Set text failed: element is disabled");
            return Err(SetTextError::ElementNotEnabled);
        }

        // Check if element is offscreen
        let offscreen_result = element.is_offscreen();
        let is_offscreen = match offscreen_result {
            Ok(val) => val,
            Err(e) => {
                error!("Failed to check if element is offscreen: {}", e);
                return Err(SetTextError::ComError(e.to_string()));
            }
        };

        if is_offscreen {
            error!("Set text failed: element is offscreen");
            return Err(SetTextError::ElementOffscreen);
        }

        // Set text using ValuePattern
        // Note: UIElement does not have is_read_only() method
        // We try to use ValuePattern and handle any errors
        let value_pattern_result = element.get_pattern::<uiautomation::patterns::UIValuePattern>();

        let value_pattern = match value_pattern_result {
            Ok(pattern) => pattern,
            Err(e) => {
                error!("Failed to get ValuePattern: {}", e);
                return Err(SetTextError::ComError(e.to_string()));
            }
        };

        let result = value_pattern.set_value(text);

        match result {
            Ok(()) => {
                info!("Set text operation completed successfully");
                Ok(())
            }
            Err(e) => {
                error!("Set text operation failed: {}", e);
                Err(SetTextError::ComError(e.to_string()))
            }
        }
    }
}
