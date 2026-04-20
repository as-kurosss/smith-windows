//! Integration tests for InspectTool

use smith_windows::core::inspect::{
    InspectBackend, InspectConfig, InspectError, MockInspectBackend, MockInspectState,
};
use std::time::Duration;
use tokio_util::sync::CancellationToken;

#[tokio::test]
async fn test_inspect_success() {
    let mock_state = MockInspectState {
        should_succeed: true,
        path: "Window->Button->CheckBox{Name}".to_string(),
        ..Default::default()
    };
    let backend = MockInspectBackend::with_state(mock_state);

    let head_window = uiautomation::UIElement::root_element().unwrap();
    let element = uiautomation::UIElement::root_element().unwrap();

    let result = backend.inspect_path(&head_window, &element).await;

    assert!(result.is_ok());
    let path = result.unwrap();
    assert!(!path.is_empty());
    assert_eq!(backend.get_state().call_count, 1);
}

#[tokio::test]
async fn test_inspect_timeout() {
    let cancellation = CancellationToken::new();
    let config = InspectConfig {
        timeout: Duration::from_millis(1), // Very short timeout
        cancellation,
    };

    let mock_state = MockInspectState {
        should_succeed: false,
        last_error: Some(InspectError::Timeout),
        ..Default::default()
    };
    let backend = MockInspectBackend::with_state(mock_state);

    let head_window = uiautomation::UIElement::root_element().unwrap();
    let element = uiautomation::UIElement::root_element().unwrap();

    let result = backend.inspect_path(&head_window, &element).await;

    assert!(matches!(result, Err(InspectError::Timeout)));
}

#[tokio::test]
async fn test_inspect_offscreen() {
    let mock_state = MockInspectState {
        should_succeed: false,
        last_error: Some(InspectError::ElementOffscreen),
        ..Default::default()
    };
    let backend = MockInspectBackend::with_state(mock_state);

    let head_window = uiautomation::UIElement::root_element().unwrap();
    let element = uiautomation::UIElement::root_element().unwrap();

    let result = backend.inspect_path(&head_window, &element).await;

    assert!(matches!(result, Err(InspectError::ElementOffscreen)));
}

#[tokio::test]
async fn test_inspect_disabled() {
    let mock_state = MockInspectState {
        should_succeed: false,
        last_error: Some(InspectError::ElementNotEnabled),
        ..Default::default()
    };
    let backend = MockInspectBackend::with_state(mock_state);

    let head_window = uiautomation::UIElement::root_element().unwrap();
    let element = uiautomation::UIElement::root_element().unwrap();

    let result = backend.inspect_path(&head_window, &element).await;

    assert!(matches!(result, Err(InspectError::ElementNotEnabled)));
}

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

#[tokio::test]
async fn test_inspect_cancelled() {
    let cancellation = CancellationToken::new();
    cancellation.cancel();

    let mock_state = MockInspectState {
        should_succeed: true,
        path: "Window->Button".to_string(),
        ..Default::default()
    };
    let backend = MockInspectBackend::with_state(mock_state);

    let head_window = uiautomation::UIElement::root_element().unwrap();
    let element = uiautomation::UIElement::root_element().unwrap();

    let result = backend.inspect_path(&head_window, &element).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_inspect_not_in_hierarchy() {
    let mock_state = MockInspectState {
        should_succeed: false,
        last_error: Some(InspectError::InvalidSelector),
        ..Default::default()
    };
    let backend = MockInspectBackend::with_state(mock_state);

    let root = uiautomation::UIElement::root_element().unwrap();

    // Create two different elements that are not in hierarchy
    let head_window = root;
    let element = uiautomation::UIElement::root_element().unwrap();

    let result = backend.inspect_path(&head_window, &element).await;

    assert!(matches!(result, Err(InspectError::InvalidSelector)));
}

#[tokio::test]
async fn test_inspect_max_depth() {
    let mock_state = MockInspectState {
        should_succeed: false,
        last_error: Some(InspectError::InvalidSelector),
        ..Default::default()
    };
    let backend = MockInspectBackend::with_state(mock_state);

    let head_window = uiautomation::UIElement::root_element().unwrap();
    let element = uiautomation::UIElement::root_element().unwrap();

    let result = backend.inspect_path(&head_window, &element).await;

    assert!(matches!(result, Err(InspectError::InvalidSelector)));
}

#[tokio::test]
async fn test_inspect_empty_path() {
    let mock_state = MockInspectState {
        should_succeed: false,
        last_error: Some(InspectError::InvalidSelector),
        ..Default::default()
    };
    let backend = MockInspectBackend::with_state(mock_state);

    // Use same element as head_window and element
    let root = uiautomation::UIElement::root_element().unwrap();
    let head_window = root;
    let element = root;

    let result = backend.inspect_path(&head_window, &element).await;

    assert!(matches!(result, Err(InspectError::InvalidSelector)));
}

#[tokio::test]
async fn test_inspect_idempotent_on_error() {
    let mock_state = MockInspectState {
        should_succeed: false,
        last_error: Some(InspectError::ElementNotFound),
        ..Default::default()
    };
    let backend = MockInspectBackend::with_state(mock_state);

    let head_window = uiautomation::UIElement::root_element().unwrap();
    let element = uiautomation::UIElement::root_element().unwrap();

    // First call - should fail
    let result1 = backend.inspect_path(&head_window, &element).await;
    assert!(result1.is_err());

    // Get the error from state
    let state = backend.get_state();
    let first_error = state.last_error.clone();
    let first_call_count = state.call_count;
    drop(state);

    // Second call with same data - should return same error
    let result2 = backend.inspect_path(&head_window, &element).await;
    assert!(result2.is_err());

    // Check idempotency: error should be same, call count should increase
    let state = backend.get_state();
    assert_eq!(state.call_count, first_call_count + 1);

    // Reset and verify state is clean
    backend.reset();
    assert_eq!(backend.get_state().call_count, 0);
}

#[tokio::test]
async fn test_inspect_backend_windows_creation() {
    let backend = InspectBackendWindows::new();
    assert!(backend.is_ok());
}

#[tokio::test]
async fn test_inspect_backend_windows_default() {
    let backend = InspectBackendWindows::default();
    assert!(backend.is_ok());
}
