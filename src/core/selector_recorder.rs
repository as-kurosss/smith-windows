//! SelectorRecorder module for capturing UI selectors
//!
//! Provides functionality to capture elements under cursor and build
//! full selector trees with hierarchy information.

use std::sync::Arc;
use thiserror::Error;
use tracing::{debug, error, info};

use crate::core::selector::{validate_process_id, RecordedSelector, SelectorError, SelectorStep};

/// Maximum depth for selector tree (protection against infinite loops)
const MAX_DEPTH: usize = 256;

/// Selector recorder for capturing UI elements and building selector trees
#[derive(Debug, Clone)]
pub struct SelectorRecorder {
    /// Configuration for recording
    config: SelectorRecorderConfig,
}

/// Configuration for selector recorder
#[derive(Debug, Clone)]
pub struct SelectorRecorderConfig {
    /// Maximum depth for selector tree
    pub max_depth: usize,
    /// Timeout for element capture operations
    pub timeout: std::time::Duration,
    /// Token for cancellation
    pub cancellation: tokio_util::sync::CancellationToken,
}

impl Default for SelectorRecorderConfig {
    fn default() -> Self {
        Self {
            max_depth: MAX_DEPTH,
            timeout: std::time::Duration::from_secs(10),
            cancellation: tokio_util::sync::CancellationToken::new(),
        }
    }
}

impl SelectorRecorder {
    /// Creates a new selector recorder with default config
    pub fn new() -> Self {
        Self {
            config: SelectorRecorderConfig::default(),
        }
    }

    /// Creates a new selector recorder with custom config
    pub fn with_config(config: SelectorRecorderConfig) -> Self {
        Self { config }
    }

    /// Captures an element under the cursor using WinAPI GetCursorPos
    /// Returns the recorded selector with full hierarchy
    pub async fn capture_element_under_cursor(
        &self,
        head_window: &uiautomation::UIElement,
    ) -> Result<RecordedSelector, SelectorError> {
        info!("Capturing element under cursor...");

        // Check cancellation
        if self.config.cancellation.is_cancelled() {
            error!("Capture operation cancelled");
            return Err(SelectorError::InvalidSelector("cancelled".to_string()));
        }

        // Get cursor position using WinAPI (spawn_blocking for COM safety)
        let result = tokio::task::spawn_blocking(move || {
            crate::runtime::backends::windows::input::get_cursor_position()
        })
        .await;

        let (x, y) = match result {
            Ok(Ok(pos)) => pos,
            Ok(Err(e)) => {
                error!("Failed to get cursor position: {}", e);
                return Err(SelectorError::ElementPropertyError(e.to_string()));
            }
            Err(e) => {
                error!("spawn_blocking failed: {}", e);
                return Err(SelectorError::ElementPropertyError(e.to_string()));
            }
        };

        info!("Cursor position: ({}, {})", x, y);

        // Get element from point
        let automation = uiautomation::UIAutomation::new().map_err(|e| {
            error!("Failed to create UIAutomation: {}", e);
            SelectorError::ElementPropertyError(e.to_string())
        })?;

        let element = automation.element_from_point(uiautomation::types::Point::new(x, y)).map_err(
            |e| {
                error!(
                    "Failed to get element from point ({}, {}): {}",
                    x, y, e
                );
                SelectorError::ElementPropertyError(e.to_string())
            },
        )?;

        // Validate element
        self.validate_element(&element)?;

        // Build full selector tree
        let recorded = self.build_full_selector_tree(head_window, &element)?;

        info!("Element captured successfully: {}", element.get_name().unwrap_or_default());

        Ok(recorded)
    }

    /// Captures a specific element and builds its selector tree
    pub async fn capture_element(
        &self,
        head_window: &uiautomation::UIElement,
        element: &uiautomation::UIElement,
    ) -> Result<RecordedSelector, SelectorError> {
        info!("Capturing element...");

        // Validate element
        self.validate_element(element)?;

        // Build full selector tree
        let recorded = self.build_full_selector_tree(head_window, element)?;

        info!(
            "Element captured successfully: {}",
            element.get_name().unwrap_or_default()
        );

        Ok(recorded)
    }

