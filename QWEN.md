# 🏗️ smith-windows

## 📋 Project Overview

`smith-windows` — это библиотека на Rust для автоматизации Windows через UI Automation API. Проект находится в активной разработке и представляет собой MVP-надстройку над `uiautomation` crate (v0.24.4) для интеграции в экосистему `smith-core`.

### 🔑 Key Characteristics

- **Язык**: Rust 1.95.0
- **Async runtime**: Tokio (полная настройка)
- **Error handling**: `thiserror` для библиотеки, `anyhow` для приложений
- **Логирование**: `tracing` с `tracing-subscriber`
- **UI Automation**: `uiautomation` crate 0.24.4
- **Сериализация**: `serde` + `serde_json`
- **Асинхронные интерфейсы**: `async-trait`

### 🎯 Architecture Goals

1. **Contracts First**: Поведение определяется через спецификации и контракты ДО реализации
2. **Zero Silent Failures**: Все ошибки явные через `Result`, запрещены `unwrap()`, `panic!`, `expect()` в `src/`
3. **Idempotency**: Повторные вызовы с теми же входными данными не изменяют состояние системы
4. **Modularity**: Чёткие границы между модулями, единичная ответственность
6. **COM Safety**: Все вызовы COM/WinAPI изолируются через `tokio::task::spawn_blocking`

### 🏗️ Project Structure

```
smith-windows/
├── src/
│   ├── core/              # Трейты, типы, ошибки, моки, тесты
│   │   ├── click.rs       # ClickTool: валидация, трейты, моки
│   │   └── r#type.rs      # TypeTool: валидация, трейты, моки
│   └── runtime/           # Windows-реализации + unsupported stub
│       ├── mod.rs         # Exports backends
│       └── backends/
│           ├── windows/   # Реализации через uiautomation crate
│           │   ├── mod.rs
│           │   ├── click.rs
│           │   └── type.rs
│           └── unsupported.rs  # Stub для non-Windows
├── tests/                 # Интеграционные тесты
├── docs/
│   ├── design/            # Рабочие документы модулей (создаются по мере разработки)
│   │   └── <module>/
│   │       ├── specification.md    # Вход/выход, границы
│   │       ├── contract.md         # Требования, гарантии, запреты, сбои
│   │       ├── test-plan.md        # Сценарии тестов, проверки
│   │       └── brief.md            # Инструкция для кодера
│   ├── templates/         # Шаблоны документов
│   │   ├── specification-template.md
│   │   ├── contract-template.md
│   │   ├── test-plan-template.md
│   │   └── brief-template.md
│   └── adr/               # Architecture Decision Records (создаются после утверждения)
├── tools/                 # Вспомогательные инструменты разработки
│   └── bundle_context.rs  # Сборщик контекста проекта (для ИИ-агентов)
├── .qwen/agents/          # Конфигурация ИИ-агентов
│   ├── smith-architect.md      # Архитектор спецификаций и документации
│   ├── smith-planner.md        # Планировщик реализации
│   ├── smith-coder.md          # Rust-инженер (TDD + документация)
│   ├── smith-crate-researcher.md # Исследователь crate-зависимостей
│   ├── smith-debugger.md       # Автономная отладка
│   └── smith-compliance.md     # Проверка архитектурной compliance
├── Cargo.toml
├── AGENTS.md              # Правила для ИИ-агентов
├── ARCHITECTURE.md        # Архитектурная спецификация
└── QWEN.md                # Этот файл
```

### 🔄 Development Workflow

Проект следует строгой процедуре разработки:

```
1. Architect → specification.md + contract.md + test-plan.md
   (docs/design/<module>/)

2. Planner → /plan (текстовый) + brief.md
   (формат: [Файл] → [Сущности] → [cfg-флаги] → [Тесты] → [Валидация])

3. Architect утверждает plan

4. Coder → production code + tests + documentation updates
   (строго по утверждённому плану + README/CHANGELOG/context_bundle)

5. Verification → cargo test && cargo clippy -- -D warnings

6. ADR → docs/adr/XXX-<module>.md (запись решения)
```

**Важно**: Генерация кода ДОЛЖНА происходить ТОЛЬКО после утверждения плана архитектором.

#### 🤖 Agent Workflow

Проект использует специализированных ИИ-агентов для разработки и поддержки качества:

```
┌─────────────────────────────────────────────────────────────────┐
│                    Development Workflow                         │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  smith-architect → smith-planner → smith-coder                 │
│       (docs)        (plan)          (code + docs)              │
│                                            ↓                   │
│                               GitHub Actions (CI)               │
│                                            ↓                   │
│                  smith-debugger / smith-compliance             │
│                    (QA / Maintenance)                          │
│                                                                 │
│  smith-crate-researcher ────────────────→ docs/crates/         │
│       (crate docs from docs.rs)        (added to context)       │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

**Агенты (`.qwen/agents/`):**

| Агент | Роль | Ключевые задачи |
|-------|------|-----------------|
| **smith-architect** | Архитектор спецификаций | Создаёт `specification.md`, `contract.md`, `test-plan.md`, `brief.md`; обновляет README, CHANGELOG, context_bundle |
| **smith-planner** | Планировщик | Преобразует спеку в `/plan`; генерирует `brief.md` для кодера |
| **smith-coder** | Senior Rust-инженер | Генерирует код + тесты по утверждённому плану; автоматически обновляет документацию |
| **smith-crate-researcher** | Исследователь зависимостей | Читает `docs.rs`, создаёт `docs/crates/*.md`, поддерживает актуальную документацию библиотек |
| **smith-debugger** | Автономная отладка | Запускает тесты, находит ошибки, генерирует фиксы, верифицирует — полностью автономный цикл |
| **smith-compliance** | Проверка compliance | Сканирует код/документы на нарушения; отчитывает по CRITICAL/WARNING/INFO |

**Процессы:**

**New Module Development:**
1. Architect → `specification.md` + `contract.md` + `test-plan.md` + `brief.md`
2. Planner → `/plan` (текстовый)
3. Architect утверждает plan
4. Coder → code + tests + README/CHANGELOG/context_bundle updates
5. Verification: `cargo test && cargo clippy -- -D warnings`
6. ADR → `docs/adr/XXX-<module>.md`

**Bug Fix / Debugging (smith-debugger):**
1. smith-debugger → `cargo test` (находит падения)
2. Анализ причины → чтение файлов
3. Генерация фикса → применение к коду/тесту
4. Верификация: `cargo test` + `cargo clippy`
5. Отчёт с доверием (high/medium/low)

**Compliance Check (smith-compliance):**
1. smith-compliance → сканирование `src/` и `docs/`
2. Проверка AGENTS.md/ARCHITECTURE.md правил
3. Запуск `cargo test`, `cargo clippy`, `cargo fmt`
4. Категоризация: CRITICAL / WARNING / INFO
5. Отчёт с compliance score (%)

### 📦 Core Concepts

#### Runtime Architecture

Вся реализация организована по принципу "backends":

- `src/core/` — трейты, типы, валидация (платформо-независимая часть)
- `src/runtime/backends/windows/` — Windows-реализация через `uiautomation`
- `src/runtime/backends/unsupported.rs` — stub для non-Windows

```
SessionHandle (core) ← uses → SessionBackend (runtime/backends)
ClickTool (core) ← uses → ClickBackend (runtime/backends/windows)
TypeTool (core) ← uses → TypeBackend (runtime/backends/windows)
```

**Изоляция COM:** Все вызовы `uiautomation`/WinAPI изолируются через `tokio::task::spawn_blocking`.

#### Task Lifecycle

```
Created → Queued → Running → [Completed | Failed | Cancelled]
```

#### Error Handling

Используется `thiserror` для определения специфичных типов ошибок:

```rust
#[derive(thiserror::Error)]
pub enum ClickError {
    #[error("invalid input selector: {0}")]
    InputSelectorError(String),
    
    #[error("element not found")]
    ElementNotFoundError,
    
    #[error("backend unavailable: {0}")]
    BackendUnavailableError(String),
    
    #[error("element not clickable")]
    ElementNotClickableError,
    
    #[error("click execution failed: {0}")]
    ClickExecutionError(String),
    
    #[error("unsupported platform")]
    UnsupportedPlatformError,
}
```

Каждая функция возвращает `Result<T, ClickError>` (пример для инструмента ClickTool).

#### Async & Threading

- **Async**: Все публичные API — async через `tokio`
- **COM Isolation**: Все вызовы COM/WinAPI изолируются через `tokio::task::spawn_blocking`
- **Cancellation**: Поддержка `CancellationToken` для раннего прерывания операций
- **Timeouts**: Явная настройка таймаутов через `ClickConfig { timeout: Duration, cancellation: CancellationToken }`

### 🧪 Testing Strategy

#### Unit Tests

- Встроенные тесты внутри `src/core/<module>.rs` через `#[cfg(test)] mod tests`
- Минимум 1 позитивный + 1 негативный тест на публичную функцию
- Граничные случаи: `0`, пустые значения, максимум
- Тесты отмены через `CancellationToken`
- Тесты таймаута (`Duration::ZERO`)

#### Integration Tests

- Расположение: `tests/integration/`
- Покрытие полного lifecycle: create → queue → run → complete/error/cancel
- Проверка идемпотентности при повторных вызовах

#### Mocking

- `MockBackend` с `Arc<Mutex<MockState>>` для изоляции состояния
- Проверка идемпотентности: повторный вызов `Err` не меняет состояние
- Проверка отсутствия сайд-эффектов при ошибках

#### Verification Commands

```bash
# Запуск всех тестов
cargo test

# Линтинг с жесткими правилами
cargo clippy -- -D warnings

# Проверка компиляции без тестов
cargo check
```

### 🚫 Strict Prohibitions

**В `src/` и `tests/`:**

- ❌ `unwrap()`, `expect()`, `panic!` — только `Result`/`Option`
- ❌ Глобальные мутабельные состояния без `Arc` + явного контекста
- ❌ Изменение контракта без обсуждения
- ❌ Генерация кода без утверждённого плана `/plan`
- ❌ Прямой вызов `CoInitializeEx`, `CoCreateInstance`, `BSTR`, `VARIANT` в бэкендах
- ❌ Использование `GetForegroundWindow()` — использовать `is_enabled()`/`is_offscreen()` через `uiautomation`

**В документации:**

- ❌ Расхождения между `brief.md`, `specification.md`, `contract.md` и `/plan`
- ❌ Отсутствие тест-планов для модулей
- ❌ Документы без явных критериев успеха

### 🛠️ Building and Running

#### Prerequisites

- Rust 1.95.0+ (edition 2021)
- Windows 10/11 (для `uiautomation` crate)
- Rust toolchain с компонентом `clippy`

#### Build

```bash
# Разработка
cargo build

# Продакшн
cargo build --release
```

#### Test

```bash
# Все тесты
cargo test

# Только юнит-тесты
cargo test --lib

# Только интеграционные тесты
cargo test --test integration

# Тесты с логированием
RUST_LOG=debug cargo test
```

#### Lint

```bash
# Строгий линтинг (предупреждения = ошибка)
cargo clippy -- -D warnings

# Стандартный линтинг
cargo clippy
```

#### Check

```bash
# Быстрая проверка компиляции
cargo check

# Проверка для конкретной платформы
cargo check --target x86_64-pc-windows-msvc
```

### 📊 Configuration Files

#### `Cargo.toml`

```toml
[package]
name = "smith-windows"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1", features = ["full"] }
tokio-util = "0.7"
thiserror = "1"
anyhow = "1"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
async-trait = "0.1"
uiautomation = "0.24.4"

[lib]
name = "smith_windows"
path = "src/lib.rs"
```

### 📝 Development Conventions

#### Code Style

- **Naming**: `snake_case` для функций/переменных, `PascalCase` для типов
- **Comments**: Минимальные, фокус на "почему", а не "что"
- **Error Messages**: Ясные, конкрементные, на английском
- **Logging**: `info!` для результатов, `error!` для ошибок, `debug!` для деталей

#### File Organization

- Один модуль = одна ответственность
- Файлы: `src/core/<module>.rs` (трейты, типы, валидация)
- Бэкенды: `src/runtime/backends/<platform>/<module>.rs`
- Stub: `src/runtime/backends/unsupported.rs` для non-Windows платформ

#### Testing Standards

- `#[cfg(test)] mod tests` внутри модуля ИЛИ `tests/integration/`
- Каждая функция: минимум 2 теста (позитивный + негативный)
- Граничные случаи обязательны
- Моки: `Arc<Mutex<MockState>>` для идемпотентности

#### Platform-Specific Code

```rust
// Windows-реализация
#[cfg(target_os = "windows")]
pub async fn click_element(...) -> Result<(), ClickError> {
    // uiautomation calls via spawn_blocking
}

// Stub для non-Windows
#[cfg(not(target_os = "windows"))]
pub async fn click_element(...) -> Result<(), ClickError> {
    Err(ClickError::UnsupportedPlatformError)
}
```

### 🌐 Integration with smith-core

`smith-windows` — это бэкенд для `smith-core`. Основные интеграционные точки (предстоит реализовать):

1. **SessionHandle**: Управление сессией UI-автоматизации
2. **ClickTool**: Инструмент клика по элементам  
3. **TypeTool**: Инструмент ввода текста

Документация интеграции (создаётся по мере разработки):
- `docs/design/automation-session/brief.md`
- `docs/design/click-tool/spec.md`
- `docs/design/click-tool/contract.md`
- `docs/design/click-tool/test-plan.md`

### 🔧 Troubleshooting

#### Common Build Issues

**Issue**: `uiautomation` crate fails to compile
- **Solution**: Убедитесь, что у вас Windows 10/11. Библиотека работает только на Windows.

**Issue**: `tokio` features conflict
- **Solution**: Используйте `tokio = { version = "1", features = ["full"] }` как в `Cargo.toml`.

#### Common Test Issues

**Issue**: Tests fail with "backend unavailable"
- **Solution**: Проверьте, что запущен на Windows и UI Automation API доступен.

**Issue**: Flaky tests с таймаутами
- **Solution**: Увеличьте `timeout` в `ClickConfig` для медленных систем.

#### Linting Issues

**Issue**: `clippy` warns about `unwrap()`/`expect()`
- **Solution**: Замените на явную обработку через `match` или `?` оператор.

**Issue**: Warnings in tests
- **Solution**: Добавьте `#[allow(clippy::...)]` только если обосновано, иначе исправьте код.

#### Common CI/CD Issues

**Issue**: GitHub Actions fails with "context_bundle.md not found"
- **Solution**: Run `cargo run --bin bundle_context` locally and commit the updated file.

**Issue**: PR fails clippy checks with "deprecated method" warnings
- **Solution**: Replace deprecated methods with recommended alternatives from clippy output.

### 📚 Additional Resources

- **Agents**: `AGENTS.md` — правила для ИИ-агентов
- **Agents**: `.qwen/agents/` — конфигурация ИИ-агентов (smith-architect, smith-planner, smith-coder, smith-debugger, smith-compliance)
- **Architecture**: `ARCHITECTURE.md` — детальная архитектура
- **Templates**: `docs/templates/` — шаблоны документов
- **CI/CD**: `.github/workflows/ci.yml` — GitHub Actions для CI, `.github/workflows/context-update.yml` — автообновление context_bundle

### 📦 Context Bundle Tool

Проект включает инструмент `tools/bundle_context.rs` для сбора эталонной документации:

**Назначение**:
- Автоматически объединяет все `.md` файлы из указанных путей в один контекстный бандл
- Используется ИИ-агентами для получения актуальной документации проекта
- Генерирует `context_bundle.md` с форматированным содержимым всех документов

**Команда запуска**:
```bash
cargo run --bin bundle_context
```

**Конфигурация**:
Пути для сбора задаются в константе `INCLUDE_PATHS`:
```rust
const INCLUDE_PATHS: &[&str] = &[
    "AGENTS.md",
    "ARCHITECTURE.md",
    "docs/templates/",           // Папка: соберёт все .md внутри
    "docs/design/click-tool/",   // Папка с модуля ClickTool
    "docs/design/automation-session/", // Папка модуля AutomationSession
    "docs/adr/",              // Папка ADR
];
```

**Важно**: ИИ-агенты могут использовать `context_bundle.md` как эталонную документацию для разработки новых модулей.

---

**Статус проекта**: ✅ ClickTool Refactoring Complete (v0.2.0)
- Обновлён ClickTool с поддержкой трёх типов кликов (LeftSingle, RightSingle, LeftDouble)
- RightClickTool теперь обёртка вокруг ClickTool
- Все тесты проходят (116/116)
- Все примеры обновлены
- Документация обновлена
- Context bundle перегенерирован
**Контрактный статус**: Планирование → Утверждение → Реализация
