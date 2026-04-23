//! Windows backend for window control operations using UI Automation

use tracing::{error, info};

use crate::core::window_control::{WindowControlAction, WindowControlConfig, WindowControlError};

/// Windows window control backend implementation
pub struct WindowControlBackendWindows;

impl WindowControlBackendWindows {
    /// Creates a new Windows window control backend
    pub fn new() -> Self {
        Self
    }
}

impl Default for WindowControlBackendWindows {
    fn default() -> Self {
        Self::new()
    }
}

impl WindowControlBackendWindows {
    /// Performs a window control operation on the given element
    pub async fn window_control(
        &self,
        element: &uiautomation::UIElement,
        action: WindowControlAction,
    ) -> Result<(), WindowControlError> {
        // Check element validity first - access a property to validate
        let _control_type = match element.get_control_type() {
            Ok(val) => val,
            Err(e) => {
                error!("Failed to get element control type: {}", e);
                return Err(WindowControlError::ElementNotFound);
            }
        };

        // Check if element is enabled
        let enabled_result = element.is_enabled();
        let is_enabled = match enabled_result {
            Ok(val) => val,
            Err(e) => {
                error!("Failed to check if element is enabled: {}", e);
                return Err(WindowControlError::ComError(e.to_string()));
            }
        };

        if !is_enabled {
            error!("Window control failed: element is disabled");
            return Err(WindowControlError::WindowNotEnabled);
        }

        // Check if element is offscreen
        let offscreen_result = element.is_offscreen();
        let is_offscreen = match offscreen_result {
            Ok(val) => val,
            Err(e) => {
                error!("Failed to check if element is offscreen: {}", e);
                return Err(WindowControlError::ComError(e.to_string()));
            }
        };

        if is_offscreen {
            error!("Window control failed: element is offscreen");
            return Err(WindowControlError::WindowOffscreen);
        }

        // Check if element has WindowPattern
        let window_pattern = match element.get_pattern::<uiautomation::patterns::UIWindowPattern>()
        {
            Ok(p) => p,
            Err(_) => {
                error!("Window control failed: WindowPattern not available");
                return Err(WindowControlError::WindowPatternNotAvailable);
            }
        };

        // Determine the visual state based on action
        let visual_state = match action {
            WindowControlAction::Maximize => uiautomation::types::WindowVisualState::Maximized,
            WindowControlAction::Restore => uiautomation::types::WindowVisualState::Normal,
            WindowControlAction::Minimize => uiautomation::types::WindowVisualState::Minimized,
        };

        // Perform the window control action
        let result = window_pattern.set_window_visual_state(visual_state);

        match result {
            Ok(()) => {
                info!(
                    "Window control operation completed successfully: {:?}",
                    action
                );
                Ok(())
            }
            Err(e) => {
                error!("Window control operation failed: {}", e);
                Err(WindowControlError::ComError(e.to_string()))
            }
        }
    }
}

/// Performs a window control operation with config validation and timeout handling
/// Note: UIElement is !Send, so we cannot use spawn_blocking or async move.
/// The backend call runs on the same thread that created the UIAutomation instance.
pub async fn window_control_with_config(
    element: &uiautomation::UIElement,
    config: &WindowControlConfig,
) -> Result<(), WindowControlError> {
    // Validate config BEFORE any backend calls
    crate::core::window_control::validate_window_control_config(config)?;

    info!(
        "Starting window control operation with timeout: {:?}, action: {:?}",
        config.timeout, config.action
    );

    // Check for cancellation before starting
    if config.cancellation.is_cancelled() {
        error!("Window control operation cancelled before start");
        return Err(WindowControlError::Cancelled);
    }

    // Create backend and perform operation with timeout
    let backend = WindowControlBackendWindows::new();
    let operation = backend.window_control(element, config.action);

    // Apply timeout using tokio::time::timeout
    let result = tokio::time::timeout(config.timeout, operation).await;

    match result {
        Ok(inner_result) => {
            if config.cancellation.is_cancelled() {
                error!("Window control operation cancelled during execution");
                return Err(WindowControlError::Cancelled);
            }
            inner_result
        }
        Err(_) => {
            error!(
                "Window control operation timed out after {:?}",
                config.timeout
            );
            Err(WindowControlError::Timeout)
        }
    }
}
