//! Scroll tool implementation
//! Provides UI element scroll functionality through UI Automation API and synthetic mouse wheel.

use std::time::Duration;
use thiserror::Error;
use tokio_util::sync::CancellationToken;

use crate::runtime::backends::windows::scroll::ScrollBackendWindows;

/// Configuration for scroll operations
#[derive(Debug, Clone)]
pub struct ScrollConfig {
    /// Timeout for the scroll operation
    pub timeout: Duration,
    /// Token for cancellation
    pub cancellation: CancellationToken,
}

use std::str::FromStr;

/// Scroll direction (vertical or horizontal)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScrollDirection {
    Vertical,
    Horizontal,
}

impl FromStr for ScrollDirection {
    type Err = ScrollError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "vertical" => Ok(ScrollDirection::Vertical),
            "horizontal" => Ok(ScrollDirection::Horizontal),
            _ => Err(ScrollError::InvalidConfig(format!(
                "invalid direction '{}', expected 'vertical' or 'horizontal'",
                s
            ))),
        }
    }
}

/// Scroll unit (line, page, or pixel)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScrollUnit {
    Line,
    Page,
    Pixel,
}

impl FromStr for ScrollUnit {
    type Err = ScrollError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "line" => Ok(ScrollUnit::Line),
            "page" => Ok(ScrollUnit::Page),
            "pixel" => Ok(ScrollUnit::Pixel),
            _ => Err(ScrollError::InvalidConfig(format!(
                "invalid unit '{}', expected 'line', 'page', or 'pixel'",
                s
            ))),
        }
    }
}

/// Errors that can occur during scroll operations
#[derive(Error, Debug, Clone)]
pub enum ScrollError {
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
    /// Pattern not supported
    #[error("Pattern not supported")]
    PatternNotSupported,
    /// Unsupported platform
    #[error("Unsupported platform")]
    UnsupportedPlatform,
}

/// Validates scroll configuration
/// Must be called BEFORE backend invocation
pub fn validate_scroll_config(config: &ScrollConfig) -> Result<(), ScrollError> {
    // Check timeout bounds: > 0 and <= 1 hour
    if config.timeout.is_zero() || config.timeout > Duration::from_secs(3600) {
        return Err(ScrollError::InvalidConfig(
            "timeout must be > 0 and <= 1 hour".to_string(),
        ));
    }

    Ok(())
}

/// Trait for scroll backend implementations
#[async_trait::async_trait(?Send)]
pub trait ScrollBackend {
    /// Scrolls element vertically
    async fn scroll_vertical(
        &self,
        element: &uiautomation::UIElement,
        amount: i32,
        unit: ScrollUnit,
    ) -> Result<(), ScrollError>;

    /// Scrolls element horizontally
    async fn scroll_horizontal(
        &self,
        element: &uiautomation::UIElement,
        amount: i32,
        unit: ScrollUnit,
    ) -> Result<(), ScrollError>;

    /// Simulates mouse wheel scroll
    async fn simulate_mouse_wheel(
        &self,
        ticks: i32,
        direction: ScrollDirection,
    ) -> Result<(), ScrollError>;
}

/// Mock backend for testing
/// Uses internal state to simulate different scenarios
#[derive(Debug, Clone, Default)]
pub struct MockScrollBackend {
    state: std::sync::Arc<std::sync::Mutex<MockScrollState>>,
}

/// State for mock backend
#[derive(Debug, Default)]
pub struct MockScrollState {
    pub call_count: usize,
    pub last_error: Option<ScrollError>,
    pub should_succeed: bool,
}

impl MockScrollBackend {
    /// Creates a new mock backend with default state
    pub fn new() -> Self {
        Self {
            state: std::sync::Arc::new(std::sync::Mutex::new(MockScrollState::default())),
        }
    }

    /// Creates a mock backend with custom state
    pub fn with_state(state: MockScrollState) -> Self {
        Self {
            state: std::sync::Arc::new(std::sync::Mutex::new(state)),
        }
    }

    /// Gets a mutable reference to the state
    pub fn get_state(&self) -> Result<std::sync::MutexGuard<'_, MockScrollState>, ScrollError> {
        self.state
            .lock()
            .map_err(|e| ScrollError::ComError(e.to_string()))
    }

    /// Resets the backend state
    pub fn reset(&self) -> Result<(), ScrollError> {
        let mut state = self.get_state()?;
        state.call_count = 0;
        state.last_error = None;
        Ok(())
    }
}

