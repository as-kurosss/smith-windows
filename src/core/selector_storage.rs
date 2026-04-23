//! SelectorStorage module for saving/loading recorded selectors to/from disk
//!
//! Provides persistent storage for UI selectors with JSON serialization,
//! path traversal protection, and validation.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::{debug, error, info};

use crate::core::selector::{RecordedSelector, SelectorStep};
use uiautomation::types::ControlType;

/// Maximum depth for selector tree (protection against infinite loops)
const MAX_DEPTH: usize = 256;

/// Maximum storage size in bytes (100MB)
const MAX_STORAGE_SIZE: u64 = 100 * 1024 * 1024;

/// Storage configuration
#[derive(Debug, Clone)]
pub struct SelectorStorageConfig {
    /// Directory for storing selectors
    pub storage_dir: PathBuf,
    /// Maximum storage size in bytes
    pub max_storage_size: u64,
    /// Maximum number of selectors
    pub max_selectors: usize,
}

impl Default for SelectorStorageConfig {
    fn default() -> Self {
        Self {
            storage_dir: std::env::temp_dir().join("smith-windows-selectors"),
            max_storage_size: MAX_STORAGE_SIZE,
            max_selectors: 1000,
        }
    }
}

impl SelectorStorageConfig {
    /// Creates a new config with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new config with custom storage directory
    pub fn with_storage_dir<P: AsRef<Path>>(storage_dir: P) -> Self {
        Self {
            storage_dir: storage_dir.as_ref().to_path_buf(),
            ..Default::default()
        }
    }

    /// Creates a new config with custom storage directory and limits
    pub fn with_limits(storage_dir: PathBuf, max_storage_size: u64, max_selectors: usize) -> Self {
        Self {
            storage_dir,
            max_storage_size,
            max_selectors,
        }
    }
}

/// Serializable representation of a single selector step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableSelectorStep {
    /// Class name (e.g., "Button", "Edit", "Notepad")
    pub classname: Option<String>,
    /// Control type as string (e.g., "Button", "Edit")
    pub control_type: Option<String>,
    /// Name/title of the element
    pub name: Option<String>,
    /// Automation ID (if available)
    pub automation_id: Option<String>,
}

impl From<&SelectorStep> for SerializableSelectorStep {
    fn from(step: &SelectorStep) -> Self {
        Self {
            classname: step.classname.clone(),
            control_type: step.control_type.as_ref().map(control_type_to_string),
            name: step.name.clone(),
            automation_id: step.automation_id.clone(),
        }
    }
}

impl From<&SerializableSelectorStep> for SelectorStep {
    fn from(step: &SerializableSelectorStep) -> Self {
        Self {
            classname: step.classname.clone(),
            control_type: step
                .control_type
                .as_ref()
                .and_then(|s| control_type_from_string(s).ok()),
            name: step.name.clone(),
            automation_id: step.automation_id.clone(),
        }
    }
}

/// Converts ControlType enum to string
pub fn control_type_to_string(control_type: &ControlType) -> String {
    match control_type {
        ControlType::Button => "Button".to_string(),
        ControlType::Calendar => "Calendar".to_string(),
        ControlType::CheckBox => "CheckBox".to_string(),
        ControlType::ComboBox => "ComboBox".to_string(),
        ControlType::Custom => "Custom".to_string(),
        ControlType::DataGrid => "DataGrid".to_string(),
        ControlType::DataItem => "DataItem".to_string(),
        ControlType::Document => "Document".to_string(),
        ControlType::Edit => "Edit".to_string(),
        ControlType::Group => "Group".to_string(),
        ControlType::Header => "Header".to_string(),
        ControlType::HeaderItem => "HeaderItem".to_string(),
        ControlType::Hyperlink => "Hyperlink".to_string(),
        ControlType::Image => "Image".to_string(),
        ControlType::ListItem => "ListItem".to_string(),
        ControlType::List => "List".to_string(),
        ControlType::Menu => "Menu".to_string(),
        ControlType::MenuBar => "MenuBar".to_string(),
        ControlType::MenuItem => "MenuItem".to_string(),
        ControlType::Pane => "Pane".to_string(),
        ControlType::ProgressBar => "ProgressBar".to_string(),
        ControlType::RadioButton => "RadioButton".to_string(),
        ControlType::ScrollBar => "ScrollBar".to_string(),
        ControlType::Separator => "Separator".to_string(),
        ControlType::Slider => "Slider".to_string(),
        ControlType::Spinner => "Spinner".to_string(),
        ControlType::SplitButton => "SplitButton".to_string(),
        ControlType::StatusBar => "StatusBar".to_string(),
        ControlType::Tab => "Tab".to_string(),
        ControlType::TabItem => "TabItem".to_string(),
        ControlType::Table => "Table".to_string(),
        ControlType::Text => "Text".to_string(),
        ControlType::Thumb => "Thumb".to_string(),
        ControlType::TitleBar => "TitleBar".to_string(),
        ControlType::ToolBar => "ToolBar".to_string(),
        ControlType::ToolTip => "ToolTip".to_string(),
        ControlType::TreeItem => "TreeItem".to_string(),
        ControlType::Tree => "Tree".to_string(),
        ControlType::Window => "Window".to_string(),
        ControlType::SemanticZoom => "SemanticZoom".to_string(),
        ControlType::AppBar => "AppBar".to_string(),
    }
}

