## 🤖 Инструкция для агента: WindowControlTool | smith-windows

**📁 Источники (чтение в приоритетном порядке):**
1. `docs/design/window-control-tool/specification.md` — вход/выход, границы, критерии
2. `docs/design/window-control-tool/contract.md` — требования, гарантии, запреты, сбои
3. `docs/design/window-control-tool/test-plan.md` — сценарии тестов, обязательные проверки
4. `docs/adr/XXX-window-control.md` — архитектурные решения
5. `AGENTS.md`, `ARCHITECTURE.md` — правила кода, стек, процесс

**🔗 Кросс-ссылки:**
- `docs/design/focus-tool/brief.md` — похожая архитектура с UI Automation окнами
- `docs/design/click-tool/specification.md` — шаблон валидации и обработки ошибок

**🎯 Задача:**
Создать документацию и реализацию `WindowControlTool` для управления состоянием окон Windows (maximize/restore/minimize) через UI Automation API.

**📋 Формат вывода (строго):**
```
[File] → [Сущности] → [cfg-флаги] → [Тесты] → [Валидация]
```

**✅ Обязательные элементы плана:**
- `validate_window_control_config()` в `src/core/window_control.rs` (валидация ДО бэкенда)
- `WindowControlError` (`thiserror`) с точными именами из контракта
- `WindowControlAction` enum: `Maximize`, `Restore`, `Minimize`
- `WindowControlConfig { timeout: Duration, cancellation: CancellationToken }`
- `WindowControlBackend::window_control(element, action)` в бэкендах
- COM вызовы изолированы через прямой вызов (UIA требует STA affinity, `?Send` для async traits)
- Тесты: `#[cfg(test)] mod tests` внутри `src/core/window_control.rs` + `tests/integration/`
- `MockWindowControlBackend` с `Arc<Mutex<MockWindowControlState>>` для проверки идемпотентности на `Err`
- Windows backend использует `UIWindowPattern.set_show_window()` через `uiautomation` crate

**🚫 Запреты:**
- Не генерировать код на этапе планирования
- Не использовать `unwrap()`, `panic!`, `expect()` даже в примерах
- Не создавать `src/core/window_control/tests.rs` (объединить в `mod tests` или перенести в `tests/`)
- Не изменять контракт без явного согласия архитектора
- Не использовать `spawn_blocking` для UIA вызовов (UIElement is !Send/!Sync)
- Не использовать `GetForegroundWindow()` или Win32 APIs для управления окнами

**🔄 Процесс:**
1. Создать файл `src/core/window_control.rs` с `WindowControlAction` enum, `WindowControlConfig`, `WindowControlError`
2. Реализовать `validate_window_control_config()` для валидации таймаута (0 < timeout <= 3600)
3. Добавить `WindowControlBackend` trait с методом `window_control(element: &UIElement, action: WindowControlAction)`
4. Создать `MockWindowControlBackend` для unit тестов
5. Создать файл `src/runtime/backends/windows/window_control.rs` с Windows реализацией
6. Реализовать Windows backend через `element.get_pattern::<UIWindowPattern>().set_show_window()`
7. Добавить unit тесты в `src/core/window_control.rs` для всех веток валидации
8. Добавить integration тесты для полного lifecycle
9. Обновить `src/core/mod.rs` для экспорта модуля
10. Обновить `src/lib.rs` для экспорта `WindowControlTool`
11. Обновить `README.md` и `CHANGELOG.md` с новым модулем
12. Запустить `cargo test` и `cargo clippy -- -D warnings`

**📝 Metadata:**
- Author: smith-architect
- Date: 2026-04-22
- Status: `[draft]`
