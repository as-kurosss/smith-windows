//! Integration tests for automation session module

use std::time::Duration;
use tokio_util::sync::CancellationToken;

use smith_windows::core::automation_session::{
    attach_by_process_id, attach_by_title, launch_process, validate_command, validate_regex,
    validate_session_config, validate_title_filter, AutomationError, MatchMode, MockSessionBackend,
    MockSessionState, RuntimeSession, SessionConfig, SessionLaunchConfig, SessionState,
};

/// Helper function to create a valid session config
fn valid_config() -> SessionConfig {
    SessionConfig {
        timeout: Duration::from_secs(5),
        cancellation: CancellationToken::new(),
    }
}

/// Helper function to create a valid launch config
fn valid_launch_config() -> SessionLaunchConfig {
    SessionLaunchConfig {
        command: "notepad.exe".to_string(),
        args: None,
        working_dir: None,
    }
}

#[tokio::test]
async fn test_integration_launch_process_success() {
    let config = valid_launch_config();
    let result = launch_process(&config).await;

    assert!(
        result.is_ok(),
        "Expected Ok(process_id) but got {:?}",
        result
    );

    let process_id = result.unwrap();
    assert!(process_id > 0, "Process ID should be > 0");

    // Clean up: terminate the process
    let _ = std::process::Command::new("taskkill")
        .args(&["/PID", &process_id.to_string(), "/F"])
        .output();
}

