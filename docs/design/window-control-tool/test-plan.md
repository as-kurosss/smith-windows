## 🧪 Test Plan: WindowControlTool | smith-windows

**✅ Positive:** `element` (valid window with WindowPattern) + `WindowControlAction::Maximize` → `Ok(())`, окно развернуто
**✅ Positive:** `element` (valid window with WindowPattern) + `WindowControlAction::Restore` → `Ok(())`, окно восстановлено
**✅ Positive:** `element` (valid window with WindowPattern) + `WindowControlAction::Minimize` → `Ok(())`, окно свернуто
**✅ Positive:** `element` (окно уже в целевом состоянии) + `Maximize` → `Ok(())`, окно не меняется (идемпотентность)
**✅ Positive:** `element` (окно отключено/включено повторно) + `Restore` → `Ok(())`, окно восстанавливается

**🔄 Boundary:** `config.timeout = Duration::ZERO` → `Err(WindowControlError::InvalidConfig)`
**🔄 Boundary:** `config.timeout = Duration::from_secs(3601)` (> 1 час) → `Err(WindowControlError::InvalidConfig)`
**🔄 Boundary:** `config.timeout = Duration::from_secs(1)` → валидный таймаут, выполнение происходит
**🔄 Boundary:** `cancellation` отменяется во время выполнения → `Err(WindowControlError::Cancelled)`
**🔄 Boundary:** `element` без `WindowPattern` → `Err(WindowControlError::WindowPatternNotAvailable)`

**❌ Negative:** `element` (отключено окно) + `Maximize` → `Err(WindowControlError::WindowNotEnabled)`
**❌ Negative:** `element` (offscreen окно) + `Restore` → `Err(WindowControlError::WindowOffscreen)`
**❌ Negative:** `element` (невалидный UIElement) + `Minimize` → `Err(WindowControlError::WindowPatternNotAvailable)`
**❌ Negative:** `action = WindowControlAction::Maximize` + `timeout = Duration::from_millis(1)` → `Err(WindowControlError::Timeout)`
**❌ Negative:** `cancellation.cancel()` вызван до выполнения → `Err(WindowControlError::Cancelled)`

**🔍 Mandatory checks:**
- [ ] On `Err`, состояние окна НЕ меняется (идемпотентность)
- [ ] No duplicate events/logs (каждый вызов фиксируется ровно 1 раз)
- [ ] No `unwrap()`, `panic!`, or blocking in async
- [ ] `validate_window_control_config()` вызывается ДО любых backend вызовов
- [ ] `WindowPattern` проверяется через `get_pattern::<UIWindowPattern>()` в бэкенде
- [ ] `MockWindowControlBackend` с `Arc<Mutex<MockWindowControlState>>` для проверки идемпотентности на `Err`
- [ ] Unit tests покрывают все ветки валидации (timeout bounds, element state, pattern availability)
- [ ] Integration tests покрывают полный lifecycle (create → execute → complete/error/cancel)

**🔍 Test scenarios for WindowPattern:**
- [ ] Окно с WindowPattern доступен → `Ok(())`
- [ ] Окно без WindowPattern → `Err(WindowControlError::WindowPatternNotAvailable)`
- [ ] WindowPattern возвращается после `get_pattern()` → вызов `set_show_window()`
- [ ] Ошибка `get_pattern()` перехватывается → `WindowControlError::ComError`

**🔍 Test scenarios for actions:**
- [ ] `WindowControlAction::Maximize` вызывает `set_show_window(WindowShow::Maximize)`
- [ ] `WindowControlAction::Restore` вызывает `set_show_window(WindowShow::Restore)`
- [ ] `WindowControlAction::Minimize` вызывает `set_show_window(WindowShow::Minimize)`

**🔍 Test scenarios for state validation:**
- [ ] `is_enabled() == false` → `Err(WindowControlError::WindowNotEnabled)`
- [ ] `is_offscreen() == true` → `Err(WindowControlError::WindowOffscreen)`
- [ ] `is_enabled() == true` AND `is_offscreen() == false` → продолжение выполнения

---
## 🗓️ Для `/plan`: tests as steps
- [ ] Создать `src/core/window_control.rs` с модулем `#[cfg(test)] mod tests`
- [ ] Реализовать unit tests для `validate_window_control_config()`: zero timeout, large timeout, valid timeout
- [ ] Реализовать unit tests для `MockWindowControlBackend`: creation, state, reset
- [ ] Добавить integration tests в `tests/integration/window_control/`
- [ ] Запустить `cargo test -- --nocapture`
