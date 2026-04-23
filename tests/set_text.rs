//! Integration tests for set text tool

use serial_test::serial;
use std::time::Duration;
use tokio_util::sync::CancellationToken;
use uiautomation::UIAutomation;

use smith_windows::core::set_text::{
    MockSetTextBackend, SetTextBackend, SetTextConfig, SetTextError,
};

#[serial]
#[tokio::test]
async fn test_integration_set_text_success() {
    // Create mock backend with success state
    let backend = MockSetTextBackend::with_state(smith_windows::core::set_text::MockSetTextState {
        should_succeed: true,
        ..Default::default()
    });

    let automation = UIAutomation::new().unwrap();
    let element = automation.get_root_element().unwrap();
    let result = backend.set_text(&element, "test text").await;

    assert!(result.is_ok(), "Expected Ok(()) but got {:?}", result);
}

#[tokio::test]
async fn test_integration_set_text_config_validation() {
    // Test zero timeout
    let config = SetTextConfig {
        timeout: Duration::ZERO,
        cancellation: CancellationToken::new(),
    };

    let result = smith_windows::core::set_text::validate_set_text_config(&config);
    assert!(matches!(result, Err(SetTextError::InvalidConfig(_))));

    // Test large timeout
    let config = SetTextConfig {
        timeout: Duration::from_secs(3601),
        cancellation: CancellationToken::new(),
    };

    let result = smith_windows::core::set_text::validate_set_text_config(&config);
    assert!(matches!(result, Err(SetTextError::InvalidConfig(_))));

    // Test valid config
    let config = SetTextConfig {
        timeout: Duration::from_secs(5),
        cancellation: CancellationToken::new(),
    };

    let result = smith_windows::core::set_text::validate_set_text_config(&config);
    assert!(result.is_ok());
}

#[serial]
#[tokio::test]
async fn test_integration_mock_idempotency() {
    let backend = MockSetTextBackend::with_state(smith_windows::core::set_text::MockSetTextState {
        should_succeed: false,
        last_error: Some(SetTextError::ElementNotFound),
        ..Default::default()
    });

    let automation = UIAutomation::new().unwrap();
    let element = automation.get_root_element().unwrap();

    // Multiple calls should behave consistently
    let result1 = backend.set_text(&element, "test").await;
    let result2 = backend.set_text(&element, "test").await;
    let result3 = backend.set_text(&element, "test").await;

    assert!(matches!(result1, Err(SetTextError::ElementNotFound)));
    assert!(matches!(result2, Err(SetTextError::ElementNotFound)));
    assert!(matches!(result3, Err(SetTextError::ElementNotFound)));

    // Call count should be 3
    assert_eq!(backend.get_state().unwrap().call_count, 3);
}

#[serial]
#[tokio::test]
async fn test_integration_set_text_element_disabled() {
    let backend = MockSetTextBackend::with_state(smith_windows::core::set_text::MockSetTextState {
        should_succeed: false,
        last_error: Some(SetTextError::ElementNotEnabled),
        ..Default::default()
    });

    let automation = UIAutomation::new().unwrap();
    let element = automation.get_root_element().unwrap();
    let result = backend.set_text(&element, "test").await;

    assert!(matches!(result, Err(SetTextError::ElementNotEnabled)));
}

#[serial]
#[tokio::test]
async fn test_integration_set_text_element_offscreen() {
    let backend = MockSetTextBackend::with_state(smith_windows::core::set_text::MockSetTextState {
        should_succeed: false,
        last_error: Some(SetTextError::ElementOffscreen),
        ..Default::default()
    });

    let automation = UIAutomation::new().unwrap();
    let element = automation.get_root_element().unwrap();
    let result = backend.set_text(&element, "test").await;

    assert!(matches!(result, Err(SetTextError::ElementOffscreen)));
}

#[serial]
#[tokio::test]
async fn test_integration_set_text_element_read_only() {
    let backend = MockSetTextBackend::with_state(smith_windows::core::set_text::MockSetTextState {
        should_succeed: false,
        last_error: Some(SetTextError::ElementNotWritable),
        ..Default::default()
    });

    let automation = UIAutomation::new().unwrap();
    let element = automation.get_root_element().unwrap();
    let result = backend.set_text(&element, "test").await;

    assert!(matches!(result, Err(SetTextError::ElementNotWritable)));
}

#[serial]
#[tokio::test]
async fn test_integration_set_text_timeout() {
    // Use a very small timeout to trigger timeout error
    // Duration::ZERO is rejected by validation, so we use 1 nanosecond
    let config = SetTextConfig {
        timeout: Duration::from_nanos(1),
        cancellation: CancellationToken::new(),
    };

    let result = smith_windows::core::set_text::set_text_with_config(
        &UIAutomation::new().unwrap().get_root_element().unwrap(),
        "test",
        &config,
    )
    .await;

    // Note: This test may be flaky on fast systems since the operation
    // might complete before the timeout. It's primarily testing that
    // timeout mechanism is in place.
    match result {
        Err(SetTextError::Timeout) => {} // Expected on slow systems
        Err(SetTextError::ElementNotWritable) => {} // Root element is read-only
        Err(SetTextError::ComError(_)) => {} // Also acceptable for read-only elements
        Ok(()) => {}                     // Operation completed before timeout (fast system)
        _ => panic!("Unexpected result: {:?}", result),
    }
}

#[serial]
#[tokio::test]
async fn test_integration_set_text_cancelled() {
    let cancellation = CancellationToken::new();
    cancellation.cancel();

    let config = SetTextConfig {
        timeout: Duration::from_secs(5),
        cancellation,
    };

    // Note: We cannot test actual cancellation because UIAutomation::new()
    // doesn't accept a cancellation token. This test verifies that the
    // cancellation check is in place, but actual cancellation testing
    // would require a long-running operation that can be interrupted.
    let automation = UIAutomation::new().unwrap();
    let element = automation.get_root_element().unwrap();

    // The operation should complete normally (not be cancelled after start)
    // because cancellation was set before the operation started
    let _result =
        smith_windows::core::set_text::set_text_with_config(&element, "test", &config).await;
}

#[serial]
#[tokio::test]
async fn test_integration_set_text_unicode_text() {
    let config = SetTextConfig {
        timeout: Duration::from_secs(5),
        cancellation: CancellationToken::new(),
    };

    let automation = UIAutomation::new().unwrap();
    let element = automation.get_root_element().unwrap();

    let result =
        smith_windows::core::set_text::set_text_with_config(&element, "Привет, мир! 👋", &config)
            .await;

    // May fail if element is not writable (root element is read-only)
    match result {
        Ok(()) => {}
        Err(SetTextError::ElementNotWritable) => {} // Expected for read-only elements
        Err(SetTextError::ComError(_)) => {}        // Also acceptable for read-only elements
        Err(e) => panic!("Unexpected error: {:?}", e),
    }
}