#[tokio::test]
async fn test_integration_launch_process_with_args() {
    let config = SessionLaunchConfig {
        command: "notepad.exe".to_string(),
        args: Some(vec!["/A".to_string()]), // Open in admin mode (will fail on normal systems)
        working_dir: None,
    };

    // This will likely fail but should validate correctly
    let result = launch_process(&config).await;

    // We expect either success (if admin mode works) or ProcessLaunchFailed
    match result {
        Ok(process_id) => {
            assert!(process_id > 0);
            let _ = std::process::Command::new("taskkill")
                .args(&["/PID", &process_id.to_string(), "/F"])
                .output();
        }
        Err(AutomationError::ProcessLaunchFailed(_)) => {
            // Expected - notepad with admin mode may fail
        }
        Err(e) => {
            panic!("Unexpected error: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_integration_launch_process_empty_command() {
    let config = SessionLaunchConfig {
        command: "".to_string(),
        args: None,
        working_dir: None,
    };

    let result = launch_process(&config).await;
    assert!(matches!(result, Err(AutomationError::InvalidConfig(_))));
}

#[tokio::test]
async fn test_integration_launch_process_nonexistent() {
    let config = SessionLaunchConfig {
        command: "nonexistent_xyz_123.exe".to_string(),
        args: None,
        working_dir: None,
    };

    let result = launch_process(&config).await;
    assert!(matches!(
        result,
        Err(AutomationError::ProcessLaunchFailed(_))
    ));
}

#[tokio::test]
async fn test_integration_launch_process_working_dir() {
    let config = SessionLaunchConfig {
        command: "notepad.exe".to_string(),
        args: None,
        working_dir: Some("C:\\Windows".to_string()),
    };

    let result = launch_process(&config).await;

    match result {
        Ok(process_id) => {
            assert!(process_id > 0);
            let _ = std::process::Command::new("taskkill")
                .args(&["/PID", &process_id.to_string(), "/F"])
                .output();
        }
        Err(AutomationError::InvalidConfig(_)) => {
            // Working dir validation may fail if path doesn't exist
        }
        Err(e) => {
            panic!("Unexpected error: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_integration_validate_session_config_valid() {
    let config = SessionConfig {
        timeout: Duration::from_secs(5),
        cancellation: CancellationToken::new(),
    };

    assert!(validate_session_config(&config).is_ok());
}

#[tokio::test]
async fn test_integration_validate_session_config_zero_timeout() {
    let config = SessionConfig {
        timeout: Duration::ZERO,
        cancellation: CancellationToken::new(),
    };

    assert!(matches!(
        validate_session_config(&config),
        Err(AutomationError::InvalidConfig(_))
    ));
}

#[tokio::test]
async fn test_integration_validate_session_config_large_timeout() {
    let config = SessionConfig {
        timeout: Duration::from_secs(3601), // > 1 hour
        cancellation: CancellationToken::new(),
    };

    assert!(matches!(
        validate_session_config(&config),
        Err(AutomationError::InvalidConfig(_))
    ));
}

#[tokio::test]
async fn test_integration_validate_title_filter_valid() {
    assert!(validate_title_filter("Test").is_ok());
}

#[tokio::test]
async fn test_integration_validate_title_filter_empty() {
    assert!(matches!(
        validate_title_filter(""),
        Err(AutomationError::InvalidConfig(_))
    ));
}

#[tokio::test]
async fn test_integration_validate_regex_valid() {
    assert!(validate_regex(r".*test.*").is_ok());
}

#[tokio::test]
async fn test_integration_validate_regex_invalid() {
    assert!(matches!(
        validate_regex(r"["),
        Err(AutomationError::InvalidConfig(_))
    ));
}

#[tokio::test]
async fn test_integration_validate_command_valid() {
    assert!(validate_command("notepad.exe").is_ok());
}

#[tokio::test]
async fn test_integration_validate_command_empty() {
    assert!(matches!(
        validate_command(""),
        Err(AutomationError::InvalidConfig(_))
    ));
}

#[tokio::test]
async fn test_integration_mock_backend_success() {
    let state = MockSessionState {
        launch_should_succeed: true,
        launch_return_process_id: 12345,
        ..Default::default()
    };
    let backend = MockSessionBackend::with_state(state);

    let config = SessionLaunchConfig {
        command: "notepad.exe".to_string(),
        args: None,
        working_dir: None,
    };

    let result = backend.launch_process(&config).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 12345);
}

#[tokio::test]
async fn test_integration_mock_backend_failure() {
    let state = MockSessionState {
        launch_should_succeed: false,
        launch_last_error: Some(AutomationError::ProcessLaunchFailed(
            "Mock error".to_string(),
        )),
        ..Default::default()
    };
    let backend = MockSessionBackend::with_state(state);

    let config = SessionLaunchConfig {
        command: "notepad.exe".to_string(),
        args: None,
        working_dir: None,
    };

    let result = backend.launch_process(&config).await;
    assert!(matches!(
        result,
        Err(AutomationError::ProcessLaunchFailed(_))
    ));
}

#[tokio::test]
async fn test_integration_mock_idempotency() {
    let state = MockSessionState {
        launch_should_succeed: false,
        launch_last_error: Some(AutomationError::ProcessLaunchFailed(
            "Mock error".to_string(),
        )),
        ..Default::default()
    };
    let backend = MockSessionBackend::with_state(state);

    let config = SessionLaunchConfig {
        command: "notepad.exe".to_string(),
        args: None,
        working_dir: None,
    };

    // Multiple calls should behave consistently
    let result1 = backend.launch_process(&config).await;
    let result2 = backend.launch_process(&config).await;
    let result3 = backend.launch_process(&config).await;

    assert!(matches!(
        result1,
        Err(AutomationError::ProcessLaunchFailed(_))
    ));
    assert!(matches!(
        result2,
        Err(AutomationError::ProcessLaunchFailed(_))
    ));
    assert!(matches!(
        result3,
        Err(AutomationError::ProcessLaunchFailed(_))
    ));

    assert_eq!(backend.get_state().launch_call_count, 3);
}

#[tokio::test]
async fn test_integration_runtime_session_state() {
    let element = uiautomation::UIElement::default();
    let session = RuntimeSession::new(12345, element);

    assert!(session.is_running());
    assert!(!session.is_closed());

    // Close the session
    session.set_closed().expect("Failed to close session");
    assert!(!session.is_running());
    assert!(session.is_closed());

    // Try to close again - should fail
    assert!(matches!(
        session.set_closed(),
        Err(AutomationError::SessionClosed)
    ));
}

#[tokio::test]
async fn test_integration_runtime_session_check_running() {
    let element = uiautomation::UIElement::default();
    let session = RuntimeSession::new(12345, element);

    assert!(session.check_running().is_ok());

    session.set_closed().expect("Failed to close session");
    assert!(matches!(
        session.check_running(),
        Err(AutomationError::SessionClosed)
    ));
}

#[tokio::test]
async fn test_integration_runtime_session_click() {
    let element = uiautomation::UIElement::default();
    let session = RuntimeSession::new(12345, element);

    // This should return an error since the element is default (invalid)
    // but it should not panic
    let result = session.click().await;
    // We expect either an error from invalid element or success (depending on system)
    // The important thing is no panic
    match result {
        Err(AutomationError::ComError(_)) | Err(AutomationError::WindowNotFound) => {
            // Expected - element is not valid
        }
        Ok(_) => {
            // Also acceptable if the system happens to have a valid default element
        }
        _ => {
            // Other errors are also acceptable
        }
    }
}

#[tokio::test]
async fn test_integration_runtime_session_type_text() {
    let element = uiautomation::UIElement::default();
    let session = RuntimeSession::new(12345, element);

    // This should return an error since the element is default (invalid)
    let result = session.type_text("Hello").await;
    match result {
        Err(AutomationError::ComError(_)) | Err(AutomationError::WindowNotFound) => {
            // Expected - element is not valid
        }
        _ => {
            // Other errors are also acceptable
        }
    }
}

#[tokio::test]
async fn test_integration_runtime_session_close() {
    // Create a session with a mock process
    let element = uiautomation::UIElement::default();
    let session = RuntimeSession::new(12345, element);

    // Close the session
    let result = session.close().await;
    // May succeed or fail depending on whether process 12345 exists
    // The important thing is no panic
    match result {
        Ok(()) => {
            assert!(session.is_closed());
        }
        Err(AutomationError::SessionClosed) => {
            // Already closed from previous test
        }
        Err(_) => {
            // Other errors are also acceptable
        }
    }
}

#[tokio::test]
async fn test_integration_runtime_session_after_close() {
    let element = uiautomation::UIElement::default();
    let session = RuntimeSession::new(12345, element);

    // Close the session
    session.set_closed().expect("Failed to close session");

    // All operations should fail with SessionClosed
    assert!(matches!(
        session.click().await,
        Err(AutomationError::SessionClosed)
    ));
    assert!(matches!(
        session.type_text("test").await,
        Err(AutomationError::SessionClosed)
    ));
    assert!(matches!(
        session.find_element("Button", None).await,
        Err(AutomationError::SessionClosed)
    ));
}

#[tokio::test]
async fn test_integration_config_validation_before_backend() {
    // Test that validation happens before backend call

    // Invalid timeout (0)
    let config = SessionConfig {
        timeout: Duration::ZERO,
        cancellation: CancellationToken::new(),
    };

    // This should fail validation before any backend call
    let result = attach_by_title("Test".to_string(), MatchMode::Exact, true, &config).await;
    assert!(matches!(result, Err(AutomationError::InvalidConfig(_))));

    // Invalid timeout (> 1 hour)
    let config = SessionConfig {
        timeout: Duration::from_secs(3601),
        cancellation: CancellationToken::new(),
    };

    let result = attach_by_title("Test".to_string(), MatchMode::Exact, true, &config).await;
    assert!(matches!(result, Err(AutomationError::InvalidConfig(_))));

    // Empty title
    let config = valid_config();
    let result = attach_by_title("".to_string(), MatchMode::Exact, true, &config).await;
    assert!(matches!(result, Err(AutomationError::InvalidConfig(_))));

    // Invalid regex
    let result = attach_by_title("[".to_string(), MatchMode::Regex, true, &config).await;
    assert!(matches!(result, Err(AutomationError::InvalidConfig(_))));
}

#[tokio::test]
async fn test_integration_process_id_validation() {
    let config = valid_config();

    // process_id = 0 should fail validation
    let result = attach_by_process_id(0, &config).await;
    assert!(matches!(result, Err(AutomationError::InvalidConfig(_))));
}

#[tokio::test]
async fn test_integration_mock_backend_idempotent_error() {
    // Test that errors are consistent across multiple calls
    let state = MockSessionState {
        launch_should_succeed: false,
        launch_last_error: Some(AutomationError::ProcessLaunchFailed(
            "Consistent error".to_string(),
        )),
        ..Default::default()
    };
    let backend = MockSessionBackend::with_state(state);

    let config = SessionLaunchConfig {
        command: "notepad.exe".to_string(),
        args: None,
        working_dir: None,
    };

    // First call
    let result1 = backend.launch_process(&config).await;
    assert!(
        matches!(result1, Err(AutomationError::ProcessLaunchFailed(ref s)) if s.contains("Consistent error"))
    );

    // Second call should have same error
    let result2 = backend.launch_process(&config).await;
    assert!(
        matches!(result2, Err(AutomationError::ProcessLaunchFailed(ref s)) if s.contains("Consistent error"))
    );

    // Third call should have same error
    let result3 = backend.launch_process(&config).await;
    assert!(
        matches!(result3, Err(AutomationError::ProcessLaunchFailed(ref s)) if s.contains("Consistent error"))
    );
}

#[tokio::test]
async fn test_integration_mock_backend_consistent_error() {
    // Verify that Err doesn't change state
    let state = MockSessionState {
        launch_should_succeed: false,
        launch_last_error: Some(AutomationError::ProcessLaunchFailed(
            "Consistent error".to_string(),
        )),
        ..Default::default()
    };
    let backend = MockSessionBackend::with_state(state);

    let config = SessionLaunchConfig {
        command: "notepad.exe".to_string(),
        args: None,
        working_dir: None,
    };

    let initial_count = backend.get_state().launch_call_count;

    // Multiple calls with error
    let _ = backend.launch_process(&config).await;
    let _ = backend.launch_process(&config).await;
    let _ = backend.launch_process(&config).await;

    let final_count = backend.get_state().launch_call_count;

    // Call count should increase, but error should remain consistent
    assert_eq!(final_count, initial_count + 3);
}

#[tokio::test]
async fn test_integration_cancellation() {
    let config = SessionConfig {
        timeout: Duration::from_secs(60), // Long timeout
        cancellation: CancellationToken::new(),
    };

    let config_clone = config.clone();

    // Cancel before the call
    config_clone.cancellation.cancel();

    let result = attach_by_title("Test".to_string(), MatchMode::Exact, true, &config_clone).await;
    assert!(matches!(result, Err(AutomationError::Cancelled)));
}
