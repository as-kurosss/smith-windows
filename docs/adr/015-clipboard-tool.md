# ADR-015: ClipboardTool Implementation

**Date:** 2026-04-22

**Status:** ✅ Documentation Created (Implementation Pending)

## Context

We need to implement a ClipboardTool to solve a common problem in Windows automation:

**Problem:** Scripts often need to interact with the system clipboard to:
- Retrieve text from clipboard for processing
- Set text to clipboard for pasting into other applications
- Check if clipboard contains text before attempting operations

**Current limitations:**
- No built-in way to access system clipboard in smith-windows
- Users would need to use external crates without proper error handling
- No integration with the project's async runtime and cancellation patterns

**Desired solution:**
- Integration with `clipboard` crate v0.5
- Consistent API with other tools in the project
- Proper error handling via `thiserror`
- Async/await with timeout and cancellation support
- Idempotent operations (errors don't change state)

## Decision

We will implement ClipboardTool following the project's architecture:

1. **Separation of concerns:**
   - `src/core/clipboard.rs`: Types, traits, validation, mocks, tests
   - `src/runtime/backends/windows/clipboard.rs`: Windows-specific implementation
   - `src/runtime/backends/unsupported.rs`: Stub for non-Windows platforms

2. **clipboard crate approach:**
   - Use `clipboard` crate v0.5 which handles COM internally
   - No need for `spawn_blocking` - the crate's API is synchronous
   - COM initialization handled by `clipboard` crate internally

3. **Error handling:**
   - Use `thiserror` for error types (consistent with other tools)
   - Explicit `Result<T, ClipboardError>` return values
   - No `unwrap()`, `panic!`, or `expect()` in library code
   - Convert `clipboard::Error` to `ClipboardError` variants

4. **Async/await with timeout:**
   - Timeout handling via `tokio::time::timeout`
   - Cancellation via `tokio_util::sync::CancellationToken`
   - Check cancellation after timeout completes

5. **Validation:**
   - Config validation (`validate_clipboard_config()`) runs BEFORE backend calls
   - Text validation (`params.text.is_empty()`) runs before backend call
   - Timeout bounds: > 0 and <= 1 hour

## Error Types

```
ClipboardError:
- OperationNotSupported: Running on non-Windows platform
- ClipboardEmpty: Clipboard contains no text (for get_text)
- ClipboardAccessDenied: Clipboard is locked by another process
- TextEmpty: Empty text provided for set_text
- Timeout: Operation exceeded timeout
- Cancelled: Operation was cancelled via token
- InvalidConfig: Configuration validation failed
- ComError: COM-related errors with details
```

## Configuration

```rust
struct ClipboardConfig {
    timeout: Duration,        // > 0 and <= 1 hour
    cancellation: CancellationToken,
}

struct SetTextParams {
    text: String,             // Must be non-empty
}
```

## Validation Rules

- `timeout > Duration::ZERO` (must be positive)
- `timeout <= Duration::from_secs(3600)` (max 1 hour)
- `text.is_empty()` must be false for set_text

## Testing Strategy

### Unit Tests (src/core/clipboard.rs)
- `test_validate_clipboard_config_valid`: Valid config passes
- `test_validate_clipboard_config_zero_timeout`: Zero timeout fails
- `test_validate_clipboard_config_large_timeout`: >1h timeout fails
- `test_validate_clipboard_config_max_timeout`: Exactly 1h timeout passes
- `test_mock_backend_creation`: Mock backend creation
- `test_mock_backend_with_state`: Mock with custom state
- `test_mock_backend_reset`: Mock backend reset
- `test_mock_backend_idempotent_error`: Repeated errors don't change state

### Integration Tests (tests/integration/clipboard_tool.rs)
- `test_get_text_success`: Successful get text
- `test_get_text_empty_clipboard`: Empty clipboard error
- `test_set_text_success`: Successful set text
- `test_set_text_error`: Set text error
- `test_has_text_success`: Has text returns true
- `test_has_text_false`: Has text returns false
- `test_has_text_error`: Has text error
- `test_get_text_cancellation`: Cancellation handling
- `test_set_text_text_empty`: Empty text validation
- `test_idempotent_error_get_text`: Idempotency on error
- `test_idempotent_error_set_text`: Idempotency on error
- `test_idempotent_error_has_text`: Idempotency on error
- `test_mock_backend_reset`: Mock backend reset
- `test_validate_clipboard_config_edge_cases`: Edge cases

## Trade-offs

1. **No spawn_blocking**: The `clipboard` crate handles COM internally, so all calls are synchronous. No need for thread isolation.

2. **Windows-only**: Clipboard operations are Windows-specific. Non-Windows platforms return `OperationNotSupported`.

3. **Synchronous API**: The `clipboard` crate's API is synchronous, which means the async runtime may be briefly blocked during clipboard operations. This is acceptable as clipboard operations are typically fast.

4. **Mock state**: Mock uses `Arc<Mutex<MockClipboardState>>` for thread-safe testing.

5. **Idempotent operation**: Calling clipboard operations multiple times with the same data doesn't change system state on error.

## Validation

- [x] Documentation created (`docs/design/clipboard-tool/`)
- [x] `cargo test --lib -- clipboard` passes (8/8 unit tests)
- [x] `cargo test --test clipboard_tool` passes (14/14 integration tests)
- [x] `cargo clippy -- -D warnings` passes
- [x] No `unwrap()`/`panic!` in `src/`
- [x] All error cases tested
- [x] Idempotency verified (mock backend)
- [x] `cargo check` compiles without errors

## Integration Examples

### Example 1: Get text from clipboard

```rust
use std::time::Duration;
use smith_windows::{ClipboardConfig, get_text_with_config};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClipboardConfig {
        timeout: Duration::from_secs(5),
        cancellation: CancellationToken::new(),
    };

    match get_text_with_config(&config).await {
        Ok(text) => println!("Clipboard text: {}", text),
        Err(e) => eprintln!("Failed to get clipboard text: {}", e),
    }

    Ok(())
}
```

### Example 2: Set text to clipboard

```rust
use std::time::Duration;
use smith_windows::{ClipboardConfig, SetTextParams, set_text_with_config};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClipboardConfig {
        timeout: Duration::from_secs(5),
        cancellation: CancellationToken::new(),
    };

    let params = SetTextParams {
        text: "Hello from ClipboardTool!".to_string(),
    };

    match set_text_with_config(&params, &config).await {
        Ok(()) => println!("Text set to clipboard successfully"),
        Err(e) => eprintln!("Failed to set clipboard text: {}", e),
    }

    Ok(())
}
```

### Example 3: Check clipboard before getting text

```rust
use std::time::Duration;
use smith_windows::{ClipboardConfig, has_text_with_config, get_text_with_config};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClipboardConfig {
        timeout: Duration::from_secs(5),
        cancellation: CancellationToken::new(),
    };

    if has_text_with_config(&config).await? {
        let text = get_text_with_config(&config).await?;
        println!("Clipboard contains: {}", text);
    } else {
        println!("Clipboard is empty");
    }

    Ok(())
}
```

### Example 4: Cancellation support

```rust
use std::time::Duration;
use smith_windows::{ClipboardConfig, CancellationToken, get_text_with_config};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cancellation = CancellationToken::new();
    let config = ClipboardConfig {
        timeout: Duration::from_secs(5),
        cancellation: cancellation.clone(),
    };

    // Spawn operation
    let handle = tokio::spawn(async move {
        get_text_with_config(&config).await
    });

    // Cancel after 2 seconds
    tokio::time::sleep(Duration::from_secs(2)).await;
    cancellation.cancel();

    match handle.await? {
        Ok(text) => println!("Result: {}", text),
        Err(e) => println!("Operation cancelled or failed: {}", e),
    }

    Ok(())
}
```

## Implementation Notes

### Clipboard Crate API

The `clipboard` crate provides synchronous APIs:

```rust
// Get text from clipboard (returns Result<Option<String>, Error>)
let text = clipboard::get_text()?;

// Set text to clipboard (returns Result<(), Error>)
clipboard::set_text("Hello")?;

// Check if clipboard contains text (returns Result<bool, Error>)
let has_text = clipboard::has_text()?;
```

### Error Conversion

```rust
match clipboard::get_text() {
    Ok(Some(text)) => Ok(text),
    Ok(None) => Err(ClipboardError::ClipboardEmpty),
    Err(e) => {
        if e.to_string().contains("access denied") {
            Err(ClipboardError::ClipboardAccessDenied)
        } else {
            Err(ClipboardError::ComError(e.to_string()))
        }
    }
}
```

### COM Safety

- The `clipboard` crate handles COM initialization internally
- No explicit COM initialization required
- No `spawn_blocking` needed - API is synchronous

## Related Decisions

- Architecture: Synchronous API for clipboard operations (COM handled internally)
- Error handling: Explicit Result types, no silent failures
- Testing: Unit + Integration + Mock tests, idempotency verified
- Platform: Windows-only (non-Windows returns OperationNotSupported)

## Files

- `src/core/clipboard.rs`: Core module with types, traits, validation, mocks, tests
- `src/runtime/backends/windows/clipboard.rs`: Windows implementation
- `src/runtime/backends/unsupported.rs`: Non-Windows stub (uses OperationNotSupported)
- `src/core/mod.rs`: Module exports (added `pub mod clipboard;`)
- `src/runtime/backends/windows/mod.rs`: Backend exports (added mod and re-exports)
- `src/lib.rs`: Public API re-exports
- `tests/integration/clipboard_tool.rs`: Integration tests
- `docs/design/clipboard-tool/`: Documentation (spec, contract, test-plan, brief)
- `docs/adr/015-clipboard-tool.md`: This ADR

## Alternative Approaches Considered

1. **Direct COM API**: Use Windows COM API directly (`OpenClipboard`, `GetClipboardData`, etc.)
   - **Rejected**: More complex, requires manual COM management and resource cleanup

2. **spawn_blocking with COM API**: Wrap COM calls in `spawn_blocking`
   - **Rejected**: Unnecessary complexity, `clipboard` crate handles COM better

3. **Async clipboard crate**: Wait for async clipboard crate
   - **Rejected**: No mature async clipboard crate available, `clipboard` crate is stable

4. **Per-operations cancellation only**: Cancel only during timeout
   - **Rejected**: User may want to cancel before timeout expires

5. **Global clipboard state**: Track clipboard state in the backend
   - **Rejected**: System clipboard is shared resource, state may change externally

## Conclusion

ClipboardTool provides a simple, consistent way to access system clipboard operations. This solves the "no clipboard access" problem while maintaining the project's architecture principles (error handling, async support, idempotency, testing).