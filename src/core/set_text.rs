//! Set text tool implementation
//! Provides UI element text setting functionality through UI Automation API.

use std::time::Duration;
use thiserror::Error;
use tokio_util::sync::CancellationToken;

use crate::runtime::backends::windows::set_text::SetTextBackendWindows;

/// Configuration for set text operations
#[derive(Debug, Clone)]
pub struct SetTextConfig {
    /// Timeout for the set text operation
    pub timeout: Duration,
    /// Token for cancellation
    pub cancellation: CancellationToken,
}

/// Errors that can occur during set text operations
#[derive(Error, Debug, Clone)]
pub enum SetTextError {
    /// Element not found or invalid
    #[error("Element not found")]
    ElementNotFound,
    /// Element is disabled
    #[error("Element is disabled")]
    ElementNotEnabled,
    /// Element is offscreen
    #[error("Element is offscreen")]
    ElementOffscreen,
    /// Element is read-only
    #[error("Element is read-only")]
    ElementNotWritable,
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

/// Validates set text configuration
/// Must be called BEFORE backend invocation
pub fn validate_set_text_config(config: &SetTextConfig) -> Result<(), SetTextError> {
    // Check timeout bounds: > 0 and <= 1 hour
    if config.timeout.is_zero() || config.timeout > Duration::from_secs(3600) {
        return Err(SetTextError::InvalidConfig(
            "timeout must be > 0 and <= 1 hour".to_string(),
        ));
    }

    Ok(())
}

/// Trait for set text backend implementations
#[async_trait::async_trait(?Send)]
pub trait SetTextBackend {
    /// Sets text on the given element
    async fn set_text(
        &self,
        element: &uiautomation::UIElement,
        text: &str,
    ) -> Result<(), SetTextError>;
}

/// Mock backend for testing
/// Uses internal state to simulate different scenarios
#[derive(Debug, Clone, Default)]
pub struct MockSetTextBackend {
    state: std::sync::Arc<std::sync::Mutex<MockSetTextState>>,
}

/// State for mock backend
#[derive(Debug, Default)]
pub struct MockSetTextState {
    pub call_count: usize,
    pub last_error: Option<SetTextError>,
    pub should_succeed: bool,
}

impl MockSetTextBackend {
    /// Creates a new mock backend with default state
    pub fn new() -> Self {
        Self {
            state: std::sync::Arc::new(std::sync::Mutex::new(MockSetTextState::default())),
        }
    }

    /// Creates a mock backend with custom state
    pub fn with_state(state: MockSetTextState) -> Self {
        Self {
            state: std::sync::Arc::new(std::sync::Mutex::new(state)),
        }
    }

    /// Gets a mutable reference to the state
    pub fn get_state(&self) -> Result<std::sync::MutexGuard<'_, MockSetTextState>, SetTextError> {
        self.state.lock().map_err(|e| {
            tracing::error!("State mutex poisoned: {}", e);
            SetTextError::ComError("State mutex poisoned".into())
        })
    }

    /// Resets the backend state
    pub fn reset(&self) -> Result<(), SetTextError> {
        let mut state = self.get_state()?;
        state.call_count = 0;
        state.last_error = None;
        Ok(())
    }
}

#[async_trait::async_trait(?Send)]
impl SetTextBackend for MockSetTextBackend {
    async fn set_text(
        &self,
        _element: &uiautomation::UIElement,
        _text: &str,
    ) -> Result<(), SetTextError> {
        let mut state = self.get_state()?;
        state.call_count += 1;

        if state.should_succeed {
            state.last_error = None;
            Ok(())
        } else {
            let error = state
                .last_error
                .clone()
                .unwrap_or(SetTextError::ElementNotFound);
            state.last_error = Some(error.clone());
            Err(error)
        }
    }
}

/// Performs a set text operation with config validation and cancellation check
/// Note: UIElement is !Send, so we cannot use spawn_blocking or async move.
/// The backend call must run on the same thread that created the UIAutomation instance.
pub async fn set_text_with_config(
    element: &uiautomation::UIElement,
    text: &str,
    config: &SetTextConfig,
) -> Result<(), SetTextError> {
    // Validate config BEFORE any backend calls
    validate_set_text_config(config)?;

    if text.is_empty() {
        return Err(SetTextError::InvalidConfig(
            "text cannot be empty".to_string(),
        ));
    }

    tracing::info!("Starting set text operation, text: {}", text);

    if config.cancellation.is_cancelled() {
        tracing::error!("Set text operation cancelled before completion");
        return Err(SetTextError::Cancelled);
    }

    let backend = SetTextBackendWindows::new();

    let result = backend.set_text(element, text).await;

    if config.cancellation.is_cancelled() {
        tracing::error!("Set text operation cancelled during completion");
        return Err(SetTextError::Cancelled);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_set_text_config_valid() {
        let cancellation = CancellationToken::new();
        let config = SetTextConfig {
            timeout: Duration::from_secs(5),
            cancellation,
        };

        assert!(validate_set_text_config(&config).is_ok());
    }

    #[test]
    fn test_validate_set_text_config_zero_timeout() {
        let cancellation = CancellationToken::new();
        let config = SetTextConfig {
            timeout: Duration::ZERO,
            cancellation,
        };

        assert!(matches!(
            validate_set_text_config(&config),
            Err(SetTextError::InvalidConfig(_))
        ));
    }

    #[test]
    fn test_validate_set_text_config_large_timeout() {
        let cancellation = CancellationToken::new();
        let config = SetTextConfig {
            timeout: Duration::from_secs(3601), // > 1 hour
            cancellation,
        };

        assert!(matches!(
            validate_set_text_config(&config),
            Err(SetTextError::InvalidConfig(_))
        ));
    }

    #[test]
    fn test_mock_backend_creation() {
        let backend = MockSetTextBackend::new();
        assert_eq!(backend.get_state().unwrap().call_count, 0);
    }

    #[test]
    fn test_mock_backend_with_state() {
        let state = MockSetTextState {
            should_succeed: true,
            ..Default::default()
        };
        let backend = MockSetTextBackend::with_state(state);
        assert!(backend.get_state().unwrap().should_succeed);
    }

    #[test]
    fn test_mock_backend_reset() {
        let backend = MockSetTextBackend::new();
        backend.reset().unwrap();
        assert_eq!(backend.get_state().unwrap().call_count, 0);
    }
}
