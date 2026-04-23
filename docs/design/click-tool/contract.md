## 📜 Контракт: ClickTool | smith-windows

**🔹 Требования (ДО вызова):**
- `element` должен быть валидным UIA элементом (проверка через `element.get_control_type()` или аналог)
- `config` должен быть валидным: `timeout > Duration::ZERO` и `timeout <= 1 час`
- `cancellation` должен быть инициализированным токеном отмены
- `config.click_type` должен быть допустимым значением `ClickType`

**🔸 Гарантии (ПОСЛЕ):**
- Если `Ok(())`: клик выполнен через UI Automation API, состояние элемента могло измениться (в зависимости от реализации элемента)
- Если `Err(ClickError::ElementNotFound)`: состояние UI не изменилось, вызов завершился без сторонних эффектов
- Если `Err(ClickError::ElementNotEnabled)`: состояние UI не изменилось
- If `Err(ClickError::ElementOffscreen)`: состояние UI не изменилось
- If `Err(ClickError::Timeout)`: состояние UI не изменилось, ожидание завершилось
- If `Err(ClickError::Cancelled)`: текущее выполнение прервано, состояние UI не изменилось
- If `Err(ClickError::InvalidConfig)`: состояние UI не изменилось, ошибка обнаружена на этапе валидации

**🔄 Поддерживаемые типы кликов:**
- `ClickType::LeftSingle`: одинарный левый клик через `element.click()`
- `ClickType::RightSingle`: одинарный правый клик через `element.right_click()`
- `ClickType::LeftDouble`: двойной левый клик через `element.double_click()`

**🚫 Запреты:**
- ❌ Использование `unwrap()`, `expect()`, `panic!` в любом месте реализации
- ❌ Глобальное мутабельное состояние без `Arc` + `Mutex`/`RwLock`
- ❌ Прямые вызовы WinAPI (CoInitializeEx, CoCreateInstance, BSTR, VARIANT)
- ❌ Изменение конфига внутри функции
- ❌ Модификация UI элемента без явной необходимости (только клик)
- ❌ Использование `spawn_blocking` для UIA вызовов (UIElement is !Send/!Sync)

**⚡ Сбои:**
- **Timeout**: вызов завершается с `ClickError::Timeout` через `tokio::time::timeout()`, состояние не меняется
- **Отмена**: проверка `cancellation.is_cancelled()` перед и во время выполнения, возврат `ClickError::Cancelled`
- **Некорректный config**: валидация в `validate_click_config()` → `ClickError::InvalidConfig` ДО вызова бэкенда
- **COM-ошибки**: перехват через `anyhow` → `ClickError::ComError(String)`

---
## 🗓️ Для `/plan`: ключевые точки валидации
- [x] Валидация входа происходит в `src/core/click.rs` → `validate_click_config()`
- [x] События/логи отправляются через `tracing::info!()`/`tracing::error!()` при начале и завершении
- [x] Ошибки обрабатываются через `ClickError` (`thiserror`), не через `panic!`
- [x] COM-вызовы выполняются на том же потоке (UIA требует STA affinity)
- [x] Проверка offscreen/disabled выполняется через `uiautomation` crate методы `is_offscreen()`, `is_enabled()`
