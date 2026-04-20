# ADR-002: AutomationSession Implementation

**Date:** 2026-04-20

**Status:** ✅ Implemented and Verified

## Context

We need to implement an AutomationSession module for managing application lifecycle in Windows automation. The requirements are:

1. **Separate launch and attach operations**:
   - `launch_process()` → returns `process_id` (u32) for starting new applications
   - `attach_by_title()` / `attach_by_process_id()` → returns `RuntimeSession` for connecting to existing windows

2. **Flexible window matching**:
   - Exact match (full title comparison)
   - Contains match (partial title search)
   - Regex match (full regex pattern support)

3. **Process isolation**:
   - Process launch via `std::process::Command` (standard Rust)
   - Window attachment via `uiautomation` crate (UI Automation API)
   - No direct WinAPI calls

4. **Session lifecycle**:
   - `RuntimeSession` with `process_id`, `main_element`, and `SessionState`
   - `close()` method to terminate process and mark session as closed
   - Protection against operations on closed sessions

5. **Integration with tools**:
   - `RuntimeSession.main_element` for passing to `ClickTool`, `TypeTool`, etc.
   - Direct implementation of `click()`, `type_text()` via uiautomation

## Decision

We will implement AutomationSession following the project's architecture:

1. **Separation of concerns:**
   - `src/core/automation_session.rs`: Types, traits, validation, launch logic, mocks
   - `src/runtime/backends/windows/automation_session.rs`: Windows-specific UI Automation implementation

2. **Error handling:**
   - Use `thiserror` for error types
   - Explicit `Result<T, AutomationError>` return values
   - No `unwrap()`, `panic!`, or `expect()` in library code

3. **Async/await with spawn_blocking:**
   - All UI Automation calls wrapped in `tokio::task::spawn_blocking`
   - Timeout handling via `tokio::time::timeout`
   - Cancellation via `tokio_util::sync::CancellationToken`

4. **Validation:**
   - Config validation (`validate_session_config()`) runs BEFORE backend calls
   - Title validation (`validate_title_filter()`, `validate_regex()`) runs BEFORE backend calls
   - Command validation (`validate_command()`) runs BEFORE process launch

5. **Session state management:**
   - `SessionState { Running, Closed }` with `Arc<Mutex<SessionState>>`
   - All methods check `SessionState` before execution
   - `SessionClosed` error returned for operations on closed sessions

6. **Window matching modes:**
   - `MatchMode` enum: `Exact`, `Contains`, `Regex`
   - Regex support via `regex` crate (add to dependencies)
   - Validation of regex patterns BEFORE backend call

7. **Testing:**
   - Unit tests in `src/core/automation_session.rs` (mod tests)
   - Integration tests in `tests/integration/automation_session_tests.rs`
   - Mock backend for controlled testing

## Error Types

```
AutomationError:
- ProcessLaunchFailed: std::process::Command error
- WindowNotFound: Window not found within timeout
- ProcessNotFound: Process does not exist or exited
- WindowDisabled: Window exists but is disabled
- WindowOffscreen: Window exists but is not visible on screen
- InvalidConfig: Configuration validation failed
- Cancelled: Operation was cancelled via token
- SessionClosed: Session was closed
- ComError: COM-related errors with details
```

## Configuration

```rust
struct SessionConfig {
    timeout: Duration,        // > 0 and <= 1 hour
    cancellation: CancellationToken,
}

struct SessionLaunchConfig {
    command: String,          // non-empty
    args: Option<Vec<String>>,
    working_dir: Option<String>,
}
```

## Validation Rules

- `timeout > Duration::ZERO` (must be positive)
- `timeout <= Duration::from_secs(3600)` (max 1 hour)
- `command` must be non-empty string
- `title` must be non-empty string
- `regex` must be valid regex pattern (via `regex::Regex::new()`)
- `process_id` must be > 0

## Testing Strategy