/// Parses ControlType from string
pub fn control_type_from_string(s: &str) -> Result<ControlType, StorageError> {
    match s {
        "Button" => Ok(ControlType::Button),
        "Calendar" => Ok(ControlType::Calendar),
        "CheckBox" => Ok(ControlType::CheckBox),
        "ComboBox" => Ok(ControlType::ComboBox),
        "Custom" => Ok(ControlType::Custom),
        "DataGrid" => Ok(ControlType::DataGrid),
        "DataItem" => Ok(ControlType::DataItem),
        "Document" => Ok(ControlType::Document),
        "Edit" => Ok(ControlType::Edit),
        "Group" => Ok(ControlType::Group),
        "Header" => Ok(ControlType::Header),
        "HeaderItem" => Ok(ControlType::HeaderItem),
        "Hyperlink" => Ok(ControlType::Hyperlink),
        "Image" => Ok(ControlType::Image),
        "ListItem" => Ok(ControlType::ListItem),
        "List" => Ok(ControlType::List),
        "Menu" => Ok(ControlType::Menu),
        "MenuBar" => Ok(ControlType::MenuBar),
        "MenuItem" => Ok(ControlType::MenuItem),
        "Pane" => Ok(ControlType::Pane),
        "ProgressBar" => Ok(ControlType::ProgressBar),
        "RadioButton" => Ok(ControlType::RadioButton),
        "ScrollBar" => Ok(ControlType::ScrollBar),
        "Separator" => Ok(ControlType::Separator),
        "Slider" => Ok(ControlType::Slider),
        "Spinner" => Ok(ControlType::Spinner),
        "SplitButton" => Ok(ControlType::SplitButton),
        "StatusBar" => Ok(ControlType::StatusBar),
        "Tab" => Ok(ControlType::Tab),
        "TabItem" => Ok(ControlType::TabItem),
        "Table" => Ok(ControlType::Table),
        "Text" => Ok(ControlType::Text),
        "Thumb" => Ok(ControlType::Thumb),
        "TitleBar" => Ok(ControlType::TitleBar),
        "ToolBar" => Ok(ControlType::ToolBar),
        "ToolTip" => Ok(ControlType::ToolTip),
        "TreeItem" => Ok(ControlType::TreeItem),
        "Tree" => Ok(ControlType::Tree),
        "Window" => Ok(ControlType::Window),
        "SemanticZoom" => Ok(ControlType::SemanticZoom),
        "AppBar" => Ok(ControlType::AppBar),
        _ => Err(StorageError::InvalidControlType(s.to_string())),
    }
}

/// Serializable representation of a recorded selector
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableRecordedSelector {
    /// Steps of the selector
    pub steps: Vec<SerializableSelectorStep>,
    /// Depth of the selector
    pub depth: usize,
}

impl From<&RecordedSelector> for SerializableRecordedSelector {
    fn from(recorded: &RecordedSelector) -> Self {
        Self {
            steps: recorded
                .steps
                .iter()
                .map(SerializableSelectorStep::from)
                .collect(),
            depth: recorded.depth,
        }
    }
}

