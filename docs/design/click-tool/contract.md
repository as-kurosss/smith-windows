## 📜 Контракт: ClickTool | smith-windows

**🔹 Требования (ДО вызова):**
- `element` должен быть валидным UIA элементом (проверка через `element.is_valid()` или аналог)
- `config` должен быть валидным: `timeout > Duration::ZERO` и `timeout <= 1 час`
- `cancellation` должен быть инициализированным токеном отмены

**🔸 Гарантии (ПОСЛЕ):**
- Если `Ok(())`: клик выполнен через UI Automation API, состояние элемента могло измениться (в зависимости от реализации элемента)
- Если `Err(ClickError::ElementNotFound)`: состояние UI не изменилось, вызов завершился без сторонних эффектов
- Если `Err(ClickError::ElementNotEnabled)`: состояние UI не изменилось
- Если `Err(ClickError::ElementOffscreen)`: состояние UI не изменилось
- Если `Err(ClickError::Timeout)`: состояние UI не изменилось, ожидание завершилось
- Если `Err(ClickError::Cancelled)`: текущее выполнение прервано, состояние UI не изменилось
- Если `Err(ClickError::InvalidConfig)`: состояние UI не изменилось, ошибка обнаружена на этапе валидации

**🚫 Запреты:**
- ❌ Использование `unwrap()`, `expect()`, `panic!` в любом месте реализации
- ❌ Глобальное мутабельное состояние без `Arc` + `Mutex`/`RwLock`
- ❌ Прямые вызовы WinAPI (CoInitializeEx, CoCreateInstance, BSTR, VARIANT)
- ❌ Изменение конфига внутри функции
- ❌ Модификация UI элемента без явной необходимости (только клик)

**⚡ Сбои:**
- **Timeout**: вызов завершается с `ClickError::Timeout` через `tokio::time::timeout()`, состояние не меняется
- **Отмена**: проверка `cancellation.is_cancelled()` перед и во время выполнения, возврат `ClickError::Cancelled`
- **Некорректный config**: валидация в `validate_click_config()` → `ClickError::InvalidConfig` ДО вызова бэкенда
- **COM-ошибки**: перехват через `anyhow` → `ClickError::ComError(String)`

---
## 🗓️ Для `/plan`: ключевые точки валидации
- [ ] Валидация входа происходит в `src/core/click.rs` → `validate_click_config()`
- [ ] События/логи отправляются через `tracing::info!()`/`tracing::error!()` при начале и завершении
- [ ] Ошибки обрабатываются через `ClickError` (`thiserror`), не через `panic!`
- [ ] COM-вызовы изолированы в `src/runtime/backends/windows/click.rs` через `tokio::task::spawn_blocking`
- [ ] Проверка offscreen/disabled выполняется через `uiautomation` crate методы `is_offscreen()`, `is_enabled()`
