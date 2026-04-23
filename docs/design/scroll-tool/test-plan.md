## 🧪 Test Plan: ScrollTool | smith-windows

### Test Categories

| Category | Description | Priority |
|----------|-------------|----------|
| Unit | Core validation and logic | High |
| Integration | Full lifecycle with backend | High |
| Mock | Isolated backend simulation | Medium |
| Edge | Boundary values and error paths | High |

---

### Unit Tests (src/core/scroll.rs)

#### validate_scroll_config
- [ ] Valid config: timeout 1s, cancellation token → `Ok(())`
- [ ] Zero timeout → `Err(ScrollError::InvalidConfig)`
- [ ] 1 hour timeout → `Ok(())`
- [ ] 1 hour + 1s timeout → `Err(ScrollError::InvalidConfig)`
- [ ] Negative values (if applicable) → `Err(ScrollError::InvalidConfig)`

#### scroll_element
- [ ] Valid element with vertical line scroll → `Ok(())`
- [ ] Valid element with horizontal page scroll → `Ok(())`
- [ ] Invalid element (null) → `Err(ScrollError::ElementNotFound)`
- [ ] Disabled element → `Err(ScrollError::ElementNotEnabled)`
- [ ] Offscreen element → `Err(ScrollError::ElementOffscreen)`

#### scroll_by_pixels
- [ ] Valid vertical scroll 100px → `Ok(())`
- [ ] Valid horizontal scroll -50px → `Ok(())`
- [ ] Zero pixels → `Err(ScrollError::InvalidConfig)`
- [ ] Exceed max pixels (10001) → `Err(ScrollError::InvalidConfig)`

#### simulate_mouse_wheel
- [ ] Valid wheel ticks (1) → `Ok(())`
- [ ] Max wheel ticks (100) → `Ok(())`
- [ ] Zero ticks → `Err(ScrollError::InvalidConfig)`
- [ ] Exceed max ticks (101) → `Err(ScrollError::InvalidConfig)`

---

### Integration Tests (tests/integration/)

#### Full Lifecycle
- [ ] Create session → Find scrollable element → Scroll vertically → Verify state change
- [ ] Create session → Find scrollable element → Scroll horizontally → Verify state change
- [ ] Create session → Scroll with cancellation token → Verify cancellation
- [ ] Create session → Scroll with timeout → Verify timeout handling

#### Fallback Testing
- [ ] Element without IScrollPattern → Try synthetic fallback → Success
- [ ] Element without IRangeValuePattern → Try synthetic fallback → Success

#### Error Scenarios
- [ ] Invalid element → Return ElementNotFound → No state change
- [ ] Timeout during scroll → Return Timeout → No state change
- [ ] Cancellation during scroll → Return Cancelled → No state change

---

### Mock Tests (src/core/scroll.rs)

#### MockScrollBackend
- [ ] Create backend with default state → call_count=0
- [ ] Execute scroll → call_count=1, success state
- [ ] Execute failure scenario → call_count=1, error state
- [ ] Reset backend → call_count=0, error reset
- [ ] Idempotency: repeat same call → same result

---

### Edge Cases

| Case | Expected Behavior |
|------|-------------------|
| Empty string for direction | `Err(ScrollError::InvalidConfig)` |
| "INVALID" for direction | `Err(ScrollError::InvalidConfig)` |
| "LINE" (uppercase) for unit | `Ok(())` (case-insensitive) |
| amount=0 for pixels | `Err(ScrollError::InvalidConfig)` |
| amount=10001 for pixels | `Err(ScrollError::InvalidConfig)` |
| amount=0 for wheel ticks | `Err(ScrollError::InvalidConfig)` |
| amount=101 for wheel ticks | `Err(ScrollError::InvalidConfig)` |
| element=null in backend | `Err(ScrollError::ElementNotFound)` |

---

### Verification Commands

```bash
# Run all scroll tests
cargo test scroll

# Run only unit tests
cargo test --lib scroll

# Run with logging
RUST_LOG=debug cargo test scroll

# Verify no warnings
cargo clippy -- -D warnings
```

---

### Success Criteria

- [ ] All 15+ tests pass
- [ ] No clippy warnings
- [ ] 100% path coverage for validation functions
- [ ] Error paths covered in tests
- [ ] Mock backend idempotency verified
- [ ] Integration tests verify full lifecycle
- [ ] Edge cases handled without panics
