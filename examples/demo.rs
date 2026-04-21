//! Smith-Windows Demo Example
//!
//! This is a comprehensive demonstration of all smith-windows features:
//! - Input: Cursor position, Ctrl+Hover element capture
//! - Inspect: Full hierarchy path and selector recording
//!
//! Usage: `cargo run --example demo`
//!
//! Press Ctrl+C to exit at any time.

use std::time::Duration;
use tracing_subscriber::EnvFilter;

use smith_windows::core::input::InputConfig;
use smith_windows::core::inspect::{InspectBackend, InspectConfig};
use smith_windows::core::selector::{RecordedSelector, SelectorStep};
use smith_windows::runtime::backends::windows::input::{
    get_cursor_position, get_element_under_ctrl_hotkey,
};
use smith_windows::runtime::backends::windows::inspect::InspectBackendWindows;

// ============================================================================
// SECTION 1: Input Module Demo
// ============================================================================

/// Demo 1: Get cursor position using WinAPI
async fn demo_get_cursor_position() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Demo 1: Get Cursor Position ===");

    let (x, y) = get_cursor_position()?;
    println!("Current cursor position: ({}, {})", x, y);
    println!("✓ Successfully got cursor position via WinAPI GetCursorPos");

    Ok(())
}

/// Demo 2: Capture element under cursor with Ctrl+Hover
async fn demo_ctrl_hover() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Demo 2: Ctrl+Hover Element Capture ===");

    let config = InputConfig {
        timeout: Duration::from_secs(10),
        cancellation: tokio_util::sync::CancellationToken::new(),
    };

    println!("Instructions:");
    println!("  1. Move cursor over any UI element");
    println!("  2. Press and hold Ctrl key");
    println!("  3. Element will be captured automatically");

    let element = get_element_under_ctrl_hotkey(&config).await?;

    println!("\nElement captured:");
    let step = SelectorStep::from_element(&element)?;
    println!("  Classname: {:?}", step.classname);
    println!("  Control type: {:?}", step.control_type);
    println!("  Name: {:?}", step.name);
    println!("  Automation ID: {:?}", step.automation_id);

    Ok(())
}

// ============================================================================
// SECTION 2: Inspect Module Demo
// ============================================================================

/// Demo 3: Build full hierarchy path
async fn demo_full_hierarchy() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Demo 3: Full Hierarchy Path ===");

    let automation = uiautomation::UIAutomation::new()?;
    let root = automation.get_root_element()?;

    // Get element under cursor for demo
    let element = get_element_under_ctrl_hotkey(&InputConfig {
        timeout: Duration::from_secs(10),
        cancellation: tokio_util::sync::CancellationToken::new(),
    })
    .await?;

    // Find the head window
    let window = find_ancestor_window(&root, &element).await?;

    // Build path
    let _inspect_config = InspectConfig {
        timeout: Duration::from_secs(5),
        cancellation: tokio_util::sync::CancellationToken::new(),
    };

    let inspect_backend = InspectBackendWindows::new();
    let path = inspect_backend.inspect_path(&window, &element).await?;

    println!("Full hierarchy path:");
    println!("  {}", path);

    Ok(())
}

/// Demo 4: Build recorded selector tree
async fn demo_recorded_selector() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Demo 4: Recorded Selector Tree ===");

    let automation = uiautomation::UIAutomation::new()?;

    // Get element under cursor for demo
    let element = get_element_under_ctrl_hotkey(&InputConfig {
        timeout: Duration::from_secs(10),
        cancellation: tokio_util::sync::CancellationToken::new(),
    })
    .await?;

    // Build selector tree
    let recorded = build_full_selector_tree(&automation, &element)?;

    println!("Recorded selector with {} steps:", recorded.depth);
    recorded.print_tree();

    if let Some(selector) = recorded.to_selector() {
        println!("\nFinal selector: {}", selector);
    }

    Ok(())
}

// ============================================================================
// SECTION 3: Click Module Demo
// ============================================================================

/// Demo 5: Click element (requires Notepad)
async fn demo_click_element() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Demo 5: Click Element (Notepad) ===");
    println!("Note: This demo requires Notepad to be installed.");

    // Open Notepad for demo
    std::process::Command::new("notepad.exe").spawn()?;
    tokio::time::sleep(Duration::from_secs(2)).await;

    // Find Notepad window and Edit control
    let automation = uiautomation::UIAutomation::new()?;
    let root = automation.get_root_element()?;

    let notepad_window = find_window_by_title(&root, "Блокнот").await?;
    let edit_element =
        find_control_by_type(&notepad_window, uiautomation::types::ControlType::Edit).await?;

    println!("Found Edit control, clicking...");

    // Click the element
    use smith_windows::core::click::ClickConfig;
    use smith_windows::runtime::backends::windows::click::ClickBackendWindows;

    let _click_config = ClickConfig {
        timeout: Duration::from_secs(5),
        cancellation: tokio_util::sync::CancellationToken::new(),
    };

    let click_backend = ClickBackendWindows::new();
    click_backend.click(&edit_element).await?;

    println!("✓ Clicked successfully!");

    // Close Notepad
    std::process::Command::new("taskkill")
        .args(["/F", "/IM", "notepad.exe"])
        .spawn()?;

    Ok(())
}

