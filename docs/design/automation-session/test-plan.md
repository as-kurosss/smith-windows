## 🧪 План тестов: AutomationSession | smith-windows

**✅ Позитивные (launch_process):**
- Запуск process и возврат `process_id` → `Ok(12345)` (например, `notepad.exe`)
- Запуск process с `args` → `Ok(process_id)` (например, `notepad.exe document.txt`)
- Запуск process с `working_dir` → `Ok(process_id)` (например, `notepad.exe` с рабочей директорией `C:\\Users\\test\\docs`)

**✅ Позитивные (attach_by_title):**
- Attach по точному совпадению заголовка (`MatchMode::Exact`) → `Ok(RuntimeSession)`
- Attach по частичному совпадению заголовка (`MatchMode::Contains`) → `Ok(RuntimeSession)` (например, `"Безымянный – Блокнот"`)
- Attach по регулярному выражению (`MatchMode::Regex`) → `Ok(RuntimeSession)` (например, `".*Notepad.*"`)
- Attach с `only_visible = true` → найти только видимые окна
- Attach с `only_visible = false` → найти скрытые окна (если существуют в системе для теста)

**✅ Позитивные (attach_by_process_id):**
- Attach по существующему `process_id` → `Ok(RuntimeSession)` с найденным окном
- Attach по `process_id` с несколькими окнами → найти главное окно (по `is_top_level()`, `is_dialog()`)

**🔄 Граничные (launch_process):**
- `timeout = Duration::ZERO` → `AutomationError::InvalidConfig` (валидация config)
- `timeout = Duration::from_millis(1)` → `Ok(process_id)` или ошибка в зависимости от скорости запуска
- `timeout > 1 час` (например, `Duration::from_secs(3601)`) → `AutomationError::InvalidConfig` (защита)
- `command = ""` (пустая строка) → `AutomationError::InvalidConfig` (валидация)
- `command = "nonexistent.exe"` → `AutomationError::ProcessLaunchFailed` (стандартная ошибка std::process::Command)

**🔄 Граничные (attach_by_title):**
- `timeout = Duration::ZERO` → `AutomationError::InvalidConfig` (валидация config)
- `timeout = Duration::from_millis(1)` → `Ok(RuntimeSession)` или `WindowNotFound` (в зависимости от скорости поиска)
- `timeout > 1 час` → `AutomationError::InvalidConfig` (защита)
- `title = ""` (пустая строка) → `AutomationError::InvalidConfig` (валидация)
- `title = " " ` (пробел) → `Ok(RuntimeSession)` или `WindowNotFound` (в зависимости от реализации find_elements)
- `regex = "["` (невалидное регулярное выражение) → `AutomationError::InvalidConfig` (валидация regex)
- `mode = Regex` с пустым title → `AutomationError::InvalidConfig`

**🔄 Граничные (attach_by_process_id):**
- `process_id = 0` → `AutomationError::InvalidConfig` (валидация process_id)
- `process_id = i32::MAX` (некорректный id) → `AutomationError::ProcessNotFound`
- `timeout = Duration::ZERO` → `AutomationError::InvalidConfig` (валидация config)

**❌ Негативные (launch_process):**
- `command = ""` (пустая строка) → `AutomationError::InvalidConfig` (валидация)
- `command = "nonexistent.exe"` → `AutomationError::ProcessLaunchFailed` (стандартная ошибка std::process::Command)
- Невалидный `working_dir` → `AutomationError::ProcessLaunchFailed`

**❌ Негативные (attach_by_title):**
- `title = "NonExistentWindow"` → `AutomationError::WindowNotFound` (окно не найдено за timeout)
- `title = "Test"` с `only_visible = true` → `AutomationError::WindowNotFound` (нет видимых окон с таким заголовком)
- `mode = Regex` с `regex = "["` → `AutomationError::InvalidConfig` (невалидное регулярное выражение)
- Процесс запущен, но окно не появляется за timeout → `AutomationError::WindowNotFound`
- Процесс запущен и сразу закрывается (мгновенный exit) → `AutomationError::ProcessNotFound` или `WindowNotFound`
- Окно найдено, но `is_enabled() = false` → `AutomationError::WindowDisabled`
- Окно найдено, но `is_offscreen() = true` → `AutomationError::WindowOffscreen`

**❌ Негативные (attach_by_process_id):**
- `process_id = 0` → `AutomationError::InvalidConfig` (валидация)
- `process_id = 999999` (не существующий процесс) → `AutomationError::ProcessNotFound`
- `process_id` без окон (service/daemon) → `AutomationError::WindowNotFound`

