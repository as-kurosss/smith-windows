//! Integration tests for ScreenshotTool

use std::time::Duration;
use smith_windows::core::screenshot::{
    validate_screenshot_config, validate_screenshot_mode, ScreenshotBackend, ScreenshotBackendWindows, ScreenshotConfig, ScreenshotError, ScreenshotMode, MockScreenshotBackend,
};
use tokio_util::sync::CancellationToken;

/// Test successful screen capture (mock)
#[tokio::test]
async fn test_capture_screen_mock() {
    let backend = MockScreenshotBackend::new();
    let mode = ScreenshotMode::Screen;
    
    let result = backend.capture(&mode).await;
    
    assert!(result.is_ok());
    let png_bytes = result.unwrap();
    
    // Verify PNG magic bytes
    assert_eq!(&png_bytes[0..8], &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]);
}

/// Test successful window capture (mock)
#[tokio::test]
async fn test_capture_window_mock() {
    let backend = MockScreenshotBackend::new();
    // Create a minimal UIElement for testing
    let automation = uiautomation::UIAutomation::new().unwrap();
    let root = automation.get_root().unwrap();
    let mode = ScreenshotMode::Window(root);
    
    let result = backend.capture(&mode).await;
    
    assert!(result.is_ok());
    let png_bytes = result.unwrap();
    
    // Verify PNG magic bytes
    assert_eq!(&png_bytes[0..8], &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]);
}

/// Test successful region capture (mock)
#[tokio::test]
async fn test_capture_region_mock() {
    let backend = MockScreenshotBackend::new();
    let mode = ScreenshotMode::Region {
        x: 0,
        y: 0,
        width: 100,
        height: 100,
    };
    
    let result = backend.capture(&mode).await;
    
    assert!(result.is_ok());
    let png_bytes = result.unwrap();
    
    // Verify PNG magic bytes
    assert_eq!(&png_bytes[0..8], &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]);
}

/// Test invalid configuration - zero timeout
#[tokio::test]
async fn test_invalid_config_zero_timeout() {
    let cancellation = CancellationToken::new();
    let config = ScreenshotConfig {
        timeout: Duration::ZERO,
        cancellation,
    };
    
    let result = validate_screenshot_config(&config);
    assert!(matches!(result, Err(ScreenshotError::InvalidConfig(_))));
}

/// Test invalid configuration - large timeout
#[tokio::test]
async fn test_invalid_config_large_timeout() {
    let cancellation = CancellationToken::new();
    let config = ScreenshotConfig {
        timeout: Duration::from_secs(3601), // > 1 hour
        cancellation,
    };
    
    let result = validate_screenshot_config(&config);
    assert!(matches!(result, Err(ScreenshotError::InvalidConfig(_))));
}

/// Test invalid region - negative coordinates
#[tokio::test]
async fn test_invalid_region_negative_coords() {
    let mode = ScreenshotMode::Region {
        x: -1,
        y: 0,
        width: 100,
        height: 100,
    };
    
    let result = validate_screenshot_mode(&mode);
    assert!(matches!(result, Err(ScreenshotError::InvalidRegion(_))));
}

/// Test invalid region - zero width
#[tokio::test]
async fn test_invalid_region_zero_width() {
    let mode = ScreenshotMode::Region {
        x: 0,
        y: 0,
        width: 0,
        height: 100,
    };
    
    let result = validate_screenshot_mode(&mode);
    assert!(matches!(result, Err(ScreenshotError::InvalidRegion(_))));
}

/// Test invalid region - zero height
#[tokio::test]
async fn test_invalid_region_zero_height() {
    let mode = ScreenshotMode::Region {
        x: 0,
        y: 0,
        width: 100,
        height: 0,
    };
    
    let result = validate_screenshot_mode(&mode);
    assert!(matches!(result, Err(ScreenshotError::InvalidRegion(_))));
}

/// Test element not found error
#[tokio::test]
async fn test_element_not_found() {
    let state = smith_windows::core::screenshot::MockScreenshotState {
        should_succeed: false,
        ..Default::default()
    };
    let backend = MockScreenshotBackend::with_state(state);
    
    let mode = ScreenshotMode::Screen;
    let result = backend.capture(&mode).await;
    
    assert!(matches!(result, Err(ScreenshotError::ElementNotFound)));
}

/// Test PNG magic bytes verification
#[tokio::test]
async fn test_png_magic_bytes() {
    let backend = MockScreenshotBackend::new();
    let mode = ScreenshotMode::Screen;
    
    let result = backend.capture(&mode).await;
    assert!(result.is_ok());
    
    let png_bytes = result.unwrap();
    
    // Verify PNG magic bytes: 89 50 4E 47 0D 0A 1A 0A
    assert_eq!(png_bytes[0], 0x89);
    assert_eq!(png_bytes[1], 0x50);
    assert_eq!(png_bytes[2], 0x4E);
    assert_eq!(png_bytes[3], 0x47);
    assert_eq!(png_bytes[4], 0x0D);
    assert_eq!(png_bytes[5], 0x0A);
    assert_eq!(png_bytes[6], 0x1A);
    assert_eq!(png_bytes[7], 0x0A);
}

/// Test idempotency - repeated calls return same error
#[tokio::test]
async fn test_idempotency_error() {
    let state = smith_windows::core::screenshot::MockScreenshotState {
        should_succeed: false,
        ..Default::default()
    };
    let backend = MockScreenshotBackend::with_state(state);
    
    let mode = ScreenshotMode::Screen;
    
    // First call
    let result1 = backend.capture(&mode).await;
    assert!(result1.is_err());
    
    // Second call should return same error
    let result2 = backend.capture(&mode).await;
    assert!(result2.is_err());
    
    // Both should return same error type
    assert_eq!(format!("{:?}", result1.unwrap_err()), format!("{:?}", result2.unwrap_err()));
}

/// Test idempotency - repeated calls return same success
#[tokio::test]
async fn test_idempotency_success() {
    let backend = MockScreenshotBackend::new();
    let mode = ScreenshotMode::Screen;
    
    // First call
    let result1 = backend.capture(&mode).await;
    assert!(result1.is_ok());
    
    // Second call should return same success
    let result2 = backend.capture(&mode).await;
    assert!(result2.is_ok());
    
    // Both should return same PNG bytes
    assert_eq!(result1.unwrap(), result2.unwrap());
}

/// Test backend state isolation
#[tokio::test]
async fn test_backend_state_isolation() {
    let backend1 = MockScreenshotBackend::new();
    let backend2 = MockScreenshotBackend::new();
    
    let mode = ScreenshotMode::Screen;
    
    // Capture with backend1
    let _ = backend1.capture(&mode).await;
    
    // Backend2 should have 0 calls
    assert_eq!(backend2.get_state().call_count, 0);
    
    // Backend1 should have 1 call
    assert_eq!(backend1.get_state().call_count, 1);
}

/// Test screenshot_with_config timeout
#[tokio::test]
async fn test_screenshot_with_config_timeout() {
    let cancellation = CancellationToken::new();
    let config = ScreenshotConfig {
        timeout: Duration::ZERO,
        cancellation,
    };
    
    let mode = ScreenshotMode::Screen;
    let result = smith_windows::core::screenshot::screenshot_with_config(&mode, &config).await;
    
    assert!(matches!(result, Err(ScreenshotError::InvalidConfig(_))));
}
