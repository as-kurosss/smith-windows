//! Input text tool implementation
//! Provides UI element text input functionality through UI Automation API
//! (keyboard emulation, not clipboard + paste).

use std::time::Duration;
use thiserror::Error;
use tokio_util::sync::CancellationToken;

use crate::runtime::backends::windows::input_text::InputTextBackendWindows;

/// Configuration for input text operations
#[derive(Debug, Clone)]
pub struct InputTextConfig {
    /// Text to input into the element
    pub text: String,
    /// Timeout for the input text operation
    pub timeout: Duration,
    /// Token for cancellation
    pub cancellation: CancellationToken,
}

/// Errors that can occur during input text operations
#[derive(Error, Debug, Clone)]
pub enum InputTextError {
    /// Invalid input selector
    #[error("Invalid input selector: {0}")]
    InputSelectorError(String),
    /// Element not found
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
    ElementReadOnly,
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
    /// Input execution error
    #[error("Input execution error: {0}")]
    InputExecutionError(String),
}

/// Validates input selector
/// Must be called BEFORE backend invocation
pub fn validate_input_selector(selector: &str) -> Result<(), InputTextError> {
    if selector.is_empty() {
        return Err(InputTextError::InputSelectorError(
            "selector cannot be empty".to_string(),
        ));
    }

    if selector.len() > 2048 {
        return Err(InputTextError::InputSelectorError(
            "selector too long".to_string(),
        ));
    }

    Ok(())
}

/// Validates input text configuration
/// Must be called BEFORE backend invocation
pub fn validate_input_text_config(config: &InputTextConfig) -> Result<(), InputTextError> {
    // Check text bounds
    if config.text.is_empty() {
        return Err(InputTextError::InvalidConfig(
            "text cannot be empty".to_string(),
        ));
    }

    if config.text.len() > 65536 {
        return Err(InputTextError::InvalidConfig(
            "text too long (max 65536 characters)".to_string(),
        ));
    }

    // Check timeout bounds: > 0 and <= 1 hour
    if config.timeout.is_zero() || config.timeout > Duration::from_secs(3600) {
        return Err(InputTextError::InvalidConfig(
            "timeout must be > 0 and <= 1 hour".to_string(),
        ));
    }

    Ok(())
}

/// Trait for input text backend implementations
#[async_trait::async_trait(?Send)]
pub trait InputTextBackend {
    /// Inputs text on the given element
    async fn input_text(
        &self,
        element: &uiautomation::UIElement,
        keys: &str,
    ) -> Result<(), InputTextError>;
}

/// Mock backend for testing
/// Uses internal state to simulate different scenarios
#[derive(Debug, Clone, Default)]
pub struct MockInputTextBackend {
    state: std::sync::Arc<std::sync::Mutex<MockInputTextState>>,
}

/// State for mock backend
#[derive(Debug, Default)]
pub struct MockInputTextState {
    pub call_count: usize,
    pub last_error: Option<InputTextError>,
    pub should_succeed: bool,
    pub last_keys: Option<String>,
}

impl MockInputTextBackend {
    /// Creates a new mock backend with default state
    pub fn new() -> Self {
        Self {
            state: std::sync::Arc::new(std::sync::Mutex::new(MockInputTextState::default())),
        }
    }

    /// Creates a mock backend with custom state
    pub fn with_state(state: MockInputTextState) -> Self {
        Self {
            state: std::sync::Arc::new(std::sync::Mutex::new(state)),
        }
    }

    /// Gets a mutable reference to the state
    pub fn get_state(
        &self,
    ) -> Result<std::sync::MutexGuard<'_, MockInputTextState>, InputTextError> {
        self.state.lock().map_err(|e| {
            tracing::error!("State mutex poisoned: {}", e);
            InputTextError::ComError("State mutex poisoned".into())
        })
    }

    /// Resets the backend state
    pub fn reset(&self) -> Result<(), InputTextError> {
        let mut state = self.get_state()?;
        state.call_count = 0;
        state.last_error = None;
        state.last_keys = None;
        Ok(())
    }
}

#[async_trait::async_trait(?Send)]
impl InputTextBackend for MockInputTextBackend {
    async fn input_text(
        &self,
        _element: &uiautomation::UIElement,
        keys: &str,
    ) -> Result<(), InputTextError> {
        let mut state = self.get_state()?;
        state.call_count += 1;
        state.last_keys = Some(keys.to_string());

        if state.should_succeed {
            state.last_error = None;
            Ok(())
        } else {
            let error = state
                .last_error
                .clone()
                .unwrap_or(InputTextError::ElementNotFound);
            state.last_error = Some(error.clone());
            Err(error)
        }
    }
}

/// Validates element is ready for input
/// Reuses SetTextTool validation logic
pub fn validate_element_ready(element: &uiautomation::UIElement) -> Result<(), InputTextError> {
    // Check if element is enabled
    let enabled_result = element.is_enabled();
    let is_enabled = match enabled_result {
        Ok(val) => val,
        Err(e) => {
            tracing::error!("Failed to check if element is enabled: {}", e);
            return Err(InputTextError::ComError(e.to_string()));
        }
    };

    if !is_enabled {
        tracing::error!("Input text failed: element is disabled");
        return Err(InputTextError::ElementNotEnabled);
    }

    // Check if element is offscreen
    let offscreen_result = element.is_offscreen();
    let is_offscreen = match offscreen_result {
        Ok(val) => val,
        Err(e) => {
            tracing::error!("Failed to check if element is offscreen: {}", e);
            return Err(InputTextError::ComError(e.to_string()));
        }
    };

    if is_offscreen {
        tracing::error!("Input text failed: element is offscreen");
        return Err(InputTextError::ElementOffscreen);
    }

    // Check if element is read-only (if available)
    if let Ok(_value_pattern) = element.get_pattern::<uiautomation::patterns::UIValuePattern>() {
        // Note: uiautomation 0.24.4 doesn't expose is_read_only directly
        // We'll check if set_value works - if it returns an error, element is read-only
        // For validation purposes, we'll assume writable if ValuePattern exists
        // Real read-only check would require testing set_value
    }

    Ok(())
}