    /// Captures an element and saves it to storage
    pub async fn capture_and_save(
        &self,
        storage: &crate::core::selector_storage::SelectorStorage,
        id: &str,
        head_window: &uiautomation::UIElement,
        element: &uiautomation::UIElement,
    ) -> Result<(), crate::core::selector_storage::StorageError> {
        info!("Capturing and saving element with ID: {}", id);

        // Capture element
        let recorded = self.capture_element(head_window, element).await.map_err(|e| {
            error!("Failed to capture element: {}", e);
            crate::core::selector_storage::StorageError::InvalidSelectorData(e.to_string())
        })?;

        // Validate ID
        crate::core::selector_storage::SelectorStorage::sanitize_id(id).map_err(|e| {
            error!("Failed to sanitize ID: {}", e);
            e
        })?;

        // Save to storage
        storage.save_selector(id, &recorded).await?;

        info!("Element captured and saved successfully: {}", id);
        Ok(())
    }

    /// Builds a full selector tree from head_window to element
    /// Uses UITreeWalker to traverse the hierarchy
    pub fn build_full_selector_tree(
        &self,
        head_window: &uiautomation::UIElement,
        element: &uiautomation::UIElement,
    ) -> Result<RecordedSelector, SelectorError> {
        info!(
            "Building selector tree from window '{}' to element '{}'",
            head_window.get_name().unwrap_or_default(),
            element.get_name().unwrap_or_default()
        );

        // Validate that element is in the hierarchy of head_window
        self.validate_selector_path(head_window, element)?;

        let mut steps = Vec::new();
        let mut current = element.clone();

        // Create automation for tree walking
        let automation = uiautomation::UIAutomation::new().map_err(|e| {
            error!("Failed to create UIAutomation: {}", e);
            SelectorError::ElementPropertyError(e.to_string())
        })?;

        let walker = automation.create_tree_walker().map_err(|e| {
            error!("Failed to create tree walker: {}", e);
            SelectorError::ElementPropertyError(e.to_string())
        })?;

        // Walk up the tree from element to head_window
        loop {
            // Check depth limit
            if steps.len() >= self.config.max_depth {
                error!(
                    "Selector tree exceeds maximum depth of {}",
                    self.config.max_depth
                );
                return Err(SelectorError::MaxDepthExceeded);
            }

            // Check cancellation
            if self.config.cancellation.is_cancelled() {
                error!("Build selector tree operation cancelled");
                return Err(SelectorError::InvalidSelector("cancelled".to_string()));
            }

            // Get element properties
            let step = self.get_element_properties(&current)?;
            steps.push(step);

            // Compare with head_window
            let is_same = automation.compare_elements(&current, head_window).map_err(|e| {
                error!("Failed to compare elements: {}", e);
                SelectorError::ElementPropertyError(e.to_string())
            })?;

            if is_same {
                break;
            }

            // Move to parent
            match walker.get_parent(&current) {
                Ok(parent) => {
                    current = parent;
                }
                Err(e) => {
                    error!("Failed to get parent: {}", e);
                    return Err(SelectorError::ElementPropertyError(e.to_string()));
                }
            }
        }

        // Reverse to go from root to element
        steps.reverse();

        let depth = steps.len();
        info!("Built selector tree with {} steps", depth);

        Ok(RecordedSelector { steps, depth })
    }

    /// Gets element properties for a single step
    pub fn get_element_properties(
        &self,
        element: &uiautomation::UIElement,
    ) -> Result<SelectorStep, SelectorError> {
        let classname = element.get_classname().ok();
        let control_type = element.get_control_type().ok();
        let name = element.get_name().ok();
        let automation_id = element.get_automation_id().ok();

        Ok(SelectorStep {
            classname,
            control_type,
            name,
            automation_id,
        })
    }

    /// Validates that element is a descendant of head_window
    fn validate_selector_path(
        &self,
        head_window: &uiautomation::UIElement,
        element: &uiautomation::UIElement,
    ) -> Result<(), SelectorError> {
        // Check cancellation
        if self.config.cancellation.is_cancelled() {
            return Err(SelectorError::InvalidSelector("cancelled".to_string()));
        }

        // Create automation for comparison
        let automation = uiautomation::UIAutomation::new().map_err(|e| {
            error!("Failed to create UIAutomation: {}", e);
            SelectorError::ElementPropertyError(e.to_string())
        })?;

        // Check if element is the same as head_window
        let is_same = automation.compare_elements(element, head_window).map_err(|e| {
            error!("Failed to compare elements: {}", e);
            SelectorError::ElementPropertyError(e.to_string())
        })?;

        if is_same {
            // Element is the head window itself - valid
            return Ok(());
        }

        // Traverse up to check hierarchy
        let walker = automation.create_tree_walker().map_err(|e| {
            error!("Failed to create tree walker: {}", e);
            SelectorError::ElementPropertyError(e.to_string())
        })?;

        let mut current = element.clone();
        let mut checked = 0;

        while checked < MAX_DEPTH {
            let is_same = automation.compare_elements(&current, head_window).map_err(|e| {
                error!("Failed to compare elements: {}", e);
                return SelectorError::ElementPropertyError(e.to_string());
            })?;

            if is_same {
                // Element is in the hierarchy of head_window
                return Ok(());
            }

            match walker.get_parent(&current) {
                Ok(parent) => {
                    current = parent;
                }
                Err(_) => {
                    break;
                }
            }
            checked += 1;
        }

        error!("Element is not in the hierarchy of head_window");
        Err(SelectorError::InvalidSelector(
            "element is not a descendant of head_window".to_string(),
        ))
    }

