//! Windows backend for selector operations using UI Automation
//!
//! Provides implementations for finding elements by process ID and
//! other selector-based operations.

use tracing::{debug, error, info};

use crate::core::selector::{find_process_root, Selector, SelectorError};

/// Windows selector backend implementation
pub struct SelectorBackendWindows;

impl SelectorBackendWindows {
    /// Creates a new Windows selector backend
    pub fn new() -> Self {
        Self
    }
}

impl Default for SelectorBackendWindows {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(target_os = "windows")]
impl SelectorBackendWindows {
    /// Finds a root element for a given process ID
    pub async fn find_process_root(
        &self,
        process_id: u32,
        config: &crate::core::automation_session::SessionConfig,
    ) -> Result<uiautomation::UIElement, SelectorError> {
        debug!("find_process_root: process_id={}", process_id);
        find_process_root(process_id, config).await
    }

    /// Finds an element using a selector, with special handling for ProcessId
    pub async fn find_element_by_selector(
        &self,
        selector: &Selector,
        config: &crate::core::automation_session::SessionConfig,
    ) -> Result<uiautomation::UIElement, SelectorError> {
        match selector {
            #[cfg(target_os = "windows")]
            Selector::ProcessId(pid) => {
                // For ProcessId selector, find the process root first
                self.find_process_root(*pid, config).await
            }
            _ => {
                // For other selectors, use the default finder
                // This requires a head_window which we don't have here
                // In practice, this would be used with a specific window context
                Err(SelectorError::InvalidSelector(
                    "Selector requires a head_window context".to_string(),
                ))
            }
        }
    }
}

//tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selector_backend_windows_new() {
        // Test new() creates backend
        let backend = SelectorBackendWindows::new();
        // Backend has no state to check
        let _ = backend;
    }

    #[test]
    fn test_selector_backend_windows_default() {
        // Test Default impl
        let backend = SelectorBackendWindows::default();
        let _ = backend;
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_find_process_root_invalid_pid() {
        // Test that invalid PID (0) returns error
        let config = crate::core::automation_session::SessionConfig {
            timeout: std::time::Duration::from_secs(5),
            cancellation: tokio_util::sync::CancellationToken::new(),
        };

        let backend = SelectorBackendWindows::new();
        let result = backend.find_process_root(0, &config).await;

        assert!(result.is_err(), "Expected Err for PID 0");
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_find_element_by_selector_invalid() {
        // Test that non-ProcessId selector returns error (no head_window)
        let config = crate::core::automation_session::SessionConfig {
            timeout: std::time::Duration::from_secs(5),
            cancellation: tokio_util::sync::CancellationToken::new(),
        };

        let backend = SelectorBackendWindows::new();
        let result = backend
            .find_element_by_selector(&Selector::Name("test".to_string()), &config)
            .await;

        assert!(result.is_err(), "Expected Err without head_window");
    }
}
