//! Integration tests for WindowControlTool

use serial_test::serial;
use smith_windows::core::window_control::{
    validate_window_control_config, MockWindowControlBackend, WindowControlAction,
    WindowControlBackend,
};
use std::time::Duration;
use tokio_util::sync::CancellationToken;

#[cfg(not(target_os = "windows"))]
use smith_windows::runtime::backends::unsupported::WindowControlBackendUnsupported;

/// Test successful maximize operation (mock)
#[serial]
#[tokio::test]
async fn test_maximize_mock_success() {
    let backend = MockWindowControlBackend::new();
    let state = backend.state();
    tracing::info!("Backend state: should_succeed={}", state.should_succeed);
    drop(state);

    let automation = uiautomation::UIAutomation::new().unwrap();
    let root = automation.get_root_element().unwrap();
    let action = WindowControlAction::Maximize;

    let result = backend.window_control(&root, action).await;

    tracing::info!("Result: {:?}", result);
    assert!(result.is_ok());
}

/// Test successful restore operation (mock)
#[serial]
#[tokio::test]
async fn test_restore_mock_success() {
    let backend = MockWindowControlBackend::new();
    let automation = uiautomation::UIAutomation::new().unwrap();
    let root = automation.get_root_element().unwrap();
    let action = WindowControlAction::Restore;

    let result = backend.window_control(&root, action).await;

    assert!(result.is_ok());
}

/// Test successful minimize operation (mock)
#[serial]
#[tokio::test]
async fn test_minimize_mock_success() {
    let backend = MockWindowControlBackend::new();
    let automation = uiautomation::UIAutomation::new().unwrap();
    let root = automation.get_root_element().unwrap();
    let action = WindowControlAction::Minimize;

    let result = backend.window_control(&root, action).await;

    assert!(result.is_ok());
}

/// Test error handling with mock (should_fail)
#[serial]
#[tokio::test]
async fn test_mock_error_handling() {
    let backend = MockWindowControlBackend::with_state(
        smith_windows::core::window_control::MockWindowControlState {
            should_succeed: false,
            ..Default::default()
        },
    );
    let automation = uiautomation::UIAutomation::new().unwrap();
    let root = automation.get_root_element().unwrap();
    let action = WindowControlAction::Maximize;

    let result = backend.window_control(&root, action).await;

    assert!(matches!(
        result,
        Err(smith_windows::core::window_control::WindowControlError::ElementNotFound)
    ));
}

/// Test valid configuration
#[tokio::test]
async fn test_valid_config() {
    let cancellation = CancellationToken::new();
    let config = smith_windows::core::window_control::WindowControlConfig {
        action: WindowControlAction::Maximize,
        timeout: Duration::from_secs(5),
        cancellation,
    };

    let result = validate_window_control_config(&config);
    assert!(result.is_ok());
}

/// Test invalid configuration - zero timeout
#[tokio::test]
async fn test_invalid_config_zero_timeout() {
    let cancellation = CancellationToken::new();
    let config = smith_windows::core::window_control::WindowControlConfig {
        action: WindowControlAction::Maximize,
        timeout: Duration::ZERO,
        cancellation,
    };

    let result = validate_window_control_config(&config);
    assert!(matches!(
        result,
        Err(smith_windows::core::window_control::WindowControlError::InvalidConfig(_))
    ));
}

/// Test invalid configuration - large timeout
#[tokio::test]
async fn test_invalid_config_large_timeout() {
    let cancellation = CancellationToken::new();
    let config = smith_windows::core::window_control::WindowControlConfig {
        action: WindowControlAction::Maximize,
        timeout: Duration::from_secs(3601),
        cancellation,
    };

    let result = validate_window_control_config(&config);
    assert!(matches!(
        result,
        Err(smith_windows::core::window_control::WindowControlError::InvalidConfig(_))
    ));
}

/// Test idempotency - repeated calls with mock succeed
#[serial]
#[tokio::test]
async fn test_idempotency_mock() {
    let backend = MockWindowControlBackend::new();
    let automation = uiautomation::UIAutomation::new().unwrap();
    let root = automation.get_root_element().unwrap();

    // First call
    let result1 = backend
        .window_control(&root, WindowControlAction::Maximize)
        .await;
    assert!(result1.is_ok());

    // Second call should also succeed (idempotent)
    let result2 = backend
        .window_control(&root, WindowControlAction::Restore)
        .await;
    assert!(result2.is_ok());
}

/// Test unsupported platform backend
#[cfg(not(target_os = "windows"))]
#[tokio::test]
async fn test_unsupported_platform_backend() {
    let backend = WindowControlBackendUnsupported::new();
    let automation = uiautomation::UIAutomation::new().unwrap();
    let root = automation.get_root_element().unwrap();

    let result = backend
        .window_control(&root, WindowControlAction::Maximize)
        .await;

    assert!(matches!(
        result,
        Err(smith_windows::core::window_control::WindowControlError::UnsupportedPlatform)
    ));
}

/// Test all action variants
#[serial]
#[tokio::test]
async fn test_all_action_variants() {
    let backend = MockWindowControlBackend::new();
    let automation = uiautomation::UIAutomation::new().unwrap();
    let root = automation.get_root_element().unwrap();

    // Test Maximize
    let result = backend
        .window_control(&root, WindowControlAction::Maximize)
        .await;
    assert!(result.is_ok());

    // Test Restore
    let result = backend
        .window_control(&root, WindowControlAction::Restore)
        .await;
    assert!(result.is_ok());

    // Test Minimize
    let result = backend
        .window_control(&root, WindowControlAction::Minimize)
        .await;
    assert!(result.is_ok());
}

/// Test mock backend state tracking
#[serial]
#[tokio::test]
async fn test_mock_backend_state_tracking() {
    let backend = MockWindowControlBackend::new();
    let automation = uiautomation::UIAutomation::new().unwrap();
    let root = automation.get_root_element().unwrap();

    // Perform an operation
    backend
        .window_control(&root, WindowControlAction::Maximize)
        .await
        .unwrap();

    // Check state
    let state = backend.state();
    assert_eq!(state.call_count, 1);
    assert_eq!(state.last_action, Some(WindowControlAction::Maximize));
    assert!(state.last_error.is_none());
}

/// Test mock backend reset
#[serial]
#[tokio::test]
async fn test_mock_backend_reset() {
    let backend = MockWindowControlBackend::new();

    // Perform some operations
    let automation = uiautomation::UIAutomation::new().unwrap();
    let root = automation.get_root_element().unwrap();
    backend
        .window_control(&root, WindowControlAction::Maximize)
        .await
        .unwrap();

    // Reset
    backend.reset();

    // Check state
    let state = backend.state();
    assert_eq!(state.call_count, 0);
    assert!(state.last_action.is_none());
    assert!(state.last_error.is_none());
}
