# 🤖 Rules for AI Agents: smith-windows

## 🎯 Context
`smith-windows` is an API for calling Windows automation tools via the `uiautomation` crate.
Goal: a reliable, testable, modular foundation for UI automation.

## 🛠️ Stack
- Rust 1.95, `tokio`, `thiserror`/`anyhow`, `serde`
- UIAutomation 0.24.4: `uiautomation`
- Tests: built-in `#[test]`, `cargo clippy -- -D warnings`

## 📐 Rules
1. **Spec first → `/plan` → code**. Never change contracts without agreement.
2. **Modularity**: one file = one responsibility.
3. **No `unwrap()`/`panic!`** in the library. Only `Result`/`Option`.
4. **Idempotency**: repeated calls with the same data don't break state.
5. **UIAutomation-first**: all UI operations via `uiautomation` crate.

## 📁 Sources (reading order)
1. `docs/design/<module>/specification.md`
2. `docs/design/<module>/contract.md`
3. `docs/design/<module>/test-plan.md`
4. `docs/design/<module>/brief.md`
5. `docs/adr/<N>-<module>.md`
6. `AGENTS.md`, `ARCHITECTURE.md`
7. `context_bundle.md` — project's canonical context (generated via `tools/bundle_context.rs`)

**Note**: Before developing a new module, run context bundle:
```bash
cargo run --bin bundle_context
```

## 🗓️ Working with `/plan` (MANDATORY)
1. After receiving spec/contract → build plan via `/plan`
2. Plan must include: files, types, functions, tests, validations
3. Don't generate code without architect's plan approval
4. If plan is incomplete → ask clarifying questions BEFORE execution

## 🧪 Tests
- For each public function: at least 1 positive + 1 negative test
- Edge cases (0, empty, max) are mandatory
- `cargo test` and `cargo clippy` must pass without warnings

## ⚠️ Prohibitions
- ❌ `unwrap()`, `expect()`, `panic!` in `src/`
- ❌ Global mutable state without `Arc` + explicit context
- ❌ Contract changes without discussion
- ❌ Code generation without approved `/plan`
- ❌ `CoInitializeEx`, `CoCreateInstance`, `BSTR`, `VARIANT` in backends
- ❌ `GetForegroundWindow()` — use `is_enabled()`/`is_offscreen()` via `uiautomation`

## 🔄 Process
1. Architect writes spec + contract + test plan → `docs/design/<module>/`
2. Planner creates `/plan` → architect approves
3. Coder generates code + tests strictly per plan
4. Verification: `cargo test && cargo clippy -- -D warnings`
5. Decision logged in `docs/adr/`
