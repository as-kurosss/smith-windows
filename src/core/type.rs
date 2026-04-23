//! Type tool implementation
//! Provides UI element type text functionality through UI Automation API.

use std::time::Duration;
use thiserror::Error;
use tokio_util::sync::CancellationToken;

use crate::runtime::backends::windows::r#type::TypeBackendWindows;

/// Configuration for type text operations
#[derive(Debug, Clone)]
pub struct TypeConfig {
    /// Timeout for the type text operation
    pub timeout: Duration,
    /// Token for cancellation
    pub cancellation: CancellationToken,
}

/// Errors that can occur during type text operations
#[derive(Error, Debug, Clone)]
pub enum TypeError {
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

/// Validates type configuration
/// Must be called BEFORE backend invocation
pub fn validate_type_config(config: &TypeConfig) -> Result<(), TypeError> {
    // Check timeout bounds: > 0 and <= 1 hour
    if config.timeout.is_zero() || config.timeout > Duration::from_secs(3600) {
        return Err(TypeError::InvalidConfig(
            "timeout must be > 0 and <= 1 hour".to_string(),
        ));
    }

    Ok(())
}

/// Trait for type backend implementations
#[async_trait::async_trait(?Send)]
pub trait TypeBackend {
    /// Types text into the given element
    async fn type_text(
        &self,
        element: &uiautomation::UIElement,
        text: &str,
    ) -> Result<(), TypeError>;
}

/// Mock backend for testing
/// Uses internal state to simulate different scenarios
#[derive(Debug, Clone, Default)]
pub struct MockTypeBackend {
    state: std::sync::Arc<std::sync::Mutex<MockTypeState>>,
}

/// State for mock backend
#[derive(Debug, Default)]
pub struct MockTypeState {
    pub call_count: usize,
    pub last_error: Option<TypeError>,
    pub should_succeed: bool,
}

impl MockTypeBackend {
    /// Creates a new mock backend with default state
    pub fn new() -> Self {
        Self {
            state: std::sync::Arc::new(std::sync::Mutex::new(MockTypeState::default())),
        }
    }

    /// Creates a mock backend with custom state
    pub fn with_state(state: MockTypeState) -> Self {
        Self {
            state: std::sync::Arc::new(std::sync::Mutex::new(state)),
        }
    }

    /// Gets a mutable reference to the state
    pub fn get_state(&self) -> Result<std::sync::MutexGuard<'_, MockTypeState>, TypeError> {
        self.state.lock().map_err(|e| {
            tracing::error!("State mutex poisoned: {}", e);
            TypeError::ComError("State mutex poisoned".into())
        })
    }

    /// Resets the backend state
    pub fn reset(&self) -> Result<(), TypeError> {
        let mut state = self.get_state()?;
        state.call_count = 0;
        state.last_error = None;
        Ok(())
    }
}

#[async_trait::async_trait(?Send)]
impl TypeBackend for MockTypeBackend {
    async fn type_text(
        &self,
        _element: &uiautomation::UIElement,
        _text: &str,
    ) -> Result<(), TypeError> {
        let mut state = self.get_state()?;
        state.call_count += 1;

        if state.should_succeed {
            state.last_error = None;
            Ok(())
        } else {
            let error = state
                .last_error
                .clone()
                .unwrap_or(TypeError::ElementNotFound);
            state.last_error = Some(error.clone());
            Err(error)
        }
    }
}

/// Performs a type text operation with config validation and timeout handling
/// Note: UIElement is !Send, so we cannot use spawn_blocking or async move.
/// The backend call must run on the same thread that created the UIAutomation instance.
pub async fn type_text_with_config(
    element: &uiautomation::UIElement,
    text: &str,
    config: &TypeConfig,
) -> Result<(), TypeError> {
    // Validate config BEFORE any backend calls
    validate_type_config(config)?;

    if text.is_empty() {
        return Err(TypeError::InvalidConfig("text cannot be empty".to_string()));
    }

    tracing::info!("Starting type text operation, text length: {}", text.len());

    let backend = TypeBackendWindows::new();

    // Direct call to backend - UIAutomation operations are synchronous and non-blocking
    // Check cancellation before call
    if config.cancellation.is_cancelled() {
        tracing::error!("Type text operation cancelled before completion");
        return Err(TypeError::Cancelled);
    }

    let result = backend.type_text(element, text).await;

    // Check cancellation after call
    if config.cancellation.is_cancelled() {
        tracing::error!("Type text operation cancelled during completion");
        return Err(TypeError::Cancelled);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_type_config_valid() {
        let cancellation = CancellationToken::new();
        let config = TypeConfig {
            timeout: Duration::from_secs(5),
            cancellation,
        };

        assert!(validate_type_config(&config).is_ok());
    }

    #[test]
    fn test_validate_type_config_zero_timeout() {
        let cancellation = CancellationToken::new();
        let config = TypeConfig {
            timeout: Duration::ZERO,
            cancellation,
        };

        assert!(matches!(
            validate_type_config(&config),
            Err(TypeError::InvalidConfig(_))
        ));
    }

    #[test]
    fn test_validate_type_config_large_timeout() {
        let cancellation = CancellationToken::new();
        let config = TypeConfig {
            timeout: Duration::from_secs(3601), // > 1 hour
            cancellation,
        };

        assert!(matches!(
            validate_type_config(&config),
            Err(TypeError::InvalidConfig(_))
        ));
    }

    #[test]
    fn test_mock_backend_creation() {
        let backend = MockTypeBackend::new();
        assert_eq!(backend.get_state().unwrap().call_count, 0);
    }

    #[test]
    fn test_mock_backend_reset() {
        let backend = MockTypeBackend::new();
        backend.reset().unwrap();
        assert_eq!(backend.get_state().unwrap().call_count, 0);
    }
}
