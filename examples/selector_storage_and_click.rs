//! Example: Save selectors to JSON and use them for clicking
//!
//! This example demonstrates:
//! 1. Recording UI selectors and saving them to JSON files
//! 2. Loading selectors from JSON files
//! 3. Using saved selectors to perform click operations
//!
//! Run this example:
//! ```bash
//! cargo run --example selector_storage_and_click
//! ```

use std::time::Duration;
use tracing_subscriber::EnvFilter;

use smith_windows::core::click::{ClickConfig, ClickType};
use smith_windows::core::selector::{RecordedSelector, SelectorStep};
use smith_windows::core::selector_storage::{SelectorStorage, SerializableRecordedSelector};
use smith_windows::runtime::backends::windows::click::ClickBackendWindows;
use uiautomation::UIAutomation;

/// Example data structure for saved selectors
#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct SavedSelectors {
    /// Application name
    app_name: String,
    /// Version of the selector schema
    version: u32,
    /// Map of named selectors
    selectors: std::collections::HashMap<String, SerializableRecordedSelector>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    println!("=== Selector Storage and Click Example ===");

    // Example 1: Create and save a selector
    example_save_selector().await?;

    // Example 2: Load and use a saved selector
    example_load_and_use_selector().await?;

    // Example 3: Batch save multiple selectors
    example_batch_save().await?;

    println!("\n=== Example Completed Successfully ===");
    Ok(())
}

/// Example: Create a selector and save it to JSON
async fn example_save_selector() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Example 1: Save Selector to JSON ---");

    // Create a selector for a Notepad window
    let notepad_selector = RecordedSelector {
        steps: vec![
            SelectorStep {
                classname: Some("Notepad".to_string()),
                control_type: None,
                name: Some("Untitled - Notepad".to_string()),
                automation_id: None,
            },
            SelectorStep {
                classname: Some("Edit".to_string()),
                control_type: None,
                name: Some("".to_string()),
                automation_id: Some("12345".to_string()),
            },
        ],
        depth: 2,
    };
    let _notepad_serializable: SerializableRecordedSelector = (&notepad_selector).into();

    // Create storage
    let storage = SelectorStorage::new();

    // Clean up any existing selector with the same ID to avoid conflicts
    let _ = storage.delete_selector("notepad_main_window").await;
    let _ = storage.delete_selector("notepad_file_menu").await;
    let _ = storage.delete_selector("calculator_button").await;

    // Save the selector with a custom ID
    let selector_id = "notepad_main_window";
    storage
        .save_selector(selector_id, &notepad_selector)
        .await
        .map_err(|e| {
            println!("Failed to save selector: {}", e);
            e
        })?;

    println!("✓ Selector '{}' saved successfully", selector_id);

    // Save to a custom JSON file as well
    let custom_save = |selector: &RecordedSelector| -> Result<String, Box<dyn std::error::Error>> {
        let serializable: SerializableRecordedSelector = selector.into();
        let json = serde_json::to_string_pretty(&serializable)?;

        // Write to a custom location
        let path = "D:\\Alexey\\rust\\smith-windows\\examples\\saved_selectors.json";
        std::fs::write(path, &json)?;
        Ok(json)
    };

    let json_str = custom_save(&notepad_selector)?;
    println!("✓ Also saved to custom JSON file");
    println!("  JSON content preview: {}", &json_str[..100]);

    Ok(())
}

