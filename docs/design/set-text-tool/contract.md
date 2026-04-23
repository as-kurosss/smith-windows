## 📜 Контракт: SetTextTool | smith-windows

**🔹 Требования (ДО вызова):**
- `element` должен быть валидным UIA элементом (проверка через `element.is_valid()` или аналог)
- `text` должна быть непустой строкой (`!text.is_empty()`)
- `config` должен быть валидным: `timeout > Duration::ZERO` и `timeout <= 1 час`
- `cancellation` должен быть инициализированным токеном отмены

**🔸 Гарантии (ПОСЛЕ):**
- Если `Ok(())`: текст элемента установлен через UI Automation API (element.set_value() или аналог), состояние элемента изменилось
- Если `Err(SetTextError::ElementNotFound)`: состояние UI не изменилось, вызов завершился без сторонних эффектов
- Если `Err(SetTextError::ElementNotEnabled)`: состояние UI не изменилось
- Если `Err(SetTextError::ElementOffscreen)`: состояние UI не изменилось
- Если `Err(SetTextError::ElementNotWritable)`: состояние UI не изменилось (элемент read-only)
- Если `Err(SetTextError::Timeout)`: состояние UI не изменилось, ожидание завершилось
- Если `Err(SetTextError::Cancelled)`: текущее выполнение прервано, состояние UI не изменилось
- Если `Err(SetTextError::InvalidConfig)`: состояние UI не изменилось, ошибка обнаружена на этапе валидации

**🚫 Запреты:**
- ❌ Использование `unwrap()`, `expect()`, `panic!` в любом месте реализации
- ❌ Глобальное мутабельное состояние без `Arc` + `Mutex`/`RwLock`
- ❌ Прямые вызовы WinAPI (CoInitializeEx, CoCreateInstance, BSTR, VARIANT)
- ❌ Изменение конфига или текста внутри функции
- ❌ Использование clipboard + Ctrl+V для установки текста (это typing, не SetText!)
- ❌ Изменение конфига без явной необходимости (только установка текста)

**⚡ Сбои:**
- **Timeout**: вызов завершается с `SetTextError::Timeout` через `tokio::time::timeout()`, состояние не меняется
- **Отмена**: проверка `cancellation.is_cancelled()` перед и во время выполнения, возврат `SetTextError::Cancelled`
- **Некорректный config**: валидация в `validate_set_text_config()` → `SetTextError::InvalidConfig` ДО вызова бэкенда
- **Непустой текст**: проверка `text.is_empty()` → `SetTextError::InvalidConfig` ДО вызова бэкенда
- **COM-ошибки**: перехват через `anyhow` → `SetTextError::ComError(String)`

**🛠️ Методы UI Automation:**
- `element.set_value(&str)` — установка текста элемента (предпочтительный метод)
- `element.is_enabled()` — проверка активности элемента
- `element.is_offscreen()` — проверка видимости элемента
- `element.is_read_only()` — проверка доступности записи (если доступно в uiautomation 0.24.4)
