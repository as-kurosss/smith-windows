//! Integration tests for ClipboardTool
//! Covers full lifecycle, cancellation, timeout, and idempotency

use std::time::Duration;
use smith_windows::core::clipboard::{
    get_text_with_config, has_text_with_config, set_text_with_config, validate_clipboard_config,
    ClipboardBackend, ClipboardConfig, ClipboardError, MockClipboardBackend, MockClipboardState,
    SetTextParams,
};
use tokio_util::sync::CancellationToken;

#[tokio::test]
async fn test_get_text_success() {
    let state = MockClipboardState {
        should_succeed: true,
        clipboard_has_text: true,
        clipboard_text: Some("Test text".to_string()),
        ..Default::default()
    };
    let backend = MockClipboardBackend::with_state(state);

    let cancellation = CancellationToken::new();
    let config = ClipboardConfig {
        timeout: Duration::from_secs(5),
        cancellation,
    };

    let result = backend.get_text().await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Test text");
}

#[tokio::test]
async fn test_get_text_empty_clipboard() {
    let state = MockClipboardState {
        should_succeed: false,
        last_error: Some(ClipboardError::ClipboardEmpty),
        ..Default::default()
    };
    let backend = MockClipboardBackend::with_state(state);

    let cancellation = CancellationToken::new();
    let config = ClipboardConfig {
        timeout: Duration::from_secs(5),
        cancellation,
    };

    let result = backend.get_text().await;
    assert!(matches!(result, Err(ClipboardError::ClipboardEmpty)));
}

