# ADR-013: FocusTool Implementation

**Date:** 2026-04-22

**Status:** ✅ Documentation Created (Implementation Pending)

## Context

We need to implement a FocusTool to solve a critical problem in UI automation:

**Problem:** When a script starts, the target window may be in the background. InputTextTool would type into the active window (where the cursor is), not the target window where we want to interact.

**Example scenario:**
1. Script launches `notepad.exe` in background
2. User's cursor is in a different application (browser, IDE)
3. `input_text()` would type into the browser/IDE instead of Notepad
4. Result: Wrong application receives the input

**Desired solution:**
- Before any interaction, activate the target window
- Ensure the window is in foreground with focus
- Then perform text input, click, or other operations

## Decision

We will implement FocusTool following the project's architecture:

1. **Separation of concerns:**
   - `src/core/focus.rs`: Types, traits, validation, mocks
   - `src/runtime/backends/windows/focus.rs`: Windows-specific implementation
   - `src/runtime/backends/unsupported.rs`: Stub for non-Windows platforms

2. **Window activation approach:**
   - Get `UIElement` representing the target
   - Access its parent window via `WindowPattern`
   - Call `wait_for_input_idle()` to ensure window is ready
   - Activate the window to bring it to foreground

3. **Error handling:**
   - Use `thiserror` for error types
   - Explicit `Result<(), FocusError>` return values
   - No `unwrap()`, `panic!`, or `expect()` in library code

4. **Async/await with COM safety:**
   - **CRITICAL**: `UIElement`, `UIAutomation`, `UIPattern` are `!Send`/`!Sync`
   - NO `spawn_blocking` for UIA calls - they must run on the same thread
   - Timeout handling via `tokio::time::timeout`
   - Cancellation via `tokio_util::sync::CancellationToken`

5. **Validation:**
   - Config validation (`validate_focus_config()`) runs BEFORE backend calls
   - Element validation (is_enabled, is_offscreen) runs inside backend
   - Element must have WindowPattern available

## Error Types

```
FocusError:
- ElementNotFound: Element is null or invalid
- ElementNotEnabled: Element exists but is disabled
- ElementOffscreen: Element is not visible on screen
- WindowPatternNotAvailable: Element lacks WindowPattern
- Timeout: Operation exceeded timeout
- Cancelled: Operation was cancelled via token
- InvalidConfig: Configuration validation failed
- ComError: COM-related errors with details
- UnsupportedPlatform: Running on non-Windows
```

## Configuration

```rust
struct FocusConfig {
    timeout: Duration,        // > 0 and <= 1 hour
    cancellation: CancellationToken,
}
```

## Validation Rules

- `timeout > Duration::ZERO` (must be positive)
- `timeout <= Duration::from_secs(3600)` (max 1 hour)
- Element must be valid UIA element
- Element must be enabled (via `is_enabled()`)
- Element must be on screen (via `is_offscreen()`)
- Element must have WindowPattern available

## Testing Strategy

### Unit Tests (src/core/focus.rs)
- `test_validate_focus_config_valid`: Valid config passes
- `test_validate_focus_config_zero_timeout`: Zero timeout fails
- `test_validate_focus_config_large_timeout`: >1h timeout fails
- `test_mock_backend_creation`: Mock backend creation
- `test_mock_backend_with_state`: Mock with custom state
- `test_mock_backend_reset`: Mock backend reset
- `test_validate_element_ready_enabled`: Enabled element passes
- `test_validate_element_ready_disabled`: Disabled element fails
- `test_validate_element_ready_offscreen`: Offscreen element fails
- `test_validate_window_pattern_available`: Element with WindowPattern passes

### Mock Tests (src/core/focus.rs)
- `test_mock_backend_creation`: Mock backend creation
- `test_mock_backend_with_state`: Mock with custom state
- `test_mock_backend_reset`: Mock backend reset
- `test_mock_backend_idempotency_on_error`: Repeated errors don't change state

## Trade-offs

1. **No spawn_blocking**: All UIA calls run on the same thread due to `!Send`/`!Sync` constraint. This means the async runtime may be blocked during UIA calls.

2. **WindowPattern requirement**: We require WindowPattern to be available, which may not exist for some elements (e.g., child controls without windows).

3. **wait_for_input_idle**: We call this to ensure the window is ready, but it may block if the window is not responding.

4. **Mock state**: Mock uses `Arc<Mutex<MockFocusState>>` for thread-safe testing.

