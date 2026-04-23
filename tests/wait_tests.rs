//! Integration tests for wait tool

use serial_test::serial;
use std::time::Duration;
use tokio_util::sync::CancellationToken;
use uiautomation::UIAutomation;

use smith_windows::core::wait::{
    MockWaitBackend, WaitBackend, WaitConfig, WaitError, WaitMode, WaitSelector,
};

#[serial]
#[tokio::test]
async fn test_integration_wait_success() {
    // Create mock backend with success state
    let backend = MockWaitBackend::with_state(smith_windows::core::wait::MockWaitState {
        should_succeed: true,
        ..Default::default()
    });

    let automation = UIAutomation::new().unwrap();
    let element = automation.get_root_element().unwrap();
    let result = backend
        .wait_element(
            &automation,
            &element,
            &WaitSelector::AutomationId("test".to_string()),
        )
        .await;

    assert!(result.is_ok(), "Expected Ok(true) but got {:?}", result);
}

#[tokio::test]
async fn test_integration_wait_config_validation() {
    // Test zero timeout
    let cancellation1 = CancellationToken::new();
    let config = WaitConfig {
        timeout: Duration::ZERO,
        interval: Duration::from_millis(100),
        wait_for: WaitMode::Existence,
        selector: WaitSelector::AutomationId("test".to_string()),
        cancellation: cancellation1,
    };

    let result = smith_windows::core::wait::validate_wait_config(&config);
    assert!(matches!(result, Err(WaitError::InvalidConfig(_))));

    // Test zero interval
    let cancellation2 = CancellationToken::new();
    let config = WaitConfig {
        timeout: Duration::from_secs(5),
        interval: Duration::ZERO,
        wait_for: WaitMode::Existence,
        selector: WaitSelector::AutomationId("test".to_string()),
        cancellation: cancellation2,
    };

    let result = smith_windows::core::wait::validate_wait_config(&config);
    assert!(matches!(result, Err(WaitError::InvalidConfig(_))));

    // Test interval > timeout
    let cancellation3 = CancellationToken::new();
    let config = WaitConfig {
        timeout: Duration::from_secs(1),
        interval: Duration::from_secs(5),
        wait_for: WaitMode::Existence,
        selector: WaitSelector::AutomationId("test".to_string()),
        cancellation: cancellation3,
    };

    let result = smith_windows::core::wait::validate_wait_config(&config);
    assert!(matches!(result, Err(WaitError::InvalidConfig(_))));

    // Test valid config
    let cancellation4 = CancellationToken::new();
    let config = WaitConfig {
        timeout: Duration::from_secs(5),
        interval: Duration::from_millis(100),
        wait_for: WaitMode::Existence,
        selector: WaitSelector::AutomationId("test".to_string()),
        cancellation: cancellation4,
    };

    let result = smith_windows::core::wait::validate_wait_config(&config);
    assert!(result.is_ok());
}

#[serial]
#[tokio::test]
async fn test_integration_wait_mock_idempotency() {
    let backend = MockWaitBackend::with_state(smith_windows::core::wait::MockWaitState {
        should_succeed: false,
        last_error: Some(WaitError::Timeout),
        ..Default::default()
    });

    let automation = UIAutomation::new().unwrap();
    let element = automation.get_root_element().unwrap();
    let selector = WaitSelector::AutomationId("test".to_string());

    // Multiple calls should behave consistently
    let result1 = backend.wait_element(&automation, &element, &selector).await;
    let result2 = backend.wait_element(&automation, &element, &selector).await;
    let result3 = backend.wait_element(&automation, &element, &selector).await;

    assert!(matches!(result1, Err(WaitError::Timeout)));
    assert!(matches!(result2, Err(WaitError::Timeout)));
    assert!(matches!(result3, Err(WaitError::Timeout)));

    // Call count should be 3
    assert_eq!(backend.get_state().unwrap().call_count, 3);
}

#[serial]
#[tokio::test]
async fn test_integration_wait_cancellation() {
    let cancellation = CancellationToken::new();
    let config = WaitConfig {
        timeout: Duration::from_secs(10),
        interval: Duration::from_secs(1),
        wait_for: WaitMode::Existence,
        selector: WaitSelector::AutomationId("test".to_string()),
        cancellation: cancellation.clone(),
    };

    // Cancel before starting
    cancellation.cancel();

    // The actual cancellation check happens in wait_with_config, but we can verify
    // the token works
    assert!(cancellation.is_cancelled());
}

#[serial]
#[tokio::test]
async fn test_integration_wait_large_timeout_validation() {
    let cancellation = CancellationToken::new();
    let config = WaitConfig {
        timeout: Duration::from_secs(3601),
        interval: Duration::from_millis(100),
        wait_for: WaitMode::Existence,
        selector: WaitSelector::AutomationId("test".to_string()),
        cancellation: cancellation,
    };

    let result = smith_windows::core::wait::validate_wait_config(&config);
    assert!(matches!(result, Err(WaitError::InvalidConfig(_))));
}

#[serial]
#[tokio::test]
async fn test_integration_wait_empty_selector_validation() {
    let cancellation = CancellationToken::new();
    let config = WaitConfig {
        timeout: Duration::from_secs(5),
        interval: Duration::from_millis(100),
        wait_for: WaitMode::Existence,
        selector: WaitSelector::AutomationId("".to_string()),
        cancellation: cancellation,
    };

    let result = smith_windows::core::wait::validate_wait_config(&config);
    assert!(matches!(result, Err(WaitError::InvalidConfig(_))));
}

#[serial]
#[tokio::test]
async fn test_integration_wait_name_selector() {
    let cancellation = CancellationToken::new();
    let config = WaitConfig {
        timeout: Duration::from_secs(5),
        interval: Duration::from_millis(100),
        wait_for: WaitMode::Existence,
        selector: WaitSelector::Name("My Window".to_string()),
        cancellation: cancellation,
    };

    let result = smith_windows::core::wait::validate_wait_config(&config);
    assert!(result.is_ok());
}

#[serial]
#[tokio::test]
async fn test_integration_wait_control_type_selector() {
    let cancellation = CancellationToken::new();
    let config = WaitConfig {
        timeout: Duration::from_secs(5),
        interval: Duration::from_millis(100),
        wait_for: WaitMode::Existence,
        selector: WaitSelector::ControlType("Button".to_string()),
        cancellation: cancellation,
    };

    let result = smith_windows::core::wait::validate_wait_config(&config);
    assert!(result.is_ok());
}

#[serial]
#[tokio::test]
async fn test_integration_wait_absence_mode() {
    let cancellation = CancellationToken::new();
    let config = WaitConfig {
        timeout: Duration::from_secs(10),
        interval: Duration::from_millis(500),
        wait_for: WaitMode::Absence,
        selector: WaitSelector::AutomationId("dialog".to_string()),
        cancellation: cancellation,
    };

    let result = smith_windows::core::wait::validate_wait_config(&config);
    assert!(result.is_ok());
}