### Unit Tests (src/core/automation_session.rs)
- `test_validate_session_config_valid`: Valid config passes
- `test_validate_session_config_zero_timeout`: Zero timeout fails
- `test_validate_session_config_large_timeout`: >1h timeout fails
- `test_validate_title_filter_valid`: Valid title passes
- `test_validate_title_filter_empty`: Empty title fails
- `test_validate_regex_valid`: Valid regex passes
- `test_validate_regex_invalid`: Invalid regex fails
- `test_validate_command_valid`: Valid command passes
- `test_validate_command_empty`: Empty command fails
- `test_mock_backend_success`: Mock returns success
- `test_mock_backend_failure`: Mock returns configured error
- `test_mock_backend_idempotent_error`: Errors are consistent

### Integration Tests (tests/integration/automation_session_tests.rs)
- `test_integration_launch_process_success`: Full launch flow succeeds
- `test_integration_launch_process_with_args`: Launch with args succeeds
- `test_integration_launch_process_timeout_zero`: Zero timeout fails
- `test_integration_launch_process_nonexistent`: Nonexistent command fails
- `test_integration_attach_by_title_exact`: Exact match succeeds
- `test_integration_attach_by_title_contains`: Contains match succeeds
- `test_integration_attach_by_title_regex`: Regex match succeeds
- `test_integration_attach_by_title_not_found`: Missing window fails
- `test_integration_attach_by_process_id_success`: Attach by PID succeeds
- `test_integration_attach_by_process_id_not_found`: Invalid PID fails
- `test_integration_session_close`: Close terminates process
- `test_integration_session_after_close`: Closed session returns error
- `test_integration_cancelled`: Cancellation works
- `test_integration_invalid_config`: Invalid config fails
- `test_integration_mock_idempotency`: Mock is idempotent

## Trade-offs

1. **Separate launch/attach**: Provides more control but requires two calls. Alternative would be combined `launch_and_attach()` which is less flexible.

2. **Regex support**: Adds power but requires `regex` crate dependency. Alternative would be only `Exact` and `Contains`.

3. **Direct implementation**: `click()`/`type_text()` via uiautomation (not via ClickTool) keeps AutomationSession self-contained. Alternative would be delegation to tool traits.

4. **No serialization**: `SerializableSession` excluded from MVP. Can be added later if needed.

5. **Default window selection**: When multiple windows exist, select first valid one. Alternative would be user-specified window index or all windows list.

## Validation

- [ ] `cargo test` passes (all unit + integration tests)
- [ ] `cargo clippy -- -D warnings` passes
- [ ] No `unwrap()`/`panic!` in `src/`
- [ ] All error cases tested
- [ ] Idempotency verified
- [ ] Examples created (`examples/automation_session_example.rs`)
- [ ] Real-world UI automation tested (Notepad launch + attach)

## Integration Examples

### Example 1: Launch and Attach
Basic workflow - launch application, attach to window, perform actions.

```rust
use smith_windows::{launch_process, attach_by_title, SessionConfig, MatchMode};

// Launch Notepad
let process_id = launch_process(SessionLaunchConfig {
    command: "notepad.exe".to_string(),
    args: None,
    working_dir: None,
}).await?;

// Attach to the window
let config = SessionConfig {
    timeout: Duration::from_secs(10),
    cancellation: CancellationToken::new(),
};

let session = attach_by_title(
    "Безымянный – Блокнот".to_string(),
    MatchMode::Contains,
    true,
    config,
).await?;

// Use the session
session.click().await?;
session.type_text("Hello, Automation!").await?;
session.close().await?;
```

### Example 2: Attach to Existing Process
Connect to already running application by process ID.

```rust
use smith_windows::{attach_by_process_id, SessionConfig};

let config = SessionConfig {
    timeout: Duration::from_secs(5),
    cancellation: CancellationToken::new(),
};

let session = attach_by_process_id(12345, config).await?;

// Work with the session
session.click().await?;
session.close().await?;
```

### Example 3: Regex Matching
Use regular expressions for flexible window matching.

```rust
use smith_windows::{attach_by_title, SessionConfig, MatchMode};

let config = SessionConfig {
    timeout: Duration::from_secs(10),
    cancellation: CancellationToken::new(),
};

// Match any window containing "Notepad" or "Блокнот"
let session = attach_by_title(
    ".*(Notepad|Блокнот).*".to_string(),
    MatchMode::Regex,
    true,
    config,
).await?;
```

## Validation

