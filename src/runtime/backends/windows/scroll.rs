//! Windows backend for scroll operations using UI Automation

use tracing::{error, info};

use crate::core::scroll::{ScrollDirection, ScrollError, ScrollUnit};

/// Windows scroll backend implementation
pub struct ScrollBackendWindows;

impl ScrollBackendWindows {
    /// Creates a new Windows scroll backend
    pub fn new() -> Self {
        Self
    }
}

impl Default for ScrollBackendWindows {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait(?Send)]
impl crate::core::scroll::ScrollBackend for ScrollBackendWindows {
    async fn scroll_vertical(
        &self,
        element: &uiautomation::UIElement,
        amount: i32,
        unit: ScrollUnit,
    ) -> Result<(), ScrollError> {
        // Check element validity first
        let _control_type = match element.get_control_type() {
            Ok(val) => val,
            Err(e) => {
                error!("Failed to get element control type: {}", e);
                return Err(ScrollError::ElementNotFound);
            }
        };

        // Check if element is enabled
        let enabled_result = element.is_enabled();
        let is_enabled = match enabled_result {
            Ok(val) => val,
            Err(e) => {
                error!("Failed to check if element is enabled: {}", e);
                return Err(ScrollError::ComError(e.to_string()));
            }
        };

        if !is_enabled {
            error!("Scroll failed: element is disabled");
            return Err(ScrollError::ElementNotEnabled);
        }

        // Check if element is offscreen
        let offscreen_result = element.is_offscreen();
        let is_offscreen = match offscreen_result {
            Ok(val) => val,
            Err(e) => {
                error!("Failed to check if element is offscreen: {}", e);
                return Err(ScrollError::ComError(e.to_string()));
            }
        };

        if is_offscreen {
            error!("Scroll failed: element is offscreen");
            return Err(ScrollError::ElementOffscreen);
        }

        // Try programmatic scrolling first (IScrollPattern or IRangeValuePattern)
        match self.scroll_programmatic(element, amount, unit).await {
            Ok(()) => {
                info!("Vertical scroll completed successfully (programmatic)");
                Ok(())
            }
            Err(ScrollError::PatternNotSupported) => {
                // Fallback to synthetic scrolling (wheel)
                info!("Pattern not supported, trying synthetic scrolling");
                // Note: mouse wheel simulation not yet implemented
                // Return error indicating limitation
                Err(ScrollError::InvalidConfig(
                    "Mouse wheel simulation not yet implemented".to_string(),
                ))
            }
            Err(e) => Err(e),
        }
    }

    async fn scroll_horizontal(
        &self,
        element: &uiautomation::UIElement,
        amount: i32,
        unit: ScrollUnit,
    ) -> Result<(), ScrollError> {
        // Check element validity first
        let _control_type = match element.get_control_type() {
            Ok(val) => val,
            Err(e) => {
                error!("Failed to get element control type: {}", e);
                return Err(ScrollError::ElementNotFound);
            }
        };

        // Check if element is enabled
        let enabled_result = element.is_enabled();
        let is_enabled = match enabled_result {
            Ok(val) => val,
            Err(e) => {
                error!("Failed to check if element is enabled: {}", e);
                return Err(ScrollError::ComError(e.to_string()));
            }
        };

        if !is_enabled {
            error!("Scroll failed: element is disabled");
            return Err(ScrollError::ElementNotEnabled);
        }

        // Check if element is offscreen
        let offscreen_result = element.is_offscreen();
        let is_offscreen = match offscreen_result {
            Ok(val) => val,
            Err(e) => {
                error!("Failed to check if element is offscreen: {}", e);
                return Err(ScrollError::ComError(e.to_string()));
            }
        };

        if is_offscreen {
            error!("Scroll failed: element is offscreen");
            return Err(ScrollError::ElementOffscreen);
        }

        // Try programmatic scrolling first (IScrollPattern or IRangeValuePattern)
        match self.scroll_programmatic(element, amount, unit).await {
            Ok(()) => {
                info!("Horizontal scroll completed successfully (programmatic)");
                Ok(())
            }
            Err(ScrollError::PatternNotSupported) => {
                // Fallback to synthetic scrolling (wheel)
                info!("Pattern not supported, trying synthetic scrolling");
                // Note: mouse wheel simulation not yet implemented
                // Return error indicating limitation
                Err(ScrollError::InvalidConfig(
                    "Mouse wheel simulation not yet implemented".to_string(),
                ))
            }
            Err(e) => Err(e),
        }
    }

