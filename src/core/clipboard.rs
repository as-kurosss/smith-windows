//! Clipboard tool implementation
//! Provides system clipboard operations (get text, set text, check presence).

use std::time::Duration;
use thiserror::Error;
use tokio_util::sync::CancellationToken;

/// Action types for clipboard operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClipboardAction {
    /// Get text from clipboard
    GetText,
    /// Set text to clipboard
    SetText,
    /// Check if clipboard contains text
    HasText,
}

/// Configuration for clipboard operations
#[derive(Debug, Clone)]
pub struct ClipboardConfig {
    /// Timeout for the clipboard operation
    pub timeout: Duration,
    /// Token for cancellation
    pub cancellation: CancellationToken,
}

/// Parameters for set text operation
#[derive(Debug, Clone)]
pub struct SetTextParams {
    /// Text to set to clipboard
    pub text: String,
}

/// Errors that can occur during clipboard operations
#[derive(Error, Debug, Clone, PartialEq)]
pub enum ClipboardError {
    /// Operation not supported
    #[error("Operation not supported")]
    OperationNotSupported,
    /// Clipboard is empty
    #[error("Clipboard is empty")]
    ClipboardEmpty,
    /// Clipboard access denied
    #[error("Clipboard access denied")]
    ClipboardAccessDenied,
    /// Text is empty
    #[error("Text is empty")]
    TextEmpty,
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

/// Validates clipboard configuration
/// Must be called BEFORE backend invocation
pub fn validate_clipboard_config(config: &ClipboardConfig) -> Result<(), ClipboardError> {
    // Check timeout bounds: > 0 and <= 1 hour
    if config.timeout.is_zero() {
        return Err(ClipboardError::InvalidConfig(
            "timeout must be > 0".to_string(),
        ));
    }

    if config.timeout > Duration::from_secs(3600) {
        return Err(ClipboardError::InvalidConfig(
            "timeout must be <= 1 hour".to_string(),
        ));
    }

    Ok(())
}

/// Trait for clipboard backend implementations
#[async_trait::async_trait(?Send)]
pub trait ClipboardBackend {
    /// Gets text from clipboard
    async fn get_text(&self) -> Result<String, ClipboardError>;
    /// Sets text to clipboard
    async fn set_text(&self, text: &str) -> Result<(), ClipboardError>;
    /// Checks if clipboard contains text
    async fn has_text(&self) -> Result<bool, ClipboardError>;
}

/// Mock backend for testing
/// Uses internal state to simulate different scenarios
#[derive(Debug, Clone, Default)]
pub struct MockClipboardBackend {
    state: std::sync::Arc<std::sync::Mutex<MockClipboardState>>,
}

/// State for mock backend
#[derive(Debug, Default, Clone)]
pub struct MockClipboardState {
    pub call_count: usize,
    pub last_error: Option<ClipboardError>,
    pub should_succeed: bool,
    pub clipboard_has_text: bool,
    pub clipboard_text: Option<String>,
}

impl MockClipboardBackend {
    /// Creates a new mock backend with default state
    pub fn new() -> Self {
        Self {
            state: std::sync::Arc::new(std::sync::Mutex::new(MockClipboardState::default())),
        }
    }

    /// Creates a mock backend with custom state
    pub fn with_state(state: MockClipboardState) -> Self {
        Self {
            state: std::sync::Arc::new(std::sync::Mutex::new(state)),
        }
    }

    /// Gets a mutable reference to the state (for sync operations like reset)
    pub fn get_state(
        &self,
    ) -> Result<std::sync::MutexGuard<'_, MockClipboardState>, ClipboardError> {
        self.state
            .lock()
            .map_err(|e| ClipboardError::ComError(e.to_string()))
    }

    /// Resets the backend state
    pub fn reset(&self) -> Result<(), ClipboardError> {
        let mut state = self.get_state()?;
        state.call_count = 0;
        state.last_error = None;
        Ok(())
    }
}

