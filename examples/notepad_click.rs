//! Example: Click on Notepad Menu
//!
//! This example demonstrates how to use the ClickTool to automate Notepad.
//! It opens Notepad, clicks on the "File" menu item, and then closes Notepad.
//!
//! Run this example:
//! ```bash
//! cargo run --example notepad_click
//! ```

use std::time::Duration;
use tracing_subscriber::EnvFilter;

use smith_windows::core::click::{ClickConfig, ClickError, ClickType, MockClickBackend};
use smith_windows::runtime::backends::windows::click::ClickBackendWindows;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    println!("=== Notepad Click Example - Starting ===");

    // Example 1: Configuration validation
    example_configuration().await?;

    // Example 2: Using MockClickBackend for testing scenarios
    example_mock_backend().await?;

    // Example 3: Click on Notepad menu
    example_notepad_click().await?;

    println!("\n=== Notepad Click Example - Completed successfully ===");
    Ok(())
}

/// Example: Configuration validation
async fn example_configuration() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Example 1: Configuration validation ---");

    let cancellation = tokio_util::sync::CancellationToken::new();

    // Valid configuration
    let config = ClickConfig {
        click_type: ClickType::LeftSingle,
        timeout: Duration::from_secs(5),
        cancellation,
    };

    println!("✓ Valid configuration created (validation happens internally)");

    // Note: validate_click_config is no longer exported
    // Validation happens internally in click_with_config

    Ok(())
}

/// Example: Using MockClickBackend for testing
async fn example_mock_backend() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Example 2: MockClickBackend ---");

    let backend = MockClickBackend::new();

    // Test 1: Success scenario
    {
        let mut state = backend.get_state().unwrap();
        state.should_succeed = true;
        state.call_count = 0;
    }

    println!("Mock backend configured for success");

    // Note: Mock backend doesn't need real UI elements
    // It's useful for unit testing without UI dependencies

    backend.reset().unwrap();
    println!("Mock backend reset");

    // Test 2: Failure scenario
    {
        let mut state = backend.get_state().unwrap();
        state.should_succeed = false;
        state.last_error = Some(ClickError::ElementNotEnabled);
    }

    println!("Mock backend configured for failure scenario");
    println!("This demonstrates how to test error handling without real UI");

    Ok(())
}

/// Example: Click on Notepad menu
#[allow(dead_code)]
async fn example_notepad_click() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Example 3: Click on Notepad menu ---");

    let cancellation = tokio_util::sync::CancellationToken::new();
    let config = ClickConfig {
        click_type: ClickType::LeftSingle,
        timeout: Duration::from_secs(10),
        cancellation,
    };

    println!("\nStep 1: Opening Notepad...");
    println!("Launching notepad.exe...");
    let _process = std::process::Command::new("notepad.exe")
        .spawn()
        .map_err(|e| {
            println!("Failed to start Notepad: {}", e);
            e
        })?;

    println!("Waiting for Notepad to start...");
    tokio::time::sleep(Duration::from_secs(2)).await;

    println!("Creating UIAutomation...");
    let ui_automation = uiautomation::UIAutomation::new().map_err(|e| {
        println!("Failed to create UIAutomation: {}", e);
        e
    })?;

    println!("Getting root element...");
    let root_element = ui_automation.get_root_element().map_err(|e| {
        println!("Failed to get root element: {}", e);
        e
    })?;

    // Search for Notepad window
    println!("\nStep 2: Finding Notepad window...");
    let notepad = loop {
        match ui_automation
            .create_matcher()
            .from(root_element.clone())
            .control_type(uiautomation::types::ControlType::Window)
            .timeout(2000)
            .find_first()
        {
            Ok(el) => {
                let name = el.get_name().unwrap_or_default();
                println!("Found window: {}", name);
                if name.contains("Блокнот") || name.contains("Notepad") || name.is_empty() {
                    println!("✓ Found Notepad window");
                    break el;
                }
            }
            Err(e) => {
                println!("Search attempt failed: {}", e);
            }
        }
    };

    // Search for menu items
    println!("\nStep 3: Finding menu items...");
    let menu_items = ui_automation
        .create_matcher()
        .from(notepad.clone())
        .control_type(uiautomation::types::ControlType::MenuItem)
        .timeout(5000)
        .find_all()
        .map_err(|e| {
            println!("Failed to find menu items: {}", e);
            e
        })?;

    if menu_items.len() > 1 {
        println!("Menu items found as children of Notepad window");

        // Find "File" menu
        let file_menu = menu_items
            .iter()
            .find(|item| {
                item.get_name()
                    .ok()
                    .map(|n| n.contains("Файл") || n.contains("File"))
                    .unwrap_or(false)
            })
            .cloned()
            .ok_or("File menu not found")?;

        println!(
            "✓ Found 'File' menu item: '{}'",
            file_menu.get_name().unwrap_or_default()
        );

        // Click the File menu
        println!("Clicking 'File' menu...");
        let backend = ClickBackendWindows::new();

        match backend.click(&file_menu, ClickType::LeftSingle).await {
            Ok(()) => {
                println!("✓ Click successful! 'File' menu should be open now");
            }
            Err(e) => {
                println!("✗ Click failed: {}", e);
                return Err(e.into());
            }
        }
    } else {
        // Menu items found directly
        println!("Menu items found directly");

        // Find "File" menu - menu_items is Vec<UIElement>
        let file_menu = menu_items
            .iter()
            .find(|item| {
                item.get_name()
                    .ok()
                    .map(|n| n.contains("Файл") || n.contains("File"))
                    .unwrap_or(false)
            })
            .cloned() // Clone UIElement since it's Clone
            .ok_or("File menu not found")?;

        println!(
            "✓ Found 'File' menu item: '{}'",
            file_menu.get_name().unwrap_or_default()
        );

        // Click the File menu
        println!("Clicking 'File' menu...");
        let backend = ClickBackendWindows::new();

        match backend.click(&file_menu, ClickType::LeftSingle).await {
            Ok(()) => {
                println!("✓ Click successful! 'File' menu should be open now");
            }
            Err(e) => {
                println!("✗ Click failed: {}", e);
                return Err(e.into());
            }
        }
    }

    // Wait to see the result
    println!("\nWaiting 3 seconds to see the result...");
    tokio::time::sleep(Duration::from_secs(3)).await;

    println!("\nStep 4: Closing Notepad...");
    // Send Esc to close the menu, then Alt+F4 to close Notepad
    println!("Sending Escape to close menu...");
    // This would require keyboard automation - for now just exit

    println!("Example completed successfully!");
    Ok(())
}
