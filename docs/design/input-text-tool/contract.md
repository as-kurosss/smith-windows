## 📜 Контракт: InputTextTool | smith-windows

**🔹 Требования (ДО вызова):**
- `selector` должен быть непустой строкой (`!selector.is_empty()`) и длиной ≤ 2048 символов
- `config.text` должна быть непустой строкой (`!text.is_empty()`) и длиной ≤ 65536 символов
- `config.timeout` должен быть > `Duration::ZERO` и ≤ 1 часа (`Duration::from_secs(3600)`)
- `config.cancellation` должен быть инициализированным токеном отмены

**🔸 Гарантии (ПОСЛЕ):**
- Если `Ok(())`: текст введён в элемент через эмуляцию клавиатуры (как будто человек нажимает клавиши), состояние элемента изменилось
- Если `Err(InputTextError::InputSelectorError)`: состояние UI не изменилось, вызов завершился без сторонних эффектов
- Если `Err(InputTextError::ElementNotFoundError)`: состояние UI не изменилось
- Если `Err(InputTextError::ElementNotEnabled)`: состояние UI не изменилось
- Если `Err(InputTextError::ElementOffscreen)`: состояние UI не изменилось
- Если `Err(InputTextError::ElementReadOnly)`: состояние UI не изменилось (элемент read-only)
- Если `Err(InputTextError::Timeout)`: состояние UI не изменилось, ожидание завершилось
- Если `Err(InputTextError::Cancelled)`: текущее выполнение прервано, состояние UI не изменилось
- Если `Err(InputTextError::InvalidConfig)`: состояние UI не изменилось, ошибка обнаружена на этапе валидации

**🚫 Запреты:**
- ❌ Использование `unwrap()`, `expect()`, `panic!` в любом месте реализации
- ❌ Глобальное мутабельное состояние без `Arc` + `Mutex`/`RwLock`
- ❌ Прямые вызовы WinAPI (CoInitializeEx, CoCreateInstance, BSTR, VARIANT)
- ❌ Изменение конфига или текста внутри функции
- ❌ Использование clipboard + Ctrl+V для ввода текста (это typing, не InputText!)
- ❌ Изменение конфига без явной необходимости (только ввод текста)
- ❌ Отправка пустого текста через `Keyboard::send_text("")`

**⚡ Сбои:**
- **Timeout**: вызов завершается с `InputTextError::Timeout` через `tokio::time::timeout()`, состояние не меняется
- **Отмена**: проверка `cancellation.is_cancelled()` перед и во время выполнения, возврат `InputTextError::Cancelled`
- **Некорректный config**: валидация в `validate_input_text_config()` → `InputTextError::InvalidConfig` ДО вызова бэкенда
- **Непустой текст**: проверка `text.is_empty()` и `text.len()` → `InputTextError::InvalidConfig` ДО вызова бэкенда
- **Непустой selector**: проверка `selector.is_empty()` и `selector.len()` → `InputTextError::InvalidConfig` ДО вызова бэкенда
- **COM-ошибки**: перехват через `anyhow` → `InputTextError::ComError(String)`
- **Ошибка ввода**: любая ошибка `Keyboard::send_keys()` → `InputTextError::InputExecutionError(String)`

**🛠️ Методы UI Automation:**
- `Keyboard::send_keys(&str)` — отправка клавиш с синтаксисом `{}` для специальных клавиш и комбинаций
- `Keyboard::send_text(&str)` — отправка обычного текста (для строк без `{}`)
- `element.is_enabled()` — проверка активности элемента (через SetTextTool или прямой вызов)
- `element.is_offscreen()` — проверка видимости элемента
- `element.is_read_only()` — проверка доступности записи (если доступно в uiautomation 0.24.4)

**💡 Реализация валидации через SetTextTool:**
Для проверки состояния элемента (enabled, offscreen, read-only) используется `validate_element_ready()` из `SetTextTool`, чтобы избежать дублирования кода.