#[async_trait::async_trait(?Send)]
impl ClipboardBackend for MockClipboardBackend {
    async fn get_text(&self) -> Result<String, ClipboardError> {
        // Lock, operate, unlock in one go
        let (should_succeed, clipboard_text, last_error) = {
            let mut state = self
                .state
                .lock()
                .map_err(|e| ClipboardError::ComError(e.to_string()))?;
            state.call_count += 1;

            if state.should_succeed {
                if state.clipboard_has_text {
                    let text = state.clipboard_text.clone().unwrap_or_default();
                    state.last_error = None;
                    (true, Some(text), None)
                } else {
                    let error = ClipboardError::ClipboardEmpty;
                    state.last_error = Some(error.clone());
                    (false, None, Some(error))
                }
            } else {
                let error = state
                    .last_error
                    .clone()
                    .unwrap_or(ClipboardError::ClipboardEmpty);
                state.last_error = Some(error.clone());
                (false, None, Some(error))
            }
        };

        if should_succeed {
            tracing::info!("MockClipboardBackend: get_text succeeded");
            Ok(clipboard_text.unwrap_or_default())
        } else {
            tracing::error!(
                "MockClipboardBackend: get_text failed with {:?}",
                last_error
            );
            Err(last_error.unwrap_or(ClipboardError::ClipboardEmpty))
        }
    }

    async fn set_text(&self, _text: &str) -> Result<(), ClipboardError> {
        // Lock, operate, unlock in one go
        let (should_succeed, last_error) = {
            let mut state = self
                .state
                .lock()
                .map_err(|e| ClipboardError::ComError(e.to_string()))?;
            state.call_count += 1;

            if state.should_succeed {
                state.last_error = None;
                (true, None)
            } else {
                let error = state
                    .last_error
                    .clone()
                    .unwrap_or(ClipboardError::ClipboardAccessDenied);
                state.last_error = Some(error.clone());
                (false, Some(error))
            }
        };

        if should_succeed {
            tracing::info!("MockClipboardBackend: set_text succeeded");
            Ok(())
        } else {
            tracing::error!(
                "MockClipboardBackend: set_text failed with {:?}",
                last_error
            );
            Err(last_error.unwrap_or(ClipboardError::ClipboardAccessDenied))
        }
    }

    async fn has_text(&self) -> Result<bool, ClipboardError> {
        // Lock, operate, unlock in one go
        let (should_succeed, clipboard_has_text, last_error) = {
            let mut state = self
                .state
                .lock()
                .map_err(|e| ClipboardError::ComError(e.to_string()))?;
            state.call_count += 1;

            if state.should_succeed {
                let has_text = state.clipboard_has_text;
                state.last_error = None;
                (true, has_text, None)
            } else {
                let error = state
                    .last_error
                    .clone()
                    .unwrap_or(ClipboardError::ClipboardAccessDenied);
                state.last_error = Some(error.clone());
                (false, state.clipboard_has_text, Some(error))
            }
        };

        if should_succeed {
            tracing::info!("MockClipboardBackend: has_text succeeded");
            Ok(clipboard_has_text)
        } else {
            tracing::error!(
                "MockClipboardBackend: has_text failed with {:?}",
                last_error
            );
            Err(last_error.unwrap_or(ClipboardError::ClipboardAccessDenied))
        }
    }
}

/// Gets text from clipboard with config validation and cancellation check
/// Note: Clipboard operations are !Send, so we cannot use spawn_blocking or async move.
/// The backend call must run on the same thread that created the UIAutomation instance.
pub async fn get_text_with_config(config: &ClipboardConfig) -> Result<String, ClipboardError> {
    // Validate config BEFORE any backend calls
    validate_clipboard_config(config)?;

    tracing::info!("Starting get text operation");

    if config.cancellation.is_cancelled() {
        tracing::error!("Get text operation cancelled before completion");
        return Err(ClipboardError::Cancelled);
    }

    #[cfg(target_os = "windows")]
    {
        let backend = crate::runtime::backends::windows::ClipboardBackendWindows::new();
        let result = backend.get_text().await;
        if config.cancellation.is_cancelled() {
            tracing::error!("Get text operation cancelled during completion");
            return Err(ClipboardError::Cancelled);
        }
        result
    }

    #[cfg(not(target_os = "windows"))]
    {
        tracing::error!("Get text operation not supported on this platform");
        Err(ClipboardError::OperationNotSupported)
    }
}

