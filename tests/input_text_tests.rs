//! Integration tests for input text tool

use serial_test::serial;
use std::time::Duration;
use tokio_util::sync::CancellationToken;
use uiautomation::UIAutomation;

use smith_windows::core::input_text::{
    InputTextBackend, InputTextConfig, InputTextError, MockInputTextBackend,
};

#[serial]
#[tokio::test]
async fn test_integration_input_text_success() {
    // Create mock backend with success state
    let backend =
        MockInputTextBackend::with_state(smith_windows::core::input_text::MockInputTextState {
            should_succeed: true,
            ..Default::default()
        });

    let automation = UIAutomation::new().unwrap();
    let element = automation.get_root_element().unwrap();
    let result = backend.input_text(&element, "test text").await;

    assert!(result.is_ok(), "Expected Ok(()) but got {:?}", result);
}

#[tokio::test]
async fn test_integration_input_text_config_validation() {
    // Test zero timeout
    let config = InputTextConfig {
        text: "Hello".to_string(),
        timeout: Duration::ZERO,
        cancellation: CancellationToken::new(),
    };

    let result = smith_windows::core::input_text::validate_input_text_config(&config);
    assert!(matches!(result, Err(InputTextError::InvalidConfig(_))));

    // Test large timeout
    let config = InputTextConfig {
        text: "Hello".to_string(),
        timeout: Duration::from_secs(3601),
        cancellation: CancellationToken::new(),
    };

    let result = smith_windows::core::input_text::validate_input_text_config(&config);
    assert!(matches!(result, Err(InputTextError::InvalidConfig(_))));

    // Test valid config
    let config = InputTextConfig {
        text: "Hello".to_string(),
        timeout: Duration::from_secs(5),
        cancellation: CancellationToken::new(),
    };

    let result = smith_windows::core::input_text::validate_input_text_config(&config);
    assert!(result.is_ok());
}

#[serial]
#[tokio::test]
async fn test_integration_input_text_mock_idempotency() {
    let backend =
        MockInputTextBackend::with_state(smith_windows::core::input_text::MockInputTextState {
            should_succeed: false,
            last_error: Some(InputTextError::ElementNotFound),
            ..Default::default()
        });

    let automation = UIAutomation::new().unwrap();
    let element = automation.get_root_element().unwrap();

    // Multiple calls should behave consistently
    let result1 = backend.input_text(&element, "test").await;
    let result2 = backend.input_text(&element, "test").await;
    let result3 = backend.input_text(&element, "test").await;

    assert!(matches!(result1, Err(InputTextError::ElementNotFound)));
    assert!(matches!(result2, Err(InputTextError::ElementNotFound)));
    assert!(matches!(result3, Err(InputTextError::ElementNotFound)));

    // Call count should be 3
    assert_eq!(backend.get_state().unwrap().call_count, 3);
}

#[serial]
#[tokio::test]
async fn test_integration_input_text_element_disabled() {
    let backend =
        MockInputTextBackend::with_state(smith_windows::core::input_text::MockInputTextState {
            should_succeed: false,
            last_error: Some(InputTextError::ElementNotEnabled),
            ..Default::default()
        });

    let automation = UIAutomation::new().unwrap();
    let element = automation.get_root_element().unwrap();
    let result = backend.input_text(&element, "test").await;

    assert!(matches!(result, Err(InputTextError::ElementNotEnabled)));
}

#[serial]
#[tokio::test]
async fn test_integration_input_text_element_offscreen() {
    let backend =
        MockInputTextBackend::with_state(smith_windows::core::input_text::MockInputTextState {
            should_succeed: false,
            last_error: Some(InputTextError::ElementOffscreen),
            ..Default::default()
        });

    let automation = UIAutomation::new().unwrap();
    let element = automation.get_root_element().unwrap();
    let result = backend.input_text(&element, "test").await;

    assert!(matches!(result, Err(InputTextError::ElementOffscreen)));
}

#[serial]
#[tokio::test]
async fn test_integration_input_text_element_read_only() {
    let backend =
        MockInputTextBackend::with_state(smith_windows::core::input_text::MockInputTextState {
            should_succeed: false,
            last_error: Some(InputTextError::ElementReadOnly),
            ..Default::default()
        });

    let automation = UIAutomation::new().unwrap();
    let element = automation.get_root_element().unwrap();
    let result = backend.input_text(&element, "test").await;

    assert!(matches!(result, Err(InputTextError::ElementReadOnly)));
}

#[serial]
#[tokio::test]
async fn test_integration_input_text_empty_text_validation() {
    let config = InputTextConfig {
        text: "".to_string(),
        timeout: Duration::from_secs(5),
        cancellation: CancellationToken::new(),
    };

    let result = smith_windows::core::input_text::validate_input_text_config(&config);
    assert!(matches!(result, Err(InputTextError::InvalidConfig(_))));
}

#[serial]
#[tokio::test]
async fn test_integration_input_text_empty_selector_validation() {
    let result = smith_windows::core::input_text::validate_input_selector("");
    assert!(matches!(result, Err(InputTextError::InputSelectorError(_))));
}

#[serial]
#[tokio::test]
async fn test_integration_input_text_unicode_text() {
    let config = InputTextConfig {
        text: "Привет, мир! 👋".to_string(),
        timeout: Duration::from_secs(5),
        cancellation: CancellationToken::new(),
    };

    let result = smith_windows::core::input_text::validate_input_text_config(&config);
    assert!(result.is_ok());
}

#[serial]
#[tokio::test]
async fn test_integration_input_text_large_text_validation() {
    let config = InputTextConfig {
        text: "a".repeat(65537),
        timeout: Duration::from_secs(5),
        cancellation: CancellationToken::new(),
    };

    let result = smith_windows::core::input_text::validate_input_text_config(&config);
    assert!(matches!(result, Err(InputTextError::InvalidConfig(_))));
}

#[serial]
#[tokio::test]
async fn test_integration_input_text_mock_last_keys() {
    let backend =
        MockInputTextBackend::with_state(smith_windows::core::input_text::MockInputTextState {
            should_succeed: true,
            ..Default::default()
        });

    let automation = UIAutomation::new().unwrap();
    let element = automation.get_root_element().unwrap();

    let result = backend.input_text(&element, "Hello {enter}World").await;
    assert!(result.is_ok());
    assert_eq!(
        backend.get_state().unwrap().last_keys,
        Some("Hello {enter}World".to_string())
    );
}
