//! Windows backend for automation session using UI Automation

use tracing::{info, debug, error};

use crate::core::automation_session::{
    AutomationError, MatchMode, RuntimeSession, SessionBackend, SessionConfig, SessionLaunchConfig,
};

/// Windows automation session backend implementation
pub struct SessionBackendWindows;

impl SessionBackendWindows {
    /// Creates a new Windows automation session backend
    pub fn new() -> Self {
        Self
    }
}

impl Default for SessionBackendWindows {
    fn default() -> Self {
        Self::new()
    }
}

impl SessionBackendWindows {
    /// Validates an element found through UI Automation
    fn validate_element(element: &uiautomation::UIElement) -> Result<(), AutomationError> {
        // Check if element is enabled
        let enabled_result = element.is_enabled();
        let is_enabled = match enabled_result {
            Ok(val) => val,
            Err(e) => {
                error!("Failed to check if element is enabled: {}", e);
                return Err(AutomationError::ComError(e.to_string()));
            }
        };

        if !is_enabled {
            error!("Window found but is disabled");
            return Err(AutomationError::WindowDisabled);
        }

        // Check if element is offscreen
        let offscreen_result = element.is_offscreen();
        let is_offscreen = match offscreen_result {
            Ok(val) => val,
            Err(e) => {
                error!("Failed to check if element is offscreen: {}", e);
                return Err(AutomationError::ComError(e.to_string()));
            }
        };

        if is_offscreen {
            error!("Window found but is offscreen");
            return Err(AutomationError::WindowOffscreen);
        }

        Ok(())
    }

    /// Gets the process ID from a UI element
    fn get_process_id(element: &uiautomation::UIElement) -> Result<u32, AutomationError> {
        element
            .get_process_id()
            .map_err(|e| {
                error!("Failed to get process ID: {}", e);
                AutomationError::ComError(e.to_string())
            })
    }

    /// Finds a window by title using UIAutomation
    fn find_window_by_title(
        &self,
        title: &str,
        mode: MatchMode,
        only_visible: bool,
    ) -> Result<RuntimeSession, AutomationError> {
        // Initialize UIAutomation
        let automation = uiautomation::UIAutomation::new().map_err(|e| {
            error!("Failed to create UIAutomation: {}", e);
            AutomationError::ComError(e.to_string())
        })?;

        // Get root element
        let root = automation.get_root_element().map_err(|e| {
            error!("Failed to get root element: {}", e);
            AutomationError::ComError(e.to_string())
        })?;

        // Use create_matcher() pattern - the correct API for uiautomation 0.24.4
        let matcher = automation.create_matcher()
            .from(root)
            .control_type(uiautomation::types::ControlType::Window);

        // Find all windows
        let elements = matcher.find_all().map_err(|e| {
            error!("Failed to find elements: {}", e);
            AutomationError::ComError(e.to_string())
        })?;

        // Filter elements based on title and mode
        for element in elements {
            // Get element name - handle errors properly
            let element_name = match element.get_name() {
                Ok(name) => name,
                Err(e) => {
                    error!("Failed to get element name: {}", e);
                    continue;
                }
            };

            // Check if element is offscreen
            if only_visible {
                let is_offscreen = element.is_offscreen().unwrap_or(true);
                if is_offscreen {
                    continue;
                }
            }

            // Match based on mode
            let matches = match mode {
                MatchMode::Exact => element_name == title,
                MatchMode::Contains => element_name.contains(title),
                MatchMode::Regex => {
                    match regex::Regex::new(title) {
                        Ok(re) => re.is_match(&element_name),
                        Err(_) => return Err(AutomationError::InvalidConfig("invalid regex pattern".to_string())),
                    }
                }
            };

            if matches {
                // Validate element
                Self::validate_element(&element)?;

                // Get process ID
                let process_id = Self::get_process_id(&element)?;

                info!(
                    "Attached to window: {} (PID: {})",
                    element_name, process_id
                );

                return Ok(RuntimeSession::new(process_id, element));
            }
        }

        error!("Window not found: title='{}', mode={:?}", title, mode);
        Err(AutomationError::WindowNotFound)
    }

