# Examples for smith-windows

This directory contains examples demonstrating how to use the `smith-windows` library.

## Available Examples

### `click_example.rs`
Basic ClickTool demonstration:
- **Configuration validation** — tests `ClickConfig` and `validate_click_config()`
- **MockClickBackend** — testing scenarios without real UI elements
- **Real UI automation** (commented out) — click on Windows Calculator

### `notepad_click.rs`
Advanced Notepad automation:
- **Launch Notepad** — opens notepad.exe
- **Find menu items** — locates menu elements via UI Automation
- **Click menu item** — clicks the "File" (Файл) menu item
- **Show results** — visual demonstration of automation

## Running Examples

```bash
# Run click example
cargo run --example click_example

# Run notepad example
cargo run --example notepad_click

# Build only
cargo build --example <name>

# Run with clippy checks
cargo clippy --example <name> -- -D warnings
```

## Prerequisites

- Windows 10/11 (required for UI Automation)
- Rust 1.95+

## Example Output

### click_example
```
=== ClickTool Example - Starting ===

--- Example: Configuration validation ---
✓ Valid configuration accepted
✓ Zero timeout correctly rejected: timeout must be > 0 and <= 1 hour

--- Example: MockClickBackend ---
Mock backend configured for success
Mock backend reset
Mock backend configured for failure scenario
This demonstrates how to test error handling without real UI

=== ClickTool Example - Completed successfully ===
```

### notepad_click
```
=== Notepad Click Example - Starting ===

--- Example 1: Configuration validation ---
✓ Valid configuration accepted
✓ Zero timeout correctly rejected: timeout must be > 0 and <= 1 hour

--- Example 2: MockClickBackend ---
Mock backend configured for success
Mock backend reset
Mock backend configured for failure scenario
This demonstrates how to test error handling without real UI

--- Example 3: Click on Notepad menu ---
This example will:
  1. Open Notepad
  2. Find and click the 'File' menu item
  3. Close Notepad
Configuration is valid

Step 1: Opening Notepad...
Launching notepad.exe...
Waiting for Notepad to start...
Creating UIAutomation...
Getting root element...

Step 2: Finding Notepad window...
Found window: Безымянный – Блокнот
✓ Found Notepad window

Step 3: Finding menu items in Notepad...
Found 14 menu items
Menu items found directly
✓ Found 'File' menu item: 'Файл'
Clicking 'File' menu...
✓ Click successful! 'File' menu should be open now

Waiting 3 seconds to see the result...

Step 4: Closing Notepad...
Sending Escape to close menu...
Example completed successfully!

=== Notepad Click Example - Completed successfully ===
```

## Notes

- Examples 1 and 2 run without requiring any UI elements
- Examples 3 requires Windows Notepad/Calculator to be accessible via UI Automation
- All examples follow the project's error handling and configuration patterns