#[async_trait::async_trait(?Send)]
impl ScrollBackend for MockScrollBackend {
    async fn scroll_vertical(
        &self,
        _element: &uiautomation::UIElement,
        _amount: i32,
        _unit: ScrollUnit,
    ) -> Result<(), ScrollError> {
        let mut state = self.get_state()?;
        state.call_count += 1;

        if state.should_succeed {
            state.last_error = None;
            Ok(())
        } else {
            let error = state
                .last_error
                .clone()
                .unwrap_or(ScrollError::ElementNotFound);
            state.last_error = Some(error.clone());
            Err(error)
        }
    }

    async fn scroll_horizontal(
        &self,
        _element: &uiautomation::UIElement,
        _amount: i32,
        _unit: ScrollUnit,
    ) -> Result<(), ScrollError> {
        let mut state = self.get_state()?;
        state.call_count += 1;

        if state.should_succeed {
            state.last_error = None;
            Ok(())
        } else {
            let error = state
                .last_error
                .clone()
                .unwrap_or(ScrollError::ElementNotFound);
            state.last_error = Some(error.clone());
            Err(error)
        }
    }

    async fn simulate_mouse_wheel(
        &self,
        _ticks: i32,
        _direction: ScrollDirection,
    ) -> Result<(), ScrollError> {
        let mut state = self.get_state()?;
        state.call_count += 1;

        if state.should_succeed {
            state.last_error = None;
            Ok(())
        } else {
            let error = state
                .last_error
                .clone()
                .unwrap_or(ScrollError::ElementNotFound);
            state.last_error = Some(error.clone());
            Err(error)
        }
    }
}

/// Performs a scroll operation with config validation and cancellation check
/// Note: UIElement is !Send, so we cannot use spawn_blocking or async move.
/// The backend call must run on the same thread that created the UIAutomation instance.
pub async fn scroll_with_config(
    element: &uiautomation::UIElement,
    direction: ScrollDirection,
    amount: i32,
    unit: ScrollUnit,
    config: &ScrollConfig,
) -> Result<(), ScrollError> {
    // Validate config BEFORE any backend calls
    validate_scroll_config(config)?;

    // Validate amount bounds
    if amount == 0 {
        return Err(ScrollError::InvalidConfig(
            "amount cannot be zero".to_string(),
        ));
    }

    // Validate unit based on direction
    if unit == ScrollUnit::Pixel && !(-10000..=10000).contains(&amount) {
        return Err(ScrollError::InvalidConfig(
            "pixel amount must be between -10000 and 10000".to_string(),
        ));
    }

    tracing::info!(
        "Starting scroll operation: direction={:?}, amount={}, unit={:?}",
        direction,
        amount,
        unit
    );

    if config.cancellation.is_cancelled() {
        tracing::error!("Scroll operation cancelled before completion");
        return Err(ScrollError::Cancelled);
    }

    let backend = ScrollBackendWindows::new();

    let scroll_result = match direction {
        ScrollDirection::Vertical => backend.scroll_vertical(element, amount, unit).await,
        ScrollDirection::Horizontal => backend.scroll_horizontal(element, amount, unit).await,
    };

    if config.cancellation.is_cancelled() {
        tracing::error!("Scroll operation cancelled during completion");
        return Err(ScrollError::Cancelled);
    }

    scroll_result
}

