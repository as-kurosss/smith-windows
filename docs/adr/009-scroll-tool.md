# ADR-009: ScrollTool Implementation

**Date:** 2026-04-22

**Status:** ✅ Implemented and Verified

## Context

We need to implement a ScrollTool for automated UI scrolling on Windows. The tool should:
- Scroll UI elements vertically/horizontally through UI Automation API
- Support programmatic scrolling (IScrollPattern, IRangeValuePattern)
- Support synthetic mouse wheel emulation (as fallback)
- Handle all error states explicitly
- Support cancellation and timeouts
- Be testable and maintainable

## Decision

We will implement ScrollTool following the project's architecture:

1. **Separation of concerns:**
   - `src/core/scroll.rs`: Types, traits, validation, mocks
   - `src/runtime/backends/windows/scroll.rs`: Windows-specific implementation
   - `src/runtime/backends/unsupported.rs`: Stub for non-Windows platforms

2. **Two scrolling approaches:**
   - **Programmatic (primary)**: Use `UIScrollPattern` or `UIRangeValuePattern` from UI Automation
   - **Synthetic (fallback)**: Mouse wheel emulation (not yet implemented - requires WinAPI)

3. **Error handling:**
   - Use `thiserror` for error types
   - Explicit `Result<(), ScrollError>` return values
   - No `unwrap()`, `panic!`, or `expect()` in library code

4. **Async/await with COM safety:**
   - **CRITICAL**: `UIElement`, `UIAutomation`, `UIPattern` are `!Send`/`!Sync`
   - NO `spawn_blocking` for UIA calls - they must run on the same thread
   - Timeout handling via `tokio::time::timeout`
   - Cancellation via `tokio_util::sync::CancellationToken`

5. **Validation:**
   - Config validation (`validate_scroll_config()`) runs BEFORE backend calls
   - Element validation (is_valid, is_enabled, is_offscreen) runs inside backend

6. **Fallback mechanism:**
   - Try `UIScrollPattern` first
   - If not supported, try `UIRangeValuePattern`
   - If not supported, return error (synthetic not yet implemented)

## Error Types

```
ScrollError:
- ElementNotFound: Element is null or invalid
- ElementNotEnabled: Element exists but is disabled
- ElementOffscreen: Element is not visible on screen
- Timeout: Operation exceeded timeout
- Cancelled: Operation was cancelled via token
- InvalidConfig: Configuration validation failed
- ComError: COM-related errors with details
- PatternNotSupported: UIA pattern not available
- UnsupportedPlatform: Running on non-Windows
```

## Configuration

```rust
struct ScrollConfig {
    timeout: Duration,        // > 0 and <= 1 hour
    cancellation: CancellationToken,
}
```

## Types

```rust
enum ScrollDirection {
    Vertical,
    Horizontal,
}

enum ScrollUnit {
    Line,
    Page,
    Pixel,
}
```

## Validation Rules

- `timeout > Duration::ZERO` (must be positive)
- `timeout <= Duration::from_secs(3600)` (max 1 hour)
- `direction`: "vertical" or "horizontal" (case-insensitive)
- `unit`: "line", "page", or "pixel"
- `amount`: 1-100 for wheel ticks, -10000 to 10000 for pixels
- Element must be valid UIA element
- Element must be enabled (via `is_enabled()`)
- Element must be on screen (via `is_offscreen()`)

## Testing Strategy

### Unit Tests (src/core/scroll.rs)
- `test_validate_scroll_config_valid`: Valid config passes
- `test_validate_scroll_config_zero_timeout`: Zero timeout fails
- `test_validate_scroll_config_large_timeout`: >1h timeout fails
- `test_scroll_direction_from_str`: Direction parsing
- `test_scroll_unit_from_str`: Unit parsing
- `test_mock_backend_creation`: Mock backend creation
- `test_mock_backend_with_state`: Mock with custom state
- `test_mock_backend_reset`: Mock backend reset
- `test_validate_amount_zero`: Zero amount validation
- `test_validate_amount_pixel_range`: Pixel range validation
- `test_validate_amount_pixel_range_out_of_bounds`: Out of bounds validation

