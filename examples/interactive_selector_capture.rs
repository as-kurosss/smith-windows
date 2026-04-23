//! Example: Interactive Selector Capture
//!
//! This example demonstrates interactive UI selector capture:
//! 1. Application starts and waits for user input
//! 2. User opens Notepad (or any app) and hovers cursor over a UI element
//! 3. User presses Ctrl while hovering over the desired element
//! 4. Selector is captured and saved with a user-friendly name
//!
//! Run this example:
//! ```bash
//! cargo run --example interactive_selector_capture
//! ```

use std::io::Write;
use std::time::Duration;
use tokio_util::sync::CancellationToken;
use tracing_subscriber::EnvFilter;

use smith_windows::core::input::InputConfig;
use smith_windows::core::selector::{RecordedSelector, SelectorStep};
use smith_windows::core::selector_storage::{SelectorStorage, SelectorStorageConfig};
use smith_windows::runtime::backends::windows::input::get_element_under_ctrl_hotkey;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    println!("=== Interactive Selector Capture ===");
    println!("\nInstructions:");
    println!("  1. The application will wait for you to press Ctrl");
    println!("  2. Open Notepad (or any application) before pressing Ctrl");
    println!("  3. Hover your cursor over the UI element you want to capture");
    println!("  4. Press and hold Ctrl to capture the element");
    println!("  5. The selector will be saved with a name based on element properties");
    println!("\nPress Enter to start...");
    std::io::stdin().read_line(&mut String::new())?;

    println!("\n--- Waiting for Ctrl+Hover ---");
    println!("(Now open Notepad if you haven't already, then hover and press Ctrl)");

    // Clean up any existing selector with the same ID to avoid conflicts
    // Use the same storage directory that will be used for saving
    let project_dir =
        std::env::current_dir().map_err(|e| format!("Failed to get current dir: {}", e))?;
    let selectors_dir = project_dir.join("examples").join("selectors");
    let _ = SelectorStorage::with_config(SelectorStorageConfig::with_storage_dir(
        selectors_dir.clone(),
    ))
    .delete_selector("element_Microsoft_Windows_Explorer")
    .await;
    let _ = SelectorStorage::with_config(SelectorStorageConfig::with_storage_dir(
        selectors_dir.clone(),
    ))
    .delete_selector("test")
    .await;

    // Wait for Ctrl+Hover
    let config = InputConfig {
        timeout: Duration::from_secs(30), // 30 seconds timeout
        cancellation: CancellationToken::new(),
    };
    let element = get_element_under_ctrl_hotkey(&config).await?;

    let element_name = element.get_name().unwrap_or_default();
    let element_class = element.get_classname().unwrap_or_default();
    let element_automation_id = element.get_automation_id().unwrap_or_default();

    println!("\n✓ Ctrl+Hover detected!");
    println!("  Element found:");
    println!("    Name: '{}'", element_name);
    println!("    Class: '{}'", element_class);
    println!("    Automation ID: '{}'", element_automation_id);

    // Create storage config with a known directory
    let storage_dir = std::env::temp_dir().join("smith-windows-selectors");
    let config = SelectorStorageConfig::with_storage_dir(storage_dir);
    let storage = SelectorStorage::with_config(config);

    // Generate a user-friendly name based on element properties
    let selector_id = generate_selector_id(&element_name, &element_class, &element_automation_id);
    println!("\nGenerated selector ID: '{}'", selector_id);

    // Ask user for confirmation or custom name
    println!("\nDo you want to save with this name? (y/n)");
    print!("  > ");
    std::io::stdout().flush()?;

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    let selector_id = if input.trim().eq_ignore_ascii_case("n") {
        println!("\nEnter a custom name for the selector:");
        print!("  > ");
        std::io::stdout().flush()?;
        let mut custom_name = String::new();
        std::io::stdin().read_line(&mut custom_name)?;
        custom_name.trim().to_string()
    } else {
        selector_id
    };

    println!("\n--- Capturing and saving selector ---");

    // Save to project directory: examples/selectors/<name>.json
    let project_dir =
        std::env::current_dir().map_err(|e| format!("Failed to get current dir: {}", e))?;
    let selectors_dir = project_dir.join("examples").join("selectors");

    // Create directory if it doesn't exist
    std::fs::create_dir_all(&selectors_dir)
        .map_err(|e| format!("Failed to create selectors directory: {}", e))?;

    let storage_dir = selectors_dir.clone();
    let config = SelectorStorageConfig::with_storage_dir(storage_dir);
    let storage = SelectorStorage::with_config(config);

    let recorded = build_selector_from_element(&element)?;

    storage
        .save_selector(&selector_id, &recorded)
        .await
        .map_err(|e| {
            println!("Failed to save selector: {}", e);
            e
        })?;

    println!("✓ Selector saved successfully with ID: '{}'", selector_id);
    println!("  Location: {}", selectors_dir.display());

    // List all saved selectors
    let saved_ids = storage.list_selectors().await.map_err(|e| {
        println!("Failed to list selectors: {}", e);
        e
    })?;

    println!("\n=== All Saved Selectors ===");
    for id in &saved_ids {
        println!("  - {}", id);
    }

    println!("\n=== Interactive Selector Capture Completed ===");
    println!("\nYou can now use the selector in your code:");
    println!("  use smith_windows::core::selector_storage::SelectorStorage;");
    println!("  let storage = SelectorStorage::new();");
    println!(
        "  let selector = storage.load_selector(\"{}\").await?;",
        selector_id
    );

    Ok(())
}

