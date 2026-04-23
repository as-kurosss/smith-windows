## 📜 Contract: ToggleTool | smith-windows

**🔹 Requirements (BEFORE call):**
- `element` должен быть валидным `UIElement` (не `null`, не удалён)
- `element.is_enabled() == true` (элемент активен)
- `element.is_offscreen() == false` (элемент видим на экране)
- `timeout` должен быть `> 0` и `<= 1 час` (3600 секунд)
- `cancellation` должен быть корректным `CancellationToken`
- Для `set_value`: `value` должен быть непустой строкой

**🔸 Guarantees (AFTER):**
- Если `Ok(())`: состояние элемента изменено на целевое (toggle/on/off) в зависимости от паттерна
- Если `Err`:
  - Состояние элемента НЕ меняется (атомарность)
  - Никаких побочных эффектов (не меняется clipboard, не создается окон и т.д.)
  - Лог/метрика фиксирует причину ошибки
- Операция idempotent: повторный вызов с теми же входными данными даёт тот же результат

**🚫 Prohibitions:**
- ❌ Использование `unwrap()`, `expect()`, `panic!` в `src/` (только `Result`/`Option`)
- ❌ Прямой вызов `CoInitializeEx`, `CoCreateInstance`, `BSTR`, `VARIANT` в бэкендах
- ❌ Передача `UIElement` через `spawn_blocking` (он `!Send`)
- ❌ Изменение контракта без обсуждения
- ❌ Глобальное мутабельное состояние без `Arc` + явного контекста
- ❌ Использование `GetForegroundWindow()` — использовать `is_enabled()`/`is_offscreen()` через `uiautomation`

**⚡ Failures:**
- `Timeout`: при превышении `timeout` → `Err(ToggleError::Timeout)`, состояние НЕ меняется
- `Cancelled`: при `cancellation.is_cancelled()` → `Err(ToggleError::Cancelled)`, состояние НЕ меняется
- `InvalidConfig`: при неверных аргументах → `Err(ToggleError::InvalidConfig)`, состояние НЕ меняется
- `ComError`: при COM-ошибке с сообщением об ошибке → `Err(ToggleError::ComError)`, состояние НЕ меняется
