//! Windows clipboard implementation using arboard crate
//! Arboard provides cross-platform clipboard access with async support

use tracing::{error, info};

use crate::core::clipboard::{ClipboardBackend, ClipboardError};

/// Windows clipboard backend implementation
pub struct ClipboardBackendWindows;

impl ClipboardBackendWindows {
    /// Creates a new Windows clipboard backend
    pub fn new() -> Self {
        Self
    }
}

impl Default for ClipboardBackendWindows {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait(?Send)]
impl ClipboardBackend for ClipboardBackendWindows {
    async fn get_text(&self) -> Result<String, ClipboardError> {
        // Try to get clipboard text with arboard
        let result = arboard::Clipboard::new().and_then(|mut clipboard| clipboard.get_text());

        match result {
            Ok(text) => {
                info!("Get text operation completed successfully");
                Ok(text)
            }
            Err(e) => {
                error!("Get text operation failed: {}", e);
                // Check error type
                let err_str = e.to_string().to_lowercase();
                if err_str.contains("not available") || err_str.contains("timeout") {
                    Err(ClipboardError::ClipboardEmpty)
                } else if err_str.contains("access") || err_str.contains("permission") {
                    Err(ClipboardError::ClipboardAccessDenied)
                } else {
                    Err(ClipboardError::ComError(e.to_string()))
                }
            }
        }
    }

    async fn set_text(&self, text: &str) -> Result<(), ClipboardError> {
        // Try to set clipboard text with arboard
        match arboard::Clipboard::new() {
            Ok(mut clipboard) => {
                if clipboard.set_text(text).is_ok() {
                    info!("Set text operation completed successfully");
                    Ok(())
                } else {
                    error!("Set text operation failed: clipboard access denied");
                    Err(ClipboardError::ClipboardAccessDenied)
                }
            }
            Err(e) => {
                error!("Failed to create clipboard: {}", e);
                let err_str = e.to_string().to_lowercase();
                if err_str.contains("access") || err_str.contains("permission") {
                    Err(ClipboardError::ClipboardAccessDenied)
                } else {
                    Err(ClipboardError::ComError(e.to_string()))
                }
            }
        }
    }

    async fn has_text(&self) -> Result<bool, ClipboardError> {
        // Check if clipboard contains text using arboard
        match arboard::Clipboard::new() {
            Ok(mut clipboard) => {
                let result = clipboard.get_text();
                match result {
                    Ok(text) => {
                        info!("Has text operation completed successfully");
                        Ok(!text.is_empty())
                    }
                    Err(e) => {
                        // If error is "not available" or "timeout", clipboard is empty
                        let err_str = e.to_string().to_lowercase();
                        if err_str.contains("not available") || err_str.contains("timeout") {
                            info!("Clipboard is empty (no text available)");
                            Ok(false)
                        } else if err_str.contains("access") || err_str.contains("permission") {
                            error!("Has text operation failed: access denied");
                            Err(ClipboardError::ClipboardAccessDenied)
                        } else {
                            error!("Has text operation failed: {}", e);
                            Err(ClipboardError::ComError(e.to_string()))
                        }
                    }
                }
            }
            Err(e) => {
                error!("Failed to create clipboard: {}", e);
                Ok(false)
            }
        }
    }
}
