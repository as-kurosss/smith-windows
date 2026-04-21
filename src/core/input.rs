//! Input module for hover and hotkey detection
//! Provides functionality to detect Ctrl+Hover and get UI element under cursor

use std::time::Duration;
use thiserror::Error;
use tokio_util::sync::CancellationToken;

/// Configuration for input operations
#[derive(Debug, Clone)]
pub struct InputConfig {
    /// Timeout for input operations
    pub timeout: Duration,
    /// Token for cancellation
    pub cancellation: CancellationToken,
}

/// Errors that can occur during input operations
#[derive(Error, Debug, Clone)]
pub enum InputError {
    /// Failed to move mouse cursor
    #[error("Failed to move mouse cursor: {0}")]
    MouseMoveError(String),
    /// Failed to click key
    #[error("Failed to click key: {0}")]
    KeyClickError(String),
    /// Failed to get element from point
    #[error("Failed to get element from point: {0}")]
    ElementFromPointError(String),
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
}

/// Validates input configuration
/// Must be called BEFORE backend invocation
pub fn validate_input_config(config: &InputConfig) -> Result<(), InputError> {
    // Check timeout bounds: > 0 and <= 1 hour
    if config.timeout.is_zero() || config.timeout > Duration::from_secs(3600) {
        return Err(InputError::InvalidConfig(
            "timeout must be > 0 and <= 1 hour".to_string(),
        ));
    }

    Ok(())
}

/// Trait for input backend implementations
#[async_trait::async_trait(?Send)]
pub trait InputBackend {
    /// Gets the UI element at specific screen coordinates
    async fn get_element_at_point(
        &self,
        x: i32,
        y: i32,
    ) -> Result<uiautomation::UIElement, InputError>;

    /// Moves mouse to specific coordinates
    async fn move_mouse(&self, x: i32, y: i32) -> Result<(), InputError>;

    /// Clicks a key by name (e.g., "CTRL")
    async fn click_key(&self, key: &str) -> Result<(), InputError>;
}

/// Mock backend for testing
/// Uses internal state to simulate different scenarios
/// Note: Uses Arc<Mutex<>> for state because UIElement is not Send + Sync,
/// so we cannot use Send wrapper types. This is consistent with other backends
/// in the project (click, inspect, type).
#[derive(Debug, Clone)]
#[allow(clippy::arc_with_non_send_sync)]
#[allow(clippy::new_without_default)]
pub struct MockInputBackend {
    state: std::sync::Arc<std::sync::Mutex<MockInputState>>,
}

/// State for mock backend
#[derive(Debug, Default)]
pub struct MockInputState {
    pub call_count: usize,
    pub last_error: Option<InputError>,
    pub should_succeed: bool,
    pub element: Option<uiautomation::UIElement>,
    pub last_x: Option<i32>,
    pub last_y: Option<i32>,
    pub last_key: Option<String>,
}

impl MockInputBackend {
    /// Creates a new mock backend with default state
    #[allow(clippy::new_without_default)]
    #[allow(clippy::arc_with_non_send_sync)]
    pub fn new() -> Self {
        Self {
            state: std::sync::Arc::new(std::sync::Mutex::new(MockInputState::default())),
        }
    }

    /// Creates a mock backend with custom state
    #[allow(clippy::arc_with_non_send_sync)]
    pub fn with_state(state: MockInputState) -> Self {
        Self {
            state: std::sync::Arc::new(std::sync::Mutex::new(state)),
        }
    }

    /// Gets a mutable reference to the state
    pub fn get_state(&self) -> std::sync::MutexGuard<'_, MockInputState> {
        self.state.lock().expect("Mock state mutex poisoned")
    }

    /// Resets the backend state
    pub fn reset(&self) {
        let mut state = self.get_state();
        state.call_count = 0;
        state.last_error = None;
        state.last_x = None;
        state.last_y = None;
        state.last_key = None;
    }
}

#[allow(clippy::arc_with_non_send_sync)]
#[async_trait::async_trait(?Send)]
impl InputBackend for MockInputBackend {
    async fn get_element_at_point(
        &self,
        x: i32,
        y: i32,
    ) -> Result<uiautomation::UIElement, InputError> {
        let mut state = self.get_state();
        state.call_count += 1;
        state.last_x = Some(x);
        state.last_y = Some(y);

        if state.should_succeed {
            if let Some(element) = state.element.clone() {
                state.last_error = None;
                Ok(element)
            } else {
                let error = InputError::ElementFromPointError("No element set".to_string());
                state.last_error = Some(error.clone());
                Err(error)
            }
        } else {
            let error = state
                .last_error
                .clone()
                .unwrap_or(InputError::ElementFromPointError(
                    "Unknown error".to_string(),
                ));
            state.last_error = Some(error.clone());
            Err(error)
        }
    }

    async fn move_mouse(&self, x: i32, y: i32) -> Result<(), InputError> {
        let mut state = self.get_state();
        state.call_count += 1;

        if state.should_succeed {
            state.last_x = Some(x);
            state.last_y = Some(y);
            state.last_error = None;
            Ok(())
        } else {
            let error = state
                .last_error
                .clone()
                .unwrap_or(InputError::MouseMoveError("Unknown error".to_string()));
            state.last_error = Some(error.clone());
            Err(error)
        }
    }

    async fn click_key(&self, key: &str) -> Result<(), InputError> {
        let mut state = self.get_state();
        state.call_count += 1;

        if state.should_succeed {
            state.last_key = Some(key.to_string());
            state.last_error = None;
            Ok(())
        } else {
            let error = state
                .last_error
                .clone()
                .unwrap_or(InputError::KeyClickError("Unknown error".to_string()));
            state.last_error = Some(error.clone());
            Err(error)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_input_config_valid() {
        let cancellation = CancellationToken::new();
        let config = InputConfig {
            timeout: Duration::from_secs(5),
            cancellation,
        };

        assert!(validate_input_config(&config).is_ok());
    }

    #[test]
    fn test_validate_input_config_zero_timeout() {
        let cancellation = CancellationToken::new();
        let config = InputConfig {
            timeout: Duration::ZERO,
            cancellation,
        };

        assert!(matches!(
            validate_input_config(&config),
            Err(InputError::InvalidConfig(_))
        ));
    }

    #[test]
    fn test_validate_input_config_large_timeout() {
        let cancellation = CancellationToken::new();
        let config = InputConfig {
            timeout: Duration::from_secs(3601), // > 1 hour
            cancellation,
        };

        assert!(matches!(
            validate_input_config(&config),
            Err(InputError::InvalidConfig(_))
        ));
    }

    #[test]
    fn test_mock_backend_creation() {
        let backend = MockInputBackend::new();
        assert_eq!(backend.get_state().call_count, 0);
    }

    #[test]
    fn test_mock_backend_with_state() {
        let state = MockInputState {
            should_succeed: true,
            ..Default::default()
        };
        let backend = MockInputBackend::with_state(state);
        assert_eq!(backend.get_state().should_succeed, true);
    }

    #[test]
    fn test_mock_backend_reset() {
        let backend = MockInputBackend::new();
        backend.reset();
        assert_eq!(backend.get_state().call_count, 0);
    }
}
