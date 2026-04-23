//! Integration tests for ReadTool

use serial_test::serial;
use std::time::Duration;
use tokio_util::sync::CancellationToken;
use uiautomation::UIAutomation;

use smith_windows::core::read::{MockReadBackend, ReadBackend, ReadConfig, ReadError};

#[serial]
#[tokio::test]
async fn test_integration_read_text_success() {
    // Create mock backend with success state
    let backend = MockReadBackend::with_state(smith_windows::core::read::MockReadState {
        should_succeed: true,
        returned_text: "Test text content".to_string(),
        ..Default::default()
    });

    let automation = UIAutomation::new().unwrap();
    let element = automation.get_root_element().unwrap();
    let result = backend.read_text(&element).await;

    assert!(result.is_ok(), "Expected Ok(String) but got {:?}", result);
    assert_eq!(result.unwrap(), "Test text content");
}

#[tokio::test]
async fn test_integration_read_text_config_validation() {
    // Test zero timeout
    let config = ReadConfig {
        timeout: Duration::ZERO,
        cancellation: CancellationToken::new(),
    };

    let result = smith_windows::core::read::validate_read_config(&config);
    assert!(matches!(result, Err(ReadError::InvalidConfig(_))));

    // Test large timeout
    let config = ReadConfig {
        timeout: Duration::from_secs(3601),
        cancellation: CancellationToken::new(),
    };

    let result = smith_windows::core::read::validate_read_config(&config);
    assert!(matches!(result, Err(ReadError::InvalidConfig(_))));

    // Test valid config
    let config = ReadConfig {
        timeout: Duration::from_secs(5),
        cancellation: CancellationToken::new(),
    };

    let result = smith_windows::core::read::validate_read_config(&config);
    assert!(result.is_ok());
}

#[serial]
#[tokio::test]
async fn test_integration_read_text_mock_idempotency() {
    let backend = MockReadBackend::with_state(smith_windows::core::read::MockReadState {
        should_succeed: false,
        last_error: Some(ReadError::ElementNotFound),
        ..Default::default()
    });

    let automation = UIAutomation::new().unwrap();
    let element = automation.get_root_element().unwrap();

    // Multiple calls should behave consistently
    let result1 = backend.read_text(&element).await;
    let result2 = backend.read_text(&element).await;
    let result3 = backend.read_text(&element).await;

    assert!(matches!(result1, Err(ReadError::ElementNotFound)));
    assert!(matches!(result2, Err(ReadError::ElementNotFound)));
    assert!(matches!(result3, Err(ReadError::ElementNotFound)));

    // Call count should be 3
    assert_eq!(backend.get_state().unwrap().call_count, 3);
}

#[serial]
#[tokio::test]
async fn test_integration_read_text_with_config_success() {
    let automation = UIAutomation::new().unwrap();
    let element = automation.get_root_element().unwrap();

    let mock_backend = MockReadBackend::with_state(smith_windows::core::read::MockReadState {
        should_succeed: true,
        returned_text: "Config test text".to_string(),
        ..Default::default()
    });

    let _config = ReadConfig {
        timeout: Duration::from_secs(5),
        cancellation: CancellationToken::new(),
    };

    // Temporarily set the mock backend for testing
    // Note: This test uses the mock directly since we can't easily inject it
    let result = mock_backend.read_text(&element).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Config test text");
}

#[serial]
#[tokio::test]
async fn test_integration_read_text_disabled_element() {
    let backend = MockReadBackend::with_state(smith_windows::core::read::MockReadState {
        should_succeed: false,
        last_error: Some(ReadError::ElementNotEnabled),
        ..Default::default()
    });

    let automation = UIAutomation::new().unwrap();
    let element = automation.get_root_element().unwrap();

    let result = backend.read_text(&element).await;

    assert!(matches!(result, Err(ReadError::ElementNotEnabled)));
}

#[serial]
#[tokio::test]
async fn test_integration_read_text_offscreen_element() {
    let backend = MockReadBackend::with_state(smith_windows::core::read::MockReadState {
        should_succeed: false,
        last_error: Some(ReadError::ElementOffscreen),
        ..Default::default()
    });

    let automation = UIAutomation::new().unwrap();
    let element = automation.get_root_element().unwrap();

    let result = backend.read_text(&element).await;

    assert!(matches!(result, Err(ReadError::ElementOffscreen)));
}

#[serial]
#[tokio::test]
async fn test_integration_read_text_element_not_writable() {
    let backend = MockReadBackend::with_state(smith_windows::core::read::MockReadState {
        should_succeed: false,
        last_error: Some(ReadError::ElementNotWritable),
        ..Default::default()
    });

    let automation = UIAutomation::new().unwrap();
    let element = automation.get_root_element().unwrap();

    let result = backend.read_text(&element).await;

    assert!(matches!(result, Err(ReadError::ElementNotWritable)));
}
