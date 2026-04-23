//! Integration tests for FocusTool

use std::time::Duration;
use smith_windows::core::focus::{
    validate_config, FocusBackend, FocusBackendWindows, FocusConfig, FocusError, MockFocusBackend,
};
use tokio_util::sync::CancellationToken;

/// Test successful focus operation (mock)
#[tokio::test]
async fn test_focus_mock_success() {
    let backend = MockFocusBackend::new();

    // Create a minimal UIElement for testing
    let automation = uiautomation::UIAutomation::new().unwrap();
    let root = automation.get_root_element().unwrap();

    let result = backend.focus(&root).await;

    // Mock succeeds by default
    assert!(result.is_ok());
}

/// Test focus operation with error (mock)
#[tokio::test]
async fn test_focus_mock_error() {
    let state = smith_windows::core::focus::MockFocusState {
        should_succeed: false,
        ..Default::default()
    };
    let backend = MockFocusBackend::with_state(state);

    let automation = uiautomation::UIAutomation::new().unwrap();
    let root = automation.get_root_element().unwrap();

    let result = backend.focus(&root).await;

    assert!(matches!(result, Err(FocusError::ElementNotFound)));
}

/// Test valid configuration
#[tokio::test]
async fn test_valid_config() {
    let cancellation = CancellationToken::new();
    let config = FocusConfig {
        timeout: Duration::from_secs(5),
        cancellation,
    };

    let result = validate_config(&config);
    assert!(result.is_ok());
}

/// Test invalid configuration - zero timeout
#[tokio::test]
async fn test_invalid_config_zero_timeout() {
    let cancellation = CancellationToken::new();
    let config = FocusConfig {
        timeout: Duration::ZERO,
        cancellation,
    };

    let result = validate_config(&config);
    assert!(matches!(result, Err(FocusError::InvalidConfig(_))));
}

/// Test invalid configuration - large timeout
#[tokio::test]
async fn test_invalid_config_large_timeout() {
    let cancellation = CancellationToken::new();
    let config = FocusConfig {
        timeout: Duration::from_secs(3601), // > 1 hour
        cancellation,
    };

    let result = validate_config(&config);
    assert!(matches!(result, Err(FocusError::InvalidConfig(_))));
}

/// Test idempotency - repeated calls return same error
#[tokio::test]
async fn test_idempotency_error() {
    let state = smith_windows::core::focus::MockFocusState {
        should_succeed: false,
        ..Default::default()
    };
    let backend = MockFocusBackend::with_state(state);

    let automation = uiautomation::UIAutomation::new().unwrap();
    let root = automation.get_root_element().unwrap();

    // First call
    let result1 = backend.focus(&root).await;
    assert!(result1.is_err());

    // Second call should return same error
    let result2 = backend.focus(&root).await;
    assert!(result2.is_err());

    // Both should return same error type
    assert_eq!(format!("{:?}", result1.unwrap_err()), format!("{:?}", result2.unwrap_err()));
}

/// Test idempotency - repeated calls return same success
#[tokio::test]
async fn test_idempotency_success() {
    let backend = MockFocusBackend::new();

    let automation = uiautomation::UIAutomation::new().unwrap();
    let root = automation.get_root_element().unwrap();

    // First call
    let result1 = backend.focus(&root).await;
    assert!(result1.is_ok());

    // Second call should return same success
    let result2 = backend.focus(&root).await;
    assert!(result2.is_ok());
}

/// Test backend state isolation
#[tokio::test]
async fn test_backend_state_isolation() {
    let backend1 = MockFocusBackend::new();
    let backend2 = MockFocusBackend::new();

    let automation = uiautomation::UIAutomation::new().unwrap();
    let root = automation.get_root_element().unwrap();

    // Focus with backend1
    let _ = backend1.focus(&root).await;

    // Backend2 should have 0 calls
    assert_eq!(backend2.get_state().call_count, 0);

    // Backend1 should have 1 call
    assert_eq!(backend1.get_state().call_count, 1);
}

/// Test cancellation token
#[tokio::test]
async fn test_cancellation_token() {
    let cancellation = CancellationToken::new();
    let config = FocusConfig {
        timeout: Duration::from_secs(5),
        cancellation: cancellation.clone(),
    };

    // Cancel before operation
    cancellation.cancel();

    let automation = uiautomation::UIAutomation::new().unwrap();
    let root = automation.get_root_element().unwrap();

    // Note: focus_with_config checks cancellation AFTER backend call
    // For proper cancellation testing, we'd need async operation with cancellation during wait
    // This test verifies the token can be created and checked
    assert!(config.cancellation.is_cancelled());
}

/// Test Windows backend creation
#[tokio::test]
async fn test_windows_backend_creation() {
    let backend = FocusBackendWindows::new();
    // Just verify we can create it - actual focus requires UI elements
    assert!(std::ptr::addr_of!(backend) as usize > 0);
}

/// Test backend reset
#[tokio::test]
async fn test_backend_reset() {
    let backend = MockFocusBackend::new();
    
    // Simulate some calls
    let automation = uiautomation::UIAutomation::new().unwrap();
    let root = automation.get_root_element().unwrap();
    let _ = backend.focus(&root).await;

    // Reset
    backend.reset();

    // State should be reset
    assert_eq!(backend.get_state().call_count, 0);
}

/// Test with minimum valid timeout
#[tokio::test]
async fn test_minimum_valid_timeout() {
    let cancellation = CancellationToken::new();
    let config = FocusConfig {
        timeout: Duration::from_secs(1),
        cancellation,
    };

    let result = validate_config(&config);
    assert!(result.is_ok());
}

/// Test with maximum valid timeout
#[tokio::test]
async fn test_maximum_valid_timeout() {
    let cancellation = CancellationToken::new();
    let config = FocusConfig {
        timeout: Duration::from_secs(3600),
        cancellation,
    };

    let result = validate_config(&config);
    assert!(result.is_ok());
}
