//! Example: Right Click Tool Usage
//!
//! This example demonstrates how to use the RightClickTool for UI automation
//! on Windows via the uiautomation crate.
//!
//! Run this example:
//! ```bash
//! cargo run --example right_click_example
//! ```

use std::time::Duration;
use tracing_subscriber::EnvFilter;

use smith_windows::core::click::ClickType;
use smith_windows::core::right_click::RightClickConfig;
use smith_windows::runtime::backends::windows::right_click::RightClickBackendWindows;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    println!("=== RightClickTool Example - Starting ===");

    // Example 1: Configuration with RightClickConfig
    example_configuration().await?;

    // Example 2: Using RightClickBackendWindows for testing scenarios
    example_mock_backend().await?;

    // Example 3: Real right click on Windows Calculator (requires Calculator to be open)
    // Uncomment the line below to run real UI automation example
    // example_with_real_right_click().await?;

    println!("\n=== RightClickTool Example - Completed successfully ===");
    Ok(())
}

/// Example: Configuration with RightClickConfig
async fn example_configuration() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Example: Configuration with RightClickConfig ---");

    let cancellation = tokio_util::sync::CancellationToken::new();

    // Valid configuration
    let config = RightClickConfig {
        timeout: Duration::from_secs(5),
        cancellation,
    };

    // RightClickConfig wraps ClickConfig with click_type set to RightSingle
    println!("✓ RightClickConfig created successfully");
    println!("  Note: RightClickConfig automatically uses ClickType::RightSingle");

    Ok(())
}

/// Example: Using RightClickBackendWindows for testing
async fn example_mock_backend() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Example: RightClickBackendWindows ---");

    let backend = RightClickBackendWindows::new();

    // Test 1: Backend creation
    println!("✓ RightClickBackendWindows created successfully");

    // Note: Backend methods are async and require real UI elements
    // For unit testing without UI dependencies, use MockRightClickBackend in tests

    Ok(())
}

/// Example: Real right click on Windows Calculator
/// This example opens Calculator and performs a right click on a button
/// Before running, make sure Calculator is open
#[allow(dead_code)]
async fn example_with_real_right_click() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Example: Real right click on Windows Calculator ---");

    let cancellation = tokio_util::sync::CancellationToken::new();
    let config = RightClickConfig {
        timeout: Duration::from_secs(10),
        cancellation,
    };

    println!("Starting Windows Calculator...");
    let ui_automation = uiautomation::UIAutomation::new().map_err(|e| {
        println!("Failed to create UIAutomation: {}", e);
        e
    })?;

    println!("Launching Calculator...");
    let _process = std::process::Command::new("calc.exe")
        .spawn()
        .map_err(|e| {
            println!("Failed to start Calculator: {}", e);
            e
        })?;

    println!("Waiting for Calculator to start...");
    tokio::time::sleep(Duration::from_secs(3)).await;

    println!("Getting root element...");
    let root_element = ui_automation.get_root_element().map_err(|e| {
        println!("Failed to get root element: {}", e);
        e
    })?;

    println!("Searching for Calculator window...");
    let calculator = ui_automation
        .create_matcher()
        .from(root_element.clone())
        .control_type(uiautomation::types::ControlType::Window)
        .timeout(10000)
        .find_first()
        .map_err(|e| {
            println!("Failed to find Calculator: {}", e);
            println!("Please open Calculator manually and try again");
            e
        })?;

    println!("Searching for buttons in Calculator...");
    let buttons = ui_automation
        .create_matcher()
        .from(calculator.clone())
        .control_type(uiautomation::types::ControlType::Button)
        .timeout(5000)
        .find_all()
        .map_err(|e| {
            println!("Failed to find buttons: {}", e);
            e
        })?;

    println!("Found {} buttons", buttons.len());

    if buttons.is_empty() {
        println!("No buttons found in Calculator");
        return Err("No buttons found".into());
    }

    let first_button = &buttons[0];
    let name = first_button.get_name().unwrap_or_default();
    println!("Right clicking button: {}", name);

    // RightClickBackendWindows uses ClickType::RightSingle internally
    let backend = RightClickBackendWindows::new();

    match backend.right_click(first_button).await {
        Ok(()) => {
            println!("✓ Right click successful! Check Calculator for context menu");
        }
        Err(e) => {
            println!("✗ Right click failed: {}", e);
            return Err(e.into());
        }
    }

    println!("Waiting 2 seconds to see the result...");
    tokio::time::sleep(Duration::from_secs(2)).await;

    println!("Example completed successfully!");
    Ok(())
}
