//! Window control tool implementation
//! Provides window state control functionality (maximize, restore, minimize) through UI Automation API.

use std::time::Duration;
use thiserror::Error;
use tokio_util::sync::CancellationToken;

/// Action to perform on a window
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowControlAction {
    /// Maximize the window (развернуть на весь экран)
    Maximize,
    /// Restore the window (восстановить предыдущий размер)
    Restore,
    /// Minimize the window (свернуть в панель задач)
    Minimize,
}

/// Configuration for window control operations
#[derive(Debug, Clone)]
pub struct WindowControlConfig {
    /// Action to perform on the window
    pub action: WindowControlAction,
    /// Timeout for the operation
    pub timeout: Duration,
    /// Token for cancellation
    pub cancellation: CancellationToken,
}

/// Errors that can occur during window control operations
#[derive(Error, Debug, Clone)]
pub enum WindowControlError {
    /// Element not found or invalid
    #[error("Element not found")]
    ElementNotFound,
    /// Element is disabled
    #[error("Element is disabled")]
    WindowNotEnabled,
    /// Element is offscreen
    #[error("Element is offscreen")]
    WindowOffscreen,
    /// Window pattern not available
    #[error("Window pattern not available")]
    WindowPatternNotAvailable,
    /// Operation timed out
    #[error("Operation timed out")]
    Timeout,
    /// Operation was cancelled
    #[error("Operation was cancelled")]
    Cancelled,
    /// Invalid configuration
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
    /// COM error
    #[error("COM error: {0}")]
    ComError(String),
    /// Unsupported platform
    #[error("Unsupported platform")]
    UnsupportedPlatform,
}

/// Validates window control configuration
/// Must be called BEFORE backend invocation
pub fn validate_window_control_config(
    config: &WindowControlConfig,
) -> Result<(), WindowControlError> {
    // Check timeout bounds: > 0 and <= 1 hour
    if config.timeout.is_zero() || config.timeout > Duration::from_secs(3600) {
        return Err(WindowControlError::InvalidConfig(
            "timeout must be > 0 and <= 1 hour".to_string(),
        ));
    }

    // Validate action - all actions are supported
    match config.action {
        WindowControlAction::Maximize
        | WindowControlAction::Restore
        | WindowControlAction::Minimize => {
            // All actions are valid
        }
    }

    Ok(())
}

/// Trait for window control backend implementations
#[async_trait::async_trait(?Send)]
pub trait WindowControlBackend {
    /// Performs a window control operation on the given element
    async fn window_control(
        &self,
        element: &uiautomation::UIElement,
        action: WindowControlAction,
    ) -> Result<(), WindowControlError>;
}

/// Mock backend for testing
/// Uses internal state to simulate different scenarios
#[derive(Debug, Clone, Default)]
pub struct MockWindowControlBackend {
    state: std::sync::Arc<std::sync::Mutex<MockWindowControlState>>,
}

/// State for mock backend
#[derive(Debug, Default)]
pub struct MockWindowControlState {
    pub call_count: usize,
    pub last_error: Option<WindowControlError>,
    pub should_succeed: bool,
    pub last_action: Option<WindowControlAction>,
}

impl MockWindowControlState {
    /// Creates a new state with success by default
    pub fn new() -> Self {
        Self {
            should_succeed: true,
            ..Default::default()
        }
    }
}

impl MockWindowControlBackend {
    /// Creates a new mock backend with default state (should_succeed = true)
    pub fn new() -> Self {
        Self {
            state: std::sync::Arc::new(std::sync::Mutex::new(MockWindowControlState::new())),
        }
    }

    /// Creates a mock backend with custom state
    pub fn with_state(state: MockWindowControlState) -> Self {
        Self {
            state: std::sync::Arc::new(std::sync::Mutex::new(state)),
        }
    }

    /// Resets the backend state
    pub fn reset(&self) {
        let mut state = self.state.lock().unwrap();
        *state = MockWindowControlState::new();
    }

    /// Returns the current state
    pub fn state(&self) -> std::sync::MutexGuard<'_, MockWindowControlState> {
        self.state.lock().unwrap()
    }
}

#[async_trait::async_trait(?Send)]
impl WindowControlBackend for MockWindowControlBackend {
    async fn window_control(
        &self,
        _element: &uiautomation::UIElement,
        action: WindowControlAction,
    ) -> Result<(), WindowControlError> {
        let mut state = self.state.lock().unwrap();
        state.call_count += 1;
        state.last_action = Some(action);

        tracing::info!(
            "MockWindowControlBackend::window_control: should_succeed={}",
            state.should_succeed
        );

        if state.should_succeed {
            Ok(())
        } else {
            let error = WindowControlError::ElementNotFound;
            state.last_error = Some(error.clone());
            Err(error)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_config_valid() {
        let config = WindowControlConfig {
            action: WindowControlAction::Maximize,
            timeout: Duration::from_secs(5),
            cancellation: CancellationToken::new(),
        };

        assert!(validate_window_control_config(&config).is_ok());
    }

    #[test]
    fn test_validate_config_zero_timeout() {
        let config = WindowControlConfig {
            action: WindowControlAction::Maximize,
            timeout: Duration::ZERO,
            cancellation: CancellationToken::new(),
        };

        let result = validate_window_control_config(&config);
        assert!(matches!(result, Err(WindowControlError::InvalidConfig(_))));
    }

    #[test]
    fn test_validate_config_large_timeout() {
        let config = WindowControlConfig {
            action: WindowControlAction::Maximize,
            timeout: Duration::from_secs(3601),
            cancellation: CancellationToken::new(),
        };

        let result = validate_window_control_config(&config);
        assert!(matches!(result, Err(WindowControlError::InvalidConfig(_))));
    }

    #[test]
    fn test_validate_config_min_valid() {
        let config = WindowControlConfig {
            action: WindowControlAction::Maximize,
            timeout: Duration::from_secs(1),
            cancellation: CancellationToken::new(),
        };

        assert!(validate_window_control_config(&config).is_ok());
    }

    #[test]
    fn test_validate_config_max_valid() {
        let config = WindowControlConfig {
            action: WindowControlAction::Maximize,
            timeout: Duration::from_secs(3600),
            cancellation: CancellationToken::new(),
        };

        assert!(validate_window_control_config(&config).is_ok());
    }

    #[test]
    fn test_mock_backend_creation() {
        let backend = MockWindowControlBackend::new();
        let state = backend.state();
        assert_eq!(state.call_count, 0);
        assert!(state.last_error.is_none());
        assert!(state.last_action.is_none());
    }

    #[test]
    fn test_mock_backend_with_state() {
        let backend = MockWindowControlBackend::with_state(MockWindowControlState {
            should_succeed: false,
            ..Default::default()
        });
        let state = backend.state();
        assert!(!state.should_succeed);
    }

    #[test]
    fn test_mock_backend_reset() {
        let backend = MockWindowControlBackend::new();
        backend.reset();
        let state = backend.state();
        assert_eq!(state.call_count, 0);
    }

    #[test]
    fn test_action_variants() {
        assert_eq!(WindowControlAction::Maximize, WindowControlAction::Maximize);
        assert_eq!(WindowControlAction::Restore, WindowControlAction::Restore);
        assert_eq!(WindowControlAction::Minimize, WindowControlAction::Minimize);
    }
}
