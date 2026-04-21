//! InspectTool interactive example: Get any element under cursor with Ctrl
//!
//! This example demonstrates the full "Ctrl+Hover" functionality:
//! 1. Wait for user to physically press Ctrl key while hovering over an element
//! 2. Capture the element under the cursor
//! 3. Build the full hierarchy path from window to element
//! 4. Display the recorded selector
//!
//! Usage:
//! 1. Run the example
//! 2. Move cursor over any UI element (button, text, etc.)
//! 3. Press and hold Ctrl key - element will be captured
//! 4. Element path and selector will be displayed
//! 5. Press Ctrl+C to exit

use std::time::Duration;
use tracing_subscriber::EnvFilter;

use smith_windows::core::input::InputConfig;
use smith_windows::core::inspect::{InspectBackend, InspectConfig};
use smith_windows::core::selector::{RecordedSelector, SelectorStep};
use smith_windows::runtime::backends::windows::input::get_element_under_ctrl_hotkey;
use smith_windows::runtime::backends::windows::inspect::InspectBackendWindows;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    println!("=== InspectTool Interactive Example (Ctrl+Hover) ===\n");
    println!("Instructions:");
    println!("1. Move cursor over any UI element (button, text, etc.)");
    println!("2. Press and hold Ctrl key - element will be captured");
    println!("3. Element path and selector will be displayed");
    println!("4. Press Ctrl+C to exit\n");

    let config = InputConfig {
        timeout: Duration::from_secs(30),
        cancellation: tokio_util::sync::CancellationToken::new(),
    };

    println!("Waiting for Ctrl+Hover interaction...\n");

    // Wait for real Ctrl+Hover (user physically presses Ctrl)
    match get_element_under_ctrl_hotkey(&config).await {
        Ok(element) => {
            println!("\n=== Element Captured ===");

            // Get element properties using SelectorStep
            let step = SelectorStep::from_element(&element)?;
            println!("Element properties:");
            step.print();

            let control_type = match element.get_control_type() {
                Ok(t) => t,
                Err(_) => uiautomation::types::ControlType::Custom,
            };
            println!("\nElement details:");
            println!("  Control type: {}", control_type);
            println!("  Name: {}", element.get_name().unwrap_or_default());
            println!("  Is enabled: {}", element.is_enabled().unwrap_or(false));
            println!(
                "  Is offscreen: {}",
                element.is_offscreen().unwrap_or(false)
            );

            // Get the element's window ancestor (head_window)
            let automation = uiautomation::UIAutomation::new()?;
            let root = automation.get_root_element()?;

            // Find the top-level window containing this element
            let window = find_ancestor_window(&root, &element).await?;

            let _inspect_config = InspectConfig {
                timeout: Duration::from_secs(5),
                cancellation: tokio_util::sync::CancellationToken::new(),
            };

            let inspect_backend = InspectBackendWindows::new();
            let path = inspect_backend.inspect_path(&window, &element).await?;

            // Build full recorded selector tree
            let recorded = build_full_selector_tree(&automation, &element)?;

            println!("\n=== Full Hierarchy Path ===");
            println!("{}", path);
            println!("\nPath format: Window{{title}}-> ControlType{{Name}}-> Element");

            println!("\n=== Recorded Selector Tree ===");
            recorded.print_tree();

            if let Some(selector) = recorded.to_selector() {
                println!("\n=== Final Selector ===");
                println!("{}", selector);
            }

            println!("\nThis path was built using:");
            println!("  1. GetCursorPos() - get cursor coordinates");
            println!("  2. UIAutomation::element_from_point() - get element at coordinates");
            println!("  3. TreeWalker::get_parent() - traverse to head window");
            println!("  4. Path reversed to show Window -> Element order");
            println!("  5. Selector steps from root to element with properties");
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

    loop {
        // Compare elements using automation.compare_elements since UIElement doesn't implement PartialEq
        let is_same = automation.compare_elements(&current, root)?;
        if is_same {
            break;
        }
        let parent = walker.get_parent(&current)?;
        current = parent;
    }

    Ok(current)
}

/// Builds the full selector tree from root to element
fn build_full_selector_tree(
    automation: &uiautomation::UIAutomation,
    element: &uiautomation::UIElement,
) -> Result<RecordedSelector, Box<dyn std::error::Error>> {
    let mut steps = Vec::new();
    let mut current = element.clone();

    let walker = automation.create_tree_walker()?;

    const MAX_DEPTH: usize = 100;

    // Walk up the tree from element to root
    loop {
        if steps.len() >= MAX_DEPTH {
            return Err("Selector tree exceeds maximum depth of 100".into());
        }

        let step = SelectorStep::from_element(&current)?;
        steps.push(step);

        match walker.get_parent(&current) {
            Ok(parent) => {
                current = parent;
            }
            Err(_) => {
                // Reached root
                break;
            }
        }
    }

    // Reverse to go from root to element
    steps.reverse();

    let depth = steps.len();

    Ok(RecordedSelector { steps, depth })
}
