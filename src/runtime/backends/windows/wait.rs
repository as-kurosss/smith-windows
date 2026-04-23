//! Windows backend for wait operations using UI Automation

use tracing::{debug, error, info};

use crate::core::wait::{WaitBackend, WaitError, WaitSelector};

/// Windows wait backend implementation
pub struct WaitBackendWindows;

impl WaitBackendWindows {
    /// Creates a new Windows wait backend
    pub fn new() -> Self {
        Self
    }
}

impl Default for WaitBackendWindows {
    fn default() -> Self {
        Self::new()
    }
}

impl WaitBackendWindows {
    /// Finds an element using the given selector
    fn find_element_by_selector(
        &self,
        automation: &uiautomation::UIAutomation,
        root: uiautomation::UIElement,
        selector: &WaitSelector,
    ) -> Result<bool, WaitError> {
        // Create matcher based on selector type
        let matcher = match selector {
            WaitSelector::AutomationId(id) | WaitSelector::Name(id) => {
                debug!("Finding element by name: {}", id);
                // In UI Automation, automation_id is stored as name property
                automation.create_matcher().from(root).name(id)
            }
            WaitSelector::ControlType(ty) => {
                debug!("Finding element by ControlType: {}", ty);
                // Parse control type string to enum
                let control_type = match ty.to_lowercase().as_str() {
                    "window" => uiautomation::types::ControlType::Window,
                    "button" => uiautomation::types::ControlType::Button,
                    "text" => uiautomation::types::ControlType::Text,
                    "edit" => uiautomation::types::ControlType::Edit,
                    "list" => uiautomation::types::ControlType::List,
                    "listitem" => uiautomation::types::ControlType::ListItem,
                    "checkbox" => uiautomation::types::ControlType::CheckBox,
                    "combobox" => uiautomation::types::ControlType::ComboBox,
                    "group" => uiautomation::types::ControlType::Group,
                    "pane" => uiautomation::types::ControlType::Pane,
                    "header" => uiautomation::types::ControlType::Header,
                    "titlebar" => uiautomation::types::ControlType::TitleBar,
                    "document" => uiautomation::types::ControlType::Document,
                    "progressbar" => uiautomation::types::ControlType::ProgressBar,
                    "slider" => uiautomation::types::ControlType::Slider,
                    "spinner" => uiautomation::types::ControlType::Spinner,
                    "scrollbar" => uiautomation::types::ControlType::ScrollBar,
                    "tooltip" => uiautomation::types::ControlType::ToolTip,
                    "menu" => uiautomation::types::ControlType::Menu,
                    "menuitem" => uiautomation::types::ControlType::MenuItem,
                    "separator" => uiautomation::types::ControlType::Separator,
                    "splitbutton" => uiautomation::types::ControlType::SplitButton,
                    "tab" => uiautomation::types::ControlType::Tab,
                    "tabitem" => uiautomation::types::ControlType::TabItem,
                    "tree" => uiautomation::types::ControlType::Tree,
                    _ => {
                        return Err(WaitError::InvalidConfig(format!(
                            "unknown control type: {}",
                            ty
                        )));
                    }
                };
                automation
                    .create_matcher()
                    .from(root)
                    .control_type(control_type)
            }
        };

        // Find first element matching the criteria
        let result = matcher.find_first();

        match result {
            Ok(element) => {
                info!("Element found via selector: {:?}", selector);
                // UIElement in 0.24.4 doesn't have is_empty() - check by accessing a property
                // If element is null/invalid, get_name() will return empty or error
                let element_name = element.get_name().unwrap_or_default();
                if element_name.is_empty() {
                    debug!("Found element is null/empty via selector: {:?}", selector);
                    Ok(false)
                } else {
                    Ok(true)
                }
            }
            Err(e) => {
                error!("Failed to find element via selector {:?}: {}", selector, e);
                Err(WaitError::ComError(e.to_string()))
            }
        }
    }
}

#[async_trait::async_trait(?Send)]
impl WaitBackend for WaitBackendWindows {
    async fn wait_element(
        &self,
        automation: &uiautomation::UIAutomation,
        root: &uiautomation::UIElement,
        selector: &WaitSelector,
    ) -> Result<bool, WaitError> {
        // UIElement is !Send, so we need to clone it for the backend call
        // The backend call itself is synchronous and does not block the async runtime
        self.find_element_by_selector(automation, root.clone(), selector)
    }
}