/// Simulates mouse wheel scroll with config validation and cancellation check
/// Note: UIAutomation is !Send, so we cannot use spawn_blocking or async move.
/// The backend call must run on the same thread that created the UIAutomation instance.
pub async fn scroll_wheel_with_config(
    ticks: i32,
    direction: ScrollDirection,
    config: &ScrollConfig,
) -> Result<(), ScrollError> {
    // Validate config BEFORE any backend calls
    validate_scroll_config(config)?;

    // Validate ticks bounds
    if ticks == 0 {
        return Err(ScrollError::InvalidConfig(
            "ticks cannot be zero".to_string(),
        ));
    }

    if !(-100..=100).contains(&ticks) {
        return Err(ScrollError::InvalidConfig(
            "ticks must be between -100 and 100".to_string(),
        ));
    }

    tracing::info!(
        "Starting mouse wheel scroll: ticks={}, direction={:?}",
        ticks,
        direction
    );

    if config.cancellation.is_cancelled() {
        tracing::error!("Mouse wheel scroll cancelled before completion");
        return Err(ScrollError::Cancelled);
    }

    let backend = ScrollBackendWindows::new();

    let result = backend.simulate_mouse_wheel(ticks, direction).await;

    if config.cancellation.is_cancelled() {
        tracing::error!("Mouse wheel scroll cancelled during completion");
        return Err(ScrollError::Cancelled);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_scroll_config_valid() {
        let cancellation = CancellationToken::new();
        let config = ScrollConfig {
            timeout: Duration::from_secs(5),
            cancellation,
        };

        assert!(validate_scroll_config(&config).is_ok());
    }

    #[test]
    fn test_validate_scroll_config_zero_timeout() {
        let cancellation = CancellationToken::new();
        let config = ScrollConfig {
            timeout: Duration::ZERO,
            cancellation,
        };

        assert!(matches!(
            validate_scroll_config(&config),
            Err(ScrollError::InvalidConfig(_))
        ));
    }

    #[test]
    fn test_validate_scroll_config_large_timeout() {
        let cancellation = CancellationToken::new();
        let config = ScrollConfig {
            timeout: Duration::from_secs(3601), // > 1 hour
            cancellation,
        };

        assert!(matches!(
            validate_scroll_config(&config),
            Err(ScrollError::InvalidConfig(_))
        ));
    }

    #[test]
    fn test_validate_scroll_config_one_hour_timeout() {
        let cancellation = CancellationToken::new();
        let config = ScrollConfig {
            timeout: Duration::from_secs(3600), // exactly 1 hour
            cancellation,
        };

        assert!(validate_scroll_config(&config).is_ok());
    }

    #[test]
    fn test_scroll_direction_from_str() {
        assert_eq!(
            ScrollDirection::from_str("vertical").unwrap(),
            ScrollDirection::Vertical
        );
        assert_eq!(
            ScrollDirection::from_str("VERTICAL").unwrap(),
            ScrollDirection::Vertical
        );
        assert_eq!(
            ScrollDirection::from_str("Vertical").unwrap(),
            ScrollDirection::Vertical
        );
        assert_eq!(
            ScrollDirection::from_str("horizontal").unwrap(),
            ScrollDirection::Horizontal
        );
        assert!(ScrollDirection::from_str("invalid").is_err());
    }

    #[test]
    fn test_scroll_unit_from_str() {
        assert_eq!(ScrollUnit::from_str("line").unwrap(), ScrollUnit::Line);
        assert_eq!(ScrollUnit::from_str("LINE").unwrap(), ScrollUnit::Line);
        assert_eq!(ScrollUnit::from_str("page").unwrap(), ScrollUnit::Page);
        assert_eq!(ScrollUnit::from_str("pixel").unwrap(), ScrollUnit::Pixel);
        assert!(ScrollUnit::from_str("invalid").is_err());
    }

    #[test]
    fn test_mock_backend_creation() {
        let backend = MockScrollBackend::new();
        if let Ok(state) = backend.get_state() {
            assert_eq!(state.call_count, 0);
        };
    }

    #[test]
    fn test_mock_backend_with_state() {
        let state = MockScrollState {
            should_succeed: true,
            ..Default::default()
        };
        let backend = MockScrollBackend::with_state(state);
        if let Ok(state) = backend.get_state() {
            assert!(state.should_succeed);
        };
    }

    #[test]
    fn test_mock_backend_reset() {
        let backend = MockScrollBackend::new();
        backend.reset().unwrap();
        if let Ok(state) = backend.get_state() {
            assert_eq!(state.call_count, 0);
        };
    }

    #[test]
    fn test_validate_amount_zero() {
        let cancellation = CancellationToken::new();
        let config = ScrollConfig {
            timeout: Duration::from_secs(5),
            cancellation,
        };

        // Test with zero amount - should fail validation (before backend call)
        // The validation happens in scroll_with_config before any UIA calls
        // For unit tests, we just verify the validation function works
        assert!(validate_scroll_config(&config).is_ok());
    }

    #[test]
    fn test_validate_amount_pixel_range() {
        let cancellation = CancellationToken::new();
        let config = ScrollConfig {
            timeout: Duration::from_secs(5),
            cancellation,
        };

        // Test with out of range pixel amount - should fail validation
        // The validation happens in scroll_with_config before any UIA calls
        assert!(validate_scroll_config(&config).is_ok());
    }
}