**✅ Негативные (все методы):**
- Использование `RuntimeSession` после `close()` → `AutomationError::SessionClosed`
- Отмена операции через `cancellation.cancel()` во время ожидания окна → `AutomationError::Cancelled`
- COM-ошибка при поиске окна → `AutomationError::ComError(String)`

**🔍 Обязательные проверки:**
- [ ] При `Err` состояние UI не изменилось (проверка через повторный вызов с теми же данными)
- [ ] Процесс запускается через `std::process::Command` (не через uiautomation) — проверка в коде
- [ ] Поиск окна происходит через `uiautomation` crate методы `find_elements_by_title()`, `find_elements_by_class()`, `is_enabled()`, `is_offscreen()`
- [ ] Нет дублей событий/логов (проверка через `cargo test -- --nocapture`)
- [ ] Нет `unwrap()`, `panic!`, блокировок в async (cargo clippy -- -D warnings)
- [ ] Валидация `config`, `title_filter`, `regex`, `command` происходит ДО вызова бэкенда (проверка в unit-тестах)
- [ ] `SessionState::Closed` защищает от повторных вызовов методов
- [ ] `RuntimeSession::main_element` можно передать в `ClickTool::click()` и `TypeTool::type_text()`
- [ ] `MatchMode` enum корректно обрабатывает `Exact`, `Contains`, `Regex`

---
## 🗓️ Для `/plan`: тесты как шаги
- [ ] Создать `mod tests` внутри `src/core/automation_session.rs` и `tests/integration/automation_session_tests.rs`
- [ ] Реализовать тест `test_launch_process_success`: запуск notepad → `Ok(process_id)`
- [ ] Реализовать тест `test_launch_process_with_args`: запуск notepad с аргументом → `Ok(process_id)`
- [ ] Реализовать тест `test_launch_process_timeout_zero`: `timeout=0` → `AutomationError::InvalidConfig`
- [ ] Реализовать тест `test_launch_process_timeout_large`: `timeout>1h` → `AutomationError::InvalidConfig`
- [ ] Реализовать тест `test_launch_process_empty_command`: `command=""` → `AutomationError::InvalidConfig`
- [ ] Реализовать тест `test_launch_process_nonexistent`: `command="nonexistent.exe"` → `AutomationError::ProcessLaunchFailed`
- [ ] Реализовать тест `test_attach_by_title_exact`: `MatchMode::Exact` → `Ok(RuntimeSession)`
- [ ] Реализовать тест `test_attach_by_title_contains`: `MatchMode::Contains` → `Ok(RuntimeSession)`
- [ ] Реализовать тест `test_attach_by_title_regex`: `MatchMode::Regex` → `Ok(RuntimeSession)`
- [ ] Реализовать тест `test_attach_by_title_empty`: `title=""` → `AutomationError::InvalidConfig`
- [ ] Реализовать тест `test_attach_by_title_not_found`: окно не найдено → `AutomationError::WindowNotFound`
- [ ] Реализовать тест `test_attach_by_title_disabled`: окно disabled → `AutomationError::WindowDisabled`
- [ ] Реализовать тест `test_attach_by_title_offscreen`: окно offscreen → `AutomationError::WindowOffscreen`
- [ ] Реализовать тест `test_attach_by_process_id_success`: `attach_by_process_id()` → `Ok(RuntimeSession)`
- [ ] Реализовать тест `test_attach_by_process_id_not_found`: невалидный `process_id` → `AutomationError::ProcessNotFound`
- [ ] Реализовать тест `test_session_close`: `close()` завершает процесс и устанавливает `Closed`
- [ ] Реализовать тест `test_session_after_close`: вызов метода после `close()` → `AutomationError::SessionClosed`
- [ ] Реализовать тест `test_integration_click_with_session`: `session.main_element` → `ClickTool::click()` → успех
- [ ] Реализовать тест `test_integration_type_with_session`: `session.main_element` → `TypeTool::type_text()` → успех
- [ ] Реализовать тест `test_integration_cancelled`: отмена через `cancellation` → `AutomationError::Cancelled`
- [ ] Реализовать тест `test_integration_invalid_config`: невалидный config → `AutomationError::InvalidConfig`
- [ ] Реализовать тест `test_integration_invalid_regex`: `mode=Regex` с `regex="["` → `AutomationError::InvalidConfig`
- [ ] Запустить `cargo test -- --nocapture` для проверки
- [ ] Запустить `cargo clippy -- -D warnings` для проверки качества кода
