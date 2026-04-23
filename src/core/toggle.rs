//! Toggle tool implementation
//! Provides UI element toggle functionality (checkboxes, radio buttons, toggle switches) through UI Automation API.

use std::time::Duration;
use thiserror::Error;
use tokio_util::sync::CancellationToken;

use crate::runtime::backends::windows::toggle::ToggleBackendWindows;

/// Configuration for toggle operations
#[derive(Debug, Clone)]
pub struct ToggleConfig {
    /// Timeout for the toggle operation
    pub timeout: Duration,
    /// Token for cancellation
    pub cancellation: CancellationToken,
}

/// Errors that can occur during toggle operations
#[derive(Error, Debug, Clone)]
pub enum ToggleError {
    /// Element not found or invalid
    #[error("Element not found")]
    ElementNotFound,
    /// Element is disabled
    #[error("Element is disabled")]
    ElementNotEnabled,
    /// Element is offscreen
    #[error("Element is offscreen")]
    ElementOffscreen,
    /// Element does not support toggle pattern
    #[error("Element does not support toggle pattern")]
    ElementNotSupported,
    /// Element value pattern is read-only
    #[error("Element value pattern is read-only")]
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

/// Validates toggle configuration
/// Must be called BEFORE backend invocation
pub fn validate_toggle_config(config: &ToggleConfig) -> Result<(), ToggleError> {
    // Check timeout bounds: > 0 and <= 1 hour (3600 seconds)
    if config.timeout.is_zero() || config.timeout > Duration::from_secs(3600) {
        return Err(ToggleError::InvalidConfig(
            "timeout must be > 0 and <= 1 hour".to_string(),
        ));
    }

    Ok(())
}

/// Trait for toggle backend implementations
#[async_trait::async_trait(?Send)]
pub trait ToggleBackend {
    /// Toggles the element state (checkbox, toggle switch)
    async fn toggle_element(&self, element: &uiautomation::UIElement) -> Result<(), ToggleError>;

    /// Sets radio button selected state
    async fn set_radio(
        &self,
        element: &uiautomation::UIElement,
        selected: bool,
    ) -> Result<(), ToggleError>;

    /// Sets toggle switch state
    async fn set_toggle(
        &self,
        element: &uiautomation::UIElement,
        state: bool,
    ) -> Result<(), ToggleError>;

    /// Checks if element is checked (checkbox)
    async fn is_checked(&self, element: &uiautomation::UIElement) -> Result<bool, ToggleError>;

    /// Checks if element is selected (radio button)
    async fn is_selected(&self, element: &uiautomation::UIElement) -> Result<bool, ToggleError>;
}

/// Mock backend for testing
/// Uses internal state to simulate different scenarios
#[derive(Debug, Clone, Default)]
pub struct MockToggleBackend {
    state: std::sync::Arc<std::sync::Mutex<MockToggleState>>,
}

/// State for mock backend
#[derive(Debug, Default)]
pub struct MockToggleState {
    pub call_count: usize,
    pub last_error: Option<ToggleError>,
    pub should_succeed: bool,
}

impl MockToggleBackend {
    /// Creates a new mock backend with default state
    pub fn new() -> Self {
        Self {
            state: std::sync::Arc::new(std::sync::Mutex::new(MockToggleState::default())),
        }
    }

    /// Creates a mock backend with custom state
    pub fn with_state(state: MockToggleState) -> Self {
        Self {
            state: std::sync::Arc::new(std::sync::Mutex::new(state)),
        }
    }

    /// Gets a mutable reference to the state
    pub fn get_state(&self) -> Result<std::sync::MutexGuard<'_, MockToggleState>, ToggleError> {
        self.state
            .lock()
            .map_err(|e| ToggleError::ComError(e.to_string()))
    }

    /// Resets the backend state
    pub fn reset(&self) -> Result<(), ToggleError> {
        let mut state = self.get_state()?;
        state.call_count = 0;
        state.last_error = None;
        Ok(())
    }
}

#[async_trait::async_trait(?Send)]
impl ToggleBackend for MockToggleBackend {
    async fn toggle_element(&self, _element: &uiautomation::UIElement) -> Result<(), ToggleError> {
        let mut state = self.get_state()?;
        state.call_count += 1;

        if state.should_succeed {
            state.last_error = None;
            Ok(())
        } else {
            let error = state
                .last_error
                .clone()
                .unwrap_or(ToggleError::ElementNotFound);
            state.last_error = Some(error.clone());
            Err(error)
        }
    }