/// Generates a user-friendly selector ID based on element properties
fn generate_selector_id(name: &str, class: &str, automation_id: &str) -> String {
    // Priority: automation_id > name > class
    if !automation_id.is_empty() {
        // Use automation_id with sanitized name
        let sanitized = automation_id
            .chars()
            .map(|c| {
                if c.is_ascii_alphanumeric() || c == '_' || c == '-' {
                    c
                } else {
                    '_'
                }
            })
            .collect::<String>();
        format!("element_{}", sanitized)
    } else if !name.is_empty() {
        // Use name (cleaned)
        let cleaned = name
            .chars()
            .map(|c| {
                if c.is_ascii_alphanumeric() || c == '_' || c == '-' || c == ' ' {
                    c
                } else {
                    '_'
                }
            })
            .collect::<String>();
        let simplified = cleaned.replace(" ", "_");
        format!("element_{}", simplified.to_lowercase())
    } else if !class.is_empty() {
        // Use class name
        let simplified = class
            .chars()
            .map(|c| {
                if c.is_ascii_alphanumeric() || c == '_' || c == '-' {
                    c
                } else {
                    '_'
                }
            })
            .collect::<String>();
        format!("element_{}", simplified.to_lowercase())
    } else {
        // Fallback
        format!(
            "element_{}",
            uuid::Uuid::new_v4().to_string()[..8].to_string()
        )
    }
}

/// Builds a selector from a UI element
fn build_selector_from_element(
    element: &uiautomation::UIElement,
) -> Result<RecordedSelector, Box<dyn std::error::Error>> {
    let classname = element.get_classname()?;
    let control_type = element.get_control_type()?;
    let name = element.get_name()?;
    let automation_id = element.get_automation_id()?;

    let step = SelectorStep {
        classname: Some(classname),
        control_type: Some(control_type),
        name: Some(name),
        automation_id: Some(automation_id),
    };

    // For a single element, depth is 1
    Ok(RecordedSelector {
        steps: vec![step],
        depth: 1,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_selector_id_with_automation_id() {
        let id = generate_selector_id("OK Button", "Button", "btn_ok_123");
        assert!(id.contains("btn_ok_123"));
    }

    #[test]
    fn test_generate_selector_id_with_name() {
        let id = generate_selector_id("Close", "Button", "");
        assert!(id.contains("close"));
    }

    #[test]
    fn test_generate_selector_id_with_class() {
        let id = generate_selector_id("", "Notepad", "");
        assert!(id.contains("notepad"));
    }

    #[test]
    fn test_generate_selector_id_fallback() {
        let id = generate_selector_id("", "", "");
        // Should generate a UUID-like string
        assert!(id.len() > 10);
    }
}
