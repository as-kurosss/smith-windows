//! Screenshot tool implementation
//! Provides screenshot capture functionality through Windows GDI/GDI+ API.

use std::time::Duration;
use thiserror::Error;
use tokio_util::sync::CancellationToken;

use crate::runtime::backends::windows::screenshot::ScreenshotBackendWindows;

/// Modes for screenshot capture
#[derive(Debug, Clone)]
pub enum ScreenshotMode {
    /// Capture the entire screen
    Screen,
    /// Capture a specific window
    Window(uiautomation::UIElement),
    /// Capture a custom region
    Region {
        x: i32,
        y: i32,
        width: u32,
        height: u32,
    },
}

/// Configuration for screenshot operations
#[derive(Debug, Clone)]
pub struct ScreenshotConfig {
    /// Timeout for the screenshot operation
    pub timeout: Duration,
    /// Token for cancellation
    pub cancellation: CancellationToken,
}

/// Errors that can occur during screenshot operations
#[derive(Error, Debug, Clone)]
pub enum ScreenshotError {
    /// Invalid region specification
    #[error("Invalid region: {0}")]
    InvalidRegion(String),
    /// Invalid configuration
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
    /// Element not found or invalid
    #[error("Element not found")]
    ElementNotFound,
    /// Operation timed out
    #[error("Operation timed out")]
    Timeout,
    /// Operation was cancelled
    #[error("Operation was cancelled")]
    Cancelled,
    /// Capture failed (GDI/GDI+ error)
    #[error("Capture failed: {0}")]
    CaptureFailed(String),
    /// Unsupported platform
    #[error("Unsupported platform")]
    UnsupportedPlatform,
    /// COM error
    #[error("COM error: {0}")]
    ComError(String),
}

/// Validates screenshot configuration
/// Must be called BEFORE backend invocation
pub fn validate_screenshot_config(config: &ScreenshotConfig) -> Result<(), ScreenshotError> {
    // Check timeout bounds: > 0 and <= 1 hour
    if config.timeout.is_zero() || config.timeout > Duration::from_secs(3600) {
        return Err(ScreenshotError::InvalidConfig(
            "timeout must be > 0 and <= 1 hour".to_string(),
        ));
    }

    Ok(())
}

/// Validates screenshot mode
/// Must be called BEFORE backend invocation
pub fn validate_screenshot_mode(mode: &ScreenshotMode) -> Result<(), ScreenshotError> {
    match mode {
        ScreenshotMode::Screen => Ok(()),
        ScreenshotMode::Window(element) => {
            // Check if element is valid by accessing a property
            let _control_type = element
                .get_control_type()
                .map_err(|_| ScreenshotError::ElementNotFound)?;
            Ok(())
        }
        ScreenshotMode::Region {
            x,
            y,
            width,
            height,
        } => {
            // Validate region bounds
            if *x < 0 || *y < 0 {
                return Err(ScreenshotError::InvalidRegion(
                    "region coordinates must be >= 0".to_string(),
                ));
            }
            if *width == 0 || *height == 0 {
                return Err(ScreenshotError::InvalidRegion(
                    "region dimensions must be > 0".to_string(),
                ));
            }
            Ok(())
        }
    }
}

/// Trait for screenshot backend implementations
#[async_trait::async_trait(?Send)]
pub trait ScreenshotBackend {
    /// Captures a screenshot based on the provided mode
    async fn capture(&self, mode: &ScreenshotMode) -> Result<Vec<u8>, ScreenshotError>;
}

/// Mock backend for testing
/// Uses internal state to simulate different scenarios
#[derive(Debug, Clone, Default)]
pub struct MockScreenshotBackend {
    state: std::sync::Arc<std::sync::Mutex<MockScreenshotState>>,
}

/// State for mock backend
#[derive(Debug, Default)]
pub struct MockScreenshotState {
    pub call_count: usize,
    pub last_error: Option<ScreenshotError>,
    pub should_succeed: bool,
}

impl MockScreenshotBackend {
    /// Creates a new mock backend with default state
    pub fn new() -> Self {
        Self {
            state: std::sync::Arc::new(std::sync::Mutex::new(MockScreenshotState::default())),
        }
    }

    /// Creates a mock backend with custom state
    pub fn with_state(state: MockScreenshotState) -> Self {
        Self {
            state: std::sync::Arc::new(std::sync::Mutex::new(state)),
        }
    }

    /// Gets a mutable reference to the state
    pub fn get_state(
        &self,
    ) -> Result<std::sync::MutexGuard<'_, MockScreenshotState>, ScreenshotError> {
        self.state.lock().map_err(|e| {
            tracing::error!("State mutex poisoned: {}", e);
            ScreenshotError::ComError("State mutex poisoned".into())
        })
    }

    /// Resets the backend state
    pub fn reset(&self) -> Result<(), ScreenshotError> {
        let mut state = self.get_state()?;
        state.call_count = 0;
        state.last_error = None;
        Ok(())
    }
}

