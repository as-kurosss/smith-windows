## 📜 Contract: ClipboardTool | smith-windows

**🔹 Requirements (ДО вызова):**
1. `config.timeout` должен быть > 0 и <= 3600 секунд
2. `text` для `set_text()` должен быть непустой строкой
3. Вызов должен происходить в контексте `tokio` runtime с `full` features

**🔸 Guarantees (ПОСЛЕ):**
- If `Ok(String/bool/())`: операция выполнена успешно, состояние системы не изменилось
- If `Err(ClipboardError)`:
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
- **Timeout**: возвращается `ClipboardError::Timeout`, состояние не меняется
- **Cancellation**: возвращается `ClipboardError::Cancelled`, состояние не меняется
- **Invalid config**: возвращается `ClipboardError::InvalidConfig`, backend не вызывается
- **Clipboard empty**: возвращается `ClipboardError::ClipboardEmpty`, состояние не меняется
- **Clipboard access denied**: возвращается `ClipboardError::ClipboardAccessDenied`, состояние не меняется
- **Text empty**: возвращается `ClipboardError::TextEmpty`, backend не вызывается
- **Operation not supported**: возвращается `ClipboardError::OperationNotSupported` (non-Windows)
- **COM error**: возвращается `ClipboardError::ComError(e.to_string())`, состояние не меняется

---
## 🗓️ For `/plan`: key validation checkpoints

- [ ] Input validation происходит в `validate_clipboard_config()` ДО любого backend call
- [ ] Events/logs отправляются через `tracing::{info, error}!` с конкретными сообщениями
- [ ] All errors обрабатываются через `ClipboardError` (thiserror), не через `anyhow` или `panic!`
- [ ] Clipboard API (синхронный) вызывается напрямую без `spawn_blocking` (проверить в docs.rs)
- [ ] Idempotency проверяется: повторный вызов с теми же входными данными не меняет состояние
