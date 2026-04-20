//! Example: AutomationSession with ClickTool Integration
//!
//! This example demonstrates how to use AutomationSession to launch an application
//! and pass the session to ClickTool for UI interactions.
//!
//! The flow is:
//! 1. Launch Notepad using AutomationSession (creates RuntimeSession with process_id)
//! 2. Attach to the Notepad window using RuntimeSession's main_element
//! 3. Find menu items within the Notepad window
//! 4. Use ClickTool to click on "Файл" menu item
//!
//! Run this example:
//! ```bash
//! cargo run --example notepad_automation_session
//! ```

use std::time::Duration;
use tracing_subscriber::EnvFilter;

use smith_windows::core::automation_session::{
    attach_by_process_id, launch_process, SessionConfig, SessionLaunchConfig,
};
use smith_windows::runtime::backends::windows::click::ClickBackendWindows;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    println!("=== Notepad AutomationSession + ClickTool Example ===");
    println!("This example demonstrates the integration between:");
    println!("  - AutomationSession (for process launch and window attachment)");
    println!("  - ClickTool (for UI element clicking)");
    println!();

    // Step 1: Launch Notepad
    println!("Step 1: Launching Notepad...");
    let launch_config = SessionLaunchConfig {
        command: "notepad.exe".to_string(),
        args: None,
        working_dir: None,
    };

    let process_id = match launch_process(&launch_config).await {
        Ok(id) => {
            println!("  ✓ Notepad launched with PID: {}", id);
            id
        }
        Err(e) => {
            println!("  ✗ Failed to launch Notepad: {}", e);
            return Err(e.into());
        }
    };

    // Wait for Notepad to fully start
    println!("  Waiting for Notepad to initialize...");
    tokio::time::sleep(Duration::from_secs(2)).await;

    // Step 2: Attach to Notepad window using the process ID
    println!("\nStep 2: Attaching to Notepad window...");
    let cancellation = tokio_util::sync::CancellationToken::new();
    let session_config = SessionConfig {
        timeout: Duration::from_secs(10),
        cancellation,
    };

    let session = match attach_by_process_id(process_id, &session_config).await {
        Ok(s) => {
            println!("  ✓ Attached to Notepad window (PID: {})", s.process_id);
            s
        }
        Err(e) => {
            println!("  ✗ Failed to attach: {}", e);
            return Err(e.into());
        }
    };

    // Step 3: Try to find menu items
    println!("\nStep 3: Finding menu items in Notepad...");

    // Use a direct search from the main element
    let automation = uiautomation::UIAutomation::new().map_err(|e| {
        println!("  Failed to create UIAutomation: {}", e);
        e
    })?;

    // Search for MenuItem directly under main window
    println!("  Searching for MenuItem elements...");
    let menu_items = automation
        .create_matcher()
        .from(session.main_element.clone())
        .control_type(uiautomation::types::ControlType::MenuItem)
        .timeout(3000)
        .find_all()
        .map_err(|e| {
            println!("  Failed to find menu items: {}", e);
            e
        })?;

    println!("  ✓ Found {} MenuItem elements", menu_items.len());

    // Print all items for debugging
    for (i, item) in menu_items.iter().enumerate() {
        let name = item.get_name().unwrap_or_default();
        let enabled = item.is_enabled().unwrap_or(false);
        println!("    [{}] '{}', enabled={}", i, name, enabled);
    }

    // Step 4: Find "Файл" menu item
    println!("\nStep 4: Finding 'Файл' menu item...");

    let file_menu = menu_items
        .iter()
        .find(|item| {
            item.get_name()
                .ok()
                .map(|name| name.contains("Файл") || name.contains("File"))
                .unwrap_or(false)
        })
        .cloned()
        .ok_or("Файл menu item not found")?;

    let file_name = file_menu.get_name().unwrap_or_default();
    println!("  ✓ Found 'Файл' menu item: '{}'", file_name);

    // Step 5: Use ClickTool to click on "Файл"
    println!("\nStep 5: Clicking 'Файл' menu item with ClickTool...");

    let click_backend = ClickBackendWindows::new();

    match click_backend.click(&file_menu).await {
        Ok(()) => {
            println!("  ✓ Click successful! 'Файл' menu should be open now");
        }
        Err(e) => {
            println!("  ✗ Click failed: {}", e);
            return Err(e.into());
        }
    }

    // Step 6: Demonstrate session cleanup
    println!("\nStep 6: Demonstrating session cleanup...");

    // Close the session (terminates the process)
    match session.close().await {
        Ok(()) => {
            println!("  ✓ Session closed (Notepad terminated)");
        }
        Err(e) => {
            println!("  Note: Session close: {}", e);
        }
    }

    println!("\n=== Example Completed Successfully ===");
    println!("\nKey points demonstrated:");
    println!("  1. AutomationSession launches process and returns process_id");
    println!("  2. RuntimeSession stores process_id + main_element for UI operations");
    println!("  3. ClickTool works with UIElement from RuntimeSession");
    println!("  4. Session provides find_element() to locate child elements");
    println!("  5. Session can be closed to terminate the application");

    Ok(())
}