/// Performs an input text operation with config validation and cancellation check
/// Note: UIElement is !Send, so we cannot use spawn_blocking or async move.
/// The backend call runs on the same thread that created the UIAutomation instance.
pub async fn input_text_with_config(
    element: &uiautomation::UIElement,
    text: &str,
    config: &InputTextConfig,
) -> Result<(), InputTextError> {
    // Validate config BEFORE any backend calls
    validate_input_text_config(config)?;

    // Validate element is ready
    validate_element_ready(element)?;

    // Validate text is not empty (double-check, though config validation covers this)
    if text.is_empty() {
        return Err(InputTextError::InvalidConfig(
            "text cannot be empty".to_string(),
        ));
    }

    tracing::info!("Starting input text operation, text: {}", text);

    if config.cancellation.is_cancelled() {
        tracing::error!("Input text operation cancelled before completion");
        return Err(InputTextError::Cancelled);
    }

    let backend = InputTextBackendWindows::new();

    let result = backend.input_text(element, text).await;

    if config.cancellation.is_cancelled() {
        tracing::error!("Input text operation cancelled during completion");
        return Err(InputTextError::Cancelled);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_input_text_config_valid() {
        let cancellation = CancellationToken::new();
        let config = InputTextConfig {
            text: "Hello".to_string(),
            timeout: Duration::from_secs(5),
            cancellation,
        };

        assert!(validate_input_text_config(&config).is_ok());
    }

    #[test]
    fn test_validate_input_text_config_empty_text() {
        let cancellation = CancellationToken::new();
        let config = InputTextConfig {
            text: "".to_string(),
            timeout: Duration::from_secs(5),
            cancellation,
        };

        assert!(matches!(
            validate_input_text_config(&config),
            Err(InputTextError::InvalidConfig(_))
        ));
    }

    #[test]
    fn test_validate_input_text_config_large_text() {
        let cancellation = CancellationToken::new();
        let config = InputTextConfig {
            text: "a".repeat(65537),
            timeout: Duration::from_secs(5),
            cancellation,
        };

        assert!(matches!(
            validate_input_text_config(&config),
            Err(InputTextError::InvalidConfig(_))
        ));
    }

    #[test]
    fn test_validate_input_text_config_zero_timeout() {
        let cancellation = CancellationToken::new();
        let config = InputTextConfig {
            text: "Hello".to_string(),
            timeout: Duration::ZERO,
            cancellation,
        };

        assert!(matches!(
            validate_input_text_config(&config),
            Err(InputTextError::InvalidConfig(_))
        ));
    }

    #[test]
    fn test_validate_input_text_config_large_timeout() {
        let cancellation = CancellationToken::new();
        let config = InputTextConfig {
            text: "Hello".to_string(),
            timeout: Duration::from_secs(3601),
            cancellation,
        };

        assert!(matches!(
            validate_input_text_config(&config),
            Err(InputTextError::InvalidConfig(_))
        ));
    }

    #[test]
    fn test_validate_input_selector_valid() {
        assert!(validate_input_selector("#editbox").is_ok());
    }

    #[test]
    fn test_validate_input_selector_empty() {
        assert!(matches!(
            validate_input_selector(""),
            Err(InputTextError::InputSelectorError(_))
        ));
    }

    #[test]
    fn test_validate_input_selector_too_long() {
        let long_selector = "a".repeat(2049);
        assert!(matches!(
            validate_input_selector(&long_selector),
            Err(InputTextError::InputSelectorError(_))
        ));
    }

    #[test]
    fn test_mock_backend_creation() {
        let backend = MockInputTextBackend::new();
        assert_eq!(backend.get_state().unwrap().call_count, 0);
    }

    #[test]
    fn test_mock_backend_with_state() {
        let state = MockInputTextState {
            should_succeed: true,
            ..Default::default()
        };
        let backend = MockInputTextBackend::with_state(state);
        assert!(backend.get_state().unwrap().should_succeed);
    }

    #[test]
    fn test_mock_backend_reset() {
        let backend = MockInputTextBackend::new();
        backend.reset().unwrap();
        assert_eq!(backend.get_state().unwrap().call_count, 0);
    }

    #[test]
    fn test_mock_backend_last_keys() {
        let state = MockInputTextState {
            should_succeed: true,
            ..Default::default()
        };
        let backend = MockInputTextBackend::with_state(state);

        // Mock doesn't actually use element, so we can skip passing it in async context
        // For testing, we just verify the state is updated correctly
        // The actual element is not used in mock implementation

        let backend_for_test = backend.clone();
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(async move {
            // Create a dummy element from UIAutomation
            let automation = uiautomation::UIAutomation::new().unwrap();
            let element = automation.get_root_element().unwrap();
            let _ = backend_for_test.input_text(&element, "test keys").await;
        });

        assert_eq!(
            backend.get_state().unwrap().last_keys,
            Some("test keys".to_string())
        );
    }
}
