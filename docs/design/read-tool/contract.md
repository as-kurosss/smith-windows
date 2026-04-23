## 📜 Contract: ReadTool | smith-windows

**🔹 Requirements (ДО вызова):**
1. `element` должен быть валидным `uiautomation::UIElement`
2. `config.timeout` должен быть > 0 и <= 3600 секунд
3. `element` должен быть видимым и включённым (`is_enabled() == true`, `is_offscreen() == false`)
4. `element` должен поддерживать текстовое содержимое (TextPattern или Name/LocalizedControlType)
5. Вызов должен происходить в контексте `tokio` runtime с `full` features

**🔸 Guarantees (ПОСЛЕ):**
- If `Ok(String)`: текст элемента успешно считан, состояние системы не изменилось
- If `Err(ReadError)`: 
  - Состояние системы НЕ изменилось (идемпотентность)
  - Никаких side effects (файлы, сеть, мутабельные глобальные переменные)
  - Ошибка содержит конкретную причину (не generic COM error)

**🚫 Prohibitions (Что модуль НЕ делает):**
1. ❌ Использование `unwrap()`, `expect()`, `panic!` — только `Result`/`Option`
2. ❌ Прямой вызов COM API (`CoInitializeEx`, `CoCreateInstance`, `BSTR`, `VARIANT`)
3. ❌ Использование `GetForegroundWindow()` — использовать `is_enabled()`/`is_offscreen()` через UIA
4. ❌ Изменение контракта без явного согласия архитектора
5. ❌ Прямой вызов UIA методов внутри `spawn_blocking` — UIA типы `!Send`/`!Sync`

**⚡ Failures:**
- **Timeout**: возвращается `ReadError::Timeout`, состояние не меняется
- **Cancellation**: возвращается `ReadError::Cancelled`, состояние не меняется
- **Invalid config**: возвращается `ReadError::InvalidConfig`, backend не вызывается
- **Element not found**: возвращается `ReadError::ElementNotFound`, состояние не меняется
- **Element disabled**: возвращается `ReadError::ElementNotEnabled`, состояние не меняется
- **Element offscreen**: возвращается `ReadError::ElementOffscreen`, состояние не меняется
- **Element not writable**: возвращается `ReadError::ElementNotWritable`, состояние не меняется
- **COM error**: возвращается `ReadError::ComError(e.to_string())`, состояние не меняется

---
## 🗓️ For `/plan`: key validation checkpoints

- [ ] Input validation происходит в `validate_read_config()` ДО любого backend call
- [ ] Events/logs отправляются через `tracing::{info, error}!` с конкретными сообщениями
- [ ] All errors обрабатываются через `ReadError` (thiserror), не через `anyhow` или `panic!`
- [ ] COM calls изолируются через прямые вызовы (не spawn_blocking) — UIA types `!Send`/`!Sync`
- [ ] Idempotency проверяется: повторный вызов с теми же входными данными не меняет состояние
