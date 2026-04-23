//! Windows backend for read operations using UI Automation

use tracing::{error, info};

use crate::core::read::{ReadBackend, ReadError};

/// Windows read backend implementation
pub struct ReadBackendWindows;

impl ReadBackendWindows {
    /// Creates a new Windows read backend
    pub fn new() -> Self {
        Self
    }
}

impl Default for ReadBackendWindows {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait(?Send)]
impl ReadBackend for ReadBackendWindows {
    async fn read_text(&self, element: &uiautomation::UIElement) -> Result<String, ReadError> {
        // Check element validity first - access a property to validate
        let _control_type = match element.get_control_type() {
            Ok(val) => val,
            Err(e) => {
                error!("Failed to get element control type: {}", e);
                return Err(ReadError::ElementNotFound);
            }
        };

        // Check if element is enabled
        let enabled_result = element.is_enabled();
        let is_enabled = match enabled_result {
            Ok(val) => val,
            Err(e) => {
                error!("Failed to check if element is enabled: {}", e);
                return Err(ReadError::ComError(e.to_string()));
            }
        };

        if !is_enabled {
            error!("Read failed: element is disabled");
            return Err(ReadError::ElementNotEnabled);
        }

        // Check if element is offscreen
        let offscreen_result = element.is_offscreen();
        let is_offscreen = match offscreen_result {
            Ok(val) => val,
            Err(e) => {
                error!("Failed to check if element is offscreen: {}", e);
                return Err(ReadError::ComError(e.to_string()));
            }
        };

        if is_offscreen {
            error!("Read failed: element is offscreen");
            return Err(ReadError::ElementOffscreen);
        }

        // Try to read text using ValuePattern first (simpler, for single-line)
        let value_pattern_result = element.get_pattern::<uiautomation::patterns::UIValuePattern>();

        match value_pattern_result {
            Ok(pattern) => {
                match pattern.get_value() {
                    Ok(val) => {
                        info!("Read operation completed successfully (ValuePattern)");
                        return Ok(val);
                    }
                    Err(e) => {
                        error!("Failed to get Value: {}", e);
                        // Fall through to try other patterns
                    }
                }
            }
            Err(_) => {
                // ValuePattern not available, continue to try other patterns
            }
        };

        // Try to read text using TextPattern (for multi-line)
        let text_pattern_result = element.get_pattern::<uiautomation::patterns::UITextPattern>();

        let text_pattern = match text_pattern_result {
            Ok(pattern) => pattern,
            Err(_) => {
                // If no text pattern available, try to get Name property
                let name_result = element.get_name();
                match name_result {
                    Ok(name) => {
                        info!("Read operation completed successfully (Name property)");
                        return Ok(name);
                    }
                    Err(e) => {
                        error!("Failed to get Name property: {}", e);
                        return Err(ReadError::ElementNotWritable);
                    }
                }
            }
        };

        // Get text from TextPattern using DocumentRange
        let range_result = text_pattern.get_document_range();
        let range = match range_result {
            Ok(range) => range,
            Err(e) => {
                error!("Failed to get document range: {}", e);
                return Err(ReadError::ComError(e.to_string()));
            }
        };

        // Get text from range (0 = unlimited length)
        let text_content = match range.get_text(0) {
            Ok(val) => val,
            Err(e) => {
                error!("Failed to get text from range: {}", e);
                return Err(ReadError::ComError(e.to_string()));
            }
        };

        info!("Read operation completed successfully (TextPattern)");
        Ok(text_content)
    }
}
