//! Example: Click Tool Usage
//!
//! This example demonstrates how to use the ClickTool for UI automation
//! on Windows via the uiautomation crate.
//!
//! Run this example:
//! ```bash
//! cargo run --example click_example
//! ```

use std::time::Duration;
use tracing_subscriber::EnvFilter;

use smith_windows::core::click::{ClickConfig, ClickError, validate_click_config, MockClickBackend};
use smith_windows::runtime::backends::windows::click::ClickBackendWindows;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    println!("=== ClickTool Example - Starting ===");

    // Example 1: Configuration validation
    example_configuration().await?;

    // Example 2: Using MockClickBackend for testing scenarios
    example_mock_backend().await?;

    // Example 3: Real click on Windows Calculator (requires Calculator to be open)
    // Uncomment the line below to run real UI automation example
    // example_with_real_click().await?;

    println!("\n=== ClickTool Example - Completed successfully ===");
    Ok(())
}

/// Example: Configuration validation
async fn example_configuration() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Example: Configuration validation ---");

    let cancellation = tokio_util::sync::CancellationToken::new();

    // Valid configuration
    let config = ClickConfig {
        timeout: Duration::from_secs(5),
        cancellation,
    };

    match validate_click_config(&config) {
        Ok(()) => println!("✓ Valid configuration accepted"),
        Err(e) => println!("✗ Configuration error: {}", e),
    }

    // Invalid configuration - zero timeout
    let config_invalid = ClickConfig {
        timeout: Duration::ZERO,
        cancellation: tokio_util::sync::CancellationToken::new(),
    };

    match validate_click_config(&config_invalid) {
        Ok(()) => println!("✗ Zero timeout should be rejected"),
        Err(ClickError::InvalidConfig(msg)) => println!("✓ Zero timeout correctly rejected: {}", msg),
        Err(e) => println!("✗ Unexpected error: {}", e),
    }

    Ok(())
}

/// Example: Using MockClickBackend for testing
async fn example_mock_backend() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Example: MockClickBackend ---");

    let backend = MockClickBackend::new();

    // Test 1: Success scenario
    {
        let mut state = backend.get_state();
        state.should_succeed = true;
        state.call_count = 0;
    }

    println!("Mock backend configured for success");

    // Note: Mock backend doesn't need real UI elements
    // It's useful for unit testing without UI dependencies

    backend.reset();
    println!("Mock backend reset");

    // Test 2: Failure scenario
    {
        let mut state = backend.get_state();
        state.should_succeed = false;
        state.last_error = Some(ClickError::ElementNotEnabled);
    }

    println!("Mock backend configured for failure scenario");
    println!("This demonstrates how to test error handling without real UI");

    Ok(())
}

/// Example: Real click on Windows Calculator
/// This example opens Calculator and clicks a button
/// Before running, make sure Calculator is open
#[allow(dead_code)]
async fn example_with_real_click() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Example: Real click on Windows Calculator ---");

    let cancellation = tokio_util::sync::CancellationToken::new();
    let config = ClickConfig {
        timeout: Duration::from_secs(10),
        cancellation,
    };

    match validate_click_config(&config) {
        Ok(()) => println!("Configuration is valid"),
        Err(e) => {
            println!("Configuration error: {}", e);
            return Err(e.into());
        }
    }

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
    let calculator = ui_automation.create_matcher()
        .from(root_element.clone())
        .control_type(uiautomation::types::ControlType::Window)
        .timeout(10000)
        .find_first().map_err(|e| {
            println!("Failed to find Calculator: {}", e);
            println!("Please open Calculator manually and try again");
            e
        })?;

    println!("Searching for buttons in Calculator...");
    let buttons = ui_automation.create_matcher()
        .from(calculator.clone())
        .control_type(uiautomation::types::ControlType::Button)
        .timeout(5000)
        .find_all().map_err(|e| {
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
    println!("Clicking button: {}", name);

    let backend = ClickBackendWindows::new();

    match backend.click(first_button).await {
        Ok(()) => {
            println!("Click successful! Check Calculator for result");
        }
        Err(e) => {
            println!("Click failed: {}", e);
            return Err(e.into());
        }
    }

    println!("Waiting 2 seconds to see the result...");
    tokio::time::sleep(Duration::from_secs(2)).await;

    println!("Example completed successfully!");
    Ok(())
}
