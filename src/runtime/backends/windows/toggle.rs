//! Windows backend for toggle operations using UI Automation

use tracing::{error, info};

use crate::core::toggle::ToggleError;

/// Windows toggle backend implementation
pub struct ToggleBackendWindows;

impl ToggleBackendWindows {
    /// Creates a new Windows toggle backend
    pub fn new() -> Self {
        Self
    }
}

impl Default for ToggleBackendWindows {
    fn default() -> Self {
        Self::new()
    }
}

impl ToggleBackendWindows {
    /// Performs a toggle element operation on the given element
    pub async fn toggle_element(
        &self,
        element: &uiautomation::UIElement,
    ) -> Result<(), ToggleError> {
        // Check element validity first - access a property to validate
        let _control_type = match element.get_control_type() {
            Ok(val) => val,
            Err(e) => {
                error!("Failed to get element control type: {}", e);
                return Err(ToggleError::ElementNotFound);
            }
        };

        // Check if element is enabled
        let enabled_result = element.is_enabled();
        let is_enabled = match enabled_result {
            Ok(val) => val,
            Err(e) => {
                error!("Failed to check if element is enabled: {}", e);
                return Err(ToggleError::ComError(e.to_string()));
            }
        };

        if !is_enabled {
            error!("Toggle failed: element is disabled");
            return Err(ToggleError::ElementNotEnabled);
        }

        // Check if element is offscreen
        let offscreen_result = element.is_offscreen();
        let is_offscreen = match offscreen_result {
            Ok(val) => val,
            Err(e) => {
                error!("Failed to check if element is offscreen: {}", e);
                return Err(ToggleError::ComError(e.to_string()));
            }
        };

        if is_offscreen {
            error!("Toggle failed: element is offscreen");
            return Err(ToggleError::ElementOffscreen);
        }

        // Try TogglePattern first
        if let Ok(toggle_pattern) = element.get_pattern::<uiautomation::patterns::UITogglePattern>()
        {
            let result = toggle_pattern.toggle();
            match result {
                Ok(()) => {
                    info!("Toggle operation completed successfully via TogglePattern");
                    Ok(())
                }
                Err(e) => {
                    error!("Toggle operation failed via TogglePattern: {}", e);
                    Err(ToggleError::ComError(e.to_string()))
                }
            }
        } else if let Ok(value_pattern) =
            element.get_pattern::<uiautomation::patterns::UIValuePattern>()
        {
            // Try ValuePattern for toggling
            let current_value = value_pattern.get_value();
            match current_value {
                Ok(val) => {
                    // For value pattern, we set the opposite value to toggle
                    let new_value =
                        if val == "true" || val == "1" || val.to_lowercase() == "checked" {
                            "false"
                        } else {
                            "true"
                        };
                    let result = value_pattern.set_value(new_value);
                    match result {
                        Ok(()) => {
                            info!("Toggle operation completed successfully via ValuePattern");
                            Ok(())
                        }
                        Err(e) => {
                            error!("Toggle operation failed via ValuePattern: {}", e);
                            Err(ToggleError::ComError(e.to_string()))
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to get current value from ValuePattern: {}", e);
                    Err(ToggleError::ComError(e.to_string()))
                }
            }
        } else {
            error!("Toggle failed: element does not support TogglePattern or ValuePattern");
            Err(ToggleError::ElementNotSupported)
        }
    }

    /// Sets radio button selected state
    pub async fn set_radio(
        &self,
        element: &uiautomation::UIElement,
        selected: bool,
    ) -> Result<(), ToggleError> {
        // Check element validity first
        let _control_type = match element.get_control_type() {
            Ok(val) => val,
            Err(e) => {
                error!("Failed to get element control type: {}", e);
                return Err(ToggleError::ElementNotFound);
            }
        };

        // Check if element is enabled
        let enabled_result = element.is_enabled();
        let is_enabled = match enabled_result {
            Ok(val) => val,
            Err(e) => {
                error!("Failed to check if element is enabled: {}", e);
                return Err(ToggleError::ComError(e.to_string()));
            }
        };

        if !is_enabled {
            error!("Set radio failed: element is disabled");
            return Err(ToggleError::ElementNotEnabled);
        }

        // Check if element is offscreen
        let offscreen_result = element.is_offscreen();
        let is_offscreen = match offscreen_result {
            Ok(val) => val,
            Err(e) => {
                error!("Failed to check if element is offscreen: {}", e);
                return Err(ToggleError::ComError(e.to_string()));
            }
        };

        if is_offscreen {
            error!("Set radio failed: element is offscreen");
            return Err(ToggleError::ElementOffscreen);
        }

        // Try SelectionPattern first (for radio buttons)
        if let Ok(_selection_pattern) =
            element.get_pattern::<uiautomation::patterns::UISelectionPattern>()
        {
            // SelectionPattern doesn't have a direct select method for single selection
            // For radio buttons, we use ValuePattern to set selection
            error!("Set radio failed: SelectionPattern does not have select() method for single selection, use ValuePattern instead");
            Err(ToggleError::ElementNotWritable)
        } else if let Ok(value_pattern) =
            element.get_pattern::<uiautomation::patterns::UIValuePattern>()
        {
            // Fallback to ValuePattern
            let current_value = value_pattern.get_value();
            match current_value {
                Ok(_val) => {
                    let new_value = if selected { "true" } else { "false" };
                    let result = value_pattern.set_value(new_value);
                    match result {
                        Ok(()) => {
                            info!("Set radio operation completed successfully via ValuePattern");
                            Ok(())
                        }
                        Err(e) => {
                            error!("Set radio operation failed via ValuePattern: {}", e);
                            Err(ToggleError::ComError(e.to_string()))
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to get current value from ValuePattern: {}", e);
                    Err(ToggleError::ComError(e.to_string()))
                }
            }
        } else {
            error!("Set radio failed: element does not support SelectionPattern or ValuePattern");
            Err(ToggleError::ElementNotSupported)
        }
    }

    /// Sets toggle switch state
    pub async fn set_toggle(
        &self,
        element: &uiautomation::UIElement,
        state: bool,
    ) -> Result<(), ToggleError> {
        // Check element validity first
        let _control_type = match element.get_control_type() {
            Ok(val) => val,
            Err(e) => {
                error!("Failed to get element control type: {}", e);
                return Err(ToggleError::ElementNotFound);
            }
        };

        // Check if element is enabled
        let enabled_result = element.is_enabled();
        let is_enabled = match enabled_result {
            Ok(val) => val,
            Err(e) => {
                error!("Failed to check if element is enabled: {}", e);
                return Err(ToggleError::ComError(e.to_string()));
            }
        };

        if !is_enabled {
            error!("Set toggle failed: element is disabled");
            return Err(ToggleError::ElementNotEnabled);
        }

        // Check if element is offscreen
        let offscreen_result = element.is_offscreen();
        let is_offscreen = match offscreen_result {
            Ok(val) => val,
            Err(e) => {
                error!("Failed to check if element is offscreen: {}", e);
                return Err(ToggleError::ComError(e.to_string()));
            }
        };

        if is_offscreen {
            error!("Set toggle failed: element is offscreen");
            return Err(ToggleError::ElementOffscreen);
        }

        // Try TogglePattern first
        if let Ok(toggle_pattern) = element.get_pattern::<uiautomation::patterns::UITogglePattern>()
        {
            // Get current state to determine what to set
            let current_state = toggle_pattern.get_toggle_state();
            match current_state {
                Ok(_) => {
                    // For set_toggle, we directly set the state via ValuePattern if available
                    // TogglePattern doesn't have a direct set method, so we use ValuePattern
                    if let Ok(value_pattern) =
                        element.get_pattern::<uiautomation::patterns::UIValuePattern>()
                    {
                        let new_value = if state { "true" } else { "false" };
                        let result = value_pattern.set_value(new_value);
                        match result {
                            Ok(()) => {
                                info!(
                                    "Set toggle operation completed successfully via ValuePattern"
                                );
                                Ok(())
                            }
                            Err(e) => {
                                error!("Set toggle operation failed via ValuePattern: {}", e);
                                Err(ToggleError::ComError(e.to_string()))
                            }
                        }
                    } else {
                        // Fallback to TogglePattern toggle method (inverts state)
                        let result = toggle_pattern.toggle();
                        match result {
                            Ok(()) => {
                                info!("Set toggle operation completed via TogglePattern");
                                Ok(())
                            }
                            Err(e) => {
                                error!("Set toggle operation failed via TogglePattern: {}", e);
                                Err(ToggleError::ComError(e.to_string()))
                            }
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to get current toggle state: {}", e);
                    Err(ToggleError::ComError(e.to_string()))
                }
            }
        } else if let Ok(value_pattern) =
            element.get_pattern::<uiautomation::patterns::UIValuePattern>()
        {
            // Fallback to ValuePattern
            let new_value = if state { "true" } else { "false" };
            let result = value_pattern.set_value(new_value);
            match result {
                Ok(()) => {
                    info!("Set toggle operation completed successfully via ValuePattern");
                    Ok(())
                }
                Err(e) => {
                    error!("Set toggle operation failed via ValuePattern: {}", e);
                    Err(ToggleError::ComError(e.to_string()))
                }
            }
        } else {
            error!("Set toggle failed: element does not support TogglePattern or ValuePattern");
            Err(ToggleError::ElementNotSupported)
        }
    }

    /// Checks if element is checked (checkbox)
    pub async fn is_checked(&self, element: &uiautomation::UIElement) -> Result<bool, ToggleError> {
        // Check element validity first
        let _control_type = match element.get_control_type() {
            Ok(val) => val,
            Err(e) => {
                error!("Failed to get element control type: {}", e);
                return Err(ToggleError::ElementNotFound);
            }
        };

        // Check if element is enabled
        let enabled_result = element.is_enabled();
        let is_enabled = match enabled_result {
            Ok(val) => val,
            Err(e) => {
                error!("Failed to check if element is enabled: {}", e);
                return Err(ToggleError::ComError(e.to_string()));
            }
        };

        if !is_enabled {
            error!("Is checked failed: element is disabled");
            return Err(ToggleError::ElementNotEnabled);
        }

        // Check if element is offscreen
        let offscreen_result = element.is_offscreen();
        let is_offscreen = match offscreen_result {
            Ok(val) => val,
            Err(e) => {
                error!("Failed to check if element is offscreen: {}", e);
                return Err(ToggleError::ComError(e.to_string()));
            }
        };

        if is_offscreen {
            error!("Is checked failed: element is offscreen");
            return Err(ToggleError::ElementOffscreen);
        }

        // Try TogglePattern first
        if let Ok(toggle_pattern) = element.get_pattern::<uiautomation::patterns::UITogglePattern>()
        {
            let current_state = toggle_pattern.get_toggle_state();
            match current_state {
                Ok(state) => {
                    // Check the state - ToggleState enum from uiautomation::types
                    let is_checked = match state {
                        uiautomation::types::ToggleState::On => true,
                        uiautomation::types::ToggleState::Off => false,
                        uiautomation::types::ToggleState::Indeterminate => false,
                    };
                    info!("Is checked operation completed: {}", is_checked);
                    Ok(is_checked)
                }
                Err(e) => {
                    error!("Failed to get toggle state: {}", e);
                    Err(ToggleError::ComError(e.to_string()))
                }
            }
        } else if let Ok(value_pattern) =
            element.get_pattern::<uiautomation::patterns::UIValuePattern>()
        {
            // Fallback to ValuePattern
            let current_value = value_pattern.get_value();
            match current_value {
                Ok(val) => {
                    let is_checked = val == "true" || val == "1" || val.to_lowercase() == "checked";
                    info!(
                        "Is checked operation completed via ValuePattern: {}",
                        is_checked
                    );
                    Ok(is_checked)
                }
                Err(e) => {
                    error!("Failed to get current value from ValuePattern: {}", e);
                    Err(ToggleError::ComError(e.to_string()))
                }
            }
        } else if let Ok(selection_pattern) =
            element.get_pattern::<uiautomation::patterns::UISelectionPattern>()
        {
            // Fallback to SelectionPattern for radio buttons
            let is_selected = selection_pattern.is_selection_required();
            match is_selected {
                Ok(selected) => {
                    info!(
                        "Is checked operation completed via SelectionPattern: {}",
                        selected
                    );
                    Ok(selected)
                }
                Err(e) => {
                    error!("Failed to get selection state: {}", e);
                    Err(ToggleError::ComError(e.to_string()))
                }
            }
        } else {
            error!("Is checked failed: element does not support TogglePattern, ValuePattern, or SelectionPattern");
            Err(ToggleError::ElementNotSupported)
        }
    }

    /// Checks if element is selected (radio button)
    pub async fn is_selected(
        &self,
        element: &uiautomation::UIElement,
    ) -> Result<bool, ToggleError> {
        // Check element validity first
        let _control_type = match element.get_control_type() {
            Ok(val) => val,
            Err(e) => {
                error!("Failed to get element control type: {}", e);
                return Err(ToggleError::ElementNotFound);
            }
        };

        // Check if element is enabled
        let enabled_result = element.is_enabled();
        let is_enabled = match enabled_result {
            Ok(val) => val,
            Err(e) => {
                error!("Failed to check if element is enabled: {}", e);
                return Err(ToggleError::ComError(e.to_string()));
            }
        };

        if !is_enabled {
            error!("Is selected failed: element is disabled");
            return Err(ToggleError::ElementNotEnabled);
        }

        // Check if element is offscreen
        let offscreen_result = element.is_offscreen();
        let is_offscreen = match offscreen_result {
            Ok(val) => val,
            Err(e) => {
                error!("Failed to check if element is offscreen: {}", e);
                return Err(ToggleError::ComError(e.to_string()));
            }
        };

        if is_offscreen {
            error!("Is selected failed: element is offscreen");
            return Err(ToggleError::ElementOffscreen);
        }

        // Try SelectionPattern first (for radio buttons)
        if let Ok(selection_pattern) =
            element.get_pattern::<uiautomation::patterns::UISelectionPattern>()
        {
            let is_selected = selection_pattern.is_selection_required();
            match is_selected {
                Ok(selected) => {
                    info!("Is selected operation completed: {}", selected);
                    Ok(selected)
                }
                Err(e) => {
                    error!("Failed to get selection state: {}", e);
                    Err(ToggleError::ComError(e.to_string()))
                }
            }
        } else if let Ok(toggle_pattern) =
            element.get_pattern::<uiautomation::patterns::UITogglePattern>()
        {
            // Fallback to TogglePattern
            let current_state = toggle_pattern.get_toggle_state();
            match current_state {
                Ok(state) => {
                    let is_selected = match state {
                        uiautomation::types::ToggleState::On => true,
                        uiautomation::types::ToggleState::Off => false,
                        uiautomation::types::ToggleState::Indeterminate => false,
                    };
                    info!(
                        "Is selected operation completed via TogglePattern: {}",
                        is_selected
                    );
                    Ok(is_selected)
                }
                Err(e) => {
                    error!("Failed to get toggle state: {}", e);
                    Err(ToggleError::ComError(e.to_string()))
                }
            }
        } else if let Ok(value_pattern) =
            element.get_pattern::<uiautomation::patterns::UIValuePattern>()
        {
            // Fallback to ValuePattern
            let current_value = value_pattern.get_value();
            match current_value {
                Ok(val) => {
                    let is_selected = val == "true" || val == "1";
                    info!(
                        "Is selected operation completed via ValuePattern: {}",
                        is_selected
                    );
                    Ok(is_selected)
                }
                Err(e) => {
                    error!("Failed to get current value from ValuePattern: {}", e);
                    Err(ToggleError::ComError(e.to_string()))
                }
            }
        } else {
            error!("Is selected failed: element does not support SelectionPattern, TogglePattern, or ValuePattern");
            Err(ToggleError::ElementNotSupported)
        }
    }
}
