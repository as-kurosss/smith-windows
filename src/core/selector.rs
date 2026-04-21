//! Selector module for UI element identification
//!
//! Provides types and utilities for building and working with UI selectors
//! based on element properties like automation_id, classname, control_type, and name.

use uiautomation::types::ControlType;

/// A single step in the selector tree
#[derive(Debug, Clone)]
pub struct SelectorStep {
    /// Class name (e.g., "Button", "Edit", "Notepad")
    pub classname: Option<String>,
    /// Control type
    pub control_type: Option<ControlType>,
    /// Name/title of the element
    pub name: Option<String>,
    /// Automation ID (if available)
    pub automation_id: Option<String>,
}

impl SelectorStep {
    /// Creates a step from a UI element
    pub fn from_element(element: &uiautomation::UIElement) -> Result<Self, SelectorError> {
        let classname = element.get_classname().ok();
        let control_type = element.get_control_type().ok();
        let name = element.get_name().ok();
        let automation_id = element.get_automation_id().ok();

        Ok(Self {
            classname,
            control_type,
            name,
            automation_id,
        })
    }

    /// Converts the step into a Selector with priority:
    /// 1. automation_id (if not empty)
    /// 2. classname (if not empty)
    /// 3. control_type
    /// 4. name (if not empty)
    pub fn to_selector(&self) -> Option<Selector> {
        // 1. Automation ID - most reliable if not empty
        if let Some(automation_id) = &self.automation_id {
            if !automation_id.is_empty() {
                return Some(Selector::AutomationId(automation_id.clone()));
            }
        }

        // 2. Classname - reliable, language-independent
        if let Some(classname) = &self.classname {
            if !classname.is_empty() {
                return Some(Selector::Classname(classname.clone()));
            }
        }

        // 3. Control Type - good fallback
        if let Some(control_type) = self.control_type {
            return Some(Selector::ControlType(control_type));
        }

        // 4. Name - only if not empty
        if let Some(name) = &self.name {
            if !name.is_empty() {
                return Some(Selector::Name(name.clone()));
            }
        }

        // Nothing matched
        None
    }

    /// Prints the step in a readable format
    pub fn print(&self) {
        let classname = self.classname.as_deref().unwrap_or("?");
        let control_type = self
            .control_type
            .map(|ct| format!("{:?}", ct))
            .unwrap_or_else(|| "?".to_string());
        let name = self.name.as_deref().unwrap_or("");
        let automation_id = self.automation_id.as_deref().unwrap_or("");

        print!(" classname={}, type={}", classname, control_type);
        if !name.is_empty() {
            print!(", name=\"{}\"", name);
        }
        if !automation_id.is_empty() {
            print!(", automation_id=\"{}\"", automation_id);
        }
        println!();
    }
}

/// A selector that can identify a UI element
#[derive(Debug, Clone, PartialEq)]
pub enum Selector {
    /// Match by automation ID (most reliable)
    AutomationId(String),
    /// Match by class name
    Classname(String),
    /// Match by control type
    ControlType(ControlType),
    /// Match by name/title
    Name(String),
}

impl std::fmt::Display for Selector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Selector::AutomationId(id) => write!(f, "AutomationId={}", id),
            Selector::Classname(class) => write!(f, "Classname={}", class),
            Selector::ControlType(ct) => write!(f, "ControlType={:?}", ct),
            Selector::Name(name) => write!(f, "Name={}", name),
        }
    }
}

/// A complete recorded selector with full hierarchy
#[derive(Debug, Clone)]
pub struct RecordedSelector {
    /// Tree of steps from root to target element
    pub steps: Vec<SelectorStep>,
    /// Depth of the element (number of steps from root)
    pub depth: usize,
}

impl RecordedSelector {
    /// Creates a final Selector from the last step
    pub fn to_selector(&self) -> Option<Selector> {
        self.steps.last().and_then(|step| step.to_selector())
    }

    /// Prints a human-readable representation of the selector
    pub fn print_tree(&self) {
        println!("📋 Recorded selector ({} steps):", self.steps.len());
        for (i, step) in self.steps.iter().enumerate() {
            let indent = "  ".repeat(i);
            print!("{}[{}] ", indent, i);
            step.print();
        }
    }

    /// Returns the full path as a string
    pub fn path_string(&self) -> String {
        let parts: Vec<String> = self
            .steps
            .iter()
            .map(|step| {
                let name = step.name.as_deref().unwrap_or("");
                let control_type = step
                    .control_type
                    .map(|ct| format!("{:?}", ct))
                    .unwrap_or_default();
                if !name.is_empty() {
                    format!("{}{{{}}}", control_type, name)
                } else {
                    control_type
                }
            })
            .collect();
        parts.join("->")
    }
}

/// Errors for selector operations
#[derive(thiserror::Error, Debug)]
pub enum SelectorError {
    #[error("failed to get element property: {0}")]
    ElementPropertyError(String),
    #[error("selector tree exceeds maximum depth")]
    MaxDepthExceeded,
    #[error("invalid selector: {0}")]
    InvalidSelector(String),
}

impl From<uiautomation::Error> for SelectorError {
    fn from(err: uiautomation::Error) -> Self {
        SelectorError::ElementPropertyError(err.to_string())
    }
}