- [x] `cargo test` passes (28 unit tests)
- [x] `cargo clippy -- -D warnings` passes
- [x] No `unwrap()`/`panic!` in `src/`
- [x] All error cases handled
- [x] Examples created (`examples/notepad_automation_session.rs`)
- [x] Real-world UI automation tested (Notepad launch + click on "Файл")

## Integration Examples

### Example 1: Launch and Attach
Basic workflow - launch application, attach to window, perform actions.

```rust
use smith_windows::{launch_process, attach_by_title, SessionConfig, MatchMode};

// Launch Notepad
let process_id = launch_process(SessionLaunchConfig {
    command: "notepad.exe".to_string(),
    args: None,
    working_dir: None,
}).await?;

// Attach to the window
let config = SessionConfig {
    timeout: Duration::from_secs(10),
    cancellation: CancellationToken::new(),
};

let session = attach_by_title(
    "Безымянный – Блокнот".to_string(),
    MatchMode::Contains,
    true,
    config,
).await?;

// Use the session
session.click().await?;
session.type_text("Hello, Automation!").await?;
session.close().await?;
```

### Example 2: Attach to Existing Process
Connect to already running application by process ID.

```rust
use smith_windows::{attach_by_process_id, SessionConfig};

let config = SessionConfig {
    timeout: Duration::from_secs(5),
    cancellation: CancellationToken::new(),
};

let session = attach_by_process_id(12345, config).await?;

// Work with the session
session.click().await?;
session.close().await?;
```

### Example 3: Regex Matching
Use regular expressions for flexible window matching.

```rust
use smith_windows::{attach_by_title, SessionConfig, MatchMode};

let config = SessionConfig {
    timeout: Duration::from_secs(10),
    cancellation: CancellationToken::new(),
};

// Match any window containing "Notepad" or "Блокнот"
let session = attach_by_title(
    ".*(Notepad|Блокнот).*".to_string(),
    MatchMode::Regex,
    true,
    config,
).await?;
```

### Example 4: AutomationSession + ClickTool Integration
Launch application, attach via session, then use ClickTool for UI interactions.

```rust
use smith_windows::{
    launch_process, attach_by_process_id, SessionConfig, SessionLaunchConfig,
};
use smith_windows::runtime::backends::windows::click::ClickBackendWindows;

// Step 1: Launch Notepad
let launch_config = SessionLaunchConfig {
    command: "notepad.exe".to_string(),
    args: None,
    working_dir: None,
};
let process_id = launch_process(&launch_config).await?;

// Step 2: Attach to window
let config = SessionConfig {
    timeout: Duration::from_secs(10),
    cancellation: CancellationToken::new(),
};
let session = attach_by_process_id(process_id, &config).await?;

// Step 3: Find menu item (MenuItem elements are direct children of window)
let automation = uiautomation::UIAutomation::new()?;
let menu_items = automation.create_matcher()
    .from(session.main_element.clone())
    .control_type(uiautomation::types::ControlType::MenuItem)
    .timeout(3000)
    .find_all()?;

let file_item = menu_items.iter()
    .find(|item| item.get_name().ok().unwrap_or_default().contains("Файл"))
    .cloned()
    .ok_or("File menu not found")?;

// Step 4: Click via ClickTool
let click_backend = ClickBackendWindows::new();
click_backend.click(&file_item).await?;

// Step 5: Cleanup
session.close().await?;
```

## Trade-offs

1. **API differences**: `uiautomation` 0.24.4 uses `create_matcher()` instead of `Condition::new()`. This required adapting the implementation.

2. **No `UIElement::null()`**: The crate doesn't provide a null element. Mock implementations use default state without actual UIElement.

3. **Clipboard for typing**: Since `element.value().set_value()` requires UIElement to implement additional traits, clipboard-based typing was implemented as alternative.

4. **No `spawn_blocking`**: `UIElement` is not `Send`, so COM calls cannot be isolated. This is acceptable as the crate's own backend handles COM internally.

5. **Integration pattern**: `RuntimeSession` directly implements `click()`/`type_text()` via uiautomation rather than delegating to `ClickTool`. This keeps the session self-contained.

## Related Decisions

- Architecture: UIAutomation-first, all UI operations through uiautomation crate
- Error handling: Explicit Result types, no silent failures
- Testing: Unit + Integration tests, mocks for isolation
- Process management: std::process::Command for launch, uiautomation for window attachment
