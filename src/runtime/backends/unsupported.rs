//! Unsupported platform backend for ScreenshotTool, InspectTool, InputTool, InputTextTool, WaitTool, ReadTool, ScrollTool, ToggleTool, FocusTool, and WindowControlTool
//!
//! This module provides stub implementations for non-Windows platforms.

use crate::core::focus::{FocusBackend, FocusError};
use crate::core::input::{InputBackend, InputError};
use crate::core::input_text::{InputTextBackend, InputTextError};
use crate::core::read::{ReadBackend, ReadError};
use crate::core::screenshot::{ScreenshotBackend, ScreenshotError};
use crate::core::scroll::{ScrollBackend, ScrollError};
use crate::core::toggle::{ToggleBackend, ToggleError};
use crate::core::wait::{WaitBackend, WaitError};
use crate::core::window_control::{WindowControlBackend, WindowControlError};

/// Unsupported focus backend implementation
pub struct FocusBackendUnsupported;

impl FocusBackendUnsupported {
    /// Creates a new unsupported focus backend
    pub fn new() -> Self {
        Self
    }
}

impl Default for FocusBackendUnsupported {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait(?Send)]
impl FocusBackend for FocusBackendUnsupported {
    async fn focus(&self, _element: &uiautomation::UIElement) -> Result<(), FocusError> {
        Err(FocusError::UnsupportedPlatform)
    }
}

/// Unsupported screenshot backend implementation
pub struct ScreenshotBackendUnsupported;

impl ScreenshotBackendUnsupported {
    /// Creates a new unsupported screenshot backend
    pub fn new() -> Self {
        Self
    }
}

impl Default for ScreenshotBackendUnsupported {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait(?Send)]
impl ScreenshotBackend for ScreenshotBackendUnsupported {
    async fn capture(
        &self,
        _mode: &crate::core::screenshot::ScreenshotMode,
    ) -> Result<Vec<u8>, ScreenshotError> {
        Err(ScreenshotError::UnsupportedPlatform)
    }
}

/// Unsupported inspect backend implementation
pub struct InspectBackendUnsupported;

impl InspectBackendUnsupported {
    /// Creates a new unsupported backend
    pub fn new() -> Self {
        Self
    }
}

impl Default for InspectBackendUnsupported {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait(?Send)]
impl crate::core::inspect::InspectBackend for InspectBackendUnsupported {
    async fn inspect_path(
        &self,
        _head_window: &uiautomation::UIElement,
        _element: &uiautomation::UIElement,
    ) -> Result<String, crate::core::inspect::InspectError> {
        Err(crate::core::inspect::InspectError::InvalidSelector)
    }
}

/// Unsupported input backend implementation
pub struct InputBackendUnsupported;

impl InputBackendUnsupported {
    /// Creates a new unsupported backend
    pub fn new() -> Self {
        Self
    }
}

impl Default for InputBackendUnsupported {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait(?Send)]
impl crate::core::input::InputBackend for InputBackendUnsupported {
    async fn get_element_at_point(
        &self,
        _x: i32,
        _y: i32,
    ) -> Result<uiautomation::UIElement, InputError> {
        Err(InputError::ElementFromPointError(
            "Unsupported platform".to_string(),
        ))
    }

    async fn move_mouse(&self, _x: i32, _y: i32) -> Result<(), InputError> {
        Err(InputError::MouseMoveError(
            "Unsupported platform".to_string(),
        ))
    }

    async fn click_key(&self, _key: &str) -> Result<(), InputError> {
        Err(InputError::KeyClickError(
            "Unsupported platform".to_string(),
        ))
    }
}

/// Unsupported input text backend implementation
pub struct InputTextBackendUnsupported;

impl InputTextBackendUnsupported {
    /// Creates a new unsupported input text backend
    pub fn new() -> Self {
        Self
    }
}

impl Default for InputTextBackendUnsupported {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait(?Send)]
impl crate::core::input_text::InputTextBackend for InputTextBackendUnsupported {
    async fn input_text(
        &self,
        _element: &uiautomation::UIElement,
        _keys: &str,
    ) -> Result<(), InputTextError> {
        Err(InputTextError::InputSelectorError(
            "Unsupported platform".to_string(),
        ))
    }
}

/// Unsupported wait backend implementation
pub struct WaitBackendUnsupported;

