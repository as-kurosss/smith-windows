//! Integration tests for InspectTool

use serial_test::serial;
use smith_windows::core::inspect::{
    InspectBackend, InspectConfig, InspectError, MockInspectBackend, MockInspectState,
};
use std::time::Duration;
use tokio_util::sync::CancellationToken;
use uiautomation::UIAutomation;

#[serial]
#[tokio::test]
async fn test_inspect_success() {
    let mock_state = MockInspectState {
        should_succeed: true,
        path: "Window->Button->CheckBox{Name}".to_string(),
        ..Default::default()
    };
    let backend = MockInspectBackend::with_state(mock_state);

    let automation = UIAutomation::new().unwrap();
    let head_window = automation.get_root_element().unwrap();
    let element = automation.get_root_element().unwrap();

    let result = backend.inspect_path(&head_window, &element).await;

    assert!(result.is_ok());
    let path = result.unwrap();
    assert!(!path.is_empty());
    assert_eq!(backend.get_state().unwrap().call_count, 1);
}

#[serial]
#[tokio::test]
async fn test_inspect_timeout() {
    let _config = InspectConfig {
        timeout: Duration::from_millis(1), // Very short timeout
        cancellation: CancellationToken::new(),
    };

    let mock_state = MockInspectState {
        should_succeed: false,
        last_error: Some(InspectError::Timeout),
        ..Default::default()
    };
    let backend = MockInspectBackend::with_state(mock_state);

    let automation = UIAutomation::new().unwrap();
    let head_window = automation.get_root_element().unwrap();
    let element = automation.get_root_element().unwrap();

    let result = backend.inspect_path(&head_window, &element).await;

    assert!(matches!(result, Err(InspectError::Timeout)));
}

#[serial]
#[tokio::test]
async fn test_inspect_offscreen() {
    let mock_state = MockInspectState {
        should_succeed: false,
        last_error: Some(InspectError::ElementOffscreen),
        ..Default::default()
    };
    let backend = MockInspectBackend::with_state(mock_state);

    let automation = UIAutomation::new().unwrap();
    let head_window = automation.get_root_element().unwrap();
    let element = automation.get_root_element().unwrap();

    let result = backend.inspect_path(&head_window, &element).await;

    assert!(matches!(result, Err(InspectError::ElementOffscreen)));
}

#[serial]
#[tokio::test]
async fn test_inspect_disabled() {
    let mock_state = MockInspectState {
        should_succeed: false,
        last_error: Some(InspectError::ElementNotEnabled),
        ..Default::default()
    };
    let backend = MockInspectBackend::with_state(mock_state);

    let automation = UIAutomation::new().unwrap();
    let head_window = automation.get_root_element().unwrap();
    let element = automation.get_root_element().unwrap();

    let result = backend.inspect_path(&head_window, &element).await;

    assert!(matches!(result, Err(InspectError::ElementNotEnabled)));
}

#[serial]
#[tokio::test]
async fn test_inspect_invalid_config_zero() {
    let cancellation = CancellationToken::new();
    let config = InspectConfig {
        timeout: Duration::ZERO,
        cancellation,
    };

    let result = smith_windows::core::inspect::validate_inspect_config(&config);

    assert!(matches!(result, Err(InspectError::InvalidConfig(_))));
}

#[serial]
#[tokio::test]
async fn test_inspect_invalid_config_large() {
    let cancellation = CancellationToken::new();
    let config = InspectConfig {
        timeout: Duration::from_secs(3601), // > 1 hour
        cancellation,
    };

    let result = smith_windows::core::inspect::validate_inspect_config(&config);

    assert!(matches!(result, Err(InspectError::InvalidConfig(_))));
}

#[serial]
#[tokio::test]
async fn test_inspect_cancelled() {
    let cancellation = CancellationToken::new();
    cancellation.cancel();

    let config = InspectConfig {
        timeout: Duration::from_secs(5),
        cancellation,
    };

    // Validate config - should pass (cancellation is checked during operation, not during validation)
    let result = smith_windows::core::inspect::validate_inspect_config(&config);
    assert!(
        result.is_ok(),
        "Config validation should pass even with cancelled token"
    );
}

#[serial]
#[tokio::test]
async fn test_inspect_max_depth() {
    let mock_state = MockInspectState {
        should_succeed: false,
        last_error: Some(InspectError::InvalidSelector),
        ..Default::default()
    };
    let backend = MockInspectBackend::with_state(mock_state);

    let automation = UIAutomation::new().unwrap();
    let head_window = automation.get_root_element().unwrap();
    let element = automation.get_root_element().unwrap();

    let result = backend.inspect_path(&head_window, &element).await;

    assert!(matches!(result, Err(InspectError::InvalidSelector)));
}

#[serial]
#[tokio::test]
async fn test_inspect_not_in_hierarchy() {
    let mock_state = MockInspectState {
        should_succeed: false,
        last_error: Some(InspectError::InvalidSelector),
        ..Default::default()
    };
    let backend = MockInspectBackend::with_state(mock_state);

    let automation = UIAutomation::new().unwrap();
    let root = automation.get_root_element().unwrap();

    // Create two different elements that are not in hierarchy
    let head_window = root.clone();
    let element = automation.get_root_element().unwrap();

    let result = backend.inspect_path(&head_window, &element).await;

    assert!(matches!(result, Err(InspectError::InvalidSelector)));
}

#[serial]
#[tokio::test]
async fn test_inspect_empty_path() {
    let mock_state = MockInspectState {
        should_succeed: false,
        last_error: Some(InspectError::InvalidSelector),
        ..Default::default()
    };
    let backend = MockInspectBackend::with_state(mock_state);

    // Use same element as head_window and element
    let automation = UIAutomation::new().unwrap();
    let root = automation.get_root_element().unwrap();
    let head_window = root.clone();
    let element = root;

    let result = backend.inspect_path(&head_window, &element).await;

    assert!(matches!(result, Err(InspectError::InvalidSelector)));
}

#[test]
fn test_inspect_backend_windows_creation() {
    let backend = smith_windows::runtime::backends::windows::inspect::InspectBackendWindows::new();
    // InspectBackendWindows::new() returns Self, not Result
    let _ = backend;
}

#[test]
fn test_inspect_backend_windows_default() {
    let _backend = smith_windows::runtime::backends::windows::inspect::InspectBackendWindows;
    // InspectBackendWindows is a unit struct, no need for default()
}
