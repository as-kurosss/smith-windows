//! Integration tests for click tool

use serial_test::serial;
use std::time::Duration;
use tokio_util::sync::CancellationToken;
use uiautomation::UIAutomation;

use smith_windows::core::click::{
    ClickBackend, ClickConfig, ClickError, ClickType, MockClickBackend,
};

#[serial]
#[tokio::test]
async fn test_integration_click_success() {
    // Create mock backend with success state
    let backend = MockClickBackend::with_state(smith_windows::core::click::MockClickState {
        should_succeed: true,
        ..Default::default()
    });

    let automation = UIAutomation::new().unwrap();
    let element = automation.get_root_element().unwrap();
    let result = backend.click(&element, ClickType::LeftSingle).await;

    assert!(result.is_ok(), "Expected Ok(()) but got {:?}", result);
}

#[tokio::test]
async fn test_integration_click_config_validation() {
    // Test zero timeout
    let config = ClickConfig {
        click_type: ClickType::LeftSingle,
        timeout: Duration::ZERO,
        cancellation: CancellationToken::new(),
    };

    let result = smith_windows::core::click::validate_click_config(&config);
    assert!(matches!(result, Err(ClickError::InvalidConfig(_))));

    // Test large timeout
    let config = ClickConfig {
        click_type: ClickType::LeftSingle,
        timeout: Duration::from_secs(3601),
        cancellation: CancellationToken::new(),
    };

    let result = smith_windows::core::click::validate_click_config(&config);
    assert!(matches!(result, Err(ClickError::InvalidConfig(_))));

    // Test valid config
    let config = ClickConfig {
        click_type: ClickType::LeftSingle,
        timeout: Duration::from_secs(5),
        cancellation: CancellationToken::new(),
    };

    let result = smith_windows::core::click::validate_click_config(&config);
    assert!(result.is_ok());
}

#[serial]
#[tokio::test]
async fn test_integration_mock_idempotency() {
    let backend = MockClickBackend::with_state(smith_windows::core::click::MockClickState {
        should_succeed: false,
        last_error: Some(ClickError::ElementNotFound),
        ..Default::default()
    });

    let automation = UIAutomation::new().unwrap();
    let element = automation.get_root_element().unwrap();

    // Multiple calls should behave consistently
    let result1 = backend.click(&element, ClickType::LeftSingle).await;
    let result2 = backend.click(&element, ClickType::LeftSingle).await;
    let result3 = backend.click(&element, ClickType::LeftSingle).await;

    assert!(matches!(result1, Err(ClickError::ElementNotFound)));
    assert!(matches!(result2, Err(ClickError::ElementNotFound)));
    assert!(matches!(result3, Err(ClickError::ElementNotFound)));

    // Call count should be 3
    assert_eq!(backend.get_state().unwrap().call_count, 3);
}
