# 🏗️ Architecture: smith-windows

## 🎯 Purpose
API for calling Windows automation tools via the `uiautomation` crate.
MVP for integration into smith-core. Windows-only, uiautomation-only.

## 🛠️ Tech Stack
- Language: Rust 1.95.0 (2024 edition)
- Async: `tokio`
- Errors: `thiserror` (lib), `anyhow` (apps)
- Serialization: `serde` + `serde_json`/`bincode`
- Testing: `#[test]`, `cargo clippy -- -D warnings`
- Logging: `tracing`
- **UIAutomation:** `uiautomation` crate 0.24.4

## 📦 Core Modules
- `src/core/` — Shared types, traits, error definitions, tests
- `src/runtime/` — Windows implementations via `uiautomation`

## 🔄 Communication & Flow
- Task lifecycle: `Created → Queued → Running → [Completed | Failed | Cancelled]`
- SessionBackend → SessionHandle → ClickBackend/TypeBackend
- `tokio::task::spawn_blocking` for all uiautomation/WinAPI calls

## 📐 Design Principles
1. **Contracts First:** Behavior defined before implementation
2. **No Silent Failures:** All errors explicit (`Result`), never `unwrap()`/`panic!`
3. **Idempotency:** Repeated calls with same input → same state/output
4. **Modularity:** Clear boundaries, single responsibility
5. **Test-Driven Verification:** Tests define correctness
6. **UIAutomation-First:** All UI operations via `uiautomation` crate

## 📁 Project Structure
```
smith-windows/
├── docs/
│   ├── design/<module>/     # Working docs: specification.md, contract.md, test-plan.md, brief.md
│   ├── templates/           # Document templates: specification.md, contract.md, test-plan.md, brief.md
│   └── adr/                 # Architecture Decision Records
├── src/
│   ├── core/                # Traits, types, errors, mocks, tests
│   └── runtime/             # Windows implementations + unsupported stub
├── tests/                   # Integration tests
├── .qwen/agents/            # smith-architect, smith-planner, smith-coder
├── tools/                   # Development helper tools
│   └── bundle_context.rs    # Project context collector (for AI agents)
├── Cargo.toml
├── AGENTS.md
└── ARCHITECTURE.md
```

## 🔄 Workflow
1. Architect writes spec + contract + test plan → `docs/design/<module>/`
2. Planner creates `/plan` → architect approves
3. Coder generates code + tests → architect verifies via `cargo test` + `clippy`
4. Decision logged in `docs/adr/`

## 📚 Additional Resources

- **Agents**: `AGENTS.md` — rules for AI agents
- **Agents**: `.qwen/agents/` — AI agents configuration
- **Context Bundle**: `tools/bundle_context.rs` — tool for collecting canonical documentation
- **Templates**: `docs/templates/` — document templates

**Note**: Before developing a new module, run context bundle:
```bash
cargo run --bin bundle_context
```
