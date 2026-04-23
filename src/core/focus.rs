//! Focus tool implementation
//! Activates a UI element's window before interaction to ensure input operations
//! (text, click) target the correct window, not the currently focused one.

use std::time::Duration;
use thiserror::Error;
use tokio_util::sync::CancellationToken;

use crate::runtime::backends::windows::focus::FocusBackendWindows;

/// Configuration for focus operations
#[derive(Debug, Clone)]
pub struct FocusConfig {
    /// Timeout for the focus operation
    pub timeout: Duration,
    /// Token for cancellation
    pub cancellation: CancellationToken,
}

/// Errors that can occur during focus operations
#[derive(Error, Debug, Clone, PartialEq)]
pub enum FocusError {
    /// Element not found or invalid
    #[error("Element not found")]
    ElementNotFound,
    /// Element is disabled
    #[error("Element is disabled")]
    ElementNotEnabled,
    /// Element is offscreen
    #[error("Element is offscreen")]
    ElementOffscreen,
    /// Window pattern not available on element
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

/// Validates focus configuration
/// Must be called BEFORE backend invocation
pub fn validate_config(config: &FocusConfig) -> Result<(), FocusError> {
    // Check timeout bounds: > 0 and <= 1 hour
    if config.timeout.is_zero() || config.timeout > Duration::from_secs(3600) {
        return Err(FocusError::InvalidConfig(
            "timeout must be > 0 and <= 1 hour".to_string(),
        ));
    }

    Ok(())
}

/// Validates element is ready for focus operation
/// Must be called BEFORE backend invocation
pub fn validate_element_ready(element: &uiautomation::UIElement) -> Result<(), FocusError> {
    // Check if element is enabled
    let enabled_result = element.is_enabled();
    let is_enabled = match enabled_result {
        Ok(val) => val,
        Err(e) => {
            tracing::error!("Failed to check if element is enabled: {}", e);
            return Err(FocusError::ComError(e.to_string()));
        }
    };

    if !is_enabled {
        tracing::error!("Focus failed: element is disabled");
        return Err(FocusError::ElementNotEnabled);
    }

    // Check if element is offscreen
    let offscreen_result = element.is_offscreen();
    let is_offscreen = match offscreen_result {
        Ok(val) => val,
        Err(e) => {
            tracing::error!("Failed to check if element is offscreen: {}", e);
            return Err(FocusError::ComError(e.to_string()));
        }
    };

    if is_offscreen {
        tracing::error!("Focus failed: element is offscreen");
        return Err(FocusError::ElementOffscreen);
    }

    Ok(())
}

/// Validates that element has WindowPattern available
/// Must be called BEFORE backend invocation
pub fn validate_window_pattern_available(
    element: &uiautomation::UIElement,
) -> Result<(), FocusError> {
    // Try to get WindowPattern - if it fails, pattern is not available
    match element.get_pattern::<uiautomation::patterns::UIWindowPattern>() {
        Ok(_) => Ok(()),
        Err(_) => {
            tracing::error!("Focus failed: element does not have WindowPattern");
            Err(FocusError::WindowPatternNotAvailable)
        }
    }
}

/// Trait for focus backend implementations
#[async_trait::async_trait(?Send)]
pub trait FocusBackend {
    /// Activates the window containing the given element
    async fn focus(&self, element: &uiautomation::UIElement) -> Result<(), FocusError>;
}

/// Mock backend for testing
/// Uses internal state to simulate different scenarios
#[derive(Debug, Clone, Default)]
pub struct MockFocusBackend {
    state: std::sync::Arc<std::sync::Mutex<MockFocusState>>,
}

/// State for mock backend
#[derive(Debug, Default)]
pub struct MockFocusState {
    pub call_count: usize,
    pub last_error: Option<FocusError>,
    pub should_succeed: bool,
    pub window_activated: bool,
}

impl MockFocusBackend {
    /// Creates a new mock backend with default state
    pub fn new() -> Self {
        Self {
            state: std::sync::Arc::new(std::sync::Mutex::new(MockFocusState::default())),
        }
    }

    /// Creates a mock backend with custom state
    pub fn with_state(state: MockFocusState) -> Self {
        Self {
            state: std::sync::Arc::new(std::sync::Mutex::new(state)),
        }
    }

