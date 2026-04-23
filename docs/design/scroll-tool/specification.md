## 📐 Specification: ScrollTool | smith-windows

**🎯 Purpose:** Provide scroll operations for UI elements through two approaches: programmatic (UIA patterns) and synthetic (mouse wheel emulation).

**Input Scenarios:**

| Scenario | Direction | Unit | Target |
|----------|-----------|------|--------|
| Scroll element content | Vertical/Horizontal | Line/Page | UIElement |
| Scroll window by offset | Vertical | Pixels | UIElement |
| Simulate mouse wheel | Vertical | Wheel ticks | Element coordinates |

**📤 Output:**
- `Result<(), ScrollError>` with side effects:
  - If programmatic: element's scroll position changes
  - If synthetic: mouse wheel event generated
- On error: error type returned, system state unchanged

**⚠️ Boundaries:**
- `direction`: "vertical" or "horizontal" (case-insensitive)
- `unit`: "line", "page", or "pixel" (for synthetic)
- `amount`: 1-100 (wheel ticks), -10000 to 10000 (pixels)
- What if element doesn't support scrolling?
- What if element is disabled/offscreen?
- What if UIA pattern unavailable?

**✅ Success criteria:**
- [ ] All boundary cases handled (empty input, out-of-range values)
- [ ] State unchanged on error
- [ ] Logs record operation result and failure cause
- [ ] Both programmatic and synthetic methods tested

---
## 🗓️ Implementation plan (for `/plan`)

- [ ] Create `src/core/scroll.rs` with types, trait, validation
- [ ] Create `src/runtime/backends/windows/scroll.rs` with implementations
- [ ] Add `mod scroll` to backends windows module
- [ ] Export `ScrollBackendWindows` from backends mod
- [ ] Add `scroll` to `src/core/mod.rs` exports
- [ ] Add tests: positive, boundary, negative (programmatic + synthetic)
- [ ] Update `ARCHITECTURE.md` with ScrollTool section
- [ ] Update `CHANGELOG.md`
- [ ] Update `context_bundle.md` via `cargo run --bin bundle_context`
- [ ] Checks: `cargo test`, `cargo clippy -- -D warnings`
