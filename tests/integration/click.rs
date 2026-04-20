//! Integration tests for click tool

use std::time::Duration;
use tokio_util::sync::CancellationToken;

use smith_windows::core::click::{ClickBackend, ClickConfig, ClickError, MockClickBackend};
use smith_windows::runtime::backends::windows::click::ClickBackendWindows;

#[tokio::test]
async fn test_integration_click_success() {
    // Create mock backend with success state
    let backend = MockClickBackend::with_state(
        smith_windows::core::click::MockClickState {
            should_succeed: true,
            ..Default::default()
        }
    );
    
    let element = uiautomation::UIElement::default();
    let result = backend.click(&element).await;
    
    assert!(result.is_ok(), "Expected Ok(()) but got {:?}", result);
}

#[tokio::test]
async fn test_integration_click_element_not_found() {
    let backend = MockClickBackend::with_state(
        smith_windows::core::click::MockClickState {
            should_succeed: false,
            last_error: Some(ClickError::ElementNotFound),
            ..Default::default()
        }
    );
    
    let element = uiautomation::UIElement::default();
    let result = backend.click(&element).await;
    
    assert!(matches!(result, Err(ClickError::ElementNotFound)));
}

#[tokio::test]
async fn test_integration_click_disabled() {
    let backend = MockClickBackend::with_state(
        smith_windows::core::click::MockClickState {
            should_succeed: false,
            last_error: Some(ClickError::ElementNotEnabled),
            ..Default::default()
        }
    );
    
    let element = uiautomation::UIElement::default();
    let result = backend.click(&element).await;
    
    assert!(matches!(result, Err(ClickError::ElementNotEnabled)));
}

#[tokio::test]
async fn test_integration_click_offscreen() {
    let backend = MockClickBackend::with_state(
        smith_windows::core::click::MockClickState {
            should_succeed: false,
            last_error: Some(ClickError::ElementOffscreen),
            ..Default::default()
        }
    );
    
    let element = uiautomation::UIElement::default();
    let result = backend.click(&element).await;
    
    assert!(matches!(result, Err(ClickError::ElementOffscreen)));
}

#[tokio::test]
async fn test_integration_click_config_validation() {
    // Test zero timeout
    let config = ClickConfig {
        timeout: Duration::ZERO,
        cancellation: CancellationToken::new(),
    };
    
    let result = smith_windows::core::click::validate_click_config(&config);
    assert!(matches!(result, Err(ClickError::InvalidConfig(_))));
    
    // Test large timeout
    let config = ClickConfig {
        timeout: Duration::from_secs(3601),
        cancellation: CancellationToken::new(),
    };
    
    let result = smith_windows::core::click::validate_click_config(&config);
    assert!(matches!(result, Err(ClickError::InvalidConfig(_))));
    
    // Test valid config
    let config = ClickConfig {
        timeout: Duration::from_secs(5),
        cancellation: CancellationToken::new(),
    };
    
    let result = smith_windows::core::click::validate_click_config(&config);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_integration_mock_idempotency() {
    let backend = MockClickBackend::with_state(
        smith_windows::core::click::MockClickState {
            should_succeed: false,
            last_error: Some(ClickError::ElementNotFound),
            ..Default::default()
        }
    );
    
    let element = uiautomation::UIElement::default();
    
    // Multiple calls should behave consistently
    let result1 = backend.click(&element).await;
    let result2 = backend.click(&element).await;
    let result3 = backend.click(&element).await;
    
    assert!(matches!(result1, Err(ClickError::ElementNotFound)));
    assert!(matches!(result2, Err(ClickError::ElementNotFound)));
    assert!(matches!(result3, Err(ClickError::ElementNotFound)));
    
    // Call count should be 3
    assert_eq!(backend.get_state().call_count, 3);
}