    async fn simulate_mouse_wheel(
        &self,
        _ticks: i32,
        _direction: ScrollDirection,
    ) -> Result<(), ScrollError> {
        // Note: uiautomation crate v0.24.4 does not support mouse wheel simulation
        // This would require WinAPI SendInput (not yet implemented)
        error!("Mouse wheel simulation not supported in uiautomation crate v0.24.4");
        Err(ScrollError::InvalidConfig(
            "Mouse wheel simulation requires WinAPI SendInput (not yet implemented)".to_string(),
        ))
    }
}

impl ScrollBackendWindows {
    /// Attempts programmatic scrolling using UI Automation patterns
    /// Tries UIScrollPattern first, then IRangeValuePattern as fallback
    async fn scroll_programmatic(
        &self,
        element: &uiautomation::UIElement,
        amount: i32,
        unit: ScrollUnit,
    ) -> Result<(), ScrollError> {
        // Try UIScrollPattern first
        let scroll_pattern = match element.get_pattern::<uiautomation::patterns::UIScrollPattern>()
        {
            Ok(pattern) => pattern,
            Err(_) => {
                // IScrollPattern not supported, try IRangeValuePattern
                return self.scroll_by_range_value(element, amount, unit).await;
            }
        };

        // Get current scroll position
        let vertical_percent = match scroll_pattern.get_vertical_scroll_percent() {
            Ok(val) => val,
            Err(e) => {
                error!("Failed to get vertical scroll percent: {}", e);
                return Err(ScrollError::ComError(e.to_string()));
            }
        };

        // Determine scroll direction based on amount
        let amount_percent = if amount > 0 {
            // Scroll down
            5.0 // 5% per tick
        } else {
            // Scroll up
            -5.0
        };

        let new_percent = (vertical_percent + amount_percent).clamp(0.0, 100.0);

        // Perform the scroll
        let result = scroll_pattern.set_scroll_percent(0.0, new_percent);

        match result {
            Ok(()) => Ok(()),
            Err(e) => {
                error!("Programmatic scroll failed: {}", e);
                Err(ScrollError::ComError(e.to_string()))
            }
        }
    }

    /// Attempts scrolling using IRangeValuePattern (for scrollbars, sliders)
    async fn scroll_by_range_value(
        &self,
        element: &uiautomation::UIElement,
        amount: i32,
        _unit: ScrollUnit,
    ) -> Result<(), ScrollError> {
        let range_value_pattern =
            match element.get_pattern::<uiautomation::patterns::UIRangeValuePattern>() {
                Ok(pattern) => pattern,
                Err(_) => {
                    return Err(ScrollError::PatternNotSupported);
                }
            };

        // Get current value and range
        let current_value = match range_value_pattern.get_value() {
            Ok(val) => val,
            Err(e) => {
                error!("Failed to get range value: {}", e);
                return Err(ScrollError::ComError(e.to_string()));
            }
        };

        let min_value = match range_value_pattern.get_minimum() {
            Ok(val) => val,
            Err(e) => {
                error!("Failed to get minimum value: {}", e);
                return Err(ScrollError::ComError(e.to_string()));
            }
        };

        let max_value = match range_value_pattern.get_maximum() {
            Ok(val) => val,
            Err(e) => {
                error!("Failed to get maximum value: {}", e);
                return Err(ScrollError::ComError(e.to_string()));
            }
        };

        // Determine new value based on amount
        let amount_value = if amount > 0 {
            // Scroll down/right
            (max_value - min_value) * 0.05 // 5% per tick
        } else {
            // Scroll up/left
            -(max_value - min_value) * 0.05
        };

        let new_value = (current_value + amount_value).clamp(min_value, max_value);

        // Perform the scroll
        let result = range_value_pattern.set_value(new_value);

        match result {
            Ok(()) => Ok(()),
            Err(e) => {
                error!("RangeValuePattern set_value failed: {}", e);
                Err(ScrollError::ComError(e.to_string()))
            }
        }
    }
}
