//! Read tool implementation
//! Provides UI element text reading functionality through UI Automation API.

use std::time::Duration;
use thiserror::Error;
use tokio_util::sync::CancellationToken;

use crate::runtime::backends::windows::read::ReadBackendWindows;

/// Configuration for read operations
#[derive(Debug, Clone)]
pub struct ReadConfig {
    /// Timeout for the read operation
    pub timeout: Duration,
    /// Token for cancellation
    pub cancellation: CancellationToken,
}

/// Errors that can occur during read operations
#[derive(Error, Debug, Clone)]
pub enum ReadError {
    /// Element not found or invalid
    #[error("Element not found")]
    ElementNotFound,
    /// Element is disabled
    #[error("Element is disabled")]
    ElementNotEnabled,
    /// Element is offscreen
    #[error("Element is offscreen")]
    ElementOffscreen,
    /// Element does not support text reading
    #[error("Element does not support text reading")]
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
    /// Unsupported platform
    #[error("Unsupported platform")]
    UnsupportedPlatform,
}

/// Validates read configuration
/// Must be called BEFORE backend invocation
pub fn validate_read_config(config: &ReadConfig) -> Result<(), ReadError> {
    // Check timeout bounds: > 0 and <= 1 hour
    if config.timeout.is_zero() || config.timeout > Duration::from_secs(3600) {
        return Err(ReadError::InvalidConfig(
            "timeout must be > 0 and <= 1 hour".to_string(),
        ));
    }

    Ok(())
}

/// Trait for read backend implementations
#[async_trait::async_trait(?Send)]
pub trait ReadBackend {
    /// Reads text from the given element
    async fn read_text(&self, element: &uiautomation::UIElement) -> Result<String, ReadError>;
}

/// Mock backend for testing
/// Uses internal state to simulate different scenarios
#[derive(Debug, Clone, Default)]
pub struct MockReadBackend {
    state: std::sync::Arc<std::sync::Mutex<MockReadState>>,
}

/// State for mock backend
#[derive(Debug, Default)]
pub struct MockReadState {
    pub call_count: usize,
    pub last_error: Option<ReadError>,
    pub should_succeed: bool,
    pub returned_text: String,
}

impl MockReadBackend {
    /// Creates a new mock backend with default state
    pub fn new() -> Self {
        Self {
            state: std::sync::Arc::new(std::sync::Mutex::new(MockReadState::default())),
        }
    }

    /// Creates a mock backend with custom state
    pub fn with_state(state: MockReadState) -> Self {
        Self {
            state: std::sync::Arc::new(std::sync::Mutex::new(state)),
        }
    }

    /// Gets a mutable reference to the state
    pub fn get_state(&self) -> Result<std::sync::MutexGuard<'_, MockReadState>, ReadError> {
        self.state.lock().map_err(|e| {
            tracing::error!("State mutex poisoned: {}", e);
            ReadError::ComError("State mutex poisoned".into())
        })
    }

    /// Resets the backend state
    pub fn reset(&self) -> Result<(), ReadError> {
        let mut state = self.get_state()?;
        state.call_count = 0;
        state.last_error = None;
        state.returned_text.clear();
        Ok(())
    }
}

#[async_trait::async_trait(?Send)]
impl ReadBackend for MockReadBackend {
    async fn read_text(&self, _element: &uiautomation::UIElement) -> Result<String, ReadError> {
        let mut state = self.get_state()?;
        state.call_count += 1;

        if state.should_succeed {
            let text = state.returned_text.clone();
            state.last_error = None;
            Ok(text)
        } else {
            let error = state
                .last_error
                .clone()
                .unwrap_or(ReadError::ElementNotFound);
            state.last_error = Some(error.clone());
            Err(error)
        }
    }
}

/// Performs a read operation with config validation and cancellation check
/// Note: UIElement is !Send, so we cannot use spawn_blocking or async move.
/// The backend call must run on the same thread that created the UIAutomation instance.
pub async fn read_text_with_config(
    element: &uiautomation::UIElement,
    config: &ReadConfig,
) -> Result<String, ReadError> {
    // Validate config BEFORE any backend calls
    validate_read_config(config)?;

    tracing::info!("Starting read operation");

    if config.cancellation.is_cancelled() {
        tracing::error!("Read operation cancelled before completion");
        return Err(ReadError::Cancelled);
    }

    let backend = ReadBackendWindows::new();

    let result = backend.read_text(element).await;

    if config.cancellation.is_cancelled() {
        tracing::error!("Read operation cancelled during completion");
        return Err(ReadError::Cancelled);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_read_config_valid() {
        let cancellation = CancellationToken::new();
        let config = ReadConfig {
            timeout: Duration::from_secs(5),
            cancellation,
        };

        assert!(validate_read_config(&config).is_ok());
    }

    #[test]
    fn test_validate_read_config_zero_timeout() {
        let cancellation = CancellationToken::new();
        let config = ReadConfig {
            timeout: Duration::ZERO,
            cancellation,
        };

        assert!(matches!(
            validate_read_config(&config),
            Err(ReadError::InvalidConfig(_))
        ));
    }

    #[test]
    fn test_validate_read_config_large_timeout() {
        let cancellation = CancellationToken::new();
        let config = ReadConfig {
            timeout: Duration::from_secs(3601), // > 1 hour
            cancellation,
        };

        assert!(matches!(
            validate_read_config(&config),
            Err(ReadError::InvalidConfig(_))
        ));
    }

    #[test]
    fn test_mock_backend_creation() {
        let backend = MockReadBackend::new();
        assert_eq!(backend.get_state().unwrap().call_count, 0);
    }

    #[test]
    fn test_mock_backend_with_state() {
        let state = MockReadState {
            should_succeed: true,
            ..Default::default()
        };
        let backend = MockReadBackend::with_state(state);
        assert!(backend.get_state().unwrap().should_succeed);
    }

    #[test]
    fn test_mock_backend_reset() {
        let backend = MockReadBackend::new();
        backend.reset().unwrap();
        assert_eq!(backend.get_state().unwrap().call_count, 0);
    }

    #[test]
    fn test_mock_backend_with_custom_text() {
        let state = MockReadState {
            should_succeed: true,
            returned_text: "Custom text".to_string(),
            ..Default::default()
        };
        let backend = MockReadBackend::with_state(state);
        assert_eq!(backend.get_state().unwrap().returned_text, "Custom text");
    }
}