impl From<&SerializableRecordedSelector> for RecordedSelector {
    fn from(recorded: &SerializableRecordedSelector) -> Self {
        Self {
            steps: recorded.steps.iter().map(SelectorStep::from).collect(),
            depth: recorded.depth,
        }
    }
}

/// Error types for selector storage operations
#[derive(Error, Debug)]
pub enum StorageError {
    #[error("invalid selector ID: {0}")]
    InvalidSelectorId(String),
    #[error("selector not found: {0}")]
    SelectorNotFound(String),
    #[error("selector already exists: {0}")]
    SelectorAlreadyExists(String),
    #[error("invalid config: {0}")]
    InvalidConfig(String),
    #[error("IO error: {0}")]
    IoError(String),
    #[error("serialization error: {0}")]
    SerializationError(String),
    #[error("path traversal detected")]
    PathTraversalDetected,
    #[error("storage size limit exceeded")]
    StorageSizeLimitExceeded,
    #[error("too many selectors")]
    TooManySelectors,
    #[error("invalid control type: {0}")]
    InvalidControlType(String),
    #[error("invalid selector data: {0}")]
    InvalidSelectorData(String),
}

impl From<std::io::Error> for StorageError {
    fn from(err: std::io::Error) -> Self {
        StorageError::IoError(err.to_string())
    }
}

impl From<serde_json::Error> for StorageError {
    fn from(err: serde_json::Error) -> Self {
        StorageError::SerializationError(err.to_string())
    }
}

/// Selector storage for persisting recorded selectors
#[derive(Debug, Clone)]
pub struct SelectorStorage {
    /// Storage configuration
    config: SelectorStorageConfig,
}

impl SelectorStorage {
    /// Creates a new selector storage with default config
    pub fn new() -> Self {
        Self {
            config: SelectorStorageConfig::new(),
        }
    }

    /// Creates a new selector storage with custom config
    pub fn with_config(config: SelectorStorageConfig) -> Self {
        Self { config }
    }

    /// Validates selector ID
    fn validate_id(id: &str) -> Result<(), StorageError> {
        if id.is_empty() {
            return Err(StorageError::InvalidSelectorId(
                "ID cannot be empty".to_string(),
            ));
        }

        if id.len() > 256 {
            return Err(StorageError::InvalidSelectorId(
                "ID too long (max 256 characters)".to_string(),
            ));
        }

        // Check for path traversal patterns
        if id.contains("..") || id.contains('/') || id.contains('\\') {
            return Err(StorageError::PathTraversalDetected);
        }

        // Allow only alphanumeric, underscore, and hyphen
        for c in id.chars() {
            if !c.is_ascii_alphanumeric() && c != '_' && c != '-' {
                return Err(StorageError::InvalidSelectorId(format!(
                    "invalid character: {}",
                    c
                )));
            }
        }

        Ok(())
    }

    /// Sanitizes selector ID
    fn sanitize_id(id: &str) -> Result<String, StorageError> {
        Self::validate_id(id)?;
        Ok(id.to_string())
    }

    /// Gets the file path for a selector ID
    fn get_selector_path(&self, id: &str) -> PathBuf {
        self.config.storage_dir.join(format!("{}.json", id))
    }

    /// Validates a recorded selector
    fn validate_selector(&self, selector: &RecordedSelector) -> Result<(), StorageError> {
        if selector.steps.is_empty() {
            return Err(StorageError::InvalidSelectorData(
                "selector steps cannot be empty".to_string(),
            ));
        }

        if selector.depth == 0 {
            return Err(StorageError::InvalidSelectorData(
                "selector depth must be greater than 0".to_string(),
            ));
        }

        if selector.depth > MAX_DEPTH {
            return Err(StorageError::InvalidSelectorData(format!(
                "selector depth exceeds maximum of {}",
                MAX_DEPTH
            )));
        }

        Ok(())
    }