#[tokio::test]
async fn test_set_text_success() {
    let state = MockClipboardState {
        should_succeed: true,
        ..Default::default()
    };
    let backend = MockClipboardBackend::with_state(state);

    let params = SetTextParams {
        text: "Test text".to_string(),
    };

    let result = backend.set_text(&params.text).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_set_text_error() {
    let state = MockClipboardState {
        should_succeed: false,
        last_error: Some(ClipboardError::ClipboardAccessDenied),
        ..Default::default()
    };
    let backend = MockClipboardBackend::with_state(state);

    let params = SetTextParams {
        text: "Test text".to_string(),
    };

    let result = backend.set_text(&params.text).await;
    assert!(matches!(result, Err(ClipboardError::ClipboardAccessDenied)));
}

#[tokio::test]
async fn test_has_text_success() {
    let state = MockClipboardState {
        should_succeed: true,
        clipboard_has_text: true,
        ..Default::default()
    };
    let backend = MockClipboardBackend::with_state(state);

    let result = backend.has_text().await;
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[tokio::test]
async fn test_has_text_false() {
    let state = MockClipboardState {
        should_succeed: true,
        clipboard_has_text: false,
        ..Default::default()
    };
    let backend = MockClipboardBackend::with_state(state);

    let result = backend.has_text().await;
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[tokio::test]
async fn test_has_text_error() {
    let state = MockClipboardState {
        should_succeed: false,
        last_error: Some(ClipboardError::ClipboardAccessDenied),
        ..Default::default()
    };
    let backend = MockClipboardBackend::with_state(state);

    let result = backend.has_text().await;
    assert!(matches!(result, Err(ClipboardError::ClipboardAccessDenied)));
}

#[tokio::test]
async fn test_get_text_with_config_validation() {
    let cancellation = CancellationToken::new();
    let config = ClipboardConfig {
        timeout: Duration::ZERO,
        cancellation,
    };

    let result = get_text_with_config(&config).await;
    assert!(matches!(result, Err(ClipboardError::InvalidConfig(_))));
}

#[tokio::test]
async fn test_set_text_with_config_validation() {
    let cancellation = CancellationToken::new();
    let config = ClipboardConfig {
        timeout: Duration::ZERO,
        cancellation,
    };

    let params = SetTextParams {
        text: "Test text".to_string(),
    };

    let result = set_text_with_config(&params, &config).await;
    assert!(matches!(result, Err(ClipboardError::InvalidConfig(_))));
}

#[tokio::test]
async fn test_has_text_with_config_validation() {
    let cancellation = CancellationToken::new();
    let config = ClipboardConfig {
        timeout: Duration::ZERO,
        cancellation,
    };

    let result = has_text_with_config(&config).await;
    assert!(matches!(result, Err(ClipboardError::InvalidConfig(_))));
}

#[tokio::test]
async fn test_get_text_cancellation() {
    let state = MockClipboardState {
        should_succeed: true,
        clipboard_has_text: true,
        clipboard_text: Some("Test text".to_string()),
        ..Default::default()
    };
    let backend = MockClipboardBackend::with_state(state);

    let cancellation = CancellationToken::new();
    cancellation.cancel(); // Cancel before operation

    let config = ClipboardConfig {
        timeout: Duration::from_secs(5),
        cancellation,
    };

    // This simulates the cancel check in get_text_with_config
    if config.cancellation.is_cancelled() {
        assert!(matches!(
            get_text_with_config(&config).await,
            Err(ClipboardError::Cancelled)
        ));
    }
}

#[tokio::test]
async fn test_set_text_text_empty() {
    let cancellation = CancellationToken::new();
    let config = ClipboardConfig {
        timeout: Duration::from_secs(5),
        cancellation,
    };

    let params = SetTextParams { text: "".to_string() };

    let result = set_text_with_config(&params, &config).await;
    assert!(matches!(result, Err(ClipboardError::InvalidConfig(_))));
}

#[tokio::test]
async fn test_idempotent_error_get_text() {
    let state = MockClipboardState {
        should_succeed: false,
        last_error: Some(ClipboardError::ClipboardAccessDenied),
        ..Default::default()
    };
    let backend = MockClipboardBackend::with_state(state);

    // First error call
    let result1 = backend.get_text().await;
    let state1 = backend.get_state();
    let call_count1 = state1.call_count;

    // Second error call - state should track calls but error should remain same
    let result2 = backend.get_text().await;
    let state2 = backend.get_state();
    let call_count2 = state2.call_count;

    assert!(result1.is_err());
    assert!(result2.is_err());
    assert_eq!(call_count1, 1);
    assert_eq!(call_count2, 2);
    assert_eq!(state1.last_error, state2.last_error);
}

#[tokio::test]
async fn test_idempotent_error_set_text() {
    let state = MockClipboardState {
        should_succeed: false,
        last_error: Some(ClipboardError::ClipboardAccessDenied),
        ..Default::default()
    };
    let backend = MockClipboardBackend::with_state(state);

    let params = SetTextParams {
        text: "Test text".to_string(),
    };

    // First error call
    let result1 = backend.set_text(&params.text).await;
    let state1 = backend.get_state();
    let call_count1 = state1.call_count;

    // Second error call
    let result2 = backend.set_text(&params.text).await;
    let state2 = backend.get_state();
    let call_count2 = state2.call_count;

    assert!(result1.is_err());
    assert!(result2.is_err());
    assert_eq!(call_count1, 1);
    assert_eq!(call_count2, 2);
    assert_eq!(state1.last_error, state2.last_error);
}

#[tokio::test]
async fn test_idempotent_error_has_text() {
    let state = MockClipboardState {
        should_succeed: false,
        last_error: Some(ClipboardError::ClipboardAccessDenied),
        ..Default::default()
    };
    let backend = MockClipboardBackend::with_state(state);

    // First error call
    let result1 = backend.has_text().await;
    let state1 = backend.get_state();
    let call_count1 = state1.call_count;

    // Second error call
    let result2 = backend.has_text().await;
    let state2 = backend.get_state();
    let call_count2 = state2.call_count;

    assert!(result1.is_err());
    assert!(result2.is_err());
    assert_eq!(call_count1, 1);
    assert_eq!(call_count2, 2);
    assert_eq!(state1.last_error, state2.last_error);
}

#[tokio::test]
async fn test_mock_backend_reset() {
    let backend = MockClipboardBackend::new();
    
    // Perform some operations
    let _ = backend.get_text().await;
    let _ = backend.set_text("test").await;
    let _ = backend.has_text().await;

    assert_eq!(backend.get_state().call_count, 3);

    // Reset
    backend.reset();
    assert_eq!(backend.get_state().call_count, 0);
}

#[tokio::test]
async fn test_validate_clipboard_config_edge_cases() {
    // Valid: 1 second
    let cancellation = CancellationToken::new();
    let config = ClipboardConfig {
        timeout: Duration::from_secs(1),
        cancellation,
    };
    assert!(validate_clipboard_config(&config).is_ok());

    // Valid: 3600 seconds (exactly 1 hour)
    let cancellation = CancellationToken::new();
    let config = ClipboardConfig {
        timeout: Duration::from_secs(3600),
        cancellation,
    };
    assert!(validate_clipboard_config(&config).is_ok());

    // Invalid: 0 seconds
    let cancellation = CancellationToken::new();
    let config = ClipboardConfig {
        timeout: Duration::ZERO,
        cancellation,
    };
    assert!(matches!(
        validate_clipboard_config(&config),
        Err(ClipboardError::InvalidConfig(_))
    ));

    // Invalid: > 1 hour
    let cancellation = CancellationToken::new();
    let config = ClipboardConfig {
        timeout: Duration::from_secs(3601),
        cancellation,
    };
    assert!(matches!(
        validate_clipboard_config(&config),
        Err(ClipboardError::InvalidConfig(_))
    ));
}
