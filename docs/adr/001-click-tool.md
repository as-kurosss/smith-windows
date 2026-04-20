# ADR-001: ClickTool Implementation

**Date:** 2026-04-19 (last updated: 2026-04-20)

**Status:** ✅ Implemented and Verified

## Context

We need to implement a ClickTool for automated UI interaction on Windows. The tool should:
- Click on UI elements through UI Automation API
- Handle all error states explicitly
- Support cancellation and timeouts
- Be testable and maintainable

## Decision

We will implement ClickTool following the project's architecture:

1. **Separation of concerns:**
   - `src/core/click.rs`: Types, traits, validation, mocks
   - `src/runtime/backends/windows/click.rs`: Windows-specific implementation

2. **Error handling:**
   - Use `thiserror` for error types
   - Explicit `Result<(), ClickError>` return values
   - No `unwrap()`, `panic!`, or `expect()` in library code

3. **Async/await with spawn_blocking:**
   - All UI Automation calls wrapped in `tokio::task::spawn_blocking`
   - Timeout handling via `tokio::time::timeout`
   - Cancellation via `tokio_util::sync::CancellationToken`

4. **Validation:**
   - Config validation (`validate_click_config()`) runs BEFORE backend calls
   - Element validation (is_valid, is_enabled, is_offscreen) runs inside backend

5. **Testing:**
   - Unit tests in `src/core/click.rs` (mod tests)
   - Integration tests in `tests/integration/click.rs`
   - Mock backend for controlled testing

## Error Types

```
ClickError:
- ElementNotFound: Element is null or invalid
- ElementNotEnabled: Element exists but is disabled
- ElementOffscreen: Element is not visible on screen
- Timeout: Operation exceeded timeout
- Cancelled: Operation was cancelled via token
- InvalidConfig: Configuration validation failed
- ComError: COM-related errors with details
```

## Configuration

```rust
struct ClickConfig {
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

## Testing Strategy

### Unit Tests (src/core/click.rs)
- `test_validate_click_config_valid`: Valid config passes
- `test_validate_click_config_zero_timeout`: Zero timeout fails
- `test_validate_click_config_large_timeout`: >1h timeout fails
- `test_mock_backend_success`: Mock returns success
- `test_mock_backend_failure`: Mock returns configured error
- `test_mock_backend_idempotent_error`: Errors are consistent

### Integration Tests (tests/integration/click.rs)
- `test_integration_click_success`: Full flow succeeds
- `test_integration_click_element_not_found`: Handles missing element
- `test_integration_click_timeout`: Respects timeout
- `test_integration_click_cancelled`: Handles cancellation
- `test_integration_click_disabled`: Handles disabled element
- `test_integration_click_offscreen`: Handles offscreen element
- `test_integration_click_config_validation`: Validates configs
- `test_integration_mock_idempotency`: Mock is idempotent

## Trade-offs

1. **.spawn_blocking**: All COM calls are blocking, so we use spawn_blocking. This adds overhead but prevents blocking the async runtime.

2. **No direct WinAPI**: We use uiautomation crate only, avoiding direct COM calls (CoInitializeEx, etc.)

3. **Validation location**: Config validation happens in caller (for better error context), but element validation happens inside backend (requires element handle).

4. **Mock state**: Mock uses `Arc<Mutex<MockState>>` for thread-safe testing.

## Validation

- [x] `cargo test` passes (6/6 unit tests)
- [x] `cargo clippy -- -D warnings` passes
- [x] No `unwrap()`/`panic!` in `src/`
- [x] All error cases tested
- [x] Idempotency verified
- [x] Examples created (`examples/click_example.rs`, `examples/notepad_click.rs`)
- [x] Real-world UI automation tested (Notepad menu click)

## Integration Examples

### Example 1: click_example
Basic configuration validation and mock backend demonstration.

```bash
cargo run --example click_example
```

### Example 2: notepad_click
Real UI automation - opens Notepad and clicks the "File" menu:

```bash
cargo run --example notepad_click
```

Output shows:
- Notepad window found: "Безымянный – Блокнот"
- Menu items located: 6 items found
- Click successful on 'Файл' menu

## Related Decisions

- Architecture: UIAutomation-first, all UI operations through uiautomation crate
- Error handling: Explicit Result types, no silent failures
- Testing: Unit + Integration tests, mocks for isolation