    /// Gets a mutable reference to the state
    pub fn get_state(&self) -> Result<std::sync::MutexGuard<'_, MockFocusState>, FocusError> {
        self.state
            .lock()
            .map_err(|e| FocusError::ComError(e.to_string()))
    }

    /// Resets the backend state
    pub fn reset(&self) -> Result<(), FocusError> {
        let mut state = self.get_state()?;
        state.call_count = 0;
        state.last_error = None;
        state.window_activated = false;
        Ok(())
    }
}

#[async_trait::async_trait(?Send)]
impl FocusBackend for MockFocusBackend {
    async fn focus(&self, _element: &uiautomation::UIElement) -> Result<(), FocusError> {
        let mut state = self.get_state()?;
        state.call_count += 1;

        if state.should_succeed {
            state.last_error = None;
            state.window_activated = true;
            Ok(())
        } else {
            let error = state
                .last_error
                .clone()
                .unwrap_or(FocusError::ElementNotFound);
            state.last_error = Some(error.clone());
            Err(error)
        }
    }
}

/// Performs a focus operation with config validation and cancellation check
pub async fn focus_with_config(
    element: &uiautomation::UIElement,
    config: &FocusConfig,
) -> Result<(), FocusError> {
    // Validate config BEFORE any backend calls
    validate_config(config)?;

    tracing::info!("Starting focus operation");

    // Validate element BEFORE backend call
    validate_element_ready(element)?;
    validate_window_pattern_available(element)?;

    if config.cancellation.is_cancelled() {
        tracing::error!("Focus operation cancelled before completion");
        return Err(FocusError::Cancelled);
    }

    let backend = FocusBackendWindows::new();

    let result = backend.focus(element).await;

    if config.cancellation.is_cancelled() {
        tracing::error!("Focus operation cancelled during completion");
        return Err(FocusError::Cancelled);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_config_valid() {
        let cancellation = CancellationToken::new();
        let config = FocusConfig {
            timeout: Duration::from_secs(5),
            cancellation,
        };

        assert!(validate_config(&config).is_ok());
    }

    #[test]
    fn test_validate_config_zero_timeout() {
        let cancellation = CancellationToken::new();
        let config = FocusConfig {
            timeout: Duration::ZERO,
            cancellation,
        };

        assert!(matches!(
            validate_config(&config),
            Err(FocusError::InvalidConfig(_))
        ));
    }

    #[test]
    fn test_validate_config_large_timeout() {
        let cancellation = CancellationToken::new();
        let config = FocusConfig {
            timeout: Duration::from_secs(3601), // > 1 hour
            cancellation,
        };

        assert!(matches!(
            validate_config(&config),
            Err(FocusError::InvalidConfig(_))
        ));
    }

    #[test]
    fn test_validate_config_minimum_valid() {
        let cancellation = CancellationToken::new();
        let config = FocusConfig {
            timeout: Duration::from_secs(1),
            cancellation,
        };

        assert!(validate_config(&config).is_ok());
    }

    #[test]
    fn test_validate_config_maximum_valid() {
        let cancellation = CancellationToken::new();
        let config = FocusConfig {
            timeout: Duration::from_secs(3600),
            cancellation,
        };

        assert!(validate_config(&config).is_ok());
    }

    #[test]
    fn test_mock_backend_creation() {
        let backend = MockFocusBackend::new();
        if let Ok(state) = backend.get_state() {
            assert_eq!(state.call_count, 0);
            assert!(!state.window_activated);
        };
    }

    #[test]
    fn test_mock_backend_with_state() {
        let state = MockFocusState {
            should_succeed: true,
            window_activated: false,
            ..Default::default()
        };
        let backend = MockFocusBackend::with_state(state);
        if let Ok(state) = backend.get_state() {
            assert!(state.should_succeed);
        };
    }

    #[test]
    fn test_mock_backend_reset() {
        let backend = MockFocusBackend::new();
        backend.reset().unwrap();
        if let Ok(state) = backend.get_state() {
            assert_eq!(state.call_count, 0);
            assert!(!state.window_activated);
        };
    }
}
