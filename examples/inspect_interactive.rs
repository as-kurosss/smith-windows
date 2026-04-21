//! InspectTool interactive example: Get any element under cursor with Ctrl
//!
//! This example demonstrates the full "Ctrl+Hover" functionality:
//! 1. Press and hold Ctrl key
//! 2. Move cursor over any UI element (button, text, etc.)
//! 3. Release Ctrl - the element path will be displayed
//!
//! Usage:
//! 1. Run the example
//! 2. Press and hold Ctrl key
//! 3. Move cursor over any UI element
//! 4. Release Ctrl - element path will be displayed
//! 5. Press Ctrl+C to exit

use std::time::Duration;
use tracing_subscriber::EnvFilter;

use smith_windows::core::input::{get_element_under_cursor, InputConfig, InputError};
use smith_windows::core::inspect::{InspectBackend, InspectConfig, InspectError};
use smith_windows::runtime::backends::windows::input::InputBackendWindows;
use smith_windows::runtime::backends::windows::inspect::InspectBackendWindows;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    println!("=== InspectTool Interactive Example (Ctrl+Hover) ===\n");
    println!("Instructions:");
    println!("1. Press and hold Ctrl key");
    println!("2. Move cursor over any UI element (button, text, etc.)");
    println!("3. Release Ctrl - element path will be displayed");
    println!("4. Press Ctrl+C to exit\n");

    let config = InputConfig {
        timeout: Duration::from_secs(10),
        cancellation: tokio_util::sync::CancellationToken::new(),
    };

    println!("Waiting for Ctrl+Hover interaction...\n");

    // Simulate Ctrl+Hover
    match get_element_with_ctrl_simulation(&config).await {
        Ok(element) => {
            println!("\n=== Element Found ===");
            println!(
                "Control type: {}",
                element.get_control_type().unwrap_or_default()
            );
            println!("Name: {}", element.get_name().unwrap_or_default());
            println!("Is enabled: {}", element.is_enabled().unwrap_or(false));
            println!("Is offscreen: {}", element.is_offscreen().unwrap_or(false));

            // Get the element's window ancestor (head_window)
            let automation = uiautomation::UIAutomation::new()?;
            let root = automation.get_root_element()?;

            // Find the top-level window containing this element
            let window = find_ancestor_window(&root, &element).await?;

            let inspect_config = InspectConfig {
                timeout: Duration::from_secs(5),
                cancellation: tokio_util::sync::CancellationToken::new(),
            };

            let inspect_backend = InspectBackendWindows::new();
            let path = inspect_backend.inspect_path(&window, &element).await?;

            println!("\n=== Full Hierarchy Path ===");
            println!("{}", path);
            println!("\nPath format: Window{{title}}-> ControlType{{Name}}-> Element");
            println!("\nThis path was built using:");
            println!("  1. GetCursorPos() - get cursor coordinates");
            println!("  2. UIAutomation::element_from_point() - get element at coordinates");
            println!("  3. UITreeWalker::get_parent() - traverse to head window");
            println!("  4. Path reversed to show Window -> Element order");
        }
        Err(e) => {
            println!("Failed to get element: {}", e);
        }
    }

    Ok(())
}

/// Finds the top-level window containing the given element
async fn find_ancestor_window(
    root: &uiautomation::UIElement,
    element: &uiautomation::UIElement,
) -> Result<uiautomation::UIElement, Box<dyn std::error::Error>> {
    // Use tree walker to traverse up to the root window
    let automation = uiautomation::UIAutomation::new()?;
    let walker = automation.create_tree_walker()?;

    let mut current = element.clone();

    while current != *root {
        let parent = walker.get_parent(&current)?;
        current = parent;
    }

    Ok(current)
}

/// Gets the element under cursor with Ctrl key simulation
async fn get_element_with_ctrl_simulation(
    config: &InputConfig,
) -> Result<uiautomation::UIElement, InputError> {
    let backend = InputBackendWindows::new();

    // Press Control key
    println!("Pressing Control key...");
    backend.click_key("{CTRL}").await?;

    // Small delay to ensure key is pressed
    tokio::time::sleep(Duration::from_millis(50)).await;

    // Get element at cursor position
    println!("Getting element under cursor...");
    let element = get_element_under_cursor().await?;

    println!("Element found: {:?}", element.get_name().ok());

    // Release Control key
    println!("Releasing Control key...");
    backend.click_key("{CTRL}").await?;

    Ok(element)
}
