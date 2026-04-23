## 📐 Specification: ToggleTool | smith-windows

**🎯 Purpose:** Управление состоянием переключателей (checkboxes, radio buttons, toggle switches) через UI Automation API. Позволяет устанавливать и проверять состояние включения/отключения элементов интерфейса.

**📥 Input:**
- `element` (`&uiautomation::UIElement`) | валидный элемент UI, поддерживающий TogglePattern или ToggleState | элемент чекбокса
- `config` (`ToggleConfig`) | конфигурация с таймаутом и токеном отмены | `ToggleConfig { timeout: 5s, cancellation: token }`

**📤 Output:**
- `()` (unit) | состояние элемента изменено на целевое (toggle/on/off) | `Ok(())` при успешной операции
- `ToggleError` | на ошибке: состояние элемента НЕ изменяется | `Err(ToggleError::ElementNotFound)` — элемент не найден

**⚠️ Boundaries:**
- Пустой `element` или `null`: `Err(ToggleError::ElementNotFound)`
- Отрицательный/ноль `timeout`: `Err(ToggleError::InvalidConfig("timeout must be > 0"))`
- `timeout > 1 час`: `Err(ToggleError::InvalidConfig("timeout must be <= 1 hour"))`
- Отмена операции через `CancellationToken`: `Err(ToggleError::Cancelled)` — состояние НЕ меняется
- Элемент отключён (`is_enabled() == false`): `Err(ToggleError::ElementNotEnabled)` — состояние НЕ меняется
- Элемент скрыт (`is_offscreen() == true`): `Err(ToggleError::ElementOffscreen)` — состояние НЕ меняется
- Элемент не поддерживает TogglePattern: `Err(ToggleError::ElementNotSupported)`
- Пустой текст/пустой `value` в конфиге для `set_value`: `Err(ToggleError::InvalidConfig("value cannot be empty"))`

**✅ Success criteria:**
- [ ] Все граничные случаи обрабатываются без паники
- [ ] На ошибке состояние элемента НЕ меняется (атомарность)
- [ ] Лог/метрика записывает результат или причину сбоя
- [ ] Поддержка всех паттернов: TogglePattern, ValuePattern (с is_readonly=false), LegacyIAccessible

---
## 🗓️ Implementation plan (for `/plan`)
- [ ] Create: `src/core/toggle.rs` — типы, трейты, валидация, моки
- [ ] Create: `src/runtime/backends/windows/toggle.rs` — реализация через `uiautomation`
- [ ] Add: `src/runtime/backends/mod.rs` — экспорт ToggleBackend
- [ ] Add: `src/runtime/backends/unsupported.rs` — stub для non-Windows
- [ ] Update: `src/lib.rs` — экспорт ToggleTool
- [ ] Update: `docs/design/toggle-tool/specification.md`
- [ ] Update: `docs/design/toggle-tool/contract.md`
- [ ] Update: `docs/design/toggle-tool/test-plan.md`
- [ ] Tests: `#[cfg(test)] mod tests` внутри `src/core/toggle.rs`
- [ ] Checks: `cargo test`, `cargo clippy -- -D warnings`
