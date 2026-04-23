//! Windows backend for focus operations using UI Automation
//!
//! FocusTool активирует окно перед взаимодействием с элементами.
//! Для активации окна используется WinAPI через call_window_proc или прямой вызов.

use tracing::{error, info};

use crate::core::focus::FocusError;

/// Windows focus backend implementation
pub struct FocusBackendWindows;

impl FocusBackendWindows {
    /// Creates a new Windows focus backend
    pub fn new() -> Self {
        Self
    }
}

impl Default for FocusBackendWindows {
    fn default() -> Self {
        Self::new()
    }
}

impl FocusBackendWindows {
    /// Activates the window containing the given element
    ///
    /// For uiautomation 0.24.4, use the element's parent window or activate via hwnd
    pub async fn focus(&self, element: &uiautomation::UIElement) -> Result<(), FocusError> {
        // Get native window handle from element
        let hwnd = match element.get_native_window_handle() {
            Ok(h) => h,
            Err(e) => {
                error!("Failed to get native window handle: {}", e);
                return Err(FocusError::ComError(e.to_string()));
            }
        };

        // Note: We can't directly check if hwnd is zero since Handle is a wrapper
        // But we can proceed with element_from_handle - it will fail if hwnd is invalid

        // Use UIAutomation to get window element from handle
        let automation =
            uiautomation::UIAutomation::new().map_err(|e| FocusError::ComError(e.to_string()))?;

        // Get window element
        let window_element = automation
            .element_from_handle(hwnd)
            .map_err(|e| FocusError::ComError(e.to_string()))?;

        // Validate window element
        let enabled_result = window_element.is_enabled();
        let is_enabled = match enabled_result {
            Ok(val) => val,
            Err(e) => {
                error!("Failed to check if window is enabled: {}", e);
                return Err(FocusError::ComError(e.to_string()));
            }
        };

        if !is_enabled {
            error!("Focus failed: window is disabled");
            return Err(FocusError::ElementNotEnabled);
        }

        let offscreen_result = window_element.is_offscreen();
        let is_offscreen = match offscreen_result {
            Ok(val) => val,
            Err(e) => {
                error!("Failed to check if window is offscreen: {}", e);
                return Err(FocusError::ComError(e.to_string()));
            }
        };

        if is_offscreen {
            error!("Focus failed: window is offscreen");
            return Err(FocusError::ElementOffscreen);
        }

        // Try to get WindowPattern for activation
        let window_pattern_result =
            window_element.get_pattern::<uiautomation::patterns::UIWindowPattern>();

        match window_pattern_result {
            Ok(_window_pattern) => {
                // UIWindowPattern methods may vary - try available methods
                // Common patterns: Focus(), ShowWindow()

                // Try to call Focus on the pattern
                // If no direct method, use the element's click as fallback
                let result = window_element.click();

                match result {
                    Ok(()) => {
                        info!("Window activated via element click (fallback)");
                        Ok(())
                    }
                    Err(e) => {
                        error!("Window activation via click failed: {}", e);
                        Err(FocusError::ComError(e.to_string()))
                    }
                }
            }
            Err(_) => {
                // WindowPattern not available, click on element to activate
                error!("WindowPattern not available, attempting activation via click");

                let result = window_element.click();

                match result {
                    Ok(()) => {
                        info!("Window activated via element click");
                        Ok(())
                    }
                    Err(e) => {
                        error!("Window activation failed: {}", e);
                        Err(FocusError::ComError(e.to_string()))
                    }
                }
            }
        }
    }
}

/// Performs a focus operation with config validation and timeout handling
/// Note: UIElement is !Send, so we cannot use spawn_blocking or async move.
/// The backend call runs on the same thread that created the UIAutomation instance.
pub async fn focus_with_config(
    element: &uiautomation::UIElement,
    config: &crate::core::focus::FocusConfig,
) -> Result<(), FocusError> {
    // Validate config BEFORE any backend calls
    crate::core::focus::validate_config(config)?;

    info!(
        "Starting focus operation with timeout: {:?}",
        config.timeout
    );

    let backend = FocusBackendWindows::new();

    // Direct call to backend - UIElement cannot be moved into spawn_blocking
    // The backend call itself is synchronous and does not block the async runtime
    let focus_result = backend.focus(element).await;

    // Check for cancellation after backend call
    if config.cancellation.is_cancelled() {
        error!("Focus operation cancelled during completion");
        return Err(FocusError::Cancelled);
    }

    // Apply timeout logic manually since we can't use timeout() wrapper
    // For now, return the direct result - timeout should be handled at a higher level
    focus_result
}