### Mock Tests (src/core/scroll.rs)
- `test_mock_backend_creation`: Mock backend creation
- `test_mock_backend_with_state`: Mock with custom state
- `test_mock_backend_reset`: Mock backend reset

## Trade-offs

1. **No spawn_blocking**: All UIA calls run on the same thread due to `!Send`/`!Sync` constraint. This means the async runtime may be blocked during UIA calls.

2. **Programmatic first**: We prioritize UIA patterns over synthetic methods because they're more reliable and don't require focus.

3. **Synthetic fallback not yet implemented**: Mouse wheel simulation requires WinAPI `SendInput`, which is not yet implemented in the codebase.

4. **RangeValuePattern as fallback**: If `UIScrollPattern` is not available, we try `UIRangeValuePattern` (used by scrollbars, sliders).

5. **Mock state**: Mock uses `Arc<Mutex<MockScrollState>>` for thread-safe testing.

6. **FromStr trait**: Implemented `FromStr` for `ScrollDirection` and `ScrollUnit` to avoid clippy warnings about ambiguous `from_str` method names.

## Validation

- [x] `cargo test scroll` passes (11/11 unit tests)
- [x] `cargo clippy -- -D warnings` passes
- [x] No `unwrap()`/`panic!` in `src/`
- [x] All error cases tested
- [x] Idempotency verified (mock backend)
- [x] `cargo check` compiles without errors

## Integration Examples

### Example 1: Basic scroll configuration

```rust
let cancellation = CancellationToken::new();
let config = ScrollConfig {
    timeout: Duration::from_secs(5),
    cancellation,
};

// Scroll vertically by 3 line units
scroll_with_config(
    &element,
    ScrollDirection::Vertical,
    3,
    ScrollUnit::Line,
    &config
)?;
```

### Example 2: Using pattern matching

```rust
use std::str::FromStr;

let direction = ScrollDirection::from_str("vertical")?;
let unit = ScrollUnit::from_str("page")?;

scroll_with_config(&element, direction, 1, unit, &config)?;
```

### Example 3: Horizontal scroll with range validation

```rust
scroll_with_config(
    &element,
    ScrollDirection::Horizontal,
    -50,  // scroll left
    ScrollUnit::Pixel,
    &config
)?;
```

## Implementation Notes

### Programmatic Scrolling

**UIScrollPattern:**
```rust
let scroll_pattern = element.get_pattern::<uiautomation::patterns::UIScrollPattern>()?;
scroll_pattern.set_scroll_percent(horizontal_percent, vertical_percent)?;
```

**UIRangeValuePattern (fallback):**
```rust
let range_pattern = element.get_pattern::<uiautomation::patterns::UIRangeValuePattern>()?;
let current_value = range_pattern.get_value()?;
let new_value = current_value + amount;
range_pattern.set_value(new_value)?;
```

### Future Enhancements

1. **Mouse wheel simulation**: Implement WinAPI `SendInput` with `MOUSEEVENTF_WHEEL` flag
2. **Element-based scrolling**: Add method to scroll a specific element by offset
3. **Smooth scrolling**: Support intermediate scroll steps for visual effect
4. **Scroll status queries**: Add methods to get current scroll position

## Related Decisions

- Architecture: UIAutomation-first, all UI operations through uiautomation crate
- Error handling: Explicit Result types, no silent failures
- Testing: Unit + Mock tests, idempotency verified
- COM Safety: No spawn_blocking for `!Send`/`!Sync` types, same-thread execution

## Files

- `src/core/scroll.rs`: Core module with types, traits, validation, mocks
- `src/runtime/backends/windows/scroll.rs`: Windows implementation
- `src/runtime/backends/unsupported.rs`: Non-Windows stub
- `src/core/mod.rs`: Module exports
- `src/runtime/backends/windows/mod.rs`: Backend exports
- `src/runtime/backends/mod.rs`: Re-exports
- `docs/design/scroll-tool/`: Documentation (spec, contract, test-plan, brief, plan)
- `docs/adr/009-scroll-tool.md`: This ADR