#[async_trait::async_trait(?Send)]
impl ScreenshotBackend for MockScreenshotBackend {
    async fn capture(&self, _mode: &ScreenshotMode) -> Result<Vec<u8>, ScreenshotError> {
        let mut state = self.get_state()?;
        state.call_count += 1;

        if state.should_succeed {
            state.last_error = None;
            // Return minimal valid PNG (1x1 pixel)
            Ok(vec![
                0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // PNG magic bytes
                0x00, 0x00, 0x00, 0x0D, // IHDR chunk length
                0x49, 0x48, 0x44, 0x52, // IHDR
                0x00, 0x00, 0x00, 0x01, // Width = 1
                0x00, 0x00, 0x00, 0x01, // Height = 1
                0x08, // Bit depth = 8
                0x02, // Color type = RGB
                0x00, 0x00, 0x00, // Compression, filter, interlace
                0x90, 0x77, 0x53, 0xDE, // IHDR CRC
                0x00, 0x00, 0x00, 0x0A, // IDAT chunk length
                0x49, 0x44, 0x41, 0x54, // IDAT
                0x08, 0xD7, 0x63, 0xF8, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x02, 0x00,
                0x01, // IDAT data
                0x44, 0xDE, 0x1E, 0x26, // IDAT CRC
                0x00, 0x00, 0x00, 0x00, // IEND chunk length
                0x49, 0x45, 0x4E, 0x44, // IEND
                0xAE, 0x42, 0x60, 0x82, // IEND CRC
            ])
        } else {
            let error = state
                .last_error
                .clone()
                .unwrap_or(ScreenshotError::ElementNotFound);
            state.last_error = Some(error.clone());
            Err(error)
        }
    }
}

/// Performs a screenshot operation with config validation and cancellation check
/// Note: UIAutomation is !Send, so we cannot use spawn_blocking or async move.
/// The backend call must run on the same thread that created the UIAutomation instance.
pub async fn screenshot_with_config(
    mode: &ScreenshotMode,
    config: &ScreenshotConfig,
) -> Result<Vec<u8>, ScreenshotError> {
    // Validate config BEFORE any backend calls
    validate_screenshot_config(config)?;

    // Validate mode BEFORE any backend calls
    validate_screenshot_mode(mode)?;

    tracing::info!("Starting screenshot operation with mode: {:?}", mode);

    if config.cancellation.is_cancelled() {
        tracing::error!("Screenshot operation cancelled before completion");
        return Err(ScreenshotError::Cancelled);
    }

    let backend = ScreenshotBackendWindows::new();

    let result = backend.capture(mode).await;

    if config.cancellation.is_cancelled() {
        tracing::error!("Screenshot operation cancelled during completion");
        return Err(ScreenshotError::Cancelled);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_screenshot_config_valid() {
        let cancellation = CancellationToken::new();
        let config = ScreenshotConfig {
            timeout: Duration::from_secs(5),
            cancellation,
        };

        assert!(validate_screenshot_config(&config).is_ok());
    }

    #[test]
    fn test_validate_screenshot_config_zero_timeout() {
        let cancellation = CancellationToken::new();
        let config = ScreenshotConfig {
            timeout: Duration::ZERO,
            cancellation,
        };

        assert!(matches!(
            validate_screenshot_config(&config),
            Err(ScreenshotError::InvalidConfig(_))
        ));
    }

    #[test]
    fn test_validate_screenshot_config_large_timeout() {
        let cancellation = CancellationToken::new();
        let config = ScreenshotConfig {
            timeout: Duration::from_secs(3601), // > 1 hour
            cancellation,
        };

        assert!(matches!(
            validate_screenshot_config(&config),
            Err(ScreenshotError::InvalidConfig(_))
        ));
    }

    #[test]
    fn test_validate_screenshot_mode_screen() {
        let mode = ScreenshotMode::Screen;
        assert!(validate_screenshot_mode(&mode).is_ok());
    }

    #[test]
    fn test_validate_screenshot_mode_region_valid() {
        let mode = ScreenshotMode::Region {
            x: 0,
            y: 0,
            width: 100,
            height: 100,
        };
        assert!(validate_screenshot_mode(&mode).is_ok());
    }

    #[test]
    fn test_validate_screenshot_mode_region_negative_coords() {
        let mode = ScreenshotMode::Region {
            x: -1,
            y: 0,
            width: 100,
            height: 100,
        };
        assert!(matches!(
            validate_screenshot_mode(&mode),
            Err(ScreenshotError::InvalidRegion(_))
        ));
    }

    #[test]
    fn test_validate_screenshot_mode_region_zero_width() {
        let mode = ScreenshotMode::Region {
            x: 0,
            y: 0,
            width: 0,
            height: 100,
        };
        assert!(matches!(
            validate_screenshot_mode(&mode),
            Err(ScreenshotError::InvalidRegion(_))
        ));
    }

    #[test]
    fn test_validate_screenshot_mode_region_zero_height() {
        let mode = ScreenshotMode::Region {
            x: 0,
            y: 0,
            width: 100,
            height: 0,
        };
        assert!(matches!(
            validate_screenshot_mode(&mode),
            Err(ScreenshotError::InvalidRegion(_))
        ));
    }

    #[test]
    fn test_mock_backend_creation() {
        let backend = MockScreenshotBackend::new();
        assert_eq!(backend.get_state().unwrap().call_count, 0);
    }

    #[test]
    fn test_mock_backend_with_state() {
        let state = MockScreenshotState {
            should_succeed: true,
            ..Default::default()
        };
        let backend = MockScreenshotBackend::with_state(state);
        if let Ok(s) = backend.get_state() {
            assert!(s.should_succeed);
        };
    }

    #[test]
    fn test_mock_backend_reset() {
        let backend = MockScreenshotBackend::new();
        backend.reset().unwrap();
        if let Ok(s) = backend.get_state() {
            assert_eq!(s.call_count, 0);
        };
    }

    #[tokio::test]
    async fn test_mock_backend_png_bytes() {
        // Set should_succeed to true in the default state
        let state = MockScreenshotState {
            should_succeed: true,
            ..Default::default()
        };
        let backend = MockScreenshotBackend::with_state(state);
        let result = backend.capture(&ScreenshotMode::Screen).await;
        assert!(result.is_ok(), "Expected success, got: {:?}", result);

        let png_bytes = result.unwrap();
        // Check PNG magic bytes
        assert_eq!(
            &png_bytes[0..8],
            &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]
        );
    }
}