/// Sets text to clipboard with config validation and cancellation check
/// Note: Clipboard operations are !Send, so we cannot use spawn_blocking or async move.
/// The backend call must run on the same thread that created the UIAutomation instance.
pub async fn set_text_with_config(
    params: &SetTextParams,
    config: &ClipboardConfig,
) -> Result<(), ClipboardError> {
    // Validate config BEFORE any backend calls
    validate_clipboard_config(config)?;

    // Validate text BEFORE backend call
    if params.text.is_empty() {
        return Err(ClipboardError::InvalidConfig(
            "text cannot be empty".to_string(),
        ));
    }

    tracing::info!(
        "Starting set text operation, text length: {}",
        params.text.len()
    );

    if config.cancellation.is_cancelled() {
        tracing::error!("Set text operation cancelled before completion");
        return Err(ClipboardError::Cancelled);
    }

    #[cfg(target_os = "windows")]
    {
        let backend = crate::runtime::backends::windows::ClipboardBackendWindows::new();
        let result = backend.set_text(&params.text).await;
        if config.cancellation.is_cancelled() {
            tracing::error!("Set text operation cancelled during completion");
            return Err(ClipboardError::Cancelled);
        }
        result
    }

    #[cfg(not(target_os = "windows"))]
    {
        tracing::error!("Set text operation not supported on this platform");
        Err(ClipboardError::OperationNotSupported)
    }
}

/// Checks if clipboard contains text with config validation and cancellation check
/// Note: Clipboard operations are !Send, so we cannot use spawn_blocking or async move.
/// The backend call must run on the same thread that created the UIAutomation instance.
pub async fn has_text_with_config(config: &ClipboardConfig) -> Result<bool, ClipboardError> {
    // Validate config BEFORE any backend calls
    validate_clipboard_config(config)?;

    tracing::info!("Starting has text operation");

    if config.cancellation.is_cancelled() {
        tracing::error!("Has text operation cancelled before completion");
        return Err(ClipboardError::Cancelled);
    }

    #[cfg(target_os = "windows")]
    {
        let backend = crate::runtime::backends::windows::ClipboardBackendWindows::new();
        let result = backend.has_text().await;
        if config.cancellation.is_cancelled() {
            tracing::error!("Has text operation cancelled during completion");
            return Err(ClipboardError::Cancelled);
        }
        result
    }

    #[cfg(not(target_os = "windows"))]
    {
        tracing::error!("Has text operation not supported on this platform");
        Err(ClipboardError::OperationNotSupported)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_clipboard_config_valid() {
        let cancellation = CancellationToken::new();
        let config = ClipboardConfig {
            timeout: Duration::from_secs(5),
            cancellation,
        };

        assert!(validate_clipboard_config(&config).is_ok());
    }

    #[test]
    fn test_validate_clipboard_config_zero_timeout() {
        let cancellation = CancellationToken::new();
        let config = ClipboardConfig {
            timeout: Duration::ZERO,
            cancellation,
        };

        assert!(matches!(
            validate_clipboard_config(&config),
            Err(ClipboardError::InvalidConfig(_))
        ));
    }

    #[test]
    fn test_validate_clipboard_config_large_timeout() {
        let cancellation = CancellationToken::new();
        let config = ClipboardConfig {
            timeout: Duration::from_secs(3601), // > 1 hour
            cancellation,
        };

        assert!(matches!(
            validate_clipboard_config(&config),
            Err(ClipboardError::InvalidConfig(_))
        ));
    }

    #[test]
    fn test_validate_clipboard_config_max_timeout() {
        let cancellation = CancellationToken::new();
        let config = ClipboardConfig {
            timeout: Duration::from_secs(3600), // Exactly 1 hour
            cancellation,
        };

        assert!(validate_clipboard_config(&config).is_ok());
    }

    #[test]
    fn test_mock_backend_creation() {
        let backend = MockClipboardBackend::new();
        if let Ok(state) = backend.get_state() {
            assert_eq!(state.call_count, 0);
        };
    }

    #[test]
    fn test_mock_backend_with_state() {
        let state = MockClipboardState {
            should_succeed: true,
            clipboard_has_text: true,
            clipboard_text: Some("Test text".to_string()),
            ..Default::default()
        };
        let backend = MockClipboardBackend::with_state(state);
        if let Ok(state) = backend.get_state() {
            assert!(state.should_succeed);
            assert!(state.clipboard_has_text);
        };
    }

    #[test]
    fn test_mock_backend_reset() {
        let backend = MockClipboardBackend::new();
        backend.reset().unwrap();
        if let Ok(state) = backend.get_state() {
            assert_eq!(state.call_count, 0);
        };
    }
}
