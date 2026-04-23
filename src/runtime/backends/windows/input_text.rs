//! Windows input text implementation via uiautomation

use tracing::{error, info};

use crate::core::input_text::{InputTextBackend, InputTextError};

/// Windows input text backend implementation
pub struct InputTextBackendWindows;

impl InputTextBackendWindows {
    /// Creates a new Windows input text backend
    pub fn new() -> Self {
        Self
    }
}

impl Default for InputTextBackendWindows {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait(?Send)]
impl InputTextBackend for InputTextBackendWindows {
    async fn input_text(
        &self,
        element: &uiautomation::UIElement,
        keys: &str,
    ) -> Result<(), InputTextError> {
        // Check if element is enabled
        let enabled_result = element.is_enabled();
        let is_enabled = match enabled_result {
            Ok(val) => val,
            Err(e) => {
                error!("Failed to check if element is enabled: {}", e);
                return Err(InputTextError::ComError(e.to_string()));
            }
        };

        if !is_enabled {
            error!("Input text failed: element is disabled");
            return Err(InputTextError::ElementNotEnabled);
        }

        // Check if element is offscreen
        let offscreen_result = element.is_offscreen();
        let is_offscreen = match offscreen_result {
            Ok(val) => val,
            Err(e) => {
                error!("Failed to check if element is offscreen: {}", e);
                return Err(InputTextError::ComError(e.to_string()));
            }
        };

        if is_offscreen {
            error!("Input text failed: element is offscreen");
            return Err(InputTextError::ElementOffscreen);
        }

        // Note: UIElement does not have is_read_only() method in uiautomation 0.24.4
        // We try to use ValuePattern and handle any errors during input
        // For validation purposes, we'll attempt actual input anyway

        // Input text using Keyboard::send_keys
        // Keyboard operations are synchronous and do not block the async runtime
        let keyboard = uiautomation::inputs::Keyboard::new();
        let result = keyboard.send_keys(keys);

        match result {
            Ok(()) => {
                info!("Input text operation completed successfully");
                Ok(())
            }
            Err(e) => {
                error!("Input text operation failed: {}", e);
                Err(InputTextError::InputExecutionError(e.to_string()))
            }
        }
    }
}