    /// Finds a window by process ID
    fn find_window_by_process_id(
        &self,
        process_id: u32,
    ) -> Result<RuntimeSession, AutomationError> {
        // Initialize UIAutomation
        let automation = uiautomation::UIAutomation::new().map_err(|e| {
            error!("Failed to create UIAutomation: {}", e);
            AutomationError::ComError(e.to_string())
        })?;

        // Get root element
        let root = automation.get_root_element().map_err(|e| {
            error!("Failed to get root element: {}", e);
            AutomationError::ComError(e.to_string())
        })?;

        // Use create_matcher() pattern - the correct API for uiautomation 0.24.4
        let matcher = automation.create_matcher()
            .from(root)
            .control_type(uiautomation::types::ControlType::Window);

        // Find all windows
        let elements = matcher.find_all().map_err(|e| {
            error!("Failed to find elements: {}", e);
            AutomationError::ComError(e.to_string())
        })?;

        // Filter by process ID
        for element in elements {
            let element_process_id = match Self::get_process_id(&element) {
                Ok(pid) => pid,
                Err(_) => continue,
            };

            if element_process_id == process_id {
                // Validate element
                Self::validate_element(&element)?;

                info!(
                    "Attached to window for process_id {}: {}",
                    process_id,
                    element.get_name().unwrap_or_default()
                );

                return Ok(RuntimeSession::new(process_id, element));
            }
        }

        error!("No window found for process_id: {}", process_id);
        Err(AutomationError::WindowNotFound)
    }
}

#[async_trait::async_trait]
impl SessionBackend for SessionBackendWindows {
    async fn launch_process(&self, _config: &SessionLaunchConfig) -> Result<u32, AutomationError> {
        unimplemented!("launch_process is handled directly in core module")
    }

    async fn attach_by_title(
        &self,
        title: String,
        mode: MatchMode,
        only_visible: bool,
        config: &SessionConfig,
    ) -> Result<RuntimeSession, AutomationError> {
        // Check cancellation before starting
        if config.cancellation.is_cancelled() {
            error!("Attach by title operation cancelled");
            return Err(AutomationError::Cancelled);
        }

        debug!(
            "attach_by_title: title='{}', mode={:?}, only_visible={}",
            title, mode, only_visible
        );

        // Use timeout for the operation
        let timeout = config.timeout;
        let title_clone = title.clone();
        
        let result = tokio::time::timeout(timeout, async move {
            let backend = SessionBackendWindows::new();
            backend.find_window_by_title(&title_clone, mode, only_visible)
        }).await;

        match result {
            Ok(Ok(session)) => Ok(session),
            Ok(Err(e)) => {
                // Check for cancellation after timeout
                if config.cancellation.is_cancelled() {
                    error!("Attach by title operation cancelled during completion");
                    Err(AutomationError::Cancelled)
                } else {
                    Err(e)
                }
            }
            Err(_) => {
                error!("Attach by title operation timed out after {:?}", timeout);
                Err(AutomationError::WindowNotFound)
            }
        }
    }

    async fn attach_by_process_id(
        &self,
        process_id: u32,
        config: &SessionConfig,
    ) -> Result<RuntimeSession, AutomationError> {
        // Check cancellation before starting
        if config.cancellation.is_cancelled() {
            error!("Attach by process_id operation cancelled");
            return Err(AutomationError::Cancelled);
        }

        debug!("attach_by_process_id: process_id={}", process_id);

        // Use timeout for the operation
        let timeout = config.timeout;
        
        let result = tokio::time::timeout(timeout, async move {
            let backend = SessionBackendWindows::new();
            backend.find_window_by_process_id(process_id)
        }).await;

        match result {
            Ok(Ok(session)) => Ok(session),
            Ok(Err(e)) => {
                // Check for cancellation after timeout
                if config.cancellation.is_cancelled() {
                    error!("Attach by process_id operation cancelled during completion");
                    Err(AutomationError::Cancelled)
                } else {
                    Err(e)
                }
            }
            Err(_) => {
                error!("Attach by process_id operation timed out after {:?}", timeout);
                Err(AutomationError::WindowNotFound)
            }
        }
    }
}