    /// Validates a serialized selector
    fn validate_serialized_selector(
        &self,
        selector: &SerializableRecordedSelector,
    ) -> Result<(), StorageError> {
        if selector.steps.is_empty() {
            return Err(StorageError::InvalidSelectorData(
                "selector steps cannot be empty".to_string(),
            ));
        }

        if selector.depth == 0 {
            return Err(StorageError::InvalidSelectorData(
                "selector depth must be greater than 0".to_string(),
            ));
        }

        Ok(())
    }

    /// Checks storage size limits
    fn check_storage_size(&self) -> Result<(), StorageError> {
        if !self.config.storage_dir.exists() {
            return Ok(());
        }

        let total_size = std::fs::read_dir(&self.config.storage_dir)
            .map_err(|e| {
                error!("Failed to read storage directory: {}", e);
                StorageError::IoError(e.to_string())
            })?
            .filter_map(Result::ok)
            .filter_map(|e| e.metadata().ok())
            .filter(|m| m.is_file())
            .map(|m| m.len())
            .sum::<u64>();

        if total_size > self.config.max_storage_size {
            error!(
                "Storage size {} exceeds limit {}",
                total_size, self.config.max_storage_size
            );
            return Err(StorageError::StorageSizeLimitExceeded);
        }

        let count = std::fs::read_dir(&self.config.storage_dir)
            .map_err(|e| {
                error!("Failed to read storage directory: {}", e);
                StorageError::IoError(e.to_string())
            })?
            .filter_map(Result::ok)
            .filter(|e| e.path().extension().is_some_and(|ext| ext == "json"))
            .count();

        if count >= self.config.max_selectors {
            error!(
                "Selector count {} exceeds limit {}",
                count, self.config.max_selectors
            );
            return Err(StorageError::TooManySelectors);
        }

        Ok(())
    }

    /// Saves a selector to storage
    pub async fn save_selector(
        &self,
        id: &str,
        recorded: &RecordedSelector,
    ) -> Result<(), StorageError> {
        info!("Saving selector with ID: {}", id);

        // Validate ID
        let sanitized_id = Self::sanitize_id(id)?;

        // Validate selector
        self.validate_selector(recorded)?;

        // Check if storage exists
        if !self.config.storage_dir.exists() {
            tokio::fs::create_dir_all(&self.config.storage_dir)
                .await
                .map_err(|e| {
                    error!("Failed to create storage directory: {}", e);
                    StorageError::IoError(e.to_string())
                })?;
        }

        // Check storage size limit
        self.check_storage_size()?;

        // Check if selector already exists
        let path = self.get_selector_path(&sanitized_id);
        if path.exists() {
            error!("Selector already exists: {}", id);
            return Err(StorageError::SelectorAlreadyExists(id.to_string()));
        }

        // Serialize selector
        let serializable = SerializableRecordedSelector::from(recorded);
        let json = serde_json::to_string_pretty(&serializable).map_err(|e| {
            error!("Failed to serialize selector: {}", e);
            StorageError::SerializationError(e.to_string())
        })?;

        // Write to file
        tokio::fs::write(&path, &json).await.map_err(|e| {
            error!("Failed to write selector file: {}", e);
            StorageError::IoError(e.to_string())
        })?;

        info!("Selector saved successfully: {}", id);
        Ok(())
    }

    /// Loads a selector from storage
    pub async fn load_selector(&self, id: &str) -> Result<RecordedSelector, StorageError> {
        info!("Loading selector with ID: {}", id);

        // Validate ID
        let sanitized_id = Self::sanitize_id(id)?;

        // Get file path
        let path = self.get_selector_path(&sanitized_id);

        if !path.exists() {
            error!("Selector not found: {}", id);
            return Err(StorageError::SelectorNotFound(id.to_string()));
        }

        // Read file
        let json = tokio::fs::read_to_string(&path).await.map_err(|e| {
            error!("Failed to read selector file: {}", e);
            StorageError::IoError(e.to_string())
        })?;

        // Deserialize selector
        let serializable: SerializableRecordedSelector =
            serde_json::from_str(&json).map_err(|e| {
                error!("Failed to deserialize selector: {}", e);
                StorageError::SerializationError(e.to_string())
            })?;

        // Validate loaded data
        self.validate_serialized_selector(&serializable)?;

        let recorded = RecordedSelector::from(&serializable);

        info!("Selector loaded successfully: {}", id);
        Ok(recorded)
    }