impl WaitBackendUnsupported {
    /// Creates a new unsupported wait backend
    pub fn new() -> Self {
        Self
    }
}

impl Default for WaitBackendUnsupported {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait(?Send)]
impl crate::core::wait::WaitBackend for WaitBackendUnsupported {
    async fn wait_element(
        &self,
        _automation: &uiautomation::UIAutomation,
        _root: &uiautomation::UIElement,
        _selector: &crate::core::wait::WaitSelector,
    ) -> Result<bool, WaitError> {
        Err(WaitError::InvalidConfig("Unsupported platform".to_string()))
    }
}

/// Unsupported read backend implementation
pub struct ReadBackendUnsupported;

impl ReadBackendUnsupported {
    /// Creates a new unsupported read backend
    pub fn new() -> Self {
        Self
    }
}

impl Default for ReadBackendUnsupported {
    fn default() -> Self {
        Self::new()
    }
}

/// Unsupported scroll backend implementation
pub struct ScrollBackendUnsupported;

impl ScrollBackendUnsupported {
    /// Creates a new unsupported scroll backend
    pub fn new() -> Self {
        Self
    }
}

impl Default for ScrollBackendUnsupported {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait(?Send)]
impl crate::core::scroll::ScrollBackend for ScrollBackendUnsupported {
    async fn scroll_vertical(
        &self,
        _element: &uiautomation::UIElement,
        _amount: i32,
        _unit: crate::core::scroll::ScrollUnit,
    ) -> Result<(), ScrollError> {
        Err(ScrollError::UnsupportedPlatform)
    }

    async fn scroll_horizontal(
        &self,
        _element: &uiautomation::UIElement,
        _amount: i32,
        _unit: crate::core::scroll::ScrollUnit,
    ) -> Result<(), ScrollError> {
        Err(ScrollError::UnsupportedPlatform)
    }

    async fn simulate_mouse_wheel(
        &self,
        _ticks: i32,
        _direction: crate::core::scroll::ScrollDirection,
    ) -> Result<(), ScrollError> {
        Err(ScrollError::UnsupportedPlatform)
    }
}

#[async_trait::async_trait(?Send)]
impl crate::core::read::ReadBackend for ReadBackendUnsupported {
    async fn read_text(&self, _element: &uiautomation::UIElement) -> Result<String, ReadError> {
        Err(ReadError::UnsupportedPlatform)
    }
}

/// Unsupported toggle backend implementation
pub struct ToggleBackendUnsupported;

impl ToggleBackendUnsupported {
    /// Creates a new unsupported toggle backend
    pub fn new() -> Self {
        Self
    }
}

impl Default for ToggleBackendUnsupported {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait(?Send)]
impl ToggleBackend for ToggleBackendUnsupported {
    async fn toggle_element(&self, _element: &uiautomation::UIElement) -> Result<(), ToggleError> {
        Err(ToggleError::UnsupportedPlatform)
    }

    async fn set_radio(
        &self,
        _element: &uiautomation::UIElement,
        _selected: bool,
    ) -> Result<(), ToggleError> {
        Err(ToggleError::UnsupportedPlatform)
    }

    async fn set_toggle(
        &self,
        _element: &uiautomation::UIElement,
        _state: bool,
    ) -> Result<(), ToggleError> {
        Err(ToggleError::UnsupportedPlatform)
    }

    async fn is_checked(&self, _element: &uiautomation::UIElement) -> Result<bool, ToggleError> {
        Err(ToggleError::UnsupportedPlatform)
    }

    async fn is_selected(&self, _element: &uiautomation::UIElement) -> Result<bool, ToggleError> {
        Err(ToggleError::UnsupportedPlatform)
    }
}

/// Unsupported window control backend implementation
pub struct WindowControlBackendUnsupported;

impl WindowControlBackendUnsupported {
    /// Creates a new unsupported window control backend
    pub fn new() -> Self {
        Self
    }
}

impl Default for WindowControlBackendUnsupported {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait(?Send)]
impl WindowControlBackend for WindowControlBackendUnsupported {
    async fn window_control(
        &self,
        _element: &uiautomation::UIElement,
        _action: crate::core::window_control::WindowControlAction,
    ) -> Result<(), WindowControlError> {
        Err(WindowControlError::UnsupportedPlatform)
    }
}