    async fn set_radio(
        &self,
        _element: &uiautomation::UIElement,
        _selected: bool,
    ) -> Result<(), ToggleError> {
        let mut state = self.get_state()?;
        state.call_count += 1;

        if state.should_succeed {
            state.last_error = None;
            Ok(())
        } else {
            let error = state
                .last_error
                .clone()
                .unwrap_or(ToggleError::ElementNotFound);
            state.last_error = Some(error.clone());
            Err(error)
        }
    }

    async fn set_toggle(
        &self,
        _element: &uiautomation::UIElement,
        _state: bool,
    ) -> Result<(), ToggleError> {
        let mut state = self.get_state()?;
        state.call_count += 1;

        if state.should_succeed {
            state.last_error = None;
            Ok(())
        } else {
            let error = state
                .last_error
                .clone()
                .unwrap_or(ToggleError::ElementNotFound);
            state.last_error = Some(error.clone());
            Err(error)
        }
    }

    async fn is_checked(&self, _element: &uiautomation::UIElement) -> Result<bool, ToggleError> {
        let mut state = self.get_state()?;
        state.call_count += 1;

        if state.should_succeed {
            state.last_error = None;
            Ok(true)
        } else {
            let error = state
                .last_error
                .clone()
                .unwrap_or(ToggleError::ElementNotFound);
            state.last_error = Some(error.clone());
            Err(error)
        }
    }

    async fn is_selected(&self, _element: &uiautomation::UIElement) -> Result<bool, ToggleError> {
        let mut state = self.get_state()?;
        state.call_count += 1;

        if state.should_succeed {
            state.last_error = None;
            Ok(true)
        } else {
            let error = state
                .last_error
                .clone()
                .unwrap_or(ToggleError::ElementNotFound);
            state.last_error = Some(error.clone());
            Err(error)
        }
    }
}

/// Performs a toggle element operation with config validation
pub async fn toggle_element_with_config(
    element: &uiautomation::UIElement,
    config: &ToggleConfig,
) -> Result<(), ToggleError> {
    // Validate config BEFORE any backend calls
    validate_toggle_config(config)?;

    tracing::info!("Starting toggle element operation");

    let backend = ToggleBackendWindows::new();

    // Direct call to backend - UIAutomation operations are synchronous and non-blocking
    // Check cancellation before call
    if config.cancellation.is_cancelled() {
        tracing::error!("Toggle element operation cancelled before completion");
        return Err(ToggleError::Cancelled);
    }

    let result = backend.toggle_element(element).await;

    // Check cancellation after call
    if config.cancellation.is_cancelled() {
        tracing::error!("Toggle element operation cancelled during completion");
        return Err(ToggleError::Cancelled);
    }

    result
}

/// Performs a set radio button operation with config validation
pub async fn set_radio_with_config(
    element: &uiautomation::UIElement,
    selected: bool,
    config: &ToggleConfig,
) -> Result<(), ToggleError> {
    // Validate config BEFORE any backend calls
    validate_toggle_config(config)?;

    tracing::info!("Starting set radio operation, selected: {}", selected);

    let backend = ToggleBackendWindows::new();

    // Direct call to backend - UIAutomation operations are synchronous and non-blocking
    // Check cancellation before call
    if config.cancellation.is_cancelled() {
        tracing::error!("Set radio operation cancelled before completion");
        return Err(ToggleError::Cancelled);
    }

    let result = backend.set_radio(element, selected).await;

    // Check cancellation after call
    if config.cancellation.is_cancelled() {
        tracing::error!("Set radio operation cancelled during completion");
        return Err(ToggleError::Cancelled);
    }

    result
}

/// Performs a set toggle operation with config validation
pub async fn set_toggle_with_config(
    element: &uiautomation::UIElement,
    state: bool,
    config: &ToggleConfig,
) -> Result<(), ToggleError> {
    // Validate config BEFORE any backend calls
    validate_toggle_config(config)?;

    tracing::info!(
        "Starting set toggle operation with timeout: {:?}, state: {}",
        config.timeout,
        state
    );

    let backend = ToggleBackendWindows::new();

    // Direct call to backend - UIElement cannot be moved into spawn_blocking
    let set_result = backend.set_toggle(element, state).await;

    // Check for cancellation after backend call
    if config.cancellation.is_cancelled() {
        tracing::error!("Set toggle operation cancelled during completion");
        return Err(ToggleError::Cancelled);
    }

    set_result
}

