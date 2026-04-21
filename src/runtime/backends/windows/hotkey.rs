//! Hotkey module for global keyboard shortcuts
//!
//! Provides functionality to wait for global keyboard shortcuts
//! using Windows Win32 API polling.

use std::time::Duration;
use tracing::{error, info};

use crate::core::input::InputError;

/// Virtual key codes
mod vk {
    pub const VK_CONTROL: i32 = 0x11;
}

/// Waits for a global hotkey (Ctrl+Hover) to be pressed
/// Returns the current cursor position when hotkey is pressed
pub async fn wait_for_hotkey(timeout: Duration) -> Result<(i32, i32), InputError> {
    info!("Waiting for global hotkey (Ctrl+Hover)...");

    // Use polling with GetAsyncKeyState instead of RegisterHotKey
    // because hotkey registration requires a window handle
    let start = std::time::Instant::now();

    while start.elapsed() < timeout {
        // Check if Ctrl is held using GetAsyncKeyState
        let ctrl_pressed = unsafe {
            windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState(vk::VK_CONTROL)
        };

        if (ctrl_pressed & (-32768i16)) != 0 {
            // Ctrl is pressed, get cursor position
            let (x, y) = get_cursor_position()?;
            info!("Hotkey pressed at ({}, {})", x, y);
            return Ok((x, y));
        }

        // Small delay to avoid busy-waiting
        tokio::time::sleep(Duration::from_millis(10)).await;
    }

    error!("Hotkey wait timed out");
    Err(InputError::ComError("Hotkey wait timed out".to_string()))
}

/// Gets the current cursor position using Win32 API
/// This is a wrapper around GetCursorPos for convenience
pub fn get_cursor_position() -> Result<(i32, i32), InputError> {
    let mut point = windows::Win32::Foundation::POINT { x: 0, y: 0 };

    unsafe {
        match windows::Win32::UI::WindowsAndMessaging::GetCursorPos(&mut point) {
            Ok(()) => Ok((point.x, point.y)),
            Err(e) => {
                error!("Failed to get cursor position: {}", e);
                Err(InputError::ComError(format!("GetCursorPos failed: {}", e)))
            }
        }
    }
}
