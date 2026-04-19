# 🏗️ Architecture: smith-windows

## 🎯 Purpose
API для вызова инструментов автоматизации Windows через `uiautomation` crate.
MVP для интеграции в smith-core. Только Windows, только uiautomation.

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
- `src/runtime/` — Windows-реализации через `uiautomation`

## 🔄 Communication & Flow
- Task lifecycle: `Created → Queued → Running → [Completed | Failed | Cancelled]`
- SessionBackend → SessionHandle → ClickBackend/TypeBackend
- `tokio::task::spawn_blocking` для всех uiautomation/WinAPI вызовов

## 📐 Design Principles
1. **Contracts First:** Behavior defined before implementation
2. **No Silent Failures:** All errors explicit (`Result`), never `unwrap()`/`panic!`
3. **Idempotency:** Repeated calls with same input → same state/output
4. **Modularity:** Clear boundaries, single responsibility
5. **Test-Driven Verification:** Tests define correctness
6. **UIAutomation-First:** Все UI-операции через `uiautomation` crate

## 📁 Project Structure
```
smith-windows/
├── docs/
│   ├── design/<module>/     # Рабочие документы: specification.md, contract.md, test-plan.md, brief.md
│   ├── templates/           # Шаблоны документов: specification.md, contract.md, test-plan.md, brief.md
│   └── adr/                 # Architecture Decision Records
├── src/
│   ├── core/                # Трейты, типы, ошибки, моки, тесты
│   └── runtime/             # Windows-реализации + unsupported stub
├── tests/                   # Интеграционные тесты
├── .qwen/agents/            # smith-architect, smith-planner, smith-coder
├── tools/                   # Вспомогательные инструменты разработки
│   └── bundle_context.rs    # Сборщик контекста проекта (для ИИ-агентов)
├── Cargo.toml
├── AGENTS.md
└── ARCHITECTURE.md
```

## 🔄 Workflow
1. Architect writes spec + contract + test plan → `docs/design/<module>/`
2. Planner creates `/plan` → architect approves
3. Coder generates code + tests → architect verifies via `cargo test` + `clippy`
4. Decision logged in `docs/adr/`

## 📚 Дополнительные ресурсы

- **Agents**: `AGENTS.md` — правила для ИИ-агентов
- **Agents**: `.qwen/agents/` — конфигурация ИИ-агентов
- **Context Bundle**: `tools/bundle_context.rs` — инструмент для сбора эталонной документации
- **Templates**: `docs/templates/` — шаблоны документов

**Примечание**: Перед разработкой нового модуля запустите сбор контекста:
```bash
cargo run --bin bundle_context
```