    /// Lists all selectors in storage
    pub async fn list_selectors(&self) -> Result<Vec<String>, StorageError> {
        info!("Listing selectors in storage");

        if !self.config.storage_dir.exists() {
            debug!("Storage directory does not exist");
            return Ok(Vec::new());
        }

        // Check storage size limit
        self.check_storage_size()?;

        // Read directory
        let mut entries = tokio::fs::read_dir(&self.config.storage_dir)
            .await
            .map_err(|e| {
                error!("Failed to read storage directory: {}", e);
                StorageError::IoError(e.to_string())
            })?;

        let mut selectors = Vec::new();

        while let Some(entry) = entries.next_entry().await.map_err(|e| {
            error!("Failed to read directory entry: {}", e);
            StorageError::IoError(e.to_string())
        })? {
            let file_name = entry.file_name();
            let file_name_str = file_name.to_string_lossy();

            if file_name_str.ends_with(".json") {
                if let Some(id) = file_name_str.strip_suffix(".json") {
                    selectors.push(id.to_string());
                }
            }
        }

        debug!("Found {} selectors", selectors.len());
        Ok(selectors)
    }

    /// Deletes a selector from storage
    pub async fn delete_selector(&self, id: &str) -> Result<(), StorageError> {
        info!("Deleting selector with ID: {}", id);

        // Validate ID
        let sanitized_id = Self::sanitize_id(id)?;

        // Get file path
        let path = self.get_selector_path(&sanitized_id);

        if !path.exists() {
            error!("Selector not found: {}", id);
            return Err(StorageError::SelectorNotFound(id.to_string()));
        }

        // Delete file
        tokio::fs::remove_file(&path).await.map_err(|e| {
            error!("Failed to delete selector file: {}", e);
            StorageError::IoError(e.to_string())
        })?;

        info!("Selector deleted successfully: {}", id);
        Ok(())
    }
}

impl Default for SelectorStorage {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::selector::SelectorStep;
    use tempfile::tempdir;

    fn create_test_selector() -> RecordedSelector {
        let step1 = SelectorStep {
            classname: Some("Notepad".to_string()),
            control_type: None,
            name: Some("Untitled - Notepad".to_string()),
            automation_id: None,
        };
        let step2 = SelectorStep {
            classname: Some("Edit".to_string()),
            control_type: None,
            name: Some("".to_string()),
            automation_id: Some("12345".to_string()),
        };
        RecordedSelector {
            steps: vec![step1, step2],
            depth: 2,
        }
    }

    #[tokio::test]
    async fn test_sanitize_id_valid() {
        assert!(SelectorStorage::sanitize_id("my_selector").is_ok());
        assert!(SelectorStorage::sanitize_id("selector-123").is_ok());
        assert!(SelectorStorage::sanitize_id("test_selector_42").is_ok());
        assert!(SelectorStorage::sanitize_id("Aa0_").is_ok());
    }

    #[tokio::test]
    async fn test_sanitize_id_invalid() {
        assert!(SelectorStorage::sanitize_id("").is_err());
        assert!(SelectorStorage::sanitize_id("..").is_err());
        assert!(SelectorStorage::sanitize_id("../test").is_err());
        assert!(SelectorStorage::sanitize_id("test/../path").is_err());
        assert!(SelectorStorage::sanitize_id("test\\path").is_err());
        assert!(SelectorStorage::sanitize_id("test/path").is_err());
        assert!(SelectorStorage::sanitize_id("test with spaces").is_err());
        assert!(SelectorStorage::sanitize_id("test@selector").is_err());
    }

    #[tokio::test]
    async fn test_save_and_load_selector() {
        let tmp_dir = tempdir().expect("Failed to create temp dir");
        let config = SelectorStorageConfig::with_storage_dir(tmp_dir.path().to_path_buf());
        let storage = SelectorStorage::with_config(config);

        let selector = create_test_selector();
        let selector_id = "test_selector";

        // Save selector
        storage
            .save_selector(selector_id, &selector)
            .await
            .expect("Failed to save selector");

        // Load selector
        let loaded = storage
            .load_selector(selector_id)
            .await
            .expect("Failed to load selector");

        // Verify content
        assert_eq!(loaded.steps.len(), selector.steps.len());
        assert_eq!(loaded.depth, selector.depth);
    }