// ============================================================================
// SECTION 4: Type Module Demo
// ============================================================================

/// Demo 6: Type text (requires Notepad)
async fn demo_type_text() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Demo 6: Type Text (Notepad) ===");
    println!("Note: This demo requires Notepad to be installed.");

    // Open Notepad for demo
    std::process::Command::new("notepad.exe").spawn()?;
    tokio::time::sleep(Duration::from_secs(2)).await;

    // Find Notepad window and Edit control
    let automation = uiautomation::UIAutomation::new()?;
    let root = automation.get_root_element()?;

    let notepad_window = find_window_by_title(&root, "Блокнот").await?;
    let edit_element =
        find_control_by_type(&notepad_window, uiautomation::types::ControlType::Edit).await?;

    println!("Typing text into Edit control...");

    // Type text
    use smith_windows::core::r#type::TypeConfig;
    use smith_windows::TypeBackend;

    let _type_config = TypeConfig {
        timeout: Duration::from_secs(5),
        cancellation: tokio_util::sync::CancellationToken::new(),
    };

    let type_backend = smith_windows::runtime::backends::windows::r#type::TypeBackendWindows::new();
    type_backend
        .type_text(&edit_element, "Hello from smith-windows!")
        .await?;

    println!("✓ Text typed successfully!");

    // Wait before closing
    tokio::time::sleep(Duration::from_secs(2)).await;

    // Close Notepad
    std::process::Command::new("taskkill")
        .args(["/F", "/IM", "notepad.exe"])
        .spawn()?;

    Ok(())
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Finds the top-level window containing the given element
async fn find_ancestor_window(
    root: &uiautomation::UIElement,
    element: &uiautomation::UIElement,
) -> Result<uiautomation::UIElement, Box<dyn std::error::Error>> {
    let automation = uiautomation::UIAutomation::new()?;
    let walker = automation.create_tree_walker()?;

    let mut current = element.clone();

    loop {
        let is_same = automation.compare_elements(&current, root)?;
        if is_same {
            break;
        }
        let parent = walker.get_parent(&current)?;
        current = parent;
    }

    Ok(current)
}

/// Finds a window by title
async fn find_window_by_title(
    _root: &uiautomation::UIElement,
    _title: &str,
) -> Result<uiautomation::UIElement, Box<dyn std::error::Error>> {
    let automation = uiautomation::UIAutomation::new()?;

    let element = automation
        .create_matcher()
        .from(root.clone())
        .control_type(uiautomation::types::ControlType::Window)
        .timeout(2000)
        .find_first()?;

    Ok(element)
}

/// Finds a control by type within a parent element
async fn find_control_by_type(
    parent: &uiautomation::UIElement,
    control_type: uiautomation::types::ControlType,
) -> Result<uiautomation::UIElement, Box<dyn std::error::Error>> {
    let automation = uiautomation::UIAutomation::new()?;

    let element = automation
        .create_matcher()
        .from(parent.clone())
        .control_type(control_type)
        .timeout(2000)
        .find_first()?;

    Ok(element)
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
                break;
            }
        }
    }

    steps.reverse();

    Ok(RecordedSelector {
        steps: steps.clone(),
        depth: steps.len(),
    })
}

// ============================================================================
// MAIN
// ============================================================================

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║         Smith-Windows Demo - All Features Overview            ║");
    println!("╚════════════════════════════════════════════════════════════════╝");
    println!("\nThis demo showcases all smith-windows features:");
    println!("  • Input Module: Cursor position, Ctrl+Hover capture");
    println!("  • Inspect Module: Full hierarchy path, selector recording");
    println!("  • Click Module: Element clicking (requires Notepad)");
    println!("  • Type Module: Text input simulation (requires Notepad)");
    println!("\nPress Ctrl+C to exit at any time.");

    // Run all demos
    demo_get_cursor_position().await?;

    demo_ctrl_hover().await?;

    demo_full_hierarchy().await?;

    demo_recorded_selector().await?;

    // Try Click demo (requires Notepad)
    match demo_click_element().await {
        Ok(_) => println!("\n✓ Click demo completed successfully"),
        Err(e) => println!("\n⚠ Click demo skipped: {}", e),
    }

    // Try Type demo (requires Notepad)
    match demo_type_text().await {
        Ok(_) => println!("\n✓ Type demo completed successfully"),
        Err(e) => println!("\n⚠ Type demo skipped: {}", e),
    }

    println!("\n╔════════════════════════════════════════════════════════════════╗");
    println!("║                    All Demos Completed!                        ║");
    println!("╚════════════════════════════════════════════════════════════════╝");
    println!("\nNote: Some demos (Click, Type) require Notepad to be installed.");
    println!("      If you don't have Notepad, those demos will be skipped.");

    Ok(())
}
