# ADR 003: SelectorStorage Module

## 📅 Date
2026-04-21

## 🎯 Status
Approved

## 📋 Overview

We added a `SelectorStorage` module for persisting recorded UI selectors to disk using JSON serialization. This module enables users to save and reuse selectors across sessions without needing to re-record them.

## 🤔 Context

### Problem
Users need to save recorded selectors for reuse across application sessions. Without persistence, each session requires re-recording selectors.

### Requirements
1. Save selectors to disk in a platform-agnostic way
2. Load selectors from disk for reuse
3. List all saved selectors
4. Delete selectors when no longer needed
5. Protect against path traversal attacks
6. Validate data integrity

### Constraints
- Must be cross-platform (no Windows-specific code)
- Must not use `spawn_blocking` for file I/O (tokio::fs is already async)
- Must not serialize raw `ControlType` enum (needs String mapping)
- Must follow project error handling patterns (`thiserror`)

## ✅ Decision

We created a new `selector_storage` module with the following design:

### File Structure
```
src/core/selector_storage.rs
docs/design/selector-storage/
├── specification.md
├── contract.md
├── test-plan.md
└── brief.md
```

### Key Components

#### SelectorStorageConfig
```rust
pub struct SelectorStorageConfig {
    pub storage_dir: PathBuf,
    pub max_storage_size: u64,      // 100MB default
    pub max_selectors: usize,       // 1000 default
}
```

#### Serializable Types
- `SerializableSelectorStep` — wraps `SelectorStep` with String control type
- `SerializableRecordedSelector` — wraps `RecordedSelector` with serializable steps

#### Core Functions
- `save_selector(id, recorded)` — saves selector to `{id}.json`
- `load_selector(id)` — loads selector from `{id}.json`
- `list_selectors()` — returns list of all selector IDs
- `delete_selector(id)` — deletes selector file
- `sanitize_id(id)` — validates and sanitizes selector ID

### Rationale

#### JSON Format
- Human-readable and editable
- Platform-agnostic
- Easy to debug

#### Path Traversal Protection
- Security: Prevents writing files outside storage directory
- Validation: Rejects dangerous characters

#### Control Type String Mapping
- JSON cannot serialize `ControlType` enum directly
- String representation is stable and readable
- Round-trip conversion: `ControlType ↔ String`

#### No spawn_blocking
- `tokio::fs` operations are already async
- No blocking I/O on async runtime

#### Validation Limits
- Max depth (256): Prevents memory exhaustion
- Max size (100MB): Prevents disk exhaustion
- Max selectors (1000): Prevents filesystem limits

## 📝 Implementation

### Files Changed
1. `Cargo.toml` — added `tempfile = "3"` to dev-dependencies
2. `src/core/selector_storage.rs` — new module
3. `src/core/mod.rs` — added `pub mod selector_storage`
4. `src/core/selector.rs` — added `Serialize`/`Deserialize` derives
5. `src/core/selector_recorder.rs` — added `capture_and_save()` method

### Dependencies
- `tempfile = "3"` (dev-dependency) for test isolation

## ✅ Validation

```bash
cargo test --lib selector_storage
cargo clippy -- -D warnings
cargo fmt --all
```

## 🔄 Future Considerations

1. **Update Selector**: Add `update_selector()` for overwriting existing selectors
2. **Selector Groups**: Add ability to group/selectors by session or project
3. **Backup/Restore**: Add backup and restore functionality
4. **Encryption**: Add optional encryption for sensitive selector data
5. **Export/Import**: Add JSON export/import for sharing selectors

## 📚 Related ADRs

- ADR 001: AutomationSession Module
- ADR 002: ClickTool Module
- ADR 003: SelectorStorage Module ← This one