    #[tokio::test]
    async fn test_save_already_exists() {
        let tmp_dir = tempdir().expect("Failed to create temp dir");
        let config = SelectorStorageConfig::with_storage_dir(tmp_dir.path().to_path_buf());
        let storage = SelectorStorage::with_config(config);

        let selector = create_test_selector();
        let selector_id = "duplicate_selector";

        // Save first time
        storage
            .save_selector(selector_id, &selector)
            .await
            .expect("Failed to save selector");

        // Save second time should fail
        let result = storage.save_selector(selector_id, &selector).await;
        assert!(matches!(
            result,
            Err(StorageError::SelectorAlreadyExists(_))
        ));
    }

    #[tokio::test]
    async fn test_load_not_found() {
        let tmp_dir = tempdir().expect("Failed to create temp dir");
        let config = SelectorStorageConfig::with_storage_dir(tmp_dir.path().to_path_buf());
        let storage = SelectorStorage::with_config(config);

        let result = storage.load_selector("nonexistent_selector").await;
        assert!(matches!(result, Err(StorageError::SelectorNotFound(_))));
    }

    #[tokio::test]
    async fn test_list_selectors() {
        let tmp_dir = tempdir().expect("Failed to create temp dir");
        let config = SelectorStorageConfig::with_storage_dir(tmp_dir.path().to_path_buf());
        let storage = SelectorStorage::with_config(config);

        let selector = create_test_selector();

        // Save multiple selectors
        let ids = vec!["selector_1", "selector_2", "selector_3"];
        for id in &ids {
            storage
                .save_selector(id, &selector)
                .await
                .expect("Failed to save");
        }

        // List selectors
        let listed = storage.list_selectors().await.expect("Failed to list");

        // Verify all IDs are present
        for id in &ids {
            assert!(
                listed.contains(&id.to_string()),
                "ID {} should be in list",
                id
            );
        }
    }

    #[tokio::test]
    async fn test_delete_selector() {
        let tmp_dir = tempdir().expect("Failed to create temp dir");
        let config = SelectorStorageConfig::with_storage_dir(tmp_dir.path().to_path_buf());
        let storage = SelectorStorage::with_config(config);

        let selector = create_test_selector();
        let selector_id = "delete_test";

        // Save selector
        storage
            .save_selector(selector_id, &selector)
            .await
            .expect("Failed to save selector");

        // Delete selector
        storage
            .delete_selector(selector_id)
            .await
            .expect("Failed to delete selector");

        // Verify file is gone
        let result = storage.load_selector(selector_id).await;
        assert!(matches!(result, Err(StorageError::SelectorNotFound(_))));
    }

    #[tokio::test]
    async fn test_delete_not_found() {
        let tmp_dir = tempdir().expect("Failed to create temp dir");
        let config = SelectorStorageConfig::with_storage_dir(tmp_dir.path().to_path_buf());
        let storage = SelectorStorage::with_config(config);

        let result = storage.delete_selector("nonexistent").await;
        assert!(matches!(result, Err(StorageError::SelectorNotFound(_))));
    }

    #[tokio::test]
    async fn test_serializable_roundtrip() {
        let original = SelectorStep {
            classname: Some("Button".to_string()),
            control_type: None,
            name: Some("OK".to_string()),
            automation_id: Some("ok_button".to_string()),
        };

        let serializable: SerializableSelectorStep = (&original).into();
        let converted_back: SelectorStep = (&serializable).into();

        assert_eq!(converted_back.classname, original.classname);
        assert_eq!(converted_back.name, original.name);
        assert_eq!(converted_back.automation_id, original.automation_id);
    }

    #[tokio::test]
    async fn test_empty_directory_list() {
        let tmp_dir = tempdir().expect("Failed to create temp dir");
        let config = SelectorStorageConfig::with_storage_dir(tmp_dir.path().to_path_buf());
        let storage = SelectorStorage::with_config(config);

        // List from empty directory should return empty vec
        let listed = storage.list_selectors().await.expect("Failed to list");
        assert!(
            listed.is_empty(),
            "List should be empty for empty directory"
        );
    }
}
