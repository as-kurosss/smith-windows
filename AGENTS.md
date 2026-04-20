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

## 🤖 Agent Roles

### Core Agents (Development Workflow)

| Agent | Color | Role | Key Tasks |
|-------|-------|------|-----------|
| **smith-architect** | Green | Documentation Architect | Creates `specification.md`, `contract.md`, `test-plan.md`, `brief.md` for new modules; updates README, CHANGELOG, context_bundle |
| **smith-planner** | Orange | Implementation Planner | Transforms specs into `/plan` text format; generates `brief.md` for coders |
| **smith-coder** | Red | Senior Rust Engineer | Generates production code and tests strictly per approved plan; updates documentation automatically |
| **smith-crate-researcher** | Purple | Crate Documentation Researcher | Fetches `docs.rs` docs, creates `docs/crates/*.md`, keeps crate knowledge up-to-date |

### Support Agents (Quality Assurance)

| Agent | Color | Role | Key Tasks |
|-------|-------|------|-----------|
| **smith-debugger** | Yellow | Autonomous Debugger | Runs tests, identifies failures, generates fixes, verifies solutions — fully autonomous debugging cycle |
| **smith-compliance** | Blue | Compliance Checker | Scans code/docs for violations; reports CRITICAL/WARNING/INFO issues; ensures AGENTS.md/ARCHITECTURE.md compliance |

## 🔄 Process

### New Module Development
1. Architect writes spec + contract + test plan → `docs/design/<module>/`
2. Planner creates `/plan` → architect approves
3. Coder generates code + tests strictly per plan
4. Documentation updates (README, CHANGELOG, context_bundle) automatically by coder
5. Verification: `cargo test && cargo clippy -- -D warnings`
6. Decision logged in `docs/adr/`

### Bug Fix / Debugging
1. smith-debugger runs `cargo test` → identifies failing tests
2. Analyzes root cause → reads relevant files
3. Generates fix → applies to implementation/test
4. Verifies fix with `cargo test` and `cargo clippy`
5. Reports summary of fixes and confidence level

### Compliance Check
1. smith-compliance scans `src/` for forbidden patterns
2. Scans `docs/` for missing/incorrect documentation
3. Runs `cargo test`, `cargo clippy`, `cargo fmt`
4. Categorizes findings: CRITICAL / WARNING / INFO
5. Reports compliance score and fix recommendations

## 📁 Agent Files
- `.qwen/agents/smith-architect.md` — specification and documentation generation
- `.qwen/agents/smith-planner.md` — plan generation and brief creation
- `.qwen/agents/smith-coder.md` — production code and documentation updates
- `.qwen/agents/smith-crate-researcher.md` — crate documentation from docs.rs
- `.qwen/agents/smith-debugger.md` — autonomous debugging
- `.qwen/agents/smith-compliance.md` — compliance checking

## 🔄 CI/CD (GitHub Actions)
- `.github/workflows/ci.yml` — run on every push/pull_request: tests, clippy, fmt, context_bundle check
- `.github/workflows/context-update.yml` — auto-update context_bundle when src/docs change

**Note**: All agents should run `cargo run --bin bundle_context` to regenerate context_bundle.md before starting work.
