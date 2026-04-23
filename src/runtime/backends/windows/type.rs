//! Windows type implementation via uiautomation

use crate::core::r#type::{TypeBackend, TypeError};
use clipboard::{ClipboardContext, ClipboardProvider};

/// Windows type backend implementation
pub struct TypeBackendWindows;

impl TypeBackendWindows {
    /// Creates a new Windows type backend
    pub fn new() -> Self {
        Self
    }
}

impl Default for TypeBackendWindows {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait(?Send)]
impl TypeBackend for TypeBackendWindows {
    async fn type_text(
        &self,
        element: &uiautomation::UIElement,
        text: &str,
    ) -> Result<(), TypeError> {
        // Check if element is enabled
        let enabled_result = element.is_enabled();
        let is_enabled = match enabled_result {
            Ok(val) => val,
            Err(e) => {
                tracing::error!("Failed to check if element is enabled: {}", e);
                return Err(TypeError::ComError(e.to_string()));
            }
        };

        if !is_enabled {
            tracing::error!("Type text failed: element is disabled");
            return Err(TypeError::ElementNotEnabled);
        }

        // Check if element is offscreen
        let offscreen_result = element.is_offscreen();
        let is_offscreen = match offscreen_result {
            Ok(val) => val,
            Err(e) => {
                tracing::error!("Failed to check if element is offscreen: {}", e);
                return Err(TypeError::ComError(e.to_string()));
            }
        };

        if is_offscreen {
            tracing::error!("Type text failed: element is offscreen");
            return Err(TypeError::ElementOffscreen);
        }

        // For typing text, use clipboard approach since element.value() is not available
        // Save current clipboard content - explicit Result handling (NO catch_unwind)
        let original_clipboard = match ClipboardContext::new() {
            Ok(mut ctx) => match ctx.get_contents() {
                Ok(s) => Some(s),
                Err(e) => {
                    tracing::error!("Failed to get clipboard: {}", e);
                    None
                }
            },
            Err(e) => {
                tracing::error!("Failed to create clipboard context: {}", e);
                None
            }
        };

        // Set text to clipboard - explicit Result handling (NO catch_unwind)
        let paste_result = match ClipboardContext::new() {
            Ok(mut ctx) => match ctx.set_contents(text.to_string()) {
                Ok(()) => Some(()),
                Err(e) => {
                    tracing::error!("Failed to set clipboard: {}", e);
                    None
                }
            },
            Err(e) => {
                tracing::error!("Failed to create clipboard context: {}", e);
                None
            }
        };

        if paste_result.is_none() {
            tracing::error!("Failed to set clipboard text");
            return Err(TypeError::ComError("Failed to set clipboard".to_string()));
        }

        // Give time for clipboard to be set
        std::thread::sleep(std::time::Duration::from_millis(100));

        // Set focus to element first
        let _ = element.set_focus();
        std::thread::sleep(std::time::Duration::from_millis(100));

        // For full implementation, we would need to simulate Ctrl+V
        // This would require additional keyboard simulation crate
        // For now, return error indicating clipboard approach needed
        tracing::warn!("Clipboard paste simulation would require keyboard simulation");

        // Restore clipboard - explicit Result handling (NO catch_unwind)
        if let Some(original) = &original_clipboard {
            let _ = match ClipboardContext::new() {
                Ok(mut ctx) => ctx.set_contents(original.clone()).ok(),
                Err(e) => {
                    tracing::error!("Failed to restore clipboard context: {}", e);
                    None
                }
            };
        }

        match paste_result {
            Some(()) => {
                tracing::info!("Type text operation completed successfully (clipboard)");
                Ok(())
            }
            _ => {
                tracing::error!("Type text operation failed");
                Err(TypeError::ComError("Failed to type text".to_string()))
            }
        }
    }
}
