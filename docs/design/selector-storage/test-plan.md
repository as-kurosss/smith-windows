# 🧪 Test Plan: SelectorStorage

## 🧪 Test Scenarios

### Unit Tests (selector_storage.rs - #[cfg(test)])

#### 1. ID Validation
```rust
#[test]
fn test_validate_id_valid() {
    // Test valid IDs: "test", "test123", "my-selector", "selector_1"
    // Expected: OK
}

#[test]
fn test_validate_id_invalid() {
    // Test invalid IDs: "", "..", "../test", "test/../../etc/passwd", "test\\..\\.."
    // Expected: InvalidSelectorId error
}
```

#### 2. ID Sanitization
```rust
#[test]
fn test_sanitize_id() {
    // Test valid ID: "test" → "test"
    // Test path traversal blocked: "../test" → PathTraversalDetected
    // Test control codes removed: "test\x00name" → "testname"
}
```

#### 3. Save/Load Operations (#[tokio::test])
```rust
#[tokio::test]
async fn test_save_and_load_selector() {
    // Setup: tempfile::tempdir()
    // Given: Valid selector with 1 step
    // When: save_selector() then load_selector()
    // Then: Loaded data matches original
}

#[tokio::test]
async fn test_save_selector_already_exists() {
    // Setup: tempfile::tempdir()
    // Given: Selector saved with ID "test"
    // When: save_selector() with same ID
    // Then: SelectorAlreadyExists error
}

#[tokio::test]
async fn test_load_selector_not_found() {
    // Setup: tempfile::tempdir()
    // Given: No selectors saved
    // When: load_selector("nonexistent")
    // Then: SelectorNotFound error
}
```

#### 4. List Operations (#[tokio::test])
```rust
#[tokio::test]
async fn test_list_selectors() {
    // Setup: tempfile::tempdir()
    // Given: 0 selectors initially
    // Then: list_selectors() → []
    // When: Save 3 selectors
    // Then: list_selectors() → ["selector_0", "selector_1", "selector_2"]
}
```

#### 5. Delete Operations (#[tokio::test])
```rust
#[tokio::test]
async fn test_delete_selector() {
    // Setup: tempfile::tempdir()
    // Given: Selector saved with ID "test"
    // When: delete_selector("test")
    // Then: File deleted
    // When: load_selector("test")
    // Then: SelectorNotFound error
}

#[tokio::test]
async fn test_delete_selector_not_found() {
    // Setup: tempfile::tempdir()
    // When: delete_selector("nonexistent")
    // Then: SelectorNotFound error
}
```

#### 6. Path Traversal Protection (#[tokio::test])
```rust
#[tokio::test]
async fn test_path_traversal_attack() {
    // Setup: tempfile::tempdir()
    // Given: Attempt to save with ID "../etc/passwd"
    // When: save_selector()
    // Then: PathTraversalDetected error
    // And: No file created outside temp dir
}
```

#### 7. Invalid Data (#[tokio::test])
```rust
#[tokio::test]
async fn test_invalid_control_type() {
    // Setup: SerializableSelectorStep with control_type = "InvalidType"
    // When: validate_serialized_selector()
    // Then: InvalidControlType error
}

#[tokio::test]
async fn test_empty_selector() {
    // Setup: RecordedSelector with steps = []
    // When: save_selector()
    // Then: InvalidSelectorData error
}
```

#### 8. Control Type Mapping (#[tokio::test])
```rust
#[tokio::test]
async fn test_control_type_mapping() {
    // Test all control types: Button, Calendar, CheckBox, Edit, Window
    // Given: ControlType::Button
    // When: control_type_to_string() then control_type_from_string()
    // Then: Round-trip conversion works
}
```

### Integration Tests (tests/integration.rs)

#### 1. Full Lifecycle
```rust
#[test]
fn test_full_selector_lifecycle() {
    // 1. Create selector storage
    // 2. Save multiple selectors
    // 3. List selectors
    // 4. Load each selector
    // 5. Delete selectors
    // 6. Verify deletion
}
```

#### 2. Concurrent Access
```rust
#[test]
fn test_concurrent_operations() {
    // Multiple threads saving/loading different selectors
    // Expected: No data corruption
}
```

#### 3. Storage Limits
```rust
#[test]
fn test_storage_size_limit() {
    // Fill storage until limit reached
    // Expected: StorageSizeLimitExceeded error
}

#[test]
fn test_selector_count_limit() {
    // Create selectors until max_selectors reached
    // Expected: TooManySelectors error
}
```

## ✅ Pass Criteria

1. **All tests pass**: `cargo test --lib selector_storage`
2. **No warnings**: `cargo clippy -- -D warnings`
3. **Code coverage**: 100% for public functions
4. **No forbidden patterns**: No `unwrap()`, `expect()`, `panic!`
5. **Idempotent errors**: Repeated `Err` calls don't change state

## 🧰 Test Setup

### Test Directory
```rust
let temp_dir = tempdir().unwrap();
let config = SelectorStorageConfig::with_storage_dir(temp_dir.path());
let storage = SelectorStorage::with_config(config);
```

### Test Selector
```rust
let step = SelectorStep {
    classname: Some("Button".to_string()),
    control_type: Some(ControlType::Button),
    name: Some("Click Me".to_string()),
    automation_id: Some("click_button".to_string()),
};
let recorded = RecordedSelector {
    steps: vec![step],
    depth: 1,
};
```
