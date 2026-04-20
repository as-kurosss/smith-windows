## 🤖 Instruction for Agent: [Module Name] | smith-windows

**📁 Sources (read in priority order):**
1. `docs/design/[module]/specification.md` — input/output, boundaries, criteria
2. `docs/design/[module]/contract.md` — requirements, guarantees, prohibitions, failures
3. `docs/design/[module]/test-plan.md` — test scenarios, mandatory checks
4. `docs/adr/XXX-[module].md` — architectural decisions
5. `AGENTS.md`, `ARCHITECTURE.md` — code rules, stack, process

**🔗 Cross-references:**
- `docs/design/[module]/brief.md` — dependency description (if any)
- `docs/adr/XXX-[module].md` — cross-platform architecture, spawn_blocking (if applicable)

**🎯 Task:**
[Describe task: form implementation plan, generate code, or another specific task]

**📋 Output format (strict):**
```
[File] → [Entities] → [cfg-flags] → [Tests] → [Validation]
```

**✅ Mandatory plan elements:**
- [Describe mandatory elements, e.g.]:
  - `validate_selector()` moved to `src/core/[module].rs` (validation BEFORE backend call)
  - Unified `[Module]Error` (`thiserror`) with exact names from contract
  - Signature with explicit `[Module]Config { timeout: Duration, cancellation: CancellationToken }`
  - COM calls isolated via `tokio::task::spawn_blocking`
  - Tests: `#[cfg(test)] mod tests` inside `src/core/[module].rs` + `tests/integration/`
  - `Mock[Module]Backend` with `Arc<Mutex<MockState>>` for idempotency check on `Err`

**🚫 Prohibitions:**
- [Describe prohibitions, e.g.]:
  - Don't generate code at plan stage
- [Describe prohibitions, e.g.]:
  - Don't use `unwrap()`/`panic!`/`expect()` even in examples
- [Describe prohibitions, e.g.]:
  - Don't create `src/core/[module]/tests.rs` (combine in `mod tests` or move to `tests/`)
- [Describe prohibitions, e.g.]:
  - Don't change contract without explicit agreement

**🔄 Process:**
1. [Describe first step]
2. [Describe second step]
3. [Describe third step, if applicable]

**📝 Metadata:**
- Author: [Agent role, e.g.: smith-core Architect]
- Date: [Creation date]
- Status: `[draft]` / `[approved]` / `[approved_with_corrections]` / `[deprecated]`
