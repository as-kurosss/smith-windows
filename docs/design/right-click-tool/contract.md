## 📜 Контракт: RightClickTool | smith-windows

**🛠️ Обязанности:**
- `RightClickBackend` реализует правый клик через `uiautomation::inputs::Mouse::right_click(&Point)`
- Все COM-вызовы изолируются через `tokio::task::spawn_blocking`
- Валидация входных параметров ДО вызова бэкенда
- Все ошибки возвращаются через `Result<T, RightClickError>`, без паник

**🔒 Гарантии:**
- Идемпотентность: повторные вызовы с теми же данными не изменяют состояние системы
- Безопасность COM: все вызовы UI Automation API выполняются в правильном потоке (STA)
- Явные ошибки: каждая ошибка имеет конкретный тип (`RightClickError`)
- Логирование: `tracing::info!` для успеха, `tracing::error!` для ошибок

**🚫 Запреты:**
- ❌ Использование `unwrap()`, `expect()`, `panic!` в `src/`
- ❌ Прямые вызовы `CoInitializeEx`, `CoCreateInstance`, `BSTR`, `VARIANT`
- ❌ Изменение контракта без обсуждения
- ❌ Генерация кода без утверждённого `/plan`
- ❌ Изменение конфигурации после валидации (immutable config)

**🚨 Сбои:**
| Сценарий | Ошибка | Обработка |
|----------|--------|-----------|
| `element` не валиден | `RightClickError::ElementNotFound` | Возврат ошибки ДО бэкенда |
| `element` disabled | `RightClickError::ElementNotEnabled` | Возврат ошибки ДО бэкенда |
| `element` offscreen | `RightClickError::ElementOffscreen` | Возврат ошибки ДО бэкенда |
| `config.timeout` <= 0 | `RightClickError::InvalidConfig` | Возврат ошибки ДО бэкенда |
| `config.timeout` > 1 час | `RightClickError::InvalidConfig` | Возврат ошибки ДО бэкенда |
| Timeout ожидания | `RightClickError::Timeout` | Возврат ошибки после завершения timeout |
| Отмена операции | `RightClickError::Cancelled` | Проверка `cancellation.is_cancelled()` |
| COM-ошибка | `RightClickError::ComError(String)` | Упаковка сообщения об ошибке |

**🧪 Тесты:**
- Позитивный: валидный элемент → правый клик выполнен
- Негативный: offscreen элемент → `RightClickError::ElementOffscreen`
- Негативный: disabled элемент → `RightClickError::ElementNotEnabled`
- Граничный: `timeout = Duration::ZERO` → `RightClickError::InvalidConfig`
- Граничный: `timeout = 1 час` →接受
- Граничный: `timeout = 1 час + 1 сек` → `RightClickError::InvalidConfig`
- Отмена: `cancellation.cancel()` во время ожидания → `RightClickError::Cancelled`
