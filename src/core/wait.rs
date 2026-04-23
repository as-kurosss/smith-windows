//! Wait tool implementation
//! Provides UI element wait functionality (wait for existence or absence).

use std::time::Duration;
use thiserror::Error;
use tokio_util::sync::CancellationToken;

use crate::runtime::backends::windows::wait::WaitBackendWindows;

/// Mode for wait operation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WaitMode {
    /// Wait for element to exist (appear)
    Existence,
    /// Wait for element to cease to exist (disappear)
    Absence,
}

/// Configuration for wait operations
#[derive(Debug, Clone)]
pub struct WaitConfig {
    /// Total timeout for the wait operation
    pub timeout: Duration,
    /// Interval between checks
    pub interval: Duration,
    /// Mode: wait for existence or absence
    pub wait_for: WaitMode,
    /// Selector for finding the element (AutomationId, Name, or ControlType)
    pub selector: WaitSelector,
    /// Token for cancellation
    pub cancellation: CancellationToken,
}

/// Selector for finding elements
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WaitSelector {
    /// Find by automation ID
    AutomationId(String),
    /// Find by element name
    Name(String),
    /// Find by control type
    ControlType(String),
}

/// Errors that can occur during wait operations
#[derive(Error, Debug, Clone)]
pub enum WaitError {
    /// Invalid configuration
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
    /// Operation timed out
    #[error("Operation timed out")]
    Timeout,
    /// Operation was cancelled
    #[error("Operation was cancelled")]
    Cancelled,
    /// COM error
    #[error("COM error: {0}")]
    ComError(String),
}

/// Validates wait configuration
/// Must be called BEFORE backend invocation
pub fn validate_wait_config(config: &WaitConfig) -> Result<(), WaitError> {
    // Check timeout bounds: > 0 and <= 1 hour
    if config.timeout.is_zero() || config.timeout > Duration::from_secs(3600) {
        return Err(WaitError::InvalidConfig(
            "timeout must be > 0 and <= 1 hour".to_string(),
        ));
    }

    // Check interval bounds: > 0 and <= timeout
    if config.interval.is_zero() {
        return Err(WaitError::InvalidConfig("interval must be > 0".to_string()));
    }

    if config.interval > config.timeout {
        return Err(WaitError::InvalidConfig(
            "interval must be <= timeout".to_string(),
        ));
    }

    // Validate selector is not empty
    match &config.selector {
        WaitSelector::AutomationId(id) => {
            if id.is_empty() {
                return Err(WaitError::InvalidConfig(
                    "selector value cannot be empty".to_string(),
                ));
            }
        }
        WaitSelector::Name(name) => {
            if name.is_empty() {
                return Err(WaitError::InvalidConfig(
                    "selector value cannot be empty".to_string(),
                ));
            }
        }
        WaitSelector::ControlType(ty) => {
            if ty.is_empty() {
                return Err(WaitError::InvalidConfig(
                    "selector value cannot be empty".to_string(),
                ));
            }
        }
    }

    Ok(())
}

/// Trait for wait backend implementations
#[async_trait::async_trait(?Send)]
pub trait WaitBackend {
    /// Waits for an element based on config
    /// Returns Ok(true) if condition met, Ok(false) if timeout
    async fn wait_element(
        &self,
        automation: &uiautomation::UIAutomation,
        root: &uiautomation::UIElement,
        selector: &WaitSelector,
    ) -> Result<bool, WaitError>;
}

/// Mock backend for testing
/// Uses internal state to simulate different scenarios
#[derive(Debug, Clone, Default)]
pub struct MockWaitBackend {
    state: std::sync::Arc<std::sync::Mutex<MockWaitState>>,
}

/// State for mock backend
#[derive(Debug, Default)]
pub struct MockWaitState {
    pub call_count: usize,
    pub last_error: Option<WaitError>,
    pub should_succeed: bool,
    pub last_selector: Option<WaitSelector>,
}

impl MockWaitBackend {
    /// Creates a new mock backend with default state
    pub fn new() -> Self {
        Self {
            state: std::sync::Arc::new(std::sync::Mutex::new(MockWaitState::default())),
        }
    }

    /// Creates a mock backend with custom state
    pub fn with_state(state: MockWaitState) -> Self {
        Self {
            state: std::sync::Arc::new(std::sync::Mutex::new(state)),
        }
    }