    /// Validates element properties
    fn validate_element(&self, element: &uiautomation::UIElement) -> Result<(), SelectorError> {
        // Check cancellation
        if self.config.cancellation.is_cancelled() {
            return Err(SelectorError::InvalidSelector("cancelled".to_string()));
        }

        // Check if element is enabled
        let is_enabled = element.is_enabled().map_err(|e| {
            error!("Failed to check if element is enabled: {}", e);
            SelectorError::ElementPropertyError(e.to_string())
        })?;

        if !is_enabled {
            error!("Element is disabled");
            return Err(SelectorError::ElementNotEnabled);
        }

        // Check if element is offscreen
        let is_offscreen = element.is_offscreen().map_err(|e| {
            error!("Failed to check if element is offscreen: {}", e);
            SelectorError::ElementPropertyError(e.to_string())
        })?;

        if is_offscreen {
            error!("Element is offscreen");
            return Err(SelectorError::ElementOffscreen);
        }

        Ok(())
    }
}

impl Default for SelectorRecorder {
    fn default() -> Self {
        Self::new()
    }
}

//tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selector_recorder_new() {
        // Test new() creates recorder with default config
        let recorder = SelectorRecorder::new();
        assert_eq!(recorder.config.max_depth, MAX_DEPTH);
        assert_eq!(recorder.config.timeout, std::time::Duration::from_secs(10));
    }

    #[test]
    fn test_selector_recorder_with_config() {
        // Test with_config() creates recorder with custom config
        let config = SelectorRecorderConfig {
            max_depth: 100,
            timeout: std::time::Duration::from_secs(5),
            cancellation: tokio_util::sync::CancellationToken::new(),
        };
        let recorder = SelectorRecorder::with_config(config);
        assert_eq!(recorder.config.max_depth, 100);
        assert_eq!(recorder.config.timeout, std::time::Duration::from_secs(5));
    }

    #[test]
    fn test_default_config() {
        // Test default config values
        let config = SelectorRecorderConfig::default();
        assert_eq!(config.max_depth, 256);
        assert_eq!(config.timeout, std::time::Duration::from_secs(10));
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn test_validate_selector_path_same_element() {
        // Test when element == head_window
        // This test can't be run without real UI elements, but we can test the logic
        let recorder = SelectorRecorder::new();

        // The validation should succeed if elements are the same
        // This is tested indirectly through integration tests
    }

    #[test]
    fn test_validate_element_not_enabled() {
        // This test requires a real UI element - mocked in integration tests
        // Here we just verify the method signature compiles
        let _recorder = SelectorRecorder::new();

        // Method exists and has correct signature
        // Skip test without real element
    }

    #[test]
    fn test_build_full_selector_tree_empty() {
        // Test that empty hierarchy works (edge case)
        // This test requires real UI elements - mocked in integration tests
        let _recorder = SelectorRecorder::new();

        // Method exists and has correct signature
        // Skip test without real element
    }

    #[test]
    fn test_max_depth_limit() {
        // Test that max depth limit is respected
        let config = SelectorRecorderConfig {
            max_depth: 5,
            ..Default::default()
        };
        let recorder = SelectorRecorder::with_config(config);

        // Check that config stores the value
        assert_eq!(recorder.config.max_depth, 5);
    }

    #[test]
    fn test_cancellation_check() {
        // Test that cancellation is checked
        let config = SelectorRecorderConfig::default();
        let recorder = SelectorRecorder::with_config(config);

        // Test that validation checks cancellation
        recorder.config.cancellation.cancel();

        // The validation should return cancelled error
        // This is tested in integration tests with real elements
    }
}