/// Performs an is checked operation with config validation
pub async fn is_checked_with_config(
    element: &uiautomation::UIElement,
    config: &ToggleConfig,
) -> Result<bool, ToggleError> {
    // Validate config BEFORE any backend calls
    validate_toggle_config(config)?;

    tracing::info!(
        "Starting is checked operation with timeout: {:?}",
        config.timeout
    );

    let backend = ToggleBackendWindows::new();

    // Direct call to backend - UIElement cannot be moved into spawn_blocking
    let check_result = backend.is_checked(element).await;

    // Check for cancellation after backend call
    if config.cancellation.is_cancelled() {
        tracing::error!("Is checked operation cancelled during completion");
        return Err(ToggleError::Cancelled);
    }

    check_result
}

/// Performs an is selected operation with config validation
pub async fn is_selected_with_config(
    element: &uiautomation::UIElement,
    config: &ToggleConfig,
) -> Result<bool, ToggleError> {
    // Validate config BEFORE any backend calls
    validate_toggle_config(config)?;

    tracing::info!(
        "Starting is selected operation with timeout: {:?}",
        config.timeout
    );

    let backend = ToggleBackendWindows::new();

    // Direct call to backend - UIElement cannot be moved into spawn_blocking
    let select_result = backend.is_selected(element).await;

    // Check for cancellation after backend call
    if config.cancellation.is_cancelled() {
        tracing::error!("Is selected operation cancelled during completion");
        return Err(ToggleError::Cancelled);
    }

    select_result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_toggle_config_valid() {
        let cancellation = CancellationToken::new();
        let config = ToggleConfig {
            timeout: Duration::from_secs(5),
            cancellation,
        };

        assert!(validate_toggle_config(&config).is_ok());
    }

    #[test]
    fn test_validate_toggle_config_zero_timeout() {
        let cancellation = CancellationToken::new();
        let config = ToggleConfig {
            timeout: Duration::ZERO,
            cancellation,
        };

        assert!(matches!(
            validate_toggle_config(&config),
            Err(ToggleError::InvalidConfig(_))
        ));
    }

    #[test]
    fn test_validate_toggle_config_large_timeout() {
        let cancellation = CancellationToken::new();
        let config = ToggleConfig {
            timeout: Duration::from_secs(3601), // > 1 hour
            cancellation,
        };

        assert!(matches!(
            validate_toggle_config(&config),
            Err(ToggleError::InvalidConfig(_))
        ));
    }

    #[test]
    fn test_validate_toggle_config_one_hour_edge() {
        let cancellation = CancellationToken::new();
        let config = ToggleConfig {
            timeout: Duration::from_secs(3600), // Exactly 1 hour
            cancellation,
        };

        assert!(validate_toggle_config(&config).is_ok());
    }

    #[test]
    fn test_validate_toggle_config_negative_timeout() {
        let cancellation = CancellationToken::new();
        let config = ToggleConfig {
            timeout: Duration::from_secs(0), // Using 0 to simulate invalid
            cancellation,
        };

        assert!(matches!(
            validate_toggle_config(&config),
            Err(ToggleError::InvalidConfig(_))
        ));
    }

    #[test]
    fn test_mock_backend_creation() {
        let backend = MockToggleBackend::new();
        if let Ok(state) = backend.get_state() {
            assert_eq!(state.call_count, 0);
        };
    }

    #[test]
    fn test_mock_backend_reset() {
        let backend = MockToggleBackend::new();
        backend.reset().unwrap();
        if let Ok(state) = backend.get_state() {
            assert_eq!(state.call_count, 0);
        };
    }

    #[test]
    fn test_mock_backend_with_state() {
        let state = MockToggleState {
            should_succeed: true,
            ..Default::default()
        };
        let backend = MockToggleBackend::with_state(state);
        if let Ok(state) = backend.get_state() {
            assert!(state.should_succeed);
        };
    }
}