/// Example: Load a selector from JSON and use it for clicking
async fn example_load_and_use_selector() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Example 2: Load and Use Selector ---");

    // Create storage
    let storage = SelectorStorage::new();

    // Load the saved selector
    let loaded_selector = storage
        .load_selector("notepad_main_window")
        .await
        .map_err(|e| {
            println!("Failed to load selector: {}", e);
            e
        })?;

    println!("✓ Selector loaded successfully");
    println!("  Steps: {}", loaded_selector.steps.len());
    println!("  Depth: {}", loaded_selector.depth);

    // Verify the loaded selector matches what we saved
    assert_eq!(loaded_selector.steps.len(), 2);
    assert_eq!(loaded_selector.depth, 2);
    assert_eq!(
        loaded_selector.steps[0].classname.as_deref(),
        Some("Notepad")
    );

    // Now use it to perform a click operation
    println!("\nUsing loaded selector to find and click element...");

    let ui_automation = UIAutomation::new().map_err(|e| {
        println!("Failed to create UIAutomation: {}", e);
        e
    })?;

    let root_element = ui_automation.get_root_element().map_err(|e| {
        println!("Failed to get root element: {}", e);
        e
    })?;

    // Create a matcher using the loaded selector steps
    let mut matcher = ui_automation.create_matcher().from(root_element);

    for step in &loaded_selector.steps {
        if let Some(classname) = &step.classname {
            matcher = matcher.classname(classname);
        }
        if let Some(name) = &step.name {
            matcher = matcher.name(name);
        }
        if let Some(control_type) = &step.control_type {
            matcher = matcher.control_type(control_type.clone());
        }
    }

    // Try to find the element
    match matcher.timeout(5000).find_first() {
        Ok(element) => {
            println!("✓ Element found using loaded selector");

            // Perform click operation with LeftSingle click type
            let cancellation = tokio_util::sync::CancellationToken::new();
            let config = ClickConfig {
                click_type: ClickType::LeftSingle,
                timeout: Duration::from_secs(5),
                cancellation,
            };

            let backend = ClickBackendWindows::new();
            match backend.click(&element, ClickType::LeftSingle).await {
                Ok(()) => println!("✓ Click operation successful!"),
                Err(e) => println!("✗ Click failed: {}", e),
            }
        }
        Err(e) => {
            println!("✗ Element not found: {}", e);
            println!("  (This is expected if Notepad is not running)");
        }
    }

    Ok(())
}

/// Example: Batch save multiple selectors
async fn example_batch_save() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Example 3: Batch Save Multiple Selectors ---");

    // Create multiple selectors for different UI elements
    let selectors = vec![
        (
            "notepad_file_menu",
            RecordedSelector {
                steps: vec![
                    SelectorStep {
                        classname: Some("Notepad".to_string()),
                        control_type: None,
                        name: Some("Untitled - Notepad".to_string()),
                        automation_id: None,
                    },
                    SelectorStep {
                        classname: Some("Edit".to_string()),
                        control_type: None,
                        name: Some("".to_string()),
                        automation_id: Some("12345".to_string()),
                    },
                    SelectorStep {
                        classname: Some("Menu".to_string()),
                        control_type: None,
                        name: Some("File".to_string()),
                        automation_id: None,
                    },
                ],
                depth: 3,
            },
        ),
        (
            "calculator_button",
            RecordedSelector {
                steps: vec![
                    SelectorStep {
                        classname: Some("CalcFrame".to_string()),
                        control_type: None,
                        name: Some("".to_string()),
                        automation_id: None,
                    },
                    SelectorStep {
                        classname: Some("Button".to_string()),
                        control_type: None,
                        name: Some("1".to_string()),
                        automation_id: None,
                    },
                ],
                depth: 2,
            },
        ),
    ];

    // Convert to serializable format
    let serializable_selectors: Vec<(String, SerializableRecordedSelector)> = selectors
        .into_iter()
        .map(|(id, selector)| (id.to_string(), (&selector).into()))
        .collect();

    // Save all selectors at once
    let storage = SelectorStorage::new();

    for (id, selector) in &serializable_selectors {
        let recorded: RecordedSelector = selector.into();
        storage.save_selector(id, &recorded).await.map_err(|e| {
            println!("Failed to save selector '{}': {}", id, e);
            e
        })?;
        println!("✓ Saved selector: '{}'", id);
    }

    // List all saved selectors
    let saved_ids = storage.list_selectors().await.map_err(|e| {
        println!("Failed to list selectors: {}", e);
        e
    })?;

    println!("\nSaved selectors: {:?}", saved_ids);

    // Create a combined selectors collection
    let mut combined_selectors = std::collections::HashMap::new();
    for (id, selector) in serializable_selectors {
        combined_selectors.insert(id, selector);
    }

    let saved_selectors = SavedSelectors {
        app_name: "Notepad".to_string(),
        version: 1,
        selectors: combined_selectors,
    };

    // Save to a structured JSON file
    let json_path = "D:\\Alexey\\rust\\smith-windows\\examples\\batch_selectors.json";
    let json = serde_json::to_string_pretty(&saved_selectors)?;
    std::fs::write(json_path, json)?;

    println!("✓ Batch saved to: {}", json_path);

    Ok(())
}

