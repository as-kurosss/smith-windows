# 📝 Brief: SelectorStorage Implementation

## 🎯 Task

Implement the `selector_storage` module strictly according to the approved plan.

## 📋 Files to Create/Update

### 1. Cargo.toml
```toml
[dev-dependencies]
tempfile = "3"
```

### 2. src/core/selector_storage.rs (NEW)
- Entities: `SelectorStorageConfig`, `StorageError`, `SerializableSelectorStep`, `SerializableRecordedSelector`
- Functions: `save_selector()`, `load_selector()`, `list_selectors()`, `delete_selector()`, `sanitize_id()`
- No `cfg` flags (pure Rust, platform-agnostic)
- All functions return `Result<_, StorageError>`

### 3. src/core/mod.rs
```rust
pub mod selector_storage;
```

### 4. src/core/selector.rs
- Add `Serialize` and `Deserialize` derives to:
  - `RecordedSelector`
  - `SelectorStep`
  - `Selector`

### 5. src/core/selector_recorder.rs
- Add `capture_and_save()` method

## 🛠️ Implementation Rules

### 1. Storage Location
- Default: `std::env::temp_dir().join("smith-windows-selectors")`
- Customizable via `SelectorStorageConfig`

### 2. File Format
- JSON with pretty printing
- Extension: `.json`
- Naming: `{sanitized_id}.json`

### 3. Path Traversal Protection
```rust
// sanitize_id() must:
- Block "..", "/", "\\"
- Remove control characters
- Remove special chars: " < > |
- Return error if result is empty
```

### 4. No spawn_blocking
- `tokio::fs` is already async — no need for `spawn_blocking`

### 5. Control Type Mapping
```rust
// Map ControlType to String for JSON:
Button → "Button"
Edit → "Edit"
Window → "Window"
// ... all control types
```

### 6. Validation
- Maximum depth: 256
- Maximum storage size: 100MB
- Maximum selectors: 1000
- Each step must have at least one identifying property

### 7. Error Handling
- All functions: `Result<_, StorageError>`
- `StorageError` enum with all variants from contract
- Idempotent: repeated `Err` calls don't change state

## 🧪 Tests

### Location
- Unit tests: `#[cfg(test)] mod tests` inside `selector_storage.rs`
- Integration tests: `tests/integration.rs`

### Test Scenarios
1. ✅ Save and load selector
2. ❌ Save with duplicate ID
3. ❌ Load non-existent selector
4. ❌ Path traversal attack
5. ❌ Invalid control type
6. ❌ Empty selector
7. ✅ List selectors (empty, then populated)
8. ✅ Delete selector

### Test Requirements
- Use `tempfile::tempdir()` for isolation
- Use `#[tokio::test]` for async functions
- At least 1 positive + 1 negative test per function
- Edge cases: 0, empty, max

## ✅ Validation

```bash
# Run tests
cargo test --lib selector_storage

# Lint
cargo clippy -- -D warnings

# Format
cargo fmt --all
```

## 📝 Notes
- Read `specification.md`, `contract.md`, `test-plan.md` BEFORE coding
- Follow the exact function signatures from contract
- No `unwrap()`, `expect()`, `panic!` in `src/`
- All errors must be propagated via `Result`
- Tests must be comprehensive and independent
