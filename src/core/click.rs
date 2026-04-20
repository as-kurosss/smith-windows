//! Click tool implementation
//! Provides UI element click functionality through UI Automation API.

use std::time::Duration;
use thiserror::Error;
use tokio_util::sync::CancellationToken;

use crate::runtime::backends::windows::click::ClickBackendWindows;

/// Configuration for click operations
#[derive(Debug, Clone)]
pub struct ClickConfig {
    /// Timeout for the click operation
    pub timeout: Duration,
    /// Token for cancellation
    pub cancellation: CancellationToken,
}

/// Errors that can occur during click operations
#[derive(Error, Debug, Clone)]
pub enum ClickError {
    /// Element not found or invalid
    #[error("Element not found")]
    ElementNotFound,
    /// Element is disabled
    #[error("Element is disabled")]
    ElementNotEnabled,
    /// Element is offscreen
    #[error("Element is offscreen")]
    ElementOffscreen,
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

/// Validates click configuration
/// Must be called BEFORE backend invocation
pub fn validate_click_config(config: &ClickConfig) -> Result<(), ClickError> {
    // Check timeout bounds: > 0 and <= 1 hour
    if config.timeout.is_zero() || config.timeout > Duration::from_secs(3600) {
        return Err(ClickError::InvalidConfig(
            "timeout must be > 0 and <= 1 hour".to_string(),
        ));
    }

    Ok(())
}

/// Trait for click backend implementations
#[async_trait::async_trait(?Send)]
pub trait ClickBackend {
    /// Performs a click on the given element
    async fn click(&self, element: &uiautomation::UIElement) -> Result<(), ClickError>;
}

/// Mock backend for testing
/// Uses internal state to simulate different scenarios
#[derive(Debug, Clone, Default)]
pub struct MockClickBackend {
    state: std::sync::Arc<std::sync::Mutex<MockClickState>>,
}

/// State for mock backend
#[derive(Debug, Default)]
pub struct MockClickState {
    pub call_count: usize,
    pub last_error: Option<ClickError>,
    pub should_succeed: bool,
}

impl MockClickBackend {
    /// Creates a new mock backend with default state
    pub fn new() -> Self {
        Self {
            state: std::sync::Arc::new(std::sync::Mutex::new(MockClickState::default())),
        }
    }

    /// Creates a mock backend with custom state
    pub fn with_state(state: MockClickState) -> Self {
        Self {
            state: std::sync::Arc::new(std::sync::Mutex::new(state)),
        }
    }

    /// Gets a mutable reference to the state
    pub fn get_state(&self) -> std::sync::MutexGuard<'_, MockClickState> {
        self.state.lock().expect("Mock state mutex poisoned")
    }

    /// Resets the backend state
    pub fn reset(&self) {
        let mut state = self.get_state();
        state.call_count = 0;
        state.last_error = None;
    }
}

#[async_trait::async_trait(?Send)]
impl ClickBackend for MockClickBackend {
    async fn click(&self, _element: &uiautomation::UIElement) -> Result<(), ClickError> {
        let mut state = self.get_state();
        state.call_count += 1;

        if state.should_succeed {
            state.last_error = None;
            Ok(())
        } else {
            let error = state
                .last_error
                .clone()
                .unwrap_or(ClickError::ElementNotFound);
            state.last_error = Some(error.clone());
            Err(error)
        }
    }
}

/// Performs a click operation with config validation and timeout handling
pub async fn click_with_config(
    element: &uiautomation::UIElement,
    config: &ClickConfig,
) -> Result<(), ClickError> {
    // Validate config BEFORE any backend calls
    validate_click_config(config)?;

    tracing::info!(
        "Starting click operation with timeout: {:?}",
        config.timeout
    );

    let backend = ClickBackendWindows::new();

    // Wrap with timeout and cancellation
    let click_future = async move { backend.click(element).await };

    // Wrap the future with timeout
    let result = tokio::time::timeout(config.timeout, click_future).await;

    match result {
        Ok(click_result) => {
            // Check for cancellation
            if config.cancellation.is_cancelled() {
                tracing::error!("Click operation cancelled during completion");
                return Err(ClickError::Cancelled);
            }
            click_result
        }
        Err(_) => {
            tracing::error!("Click operation timed out after {:?}", config.timeout);
            Err(ClickError::Timeout)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_click_config_valid() {
        let cancellation = CancellationToken::new();
        let config = ClickConfig {
            timeout: Duration::from_secs(5),
            cancellation,
        };

        assert!(validate_click_config(&config).is_ok());
    }

    #[test]
    fn test_validate_click_config_zero_timeout() {
        let cancellation = CancellationToken::new();
        let config = ClickConfig {
            timeout: Duration::ZERO,
            cancellation,
        };

        assert!(matches!(
            validate_click_config(&config),
            Err(ClickError::InvalidConfig(_))
        ));
    }

    #[test]
    fn test_validate_click_config_large_timeout() {
        let cancellation = CancellationToken::new();
        let config = ClickConfig {
            timeout: Duration::from_secs(3601), // > 1 hour
            cancellation,
        };

        assert!(matches!(
            validate_click_config(&config),
            Err(ClickError::InvalidConfig(_))
        ));
    }

    #[test]
    fn test_mock_backend_creation() {
        let backend = MockClickBackend::new();
        assert_eq!(backend.get_state().call_count, 0);
    }

    #[test]
    fn test_mock_backend_with_state() {
        let state = MockClickState {
            should_succeed: true,
            ..Default::default()
        };
        let backend = MockClickBackend::with_state(state);
        assert_eq!(backend.get_state().should_succeed, true);
    }

    #[test]
    fn test_mock_backend_reset() {
        let backend = MockClickBackend::new();
        backend.reset();
        assert_eq!(backend.get_state().call_count, 0);
    }
}
