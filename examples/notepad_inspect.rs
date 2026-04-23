//! InspectTool example: Inspect a Notepad element and print its full path
//!
//! This example:
//! 1. Creates a UI Automation session
//! 2. Finds the Notepad window
//! 3. Finds a target element (Edit control)
//! 4. Uses InspectTool to get the full hierarchy path
//! 5. Prints the path in format: "Window{Notepad}-> Pane-> Edit"

use std::time::Duration;
use tracing_subscriber::EnvFilter;

use smith_windows::core::inspect::{InspectBackend, InspectConfig};
use smith_windows::runtime::backends::windows::inspect::InspectBackendWindows;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing subscriber for logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    println!("=== InspectTool Example ===\n");

    // Example 1: Configuration validation
    example_configuration().await?;

    // Example 2: Using MockInspectBackend
    example_mock_backend().await?;

    // Example 3: Inspect Notepad element
    example_notepad_inspect().await?;

    println!("\n=== InspectTool Example - Completed successfully ===");
    Ok(())
}

/// Example: Configuration validation
async fn example_configuration() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Example 1: Configuration validation ---");

    let cancellation = tokio_util::sync::CancellationToken::new();

    // Valid configuration
    let config = InspectConfig {
        timeout: Duration::from_secs(5),
        cancellation,
    };

    match smith_windows::core::inspect::validate_inspect_config(&config) {
        Ok(()) => println!("✓ Valid configuration accepted"),
        Err(e) => println!("✗ Configuration error: {}", e),
    }

    // Invalid configuration - zero timeout
    let config_invalid = InspectConfig {
        timeout: Duration::ZERO,
        cancellation: tokio_util::sync::CancellationToken::new(),
    };

    match smith_windows::core::inspect::validate_inspect_config(&config_invalid) {
        Ok(()) => println!("✗ Zero timeout should be rejected"),
        Err(smith_windows::core::inspect::InspectError::InvalidConfig(msg)) => {
            println!("✓ Zero timeout correctly rejected: {}", msg)
        }
        Err(e) => println!("✗ Unexpected error: {}", e),
    }

    Ok(())
}

/// Example: Using MockInspectBackend for testing
async fn example_mock_backend() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Example 2: MockInspectBackend ---");

    let backend = smith_windows::core::inspect::MockInspectBackend::new();

    // Test 1: Success scenario
    {
        let mut state = backend.get_state().unwrap();
        state.should_succeed = true;
        state.path = "Window->Button->CheckBox{Name}".to_string();
        state.call_count = 0;
    }

    println!("Mock backend configured for success");
    println!("This demonstrates how to test without real UI elements");

    // Test 2: Failure scenario
    {
        let mut state = backend.get_state().unwrap();
        state.should_succeed = false;
        state.last_error = Some(smith_windows::core::inspect::InspectError::ElementNotEnabled);
    }

    println!("Mock backend configured for failure scenario");
    println!("This demonstrates how to test error handling without real UI");

    Ok(())
}

/// Example: Inspect Notepad element
async fn example_notepad_inspect() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Example 3: Inspect Notepad element ---");
    println!("This example will:");
    println!("  1. Open Notepad");
    println!("  2. Find the Notepad window");
    println!("  3. Find the Edit control inside Notepad");
    println!("  4. Build the full hierarchy path from window to edit");
    println!("  5. Print the path: Window{{Notepad}}-> Pane-> Edit");

    let cancellation = tokio_util::sync::CancellationToken::new();
    let config = InspectConfig {
        timeout: Duration::from_secs(10),
        cancellation,
    };

    match smith_windows::core::inspect::validate_inspect_config(&config) {
        Ok(()) => println!("Configuration is valid"),
        Err(e) => {
            println!("Configuration error: {}", e);
            return Err(e.into());
        }
    }

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

        tokio::time::sleep(Duration::from_secs(1)).await;
    };

    // Check Notepad properties
    let notepad_enabled = notepad.is_enabled().unwrap_or(false);
    let notepad_offscreen = notepad.is_offscreen().unwrap_or(false);
    println!("Notepad properties:");
    println!("  Is enabled: {}", notepad_enabled);
    println!("  Is offscreen: {}", notepad_offscreen);

    // Search for Edit control inside Notepad
    println!("\nStep 3: Finding Edit control inside Notepad...");
    let edit_element = loop {
        match ui_automation
            .create_matcher()
            .from(notepad.clone())
            .control_type(uiautomation::types::ControlType::Edit)
            .timeout(2000)
            .find_first()
        {
            Ok(el) => {
                println!("✓ Found Edit control");
                break el;
            }
            Err(e) => {
                println!("Search attempt failed: {}", e);
            }
        }

        tokio::time::sleep(Duration::from_secs(1)).await;
    };

    // Check Edit element properties
    let edit_enabled = edit_element.is_enabled().unwrap_or(false);
    let edit_offscreen = edit_element.is_offscreen().unwrap_or(false);
    let edit_control_type = edit_element
        .get_control_type()
        .unwrap_or(uiautomation::types::ControlType::Custom);
    let edit_name = edit_element.get_name().unwrap_or_default();

    println!("Edit control properties:");
    println!("  Control type: {}", edit_control_type);
    println!("  Name: {}", edit_name);
    println!("  Is enabled: {}", edit_enabled);
    println!("  Is offscreen: {}", edit_offscreen);

    println!("\n=== Building full hierarchy path ===");
    println!("Using InspectTool to build path from Notepad window to Edit control");
    let notepad_name = notepad.get_name().unwrap_or_default();
    println!(
        "Path format: Window{{{}}}-> ControlType{{Name}}-> Element\n",
        notepad_name
    );

    // Perform inspect operation using the backend directly
    let backend = InspectBackendWindows::new();

    match backend.inspect_path(&notepad, &edit_element).await {
        Ok(path) => {
            println!("✓ Inspect operation completed successfully!");
            println!("\n=== Full Hierarchy Path ===");
            println!("{}", path);
            println!("\nPath format explanation:");
            println!("  - Window{{Notepad}}: Head window (start of path)");
            println!("  - ControlType{{Name}}: Intermediate elements");
            println!("  - Edit: Target element (end of path)");
            println!("\nThis path was built using:");
            println!("  1. UIAutomation::create_tree_walker()");
            println!("  2. UITreeWalker::get_parent() to traverse from Edit to Window");
            println!("  3. UIAutomation::compare_elements() to find the root");
            println!("  4. Path reversed to show Window -> Edit order");
        }
        Err(e) => {
            println!("✗ Inspect failed: {}", e);
            return Err(e.into());
        }
    }

    // Wait to see the result
    println!("\nWaiting 3 seconds before closing Notepad...");
    tokio::time::sleep(Duration::from_secs(3)).await;

    println!("\nStep 4: Closing Notepad...");
    println!("Example completed successfully!");

    Ok(())
}
