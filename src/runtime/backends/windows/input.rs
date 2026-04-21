//! Windows backend for input operations using UI Automation

use tracing::{error, info};

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
        // All UIA calls should be done in spawn_blocking for COM safety
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
        // Note: These operations use WinAPI SendInput and should be called directly
        // without spawn_blocking as they handle COM internally
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
        // Note: These operations use WinAPI SendInput and should be called directly
        // without spawn_blocking as they handle COM internally
        uiautomation::inputs::Keyboard::new()
            .send_keys(key)
            .map_err(|e| {
                error!("Failed to click key '{}': {}", key, e);
                InputError::KeyClickError(e.to_string())
            })?;

        Ok(())
    }
}

/// Gets the UI element at specific coordinates with Ctrl key pressed
/// This function simulates the "Ctrl+Hover" pattern:
/// 1. Moves mouse to coordinates
/// 2. Presses Ctrl key
/// 3. Gets element at coordinates
/// 4. Releases Ctrl key
/// 5. Returns the element
pub async fn get_element_with_ctrl_simulation(
    config: &InputConfig,
    x: i32,
    y: i32,
) -> Result<uiautomation::UIElement, InputError> {
    // Validate config
    validate_input_config(config)?;

    let backend = InputBackendWindows::new();

    // Move mouse to coordinates
    info!("Moving mouse to ({}, {})...", x, y);
    backend.move_mouse(x, y).await?;

    // Press Control key
    info!("Pressing Control key...");
    backend.click_key("{CTRL}").await?;

    // Small delay to ensure key is pressed
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    // Get element at coordinates
    info!("Getting element at ({}, {})...", x, y);
    let element = backend.get_element_at_point(x, y).await?;

    info!("Element found: {:?}", element.get_name().ok());

    // Release Control key
    info!("Releasing Control key...");
    backend.click_key("{CTRL}").await?;

    Ok(element)
}