#[cfg(test)]
mod tests {
    use smith_windows::core::selector::RecordedSelector;
    use smith_windows::core::selector_storage::{
        SelectorStorage, SelectorStorageConfig, SerializableRecordedSelector,
    };
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_save_and_load_selector() {
        let tmp_dir = tempdir().expect("Failed to create temp dir");
        let config = SelectorStorageConfig::with_storage_dir(tmp_dir.path().to_path_buf());
        let storage = SelectorStorage::with_config(config);

        let selector = RecordedSelector {
            steps: vec![
                smith_windows::core::selector::SelectorStep {
                    classname: Some("TestApp".to_string()),
                    control_type: None,
                    name: Some("Test Window".to_string()),
                    automation_id: None,
                },
                smith_windows::core::selector::SelectorStep {
                    classname: Some("Button".to_string()),
                    control_type: None,
                    name: Some("OK".to_string()),
                    automation_id: Some("btn_ok".to_string()),
                },
            ],
            depth: 2,
        };

        // Save
        storage
            .save_selector("test_button", &selector)
            .await
            .expect("Failed to save selector");

        // Load
        let loaded = storage
            .load_selector("test_button")
            .await
            .expect("Failed to load selector");

        // Verify
        assert_eq!(loaded.steps.len(), 2);
        assert_eq!(loaded.depth, 2);
        assert_eq!(loaded.steps[0].classname.as_deref(), Some("TestApp"));
    }

    #[tokio::test]
    async fn test_batch_save_selectors() {
        let tmp_dir = tempdir().expect("Failed to create temp dir");
        let config = SelectorStorageConfig::with_storage_dir(tmp_dir.path().to_path_buf());
        let storage = SelectorStorage::with_config(config);

        let selectors = vec![
            (
                "selector1",
                RecordedSelector {
                    steps: vec![smith_windows::core::selector::SelectorStep {
                        classname: Some("App1".to_string()),
                        control_type: None,
                        name: Some("Window1".to_string()),
                        automation_id: None,
                    }],
                    depth: 1,
                },
            ),
            (
                "selector2",
                RecordedSelector {
                    steps: vec![smith_windows::core::selector::SelectorStep {
                        classname: Some("App2".to_string()),
                        control_type: None,
                        name: Some("Window2".to_string()),
                        automation_id: None,
                    }],
                    depth: 1,
                },
            ),
        ];

        for (id, selector) in selectors {
            storage
                .save_selector(id, &selector)
                .await
                .expect("Failed to save selector");
        }

        let listed = storage
            .list_selectors()
            .await
            .expect("Failed to list selectors");
        assert_eq!(listed.len(), 2);
        assert!(listed.contains(&"selector1".to_string()));
        assert!(listed.contains(&"selector2".to_string()));
    }

    #[tokio::test]
    async fn test_serialize_roundtrip() {
        let original = RecordedSelector {
            steps: vec![
                smith_windows::core::selector::SelectorStep {
                    classname: Some("Window".to_string()),
                    control_type: None,
                    name: Some("Main".to_string()),
                    automation_id: None,
                },
                smith_windows::core::selector::SelectorStep {
                    classname: Some("Button".to_string()),
                    control_type: None,
                    name: Some("Click".to_string()),
                    automation_id: Some("id123".to_string()),
                },
            ],
            depth: 2,
        };

        // Serialize to SerializableRecordedSelector
        let serializable: SerializableRecordedSelector = (&original).into();

        // Convert back
        let converted: RecordedSelector = (&serializable).into();

        // Verify equality
        assert_eq!(original.steps.len(), converted.steps.len());
        assert_eq!(original.depth, converted.depth);
    }
}
