## 🧪 Test Plan: ToggleTool | smith-windows

### ✅ Positive Scenarios:
- **Toggle on checkbox (unchecked → checked):** `toggle_element` → `Ok(())`, состояние `IsToggled == true`
- **Toggle off checkbox (checked → unchecked):** `toggle_element` → `Ok(())`, состояние `IsToggled == false`
- **Set on radio button:** `set_radio(element, true)` → `Ok(())`, состояние `IsSelected == true`
- **Set off radio button:** `set_radio(element, false)` → `Ok(())`, состояние `IsSelected == false`
- **Set toggle switch:** `set_toggle(element, true)` → `Ok(())`, состояние `IsToggled == true`
- **Set toggle switch off:** `set_toggle(element, false)` → `Ok(())`, состояние `IsToggled == false`
- **Verify state (is_checked):** `is_checked(element)` → `Ok(true/false)` для checkbox
- **Verify state (is_selected):** `is_selected(element)` → `Ok(true/false)` для radio button
- **Valid timeout (5s):** `Ok(())` при успешной операции в пределах таймаута
- **Idempotency:** Повторный вызов с теми же данными не меняет состояние после первого вызова

### 🔄 Boundary Scenarios:
- **Empty element (null pointer):** `Err(ToggleError::ElementNotFound)`
- **Zero timeout:** `Err(ToggleError::InvalidConfig("timeout must be > 0"))`
- **Timeout = 1 hour (edge):** `Ok(())` при успешной операции в пределах таймаута
- **Timeout > 1 hour (3601s):** `Err(ToggleError::InvalidConfig("timeout must be <= 1 hour"))`
- **Empty value for set_value:** `Err(ToggleError::InvalidConfig("value cannot be empty"))`
- **Max timeout (3600s):** `Ok(())` при успешной операции

### ❌ Negative Scenarios:
- **Element disabled:** `Err(ToggleError::ElementNotEnabled)`, состояние НЕ меняется
- **Element offscreen:** `Err(ToggleError::ElementOffscreen)`, состояние НЕ меняется
- **Element not supported (no TogglePattern):** `Err(ToggleError::ElementNotSupported)`, состояние НЕ меняется
- **Element does not support ValuePattern (for set_value):** `Err(ToggleError::ElementNotWritable)`, состояние НЕ меняется
- **Operation cancelled mid-way:** `Err(ToggleError::Cancelled)`, состояние НЕ меняется
- **COM error (e.g., element deleted):** `Err(ToggleError::ComError)` с сообщением об ошибке, состояние НЕ меняется
- **Invalid config (negative timeout):** `Err(ToggleError::InvalidConfig)` при отрицательном таймауте

### 🔍 Mandatory Checks:
- [ ] На `Err` состояние элемента НЕ меняется
- [ ] Нет дублируемых событий/логов
- [ ] Нет `unwrap()`, `panic!`, блокирующих вызовов в async коде
- [ ] `#[async_trait::async_trait(?Send)]` для всех трейтов (COM safety)
- [ ] Все ошибки через `thiserror::Error`
- [ ] Валидация config ДО любого backend вызова
- [ ] Мок-backend идемпотентен при повторных ошибках
- [ ] Тесты `cargo test -- --nocapture` работают без вывода в stderr
- [ ] `cargo clippy -- -D warnings` без предупреждений

---
## 🗓️ For `/plan`: tests as steps
- [ ] Create test file: `src/core/toggle.rs` (inline `#[cfg(test)] mod tests`)
- [ ] Implement tests: `test_validate_toggle_config_valid`, `test_validate_toggle_config_zero_timeout`, `test_validate_toggle_config_large_timeout`
- [ ] Implement backend tests: `test_mock_backend_creation`, `test_mock_backend_reset`, `test_mock_backend_with_state`
- [ ] Run `cargo test -- --nocapture`