5. **Idempotent operation**: Calling focus on an already-active window returns success (no error).

## Validation

- [x] Documentation created (`docs/design/focus-tool/`)
- [ ] `cargo test focus` passes (10/10 unit tests) - pending implementation
- [ ] `cargo clippy -- -D warnings` passes - pending implementation
- [ ] No `unwrap()`/`panic!` in `src/` - pending implementation
- [ ] All error cases tested - pending implementation
- [ ] Idempotency verified (mock backend) - pending implementation
- [ ] `cargo check` compiles without errors - pending implementation

## Integration Examples

### Example 1: Basic focus configuration

```rust
use std::time::Duration;
use smith_windows::{launch_process, FocusConfig, FocusTool};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = SessionConfig::new("notepad.exe");
    let session = launch_process(config).await?;

    // Focus the target window before interaction
    let element = session.find_by_automation_id("target_element").await?;
    let focus_config = FocusConfig {
        timeout: Duration::from_secs(5),
        cancellation: CancellationToken::new(),
    };
    session.focus().focus_element(&element, &focus_config).await?;

    // Now input text or click safely
    session.input_text().type_text("Hello World").await_by_automation_id("input_field").await?;

    Ok(())
}
```

### Example 2: Focus before read operation

```rust
let element = session.find_by_name("Output").await?;
let focus_config = FocusConfig::new(Duration::from_secs(5));
session.focus().focus_element(&element, &focus_config).await?;

let text = session.read().await_by_name("Output").await?;
println!("Output: {}", text);
```

### Example 3: Focus before click operation

```rust
let button = session.find_by_automation_id("save_button").await?;
let focus_config = FocusConfig::new(Duration::from_secs(5));
session.focus().focus_element(&button, &focus_config).await?;

session.click().await_by_automation_id("save_button").await?;
```

## Implementation Notes

### Window Pattern Access

```rust
let window_pattern = element.get_pattern::<uiautomation::patterns::UIWindowPattern>()?;
// Wait for the window to be ready
window_pattern.wait_for_input_idle(5000)?;  // 5 second timeout
// Activate the window
window_pattern.set_window_visual_state(WindowVisualState::Normal)?;
window_pattern.focus()?;
```

### COM Safety

- All UIA calls run on the same thread that created the `UIAutomation` instance
- No `spawn_blocking` - UIElement is `!Send`/`!Sync`
- Async traits use `#[async_trait::async_trait(?Send)]`

## Related Decisions

- Architecture: UIAutomation-first, all UI operations through uiautomation crate
- Error handling: Explicit Result types, no silent failures
- Testing: Unit + Mock tests, idempotency verified
- COM Safety: No spawn_blocking for `!Send`/`!Sync` types, same-thread execution

## Files

- `src/core/focus.rs`: Core module with types, traits, validation, mocks (pending implementation)
- `src/runtime/backends/windows/focus.rs`: Windows implementation (pending implementation)
- `src/runtime/backends/unsupported.rs`: Non-Windows stub (pending implementation)
- `src/core/mod.rs`: Module exports (pending update)
- `src/runtime/backends/windows/mod.rs`: Backend exports (pending update)
- `src/runtime/backends/mod.rs`: Re-exports (pending update)
- `docs/design/focus-tool/`: Documentation (spec, contract, test-plan, brief)
- `docs/adr/013-focus-tool.md`: This ADR

## Usage in Workflow

FocusTool is designed to be used BEFORE any interaction tool:

1. **Find element**: `session.find_by_automation_id("element").await?`
2. **Focus window**: `session.focus().focus_element(&element, &config).await?`
3. **Interact**: `session.input_text().type_text("text").await?` or `session.click().await?`

This ensures the target window is foreground and will receive the input.

## Alternative Approaches Considered

1. **Auto-focus in every tool**: Add focus call internally to InputTextTool, ClickTool, etc.
   - **Rejected**: Too implicit, user may not want focus in some cases

2. **Global focus setting**: Set focus once per session
   - **Rejected**: Different elements may be in different windows

3. **Window selector**: Focus by window selector instead of element
   - **Rejected**: Less flexible, user may only have element reference

4. **Foreground check + focus only if needed**: Check if window is foreground first
   - **Rejected**: Extra COM call overhead, focus is idempotent anyway

## Conclusion

FocusTool provides an explicit, flexible way to ensure the target window is foreground before interaction. This solves the "window in background" problem while maintaining the project's architecture principles.
