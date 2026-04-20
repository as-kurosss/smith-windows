//! Windows backend for inspect operations using UI Automation

use tracing::{error, info};

use crate::core::inspect::validate_inspect_path;
use crate::core::inspect::{InspectBackend, InspectError};

/// Windows inspect backend implementation
pub struct InspectBackendWindows;

impl InspectBackendWindows {
    /// Creates a new Windows inspect backend
    pub fn new() -> Self {
        Self
    }
}

impl Default for InspectBackendWindows {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait(?Send)]
impl InspectBackend for InspectBackendWindows {
    async fn inspect_path(
        &self,
        head_window: &uiautomation::UIElement,
        element: &uiautomation::UIElement,
    ) -> Result<String, InspectError> {
        // Check element validity first - access a property to validate
        let _control_type = match element.get_control_type() {
            Ok(val) => val,
            Err(e) => {
                error!("Failed to get element control type: {}", e);
                return Err(InspectError::ElementNotFound);
            }
        };

        // Check if element is enabled
        let enabled_result = element.is_enabled();
        let is_enabled = match enabled_result {
            Ok(val) => val,
            Err(e) => {
                error!("Failed to check if element is enabled: {}", e);
                return Err(InspectError::ComError(e.to_string()));
            }
        };

        if !is_enabled {
            error!("Inspect failed: element is disabled");
            return Err(InspectError::ElementNotEnabled);
        }

        // Check if element is offscreen
        let offscreen_result = element.is_offscreen();
        let is_offscreen = match offscreen_result {
            Ok(val) => val,
            Err(e) => {
                error!("Failed to check if element is offscreen: {}", e);
                return Err(InspectError::ComError(e.to_string()));
            }
        };

        if is_offscreen {
            error!("Inspect failed: element is offscreen");
            return Err(InspectError::ElementOffscreen);
        }

        // Validate hierarchy (element must be in head_window's subtree)
        match validate_inspect_path(head_window, element) {
            Ok(()) => {}
            Err(e) => {
                error!("Inspect failed: invalid hierarchy - {}", e);
                return Err(e);
            }
        }

        // Build the full inspect path using UITreeWalker
        // Since UIElement doesn't expose get_parent(), we use UIAutomation's tree walker
        let path = match build_inspect_path_with_walker(head_window, element) {
            Ok(p) => p,
            Err(e) => {
                error!("Failed to build inspect path: {}", e);
                return Err(e);
            }
        };

        info!("Inspect operation completed successfully, path: {}", path);
        Ok(path)
    }
}

/// Builds the inspect path from head_window to element using UITreeWalker
/// This allows us to traverse up the tree and build the full path
fn build_inspect_path_with_walker(
    head_window: &uiautomation::UIElement,
    element: &uiautomation::UIElement,
) -> Result<String, InspectError> {
    // Get UIAutomation instance to create tree walker
    let automation = uiautomation::UIAutomation::new().map_err(|e| {
        error!("Failed to create UIAutomation: {}", e);
        InspectError::ComError(e.to_string())
    })?;

    // Create tree walker for traversing the UI tree
    let walker = automation.create_tree_walker().map_err(|e| {
        error!("Failed to create tree walker: {}", e);
        InspectError::ComError(e.to_string())
    })?;

    // Use UIAutomation::compare_elements to compare elements since UIElement doesn't implement PartialEq
    let mut current = element.clone();
    let mut path_parts: Vec<String> = Vec::new();

    loop {
        // Get control type
        let control_type = match current.get_control_type() {
            Ok(val) => val,
            Err(_) => {
                return Err(InspectError::ComError(
                    "Failed to get control type".to_string(),
                ));
            }
        };

        // Get name
        let name = current.get_name().unwrap_or_default();

        // Format part: ControlType{Name} or ControlType
        let part = if name.is_empty() {
            control_type.to_string()
        } else {
            format!("{}{{{}}}", control_type, name)
        };

        path_parts.push(part);

        // Check if we reached head_window
        let is_same = automation
            .compare_elements(&current, head_window)
            .map_err(|e| {
                error!("Failed to compare elements: {}", e);
                InspectError::ComError(e.to_string())
            })?;

        if is_same {
            break;
        }

        // Get parent using tree walker
        let parent = walker.get_parent(&current).map_err(|e| {
            error!("Failed to get parent element: {}", e);
            InspectError::ComError(e.to_string())
        })?;

        current = parent;

        // Safety check - max depth
        if path_parts.len() > 256 {
            return Err(InspectError::InvalidSelector);
        }
    }

    // Add head_window to path (first element)
    let head_control_type = head_window.get_control_type().map_err(|_| {
        InspectError::ComError("Failed to get head window control type".to_string())
    })?;
    let head_name = head_window.get_name().unwrap_or_default();
    let head_part = if head_name.is_empty() {
        head_control_type.to_string()
    } else {
        format!("{}{{{}}}", head_control_type, head_name)
    };
    path_parts.push(head_part);

    // Reverse to get path from head_window to element
    path_parts.reverse();

    // Join with ->
    Ok(path_parts.join("->"))
}
