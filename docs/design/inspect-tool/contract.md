## 📜 Контракт: InspectTool | smith-windows

**🔹 Требования (ДО вызова):**
- `element` должен быть валидным UIA элементом (проверка через получение свойства или метода uiautomation)
- `head_window` должен быть валидным UIA элементом, представляющим корневое окно (обычно `GetTopLevelWindows()` + фильтрация)
- `element` должен быть потомком `head_window` (проверка иерархии через UITreeWalker)
- `config` должен быть валидным: `timeout > Duration::ZERO` и `timeout <= 1 час`
- `cancellation` должен быть инициализированным токеном отмены

**🔸 Гарантии (ПОСЛЕ):**
- Если `Ok(String)`: полный путь собран от `head_window` до `element`, формат — `ElementControlType{Name}` или `ElementControlType`, состояние UI и курсора не изменилось
- Если `Err(InspectError::ElementNotFound)`: путь не создан, вызов завершился без сторонних эффектов
- Если `Err(InspectError::ElementNotEnabled)`: путь не создан, состояние UI не изменилось
- Если `Err(InspectError::ElementOffscreen)`: путь не создан, состояние UI не изменилось
- Если `Err(InspectError::Timeout)`: путь не создан, ожидание завершилось, состояние UI не изменилось
- If `Err(InspectError::Cancelled)`: путь не создан, выполнение прервано, состояние UI не изменилось
- If `Err(InspectError::InvalidConfig)`: путь не создан, ошибка обнаружена на этапе валидации
- If `Err(InspectError::ComError)`: путь не создан, ошибка COM-инферфейса
- If `Err(InspectError::InvalidSelector)`: путь не создан, иерархическая проверка не удалась (element не потомок head_window или пустой путь)

**🚫 Запреты:**
- ❌ Использование `unwrap()`, `expect()`, `panic!` в любом месте реализации
- ❌ Глобальное мутабельное состояние без `Arc` + `Mutex`/`RwLock`
- ❌ Прямые вызовы WinAPI (CoInitializeEx, CoCreateInstance, BSTR, VARIANT)
- ❌ Изменение конфига внутри функции
- ❌ Изменение состояния UI (клик, ввод, фокус)
- ❌ Использование `GetForegroundWindow()` — использовать `is_enabled()`/`is_offscreen()` через `uiautomation`

**⚡ Сбои:**
- **Timeout**: вызов завершается с `InspectError::Timeout` через `tokio::time::timeout()`, путь не создаётся
- **Отмена**: проверка `cancellation.is_cancelled()` перед и во время выполнения, возврат `InspectError::Cancelled`
- **Некорректный config**: валидация в `validate_inspect_config()` → `InspectError::InvalidConfig` ДО вызова бэкенда
- **COM-ошибки**: перехват через `uiautomation` → `InspectError::ComError(String)`
- **Невалидная иерархия**: проверка через `UITreeWalker` и `compare_elements()` → `InspectError::InvalidSelector`
- **Максимальная глубина**: путь > 256 элементов → `InspectError::InvalidSelector`
