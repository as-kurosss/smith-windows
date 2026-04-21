//! Windows backend for input operations using UI Automation

use tracing::{error, info};
use windows::Win32::Foundation::POINT;
use windows::Win32::UI::WindowsAndMessaging::GetCursorPos;

use crate::core::input::{validate_input_config, InputBackend, InputConfig, InputError};

/// Windows input backend implementation
pub struct InputBackendWindows;

impl InputBackendWindows {
    /// Creates a new Windows input backend
    pub fn new() -> Self {
        Self
    }
}

impl Default for InputBackendWindows {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait(?Send)]
impl InputBackend for InputBackendWindows {
    async fn get_element_at_point(
        &self,
        x: i32,
        y: i32,
    ) -> Result<uiautomation::UIElement, InputError> {
        // Use UIAutomation to get element from point
        let automation = uiautomation::UIAutomation::new().map_err(|e| {
            error!("Failed to create UIAutomation: {}", e);
            InputError::ComError(e.to_string())
        })?;

        let element = automation
            .element_from_point(uiautomation::types::Point::new(x, y))
            .map_err(|e| {
                error!("Failed to get element from point ({}, {}): {}", x, y, e);
                InputError::ElementFromPointError(e.to_string())
            })?;

        Ok(element)
    }

    async fn move_mouse(&self, x: i32, y: i32) -> Result<(), InputError> {
        // Move mouse to coordinates using uiautomation::inputs
        uiautomation::inputs::Mouse::new()
            .move_to(&uiautomation::types::Point::new(x, y))
            .map_err(|e| {
                error!("Failed to move mouse: {}", e);
                InputError::MouseMoveError(e.to_string())
            })?;

        Ok(())
    }

    async fn click_key(&self, key: &str) -> Result<(), InputError> {
        // Click key using uiautomation::inputs
        uiautomation::inputs::Keyboard::new()
            .send_keys(key)
            .map_err(|e| {
                error!("Failed to click key '{}': {}", key, e);
                InputError::KeyClickError(e.to_string())
            })?;

        Ok(())
    }
}

/// Gets the current cursor position using WinAPI GetCursorPos
/// Returns (x, y) coordinates relative to the primary monitor
pub fn get_cursor_position() -> Result<(i32, i32), InputError> {
    let mut point = POINT { x: 0, y: 0 };

    unsafe {
        match GetCursorPos(&mut point) {
            Ok(()) => Ok((point.x, point.y)),
            Err(_) => {
                error!("Failed to get cursor position");
                Err(InputError::ComError("GetCursorPos failed".to_string()))
            }
        }
    }
}

/// Gets the UI element under the cursor (at current cursor position)
/// Uses GetCursorPos to get coordinates, then element_from_point to get the element
pub async fn get_element_under_cursor() -> Result<uiautomation::UIElement, InputError> {
    info!("Getting cursor position...");
    let (x, y) = get_cursor_position()?;
    info!("Cursor position: ({}, {})", x, y);

    let backend = InputBackendWindows::new();
    backend.get_element_at_point(x, y).await
}

/// Gets the UI element at specific coordinates with Ctrl key pressed
/// This function simulates the "Ctrl+Hover" pattern:
/// 1. Gets current cursor position
/// 2. Presses Ctrl key
/// 3. Gets element at cursor position
/// 4. Releases Ctrl key
/// 5. Returns the element
pub async fn get_element_with_ctrl_simulation(
    config: &InputConfig,
) -> Result<uiautomation::UIElement, InputError> {
    // Validate config
    validate_input_config(config)?;

    info!("Getting cursor position...");
    let (x, y) = get_cursor_position()?;
    info!("Cursor position: ({}, {})", x, y);

    let backend = InputBackendWindows::new();

    // Press Control key
    info!("Pressing Control key...");
    backend.click_key("{CTRL}").await?;

    // Small delay to ensure key is pressed
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    // Get element at cursor position
    info!("Getting element at cursor position...");
    let element = backend.get_element_at_point(x, y).await?;

    info!("Element found: {:?}", element.get_name().ok());

    // Release Control key
    info!("Releasing Control key...");
    backend.click_key("{CTRL}").await?;

    Ok(element)
}

/// Gets the UI element under cursor when user presses Ctrl (real Ctrl+Hover)
/// This function waits for the user to physically press Ctrl while hovering over an element
/// Uses hotkey registration and polling to detect the Ctrl key press
pub async fn get_element_under_ctrl_hotkey(
    config: &InputConfig,
) -> Result<uiautomation::UIElement, InputError> {
    // Validate config
    validate_input_config(config)?;

    info!("Waiting for real Ctrl+Hover (user will press Ctrl)...");

    // Get timeout from config
    let timeout = config.timeout;

    // Register hotkey and wait for Ctrl press
    let (x, y) = crate::runtime::backends::windows::hotkey::wait_for_hotkey(timeout).await?;

    info!("Ctrl+Hover detected at ({}, {})", x, y);

    // Get element at cursor position
    let element = InputBackendWindows::new()
        .get_element_at_point(x, y)
        .await?;

    info!("Element found: {:?}", element.get_name().ok());

    Ok(element)
}

/// Unregisters the hotkey (no-op in polling implementation)
pub fn unregister_hotkey() {
    // No-op in polling mode
}