    /// Gets a mutable reference to the state
    pub fn get_state(&self) -> Result<std::sync::MutexGuard<'_, MockWaitState>, WaitError> {
        self.state.lock().map_err(|e| {
            tracing::error!("State mutex poisoned: {}", e);
            WaitError::ComError("State mutex poisoned".into())
        })
    }

    /// Resets the backend state
    pub fn reset(&self) -> Result<(), WaitError> {
        let mut state = self.get_state()?;
        state.call_count = 0;
        state.last_error = None;
        state.last_selector = None;
        Ok(())
    }
}

#[async_trait::async_trait(?Send)]
impl WaitBackend for MockWaitBackend {
    async fn wait_element(
        &self,
        _automation: &uiautomation::UIAutomation,
        _root: &uiautomation::UIElement,
        selector: &WaitSelector,
    ) -> Result<bool, WaitError> {
        let mut state = self.get_state()?;
        state.call_count += 1;
        state.last_selector = Some(selector.clone());

        if state.should_succeed {
            state.last_error = None;
            Ok(true)
        } else {
            let error = state.last_error.clone().unwrap_or(WaitError::Timeout);
            state.last_error = Some(error.clone());
            Err(error)
        }
    }
}

/// Performs a wait operation with config validation and timeout handling
/// Note: UIAutomation and UIElement are !Send, so we cannot use spawn_blocking or async move.
/// The backend call runs on the same thread that created the UIAutomation instance.
pub async fn wait_with_config(
    automation: &uiautomation::UIAutomation,
    root: &uiautomation::UIElement,
    config: &WaitConfig,
) -> Result<bool, WaitError> {
    // Validate config BEFORE any backend calls
    validate_wait_config(config)?;

    tracing::info!(
        "Starting wait operation with timeout: {:?}, interval: {:?}, mode: {:?}, selector: {:?}",
        config.timeout,
        config.interval,
        config.wait_for,
        config.selector
    );

    let backend = WaitBackendWindows::new();

    // Calculate number of iterations
    let max_iterations = (config.timeout.as_millis() / config.interval.as_millis()) as u32;
    let mut iteration = 0u32;

    // Start timing
    let start_time = std::time::Instant::now();

    // Poll loop
    loop {
        // Check cancellation before each iteration
        if config.cancellation.is_cancelled() {
            tracing::error!("Wait operation cancelled");
            return Err(WaitError::Cancelled);
        }

        // Call backend to check element
        let check_result = backend
            .wait_element(automation, root, &config.selector)
            .await;

        // Check for cancellation after backend call
        if config.cancellation.is_cancelled() {
            tracing::error!("Wait operation cancelled during backend call");
            return Err(WaitError::Cancelled);
        }

        match check_result {
            Ok(found) => {
                // Check if condition is met based on mode
                let condition_met = match config.wait_for {
                    WaitMode::Existence => found, // Found = true means condition met
                    WaitMode::Absence => !found,  // Not found = true means condition met
                };

                if condition_met {
                    let elapsed = start_time.elapsed();
                    tracing::info!(
                        "Wait condition met after {:?} (iteration {}/{})",
                        elapsed,
                        iteration + 1,
                        max_iterations + 1
                    );
                    return Ok(true);
                }

                // Increment iteration
                iteration += 1;

                // Check if we've exceeded max iterations
                if iteration > max_iterations {
                    let elapsed = start_time.elapsed();
                    tracing::info!(
                        "Wait timeout after {:?} (max iterations: {})",
                        elapsed,
                        max_iterations + 1
                    );
                    return Ok(false);
                }

                // Sleep for interval
                tokio::time::sleep(config.interval).await;
            }
            Err(e) => {
                // Error from backend - return immediately
                tracing::error!("Wait operation failed: {}", e);
                return Err(e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_wait_config_valid_existence() {
        let cancellation = CancellationToken::new();
        let config = WaitConfig {
            timeout: Duration::from_secs(5),
            interval: Duration::from_millis(100),
            wait_for: WaitMode::Existence,
            selector: WaitSelector::AutomationId("myButton".to_string()),
            cancellation,
        };

        assert!(validate_wait_config(&config).is_ok());
    }

    #[test]
    fn test_validate_wait_config_valid_absence() {
        let cancellation = CancellationToken::new();
        let config = WaitConfig {
            timeout: Duration::from_secs(10),
            interval: Duration::from_millis(500),
            wait_for: WaitMode::Absence,
            selector: WaitSelector::Name("dialog".to_string()),
            cancellation: cancellation,
        };

        assert!(validate_wait_config(&config).is_ok());
    }

    #[test]
    fn test_validate_wait_config_zero_timeout() {
        let cancellation = CancellationToken::new();
        let config = WaitConfig {
            timeout: Duration::ZERO,
            interval: Duration::from_millis(100),
            wait_for: WaitMode::Existence,
            selector: WaitSelector::AutomationId("myButton".to_string()),
            cancellation: cancellation,
        };

        assert!(matches!(
            validate_wait_config(&config),
            Err(WaitError::InvalidConfig(_))
        ));
    }

    #[test]
    fn test_validate_wait_config_zero_interval() {
        let cancellation = CancellationToken::new();
        let config = WaitConfig {
            timeout: Duration::from_secs(5),
            interval: Duration::ZERO,
            wait_for: WaitMode::Existence,
            selector: WaitSelector::AutomationId("myButton".to_string()),
            cancellation,
        };

        assert!(matches!(
            validate_wait_config(&config),
            Err(WaitError::InvalidConfig(_))
        ));
    }

    #[test]
    fn test_validate_wait_config_interval_greater_than_timeout() {
        let cancellation = CancellationToken::new();
        let config = WaitConfig {
            timeout: Duration::from_secs(1),
            interval: Duration::from_secs(5),
            wait_for: WaitMode::Existence,
            selector: WaitSelector::AutomationId("myButton".to_string()),
            cancellation,
        };

        assert!(matches!(
            validate_wait_config(&config),
            Err(WaitError::InvalidConfig(_))
        ));
    }

    #[test]
    fn test_validate_wait_config_large_timeout() {
        let cancellation = CancellationToken::new();
        let config = WaitConfig {
            timeout: Duration::from_secs(3601),
            interval: Duration::from_millis(100),
            wait_for: WaitMode::Existence,
            selector: WaitSelector::AutomationId("myButton".to_string()),
            cancellation,
        };

        assert!(matches!(
            validate_wait_config(&config),
            Err(WaitError::InvalidConfig(_))
        ));
    }

    #[test]
    fn test_validate_wait_config_empty_selector() {
        let cancellation = CancellationToken::new();
        let config = WaitConfig {
            timeout: Duration::from_secs(5),
            interval: Duration::from_millis(100),
            wait_for: WaitMode::Existence,
            selector: WaitSelector::AutomationId("".to_string()),
            cancellation,
        };

        assert!(matches!(
            validate_wait_config(&config),
            Err(WaitError::InvalidConfig(_))
        ));
    }

    #[test]
    fn test_mock_backend_creation() {
        let backend = MockWaitBackend::new();
        assert_eq!(backend.get_state().unwrap().call_count, 0);
    }

    #[test]
    fn test_mock_backend_with_state() {
        let state = MockWaitState {
            should_succeed: true,
            ..Default::default()
        };
        let backend = MockWaitBackend::with_state(state);
        assert!(backend.get_state().unwrap().should_succeed);
    }

    #[test]
    fn test_mock_backend_reset() {
        let backend = MockWaitBackend::new();
        backend.reset().unwrap();
        assert_eq!(backend.get_state().unwrap().call_count, 0);
    }

    #[test]
    fn test_wait_mode_enum() {
        assert_eq!(WaitMode::Existence, WaitMode::Existence);
        assert_eq!(WaitMode::Absence, WaitMode::Absence);
        assert_ne!(WaitMode::Existence, WaitMode::Absence);
    }

    #[test]
    fn test_wait_selector_enum() {
        let selector1 = WaitSelector::AutomationId("id1".to_string());
        let selector2 = WaitSelector::AutomationId("id2".to_string());
        let selector3 = WaitSelector::Name("name1".to_string());
        let selector4 = WaitSelector::ControlType("button".to_string());

        assert_eq!(selector1, selector1);
        assert_ne!(selector1, selector2);
        assert_ne!(selector1, selector3);
        assert_ne!(selector1, selector4);
        assert_ne!(selector3, selector4);
    }
}
