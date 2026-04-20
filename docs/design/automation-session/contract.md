## 📜 Контракт: AutomationSession | smith-windows

**🔹 Требования (ДО вызова launch_process):**
- `config.command` должен быть непустой строкой с валидным путём к исполняемому файлу или командой (валидация ДО вызова бэкенда)
- `config.args` может быть `None` или валидным вектором строк (аргументы командной строки)
- `config.working_dir` может быть `None` или валидным путём к директории
- `config` должен быть валидным: `timeout > Duration::ZERO` и `timeout <= 1 час` (валидация ДО вызова бэкенда)
- `cancellation` должен быть инициализированным токеном отмены

**🔹 Требования (ДО вызова attach_by_title):**
- `title` должен быть непустой строкой (валидация ДО вызова бэкенда)
- `mode` должен быть валидным: `Exact`, `Contains`, `Regex` (валидация ДО вызова бэкенда)
- `only_visible` — булево значение для фильтрации видимых окон
- `config` должен быть валидным: `timeout > Duration::ZERO` и `timeout <= 1 час` (валидация ДО вызова бэкенда)
- `cancellation` должен быть инициализированным токеном отмены
- Если `mode == Regex`, `title` должен быть валидным регулярным выражением (валидация ДО вызова бэкенда)

**🔹 Требования (ДО вызова attach_by_process_id):**
- `process_id` должен быть валидным идентификатором процесса (> 0)
- `config` должен быть валидным: `timeout > Duration::ZERO` и `timeout <= 1 час` (валидация ДО вызова бэкенда)
- `cancellation` должен быть инициализированным токеном отмены

**🔸 Гарантии (ПОСЛЕ launch_process):**
- Если `Ok(process_id)`: процесс запущен через `std::process::Command`, возвращён `process_id` (u32), состояние UI не изменилось
- Если `Err(AutomationError::ProcessLaunchFailed)`: процесс не запущен, вызов завершился без сторонних эффектов, ошибка содержит `std::io::Error` описание
- Если `Err(AutomationError::InvalidConfig)`: процесс не запущен, ошибка обнаружена на этапе валидации

**🔸 Гарантии (ПОСЛЕ attach_by_title):**
- Если `Ok(RuntimeSession)`: окно найдено через `uiautomation`, `RuntimeSession` содержит `process_id` и `main_element`, состояние UI не изменилось
- Если `Err(AutomationError::WindowNotFound)`: окно не найдено за timeout, состояние UI не изменилось
- Если `Err(AutomationError::ProcessNotFound)`: процесс закрылся во время ожидания окна, состояние UI не изменилось
- Если `Err(AutomationError::WindowDisabled)`: окно найдено, но отключено, состояние UI не изменилось
- If `Err(AutomationError::WindowOffscreen)`: окно найдено, но не отображается на экране, состояние UI не изменилось
- Если `Err(AutomationError::InvalidConfig)`: окно не найдено, ошибка обнаружена на этапе валидации
- Если `Err(AutomationError::Cancelled)`: текущее выполнение прервано, состояние UI не изменилось

**🔸 Гарантии (ПОСЛЕ attach_by_process_id):**
- Если `Ok(RuntimeSession)`: окно найдено через `uiautomation`, `RuntimeSession` содержит `process_id` и `main_element`, состояние UI не изменилось
- Если `Err(AutomationError::ProcessNotFound)`: процесс не найден или закрыт, состояние UI не изменилось
- Если `Err(AutomationError::WindowNotFound)`: у процесса не найдено окно, состояние UI не изменилось
- Если `Err(AutomationError::WindowDisabled)`: окно найдено, но отключено, состояние UI не изменилось
- If `Err(AutomationError::WindowOffscreen)`: окно найдено, но не отображается на экране, состояние UI не изменилось
- Если `Err(AutomationError::Cancelled)`: текущее выполнение прервано, состояние UI не изменилось

**🔸 Гарантии (ПОСЛЕ close):**
- Если `Ok(())`: процесс завершён, `SessionState` становится `Closed`, методы сессии больше недоступны
- Если `Err(AutomationError::SessionClosed)`: сессия уже закрыта, состояние не изменяется

**🚫 Запреты:**
- ❌ Использование `unwrap()`, `expect()`, `panic!` в любом месте реализации
- ❌ Глобальное мутабельное состояние без `Arc` + `Mutex`/`RwLock`
- ❌ Прямые вызовы WinAPI (CoInitializeEx, CoCreateInstance, BSTR, VARIANT) — только через `uiautomation` crate
- ❌ Изменение конфига внутри функции
- ❌ Изменение UI элемента без явной необходимости (только поиск и получение ссылок)
- ❌ Использование `GetForegroundWindow()` — использовать `is_enabled()`, `is_offscreen()` через `uiautomation`
- ❌ Изменение `RuntimeSession` после `close()` — все методы должны проверять `SessionState::Closed`
- ❌ Комбинировать launch и attach в один метод — всегда раздельно

**⚡ Сбои (launch_process):**
- **std::process::Command ошибки**: перехват через `anyhow` → `AutomationError::ProcessLaunchFailed(String)`
- **Некорректный config**: валидация в `validate_session_config()` → `AutomationError::InvalidConfig`
- **Некорректный command**: валидация в `validate_command()` → `AutomationError::InvalidConfig`

**⚡ Сбои (attach_by_title / attach_by_process_id):**
- **Timeout**: вызов завершается с `AutomationError::WindowNotFound` через `tokio::time::timeout()`, состояние не меняется
- **Отмена**: проверка `cancellation.is_cancelled()` перед и во время выполнения, возврат `AutomationError::Cancelled`
- **Некорректный title**: валидация в `validate_title_filter()` → `AutomationError::InvalidConfig`
- **Некорректный regex**: валидация в `validate_regex()` → `AutomationError::InvalidConfig`
- **COM-ошибки**: перехват через `anyhow` → `AutomationError::ComError(String)`
- **Окно отключено/offscreen**: проверка через `uiautomation` методы `is_enabled()`, `is_offscreen()` → `AutomationError::WindowDisabled` / `AutomationError::WindowOffscreen`

**📌 Интеграция с инструментами (ClickTool, TypeTool):**
- `RuntimeSession` предоставляет `main_element: &UIElement` для передачи в инструменты
- Инструменты вызываются с `element = &session.main_element`, например: `ClickTool::click(&session.main_element, &config, cancellation)`
- `RuntimeSession::click()` — прямая реализация через uiautomation (не через ClickTool), использует `session.main_element`
- `RuntimeSession::type_text()` — прямая реализация через uiautomation (не через TypeTool), использует `session.main_element`

---
## 🗓️ Для `/plan`: ключевые точки валидации
- [ ] Валидация входа происходит в `src/core/automation_session.rs` → `validate_session_config()`, `validate_title_filter()`, `validate_regex()`, `validate_command()`
- [ ] События/логи отправляются через `tracing::info!()` при запуске процесса, `tracing::debug!()` при поиске окна, `tracing::error!()` при ошибках
- [ ] Ошибки обрабатываются через `AutomationError` (`thiserror`), не через `panic!`
- [ ] `std::process::Command` запуск изолирован в `src/core/automation_session.rs` (launch logic)
- [ ] COM-вызовы изолированы в `src/runtime/backends/windows/automation_session.rs` через `tokio::task::spawn_blocking`
- [ ] Поиск окна через `uiautomation` crate: `find_elements_by_title()`, `find_elements_by_class()`, фильтрация `is_enabled()`, `is_offscreen()`
- [ ] `SessionState` управляет состоянием сессии (`Running`, `Closed`) для защиты от повторных вызовов
