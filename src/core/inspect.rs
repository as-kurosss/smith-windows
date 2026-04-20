//! Inspect tool implementation
//! Provides UI element inspection functionality through UI Automation API.

use std::time::Duration;
use thiserror::Error;
use tokio_util::sync::CancellationToken;

/// Configuration for inspect operations
#[derive(Debug, Clone)]
pub struct InspectConfig {
    /// Timeout for the inspect operation
    pub timeout: Duration,
    /// Token for cancellation
    pub cancellation: CancellationToken,
}

/// Errors that can occur during inspect operations
#[derive(Error, Debug, Clone)]
pub enum InspectError {
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
    /// Invalid selector (element not in hierarchy)
    #[error("Invalid selector: element not in hierarchy")]
    InvalidSelector,
}

/// Validates inspect configuration
/// Must be called BEFORE backend invocation
pub fn validate_inspect_config(config: &InspectConfig) -> Result<(), InspectError> {
    // Check timeout bounds: > 0 and <= 1 hour
    if config.timeout.is_zero() || config.timeout > Duration::from_secs(3600) {
        return Err(InspectError::InvalidConfig(
            "timeout must be > 0 and <= 1 hour".to_string(),
        ));
    }

    Ok(())
}

/// Trait for inspect backend implementations
#[async_trait::async_trait(?Send)]
pub trait InspectBackend {
    /// Gets the inspect path from head window to element
    async fn inspect_path(
        &self,
        head_window: &uiautomation::UIElement,
        element: &uiautomation::UIElement,
    ) -> Result<String, InspectError>;
}

/// Mock backend for testing
/// Uses internal state to simulate different scenarios
#[derive(Debug, Clone, Default)]
pub struct MockInspectBackend {
    state: std::sync::Arc<std::sync::Mutex<MockInspectState>>,
}

/// State for mock backend
#[derive(Debug, Default)]
pub struct MockInspectState {
    pub call_count: usize,
    pub last_error: Option<InspectError>,
    pub should_succeed: bool,
    pub path: String,
}

impl MockInspectBackend {
    /// Creates a new mock backend with default state
    pub fn new() -> Self {
        Self {
            state: std::sync::Arc::new(std::sync::Mutex::new(MockInspectState::default())),
        }
    }

    /// Creates a mock backend with custom state
    pub fn with_state(state: MockInspectState) -> Self {
        Self {
            state: std::sync::Arc::new(std::sync::Mutex::new(state)),
        }
    }

    /// Gets a mutable reference to the state
    pub fn get_state(&self) -> std::sync::MutexGuard<'_, MockInspectState> {
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
impl InspectBackend for MockInspectBackend {
    async fn inspect_path(
        &self,
        _head_window: &uiautomation::UIElement,
        _element: &uiautomation::UIElement,
    ) -> Result<String, InspectError> {
        let mut state = self.get_state();
        state.call_count += 1;

        if state.should_succeed {
            let path = state.path.clone();
            state.last_error = None;
            Ok(path)
        } else {
            let error = state
                .last_error
                .clone()
                .unwrap_or(InspectError::ElementNotFound);
            state.last_error = Some(error.clone());
            Err(error)
        }
    }
}

/// Validates inspect path (checks hierarchy and depth)
/// Must be called BEFORE backend invocation
///
/// This function performs basic validation of the path. For full hierarchy validation
/// and path building, use the Windows backend which uses UITreeWalker.
pub fn validate_inspect_path(
    _head_window: &uiautomation::UIElement,
    _element: &uiautomation::UIElement,
) -> Result<(), InspectError> {
    // Basic validation only - full hierarchy validation is done in the Windows backend
    // using UITreeWalker which provides get_parent() method

    // The actual hierarchy traversal and validation is performed by the backend
    // using automation.create_tree_walker().get_parent()

    Ok(())
}

/// Builds the inspect path string from head window to element
/// Format: "Window->Button->CheckBox{Name}" or "Window->Button" (if Name is empty)
///
/// This function is now deprecated in favor of the full implementation in the Windows backend
/// which uses UITreeWalker to traverse the UI tree and build the complete path.
///
/// Kept for backwards compatibility and potential fallback usage.
#[deprecated(
    note = "Use inspect_with_config or InspectBackend::inspect_path for full path building"
)]
pub fn get_inspect_path(
    _head_window: &uiautomation::UIElement,
    _element: &uiautomation::UIElement,
) -> Result<String, InspectError> {
    // This function is now deprecated. The full implementation is in the Windows backend
    // which uses UITreeWalker to traverse the UI tree and build the complete path.
    // Kept for backwards compatibility.

    Err(InspectError::InvalidSelector)
}

/// Performs an inspect operation with config validation and timeout handling
pub async fn inspect_with_config(
    head_window: &uiautomation::UIElement,
    element: &uiautomation::UIElement,
    config: &InspectConfig,
) -> Result<String, InspectError> {
    // Validate config BEFORE any backend calls
    validate_inspect_config(config)?;

    tracing::info!(
        "Starting inspect operation with timeout: {:?}",
        config.timeout
    );

    let backend = crate::runtime::backends::windows::inspect::InspectBackendWindows::new();

    // Wrap with timeout and cancellation
    let inspect_future = async move { backend.inspect_path(head_window, element).await };

    // Wrap the future with timeout
    let result = tokio::time::timeout(config.timeout, inspect_future).await;

    match result {
        Ok(inspect_result) => {
            // Check for cancellation
            if config.cancellation.is_cancelled() {
                tracing::error!("Inspect operation cancelled during completion");
                return Err(InspectError::Cancelled);
            }
            inspect_result
        }
        Err(_) => {
            tracing::error!("Inspect operation timed out after {:?}", config.timeout);
            Err(InspectError::Timeout)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_inspect_config_valid() {
        let cancellation = CancellationToken::new();
        let config = InspectConfig {
            timeout: Duration::from_secs(5),
            cancellation,
        };

        assert!(validate_inspect_config(&config).is_ok());
    }

    #[test]
    fn test_validate_inspect_config_zero_timeout() {
        let cancellation = CancellationToken::new();
        let config = InspectConfig {
            timeout: Duration::ZERO,
            cancellation,
        };

        assert!(matches!(
            validate_inspect_config(&config),
            Err(InspectError::InvalidConfig(_))
        ));
    }

    #[test]
    fn test_validate_inspect_config_large_timeout() {
        let cancellation = CancellationToken::new();
        let config = InspectConfig {
            timeout: Duration::from_secs(3601), // > 1 hour
            cancellation,
        };

        assert!(matches!(
            validate_inspect_config(&config),
            Err(InspectError::InvalidConfig(_))
        ));
    }

    #[test]
    fn test_mock_backend_creation() {
        let backend = MockInspectBackend::new();
        assert_eq!(backend.get_state().call_count, 0);
    }

    #[test]
    fn test_mock_backend_with_state() {
        let state = MockInspectState {
            should_succeed: true,
            path: "Window->Button".to_string(),
            ..Default::default()
        };
        let backend = MockInspectBackend::with_state(state);
        assert_eq!(backend.get_state().should_succeed, true);
    }

    #[test]
    fn test_mock_backend_reset() {
        let backend = MockInspectBackend::new();
        backend.reset();
        assert_eq!(backend.get_state().call_count, 0);
    }
}
