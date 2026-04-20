## 🤖 Инструкция для агента: AutomationSession | smith-windows

**📁 Источники (читать в порядке приоритета):**
1. `docs/design/automation-session/specification.md` — вход/выход, границы, критерии
2. `docs/design/automation-session/contract.md` — требования, гарантии, запреты, сбои
3. `docs/design/automation-session/test-plan.md` — сценарии тестов, обязательные проверки
4. `docs/design/click-tool/specification.md` — аналогичный модуль ClickTool (рефакторинг шаблонов)
5. `AGENTS.md`, `ARCHITECTURE.md` — правила кода, стек, процесс

**🔗 Перекрёстные ссылки:**
- `docs/design/automation-session/brief.md` — текущий документ
- `docs/adr/001-click-tool.md` — архитектура ClickTool (spawn_blocking, validation pattern)

**🎯 Задача:**
Сгенерировать код реализации модуля AutomationSession для Rust automation на Windows. Модуль должен управлять жизненным циклом приложений — отдельно запускать процессы через `std::process::Command` (возврат `process_id`) и отдельно подключаться к существующим окнам через uiautomation crate 0.24.4 для выполнения операций автоматизации.

**📋 Формат вывода (строго):**
```
[File] → [Entities] → [cfg-flags] → [Tests] → [Validation]
```

**✅ Обязательные элементы в плане:**
- `SessionConfig { timeout: Duration, cancellation: CancellationToken }` с валидацией `timeout > 0 и <= 1 час`
- `SessionLaunchConfig { command: String, args: Option<Vec<String>>, working_dir: Option<String> }` с валидацией `command` и `args`
- `validate_session_config()` и `validate_title_filter()` в `src/core/automation_session.rs` (валидация ДО бэкенда)
- Единый `AutomationError` (`thiserror`) с точными именами из контракта: `ProcessLaunchFailed`, `WindowNotFound`, `ProcessNotFound`, `WindowDisabled`, `WindowOffscreen`, `InvalidConfig`, `Cancelled`, `SessionClosed`, `ComError`
- `SessionState { Running, Closed }` с `Arc<Mutex<SessionState>>` для управления состоянием сессии
- `RuntimeSession { process_id: u32, main_element: UIElement, state: Arc<Mutex<SessionState>> }` с методами `click()`, `type_text()`, `close()`, `find_element()`
- `launch_process(config: SessionLaunchConfig) → Result<u32, AutomationError>` — запуск процесса, возврат `process_id`
- `attach_by_title(title: String, mode: MatchMode, only_visible: bool, config: SessionConfig) → Result<RuntimeSession, AutomationError>` — присоединение к окну по заголовку (точное/частичное/regex)
- `attach_by_process_id(process_id: u32, config: SessionConfig) → Result<RuntimeSession, AutomationError>` — присоединение к окну по `process_id`
- Поиск окна через `uiautomation` crate методы `find_elements_by_title()`, `find_elements_by_class()` с фильтрацией `is_enabled()`, `is_offscreen()`
- `MatchMode` enum: `Exact`, `Contains`, `Regex` (regex через `regex` crate)
- Изоляция COM-вызовов через `tokio::task::spawn_blocking` в `src/runtime/backends/windows/automation_session.rs`
- Трейт `SessionBackend` в core для абстракции реализации
- Поддержка интеграции с инструментами через `main_element: &UIElement` (например, `ClickTool::click(&session.main_element, &config, cancellation)`)
- Тесты: `#[cfg(test)] mod tests` внутри `src/core/automation_session.rs` + `tests/integration/automation_session_tests.rs`
- `MockSessionBackend` с `Arc<Mutex<MockState>>` для проверки идемпотентности при `Err`

**🚫 Запреты:**
- ❌ Не генерировать код на этапе плана (только план, генерация через `/plan` → `smith-coder`)
- ❌ Не использовать `unwrap()`/`panic!`/`expect()` даже в примерах
- ❌ Не создавать `src/core/automation_session/tests.rs` (объединить в `mod tests` внутри `automation_session.rs`)
- ❌ Не менять контракт без явного согласования (проверить `docs/design/automation-session/contract.md`)
- ❌ Не использовать прямые WinAPI вызовы (CoInitializeEx, CoCreateInstance, BSTR, VARIANT)
- ❌ Не использовать `GetForegroundWindow()` — использовать `is_enabled()`/`is_offscreen()` через `uiautomation`
- ❌ Не использовать `std::process::Command` внутри `spawn_blocking` (Command — синхронный, не требует изоляции)
- ❌ Не включать `SerializableSession` в MVP — только `RuntimeSession` (по требованию)
- ❌ Не комбинировать launch и attach в один метод — всегда раздельно

**🔄 Процесс:**
1. Создать `src/core/automation_session.rs` с типами `SessionConfig`, `SessionLaunchConfig`, `AutomationError`, `SessionState`, `MatchMode`, `RuntimeSession` и функциями `validate_session_config()`, `validate_title_filter()`, `validate_regex()`, `launch_process()`
2. Создать `src/runtime/backends/windows/automation_session.rs` с реализацией `SessionBackendWindows::launch_process()`, `SessionBackendWindows::attach_by_title()`, `SessionBackendWindows::attach_by_process_id()`, изолирующей COM-вызовы
3. Добавить тесты в `mod tests` внутри `src/core/automation_session.rs` для валидации config, title_filter, regex
4. Добавить интеграционные тесты в `tests/integration/automation_session_tests.rs` для всех сценариев (launch, attach, click, type_text, close)
5. Обновить `src/core/mod.rs` для экспорта `automation_session`
6. Запустить `cargo test && cargo clippy -- -D warnings` для проверки

**📝 Метаданные:**
- Автор: smith-planner
- Дата: 2026-04-20
- Статус: `[awaiting_approval]`
