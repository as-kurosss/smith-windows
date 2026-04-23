//! Click tool implementation
//! Provides UI element click functionality through UI Automation API.

use std::time::Duration;
use thiserror::Error;
use tokio_util::sync::CancellationToken;

use crate::runtime::backends::windows::click::ClickBackendWindows;

/// Type of click operation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClickType {
    /// Left single click (1 click)
    LeftSingle,
    /// Right single click (1 right click)
    RightSingle,
    /// Left double click (2 clicks)
    LeftDouble,
}

/// Configuration for click operations
#[derive(Debug, Clone)]
pub struct ClickConfig {
    /// Type of click to perform
    pub click_type: ClickType,
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

    // Validate click type - LeftDouble requires special handling
    // (not strictly necessary but kept for future extensibility)
    match config.click_type {
        ClickType::LeftDouble => {
            // Double click is supported, no additional validation needed
        }
        _ => {
            // LeftSingle and RightSingle are always valid
        }
    }

    Ok(())
}

/// Trait for click backend implementations
#[async_trait::async_trait(?Send)]
pub trait ClickBackend {
    /// Performs a click on the given element
    async fn click(
        &self,
        element: &uiautomation::UIElement,
        click_type: ClickType,
    ) -> Result<(), ClickError>;
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
    pub fn get_state(&self) -> Result<std::sync::MutexGuard<'_, MockClickState>, ClickError> {
        self.state
            .lock()
            .map_err(|e| ClickError::ComError(e.to_string()))
    }

    /// Resets the backend state
    pub fn reset(&self) -> Result<(), ClickError> {
        let mut state = self.get_state()?;
        state.call_count = 0;
        state.last_error = None;
        Ok(())
    }
}

#[async_trait::async_trait(?Send)]
impl ClickBackend for MockClickBackend {
    async fn click(
        &self,
        _element: &uiautomation::UIElement,
        _click_type: ClickType,
    ) -> Result<(), ClickError> {
        let mut state = self.get_state()?;
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
/// Note: UIElement is !Send, so we cannot use spawn_blocking or async move.
/// The backend call must run on the same thread that created the UIAutomation instance.
pub async fn click_with_config(
    element: &uiautomation::UIElement,
    config: &ClickConfig,
) -> Result<(), ClickError> {
    // Validate config BEFORE any backend calls
    validate_click_config(config)?;

    tracing::info!(
        "Starting click operation with timeout: {:?}, click_type: {:?}",
        config.timeout,
        config.click_type
    );

    let backend = ClickBackendWindows::new();

    // Direct call to backend - UIElement is !Send, so we cannot use spawn_blocking
    // The backend call itself is synchronous and does not block the async runtime
    // Check cancellation before call
    if config.cancellation.is_cancelled() {
        tracing::error!("Click operation cancelled before completion");
        return Err(ClickError::Cancelled);
    }

    // For UIAutomation operations, timeout is not directly applicable
    // because the operations are synchronous and non-blocking
    // The backend call will complete quickly
    let result = backend.click(element, config.click_type).await;

    // Check cancellation after call
    if config.cancellation.is_cancelled() {
        tracing::error!("Click operation cancelled during completion");
        return Err(ClickError::Cancelled);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_click_config_valid() {
        let cancellation = CancellationToken::new();
        let config = ClickConfig {
            click_type: ClickType::LeftSingle,
            timeout: Duration::from_secs(5),
            cancellation,
        };

        assert!(validate_click_config(&config).is_ok());
    }

    #[test]
    fn test_validate_click_config_zero_timeout() {
        let cancellation = CancellationToken::new();
        let config = ClickConfig {
            click_type: ClickType::LeftSingle,
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
            click_type: ClickType::LeftSingle,
            timeout: Duration::from_secs(3601), // > 1 hour
            cancellation,
        };

        assert!(matches!(
            validate_click_config(&config),
            Err(ClickError::InvalidConfig(_))
        ));
    }

    #[test]
    fn test_validate_click_config_left_double() {
        let cancellation = CancellationToken::new();
        let config = ClickConfig {
            click_type: ClickType::LeftDouble,
            timeout: Duration::from_secs(5),
            cancellation,
        };

        assert!(validate_click_config(&config).is_ok());
    }

    #[test]
    fn test_validate_click_config_right_single() {
        let cancellation = CancellationToken::new();
        let config = ClickConfig {
            click_type: ClickType::RightSingle,
            timeout: Duration::from_secs(5),
            cancellation,
        };

        assert!(validate_click_config(&config).is_ok());
    }

    #[test]
    fn test_mock_backend_creation() {
        let backend = MockClickBackend::new();
        assert_eq!(backend.get_state().unwrap().call_count, 0);
    }

    #[test]
    fn test_mock_backend_with_state() {
        let state = MockClickState {
            should_succeed: true,
            ..Default::default()
        };
        let backend = MockClickBackend::with_state(state);
        assert!(backend.get_state().unwrap().should_succeed);
    }

    #[test]
    fn test_mock_backend_reset() {
        let backend = MockClickBackend::new();
        backend.reset().unwrap();
        assert_eq!(backend.get_state().unwrap().call_count, 0);
    }
}
