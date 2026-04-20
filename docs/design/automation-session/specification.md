## 📐 Спека: AutomationSession | smith-windows

**🎯 Цель:** Управление жизненным циклом приложений через UI Automation API — отдельно запуск процессов через std::process::Command (возврат process_id) и отдельно подключение к существующим окнам через uiautomation crate для выполнения операций автоматизации (клик, ввод текста и др.).

**📥 Вход (launch_process):**
- `config: SessionLaunchConfig` | конфигурация запуска процесса | `SessionLaunchConfig { command: "notepad.exe".to_string(), args: Some(vec!["document.txt".to_string()]), working_dir: None }`

**📥 Вход (attach_by_title):**
- `title: String` | заголовок окна для поиска | `"Безымянный – Блокнот"` или `"Untitled - Notepad"`
- `mode: MatchMode` | режим совпадения: `Exact`, `Contains`, `Regex` | `MatchMode::Contains`
- `only_visible: bool` | фильтр только видимых окон | `true`
- `config: SessionConfig` | валидная конфигурация сессии | `SessionConfig { timeout: Duration::from_secs(10), cancellation: CancellationToken }`

**📥 Вход (attach_by_process_id):**
- `process_id: u32` | идентификатор процесса | `12345`
- `config: SessionConfig` | валидная конфигурация сессии | `SessionConfig { timeout: Duration::from_secs(10), cancellation: CancellationToken }`

**📤 Выход:**
- `launch_process() → Result<u32, AutomationError>` | идентификатор запущенного процесса
- `attach_by_title() → Result<RuntimeSession, AutomationError>` | сессия с `process_id` и `main_element`
- `attach_by_process_id() → Result<RuntimeSession, AutomationError>` | сессия с `process_id` и `main_element`
- При успехе: `RuntimeSession` содержит `process_id` (u32) и `main_element` (UIElement главного окна), доступны методы `click()`, `type_text()`, `close()`
- При ошибке: состояние UI не изменилось (для attach) или процесс не запущен (для launch)

**⚠️ Границы (launch_process):**
- **Процесс не запускается**: `std::process::Command` возвращает ошибку → `AutomationError::ProcessLaunchFailed(String)` (стандартная ошибка команды)
- **Пустой command**: `validate_command()` → `AutomationError::InvalidConfig` (валидация ДО вызова бэкенда)
- **Невалидный config**: `validate_session_config()` → `AutomationError::InvalidConfig`

**⚠️ Границы (attach_by_title):**
- **Окно не найдено за timeout**: `AutomationError::WindowNotFound`
- **Процесс закрылся во время ожидания окна**: `AutomationError::ProcessNotFound`
- **Невалидный title**: пустая строка → `AutomationError::InvalidConfig` (валидация ДО вызова бэкенда)
- **Невалидный regex**: `validate_regex()` → `AutomationError::InvalidConfig`
- **Отмена операции**: `cancellation.is_cancelled()` → `AutomationError::Cancelled`
- **Окно отключено (disabled)**: `AutomationError::WindowDisabled`
- **Окно offscreen**: `AutomationError::WindowOffscreen`

**⚠️ Границы (attach_by_process_id):**
- **Процесс не найден**: `AutomationError::ProcessNotFound`
- **Окно не найдено у процесса**: `AutomationError::WindowNotFound`
- **Окно отключено/offscreen**: `AutomationError::WindowDisabled` / `AutomationError::WindowOffscreen`

**✅ Критерии успеха:**
- [ ] Все сценарии из «Границ» обработаны без паник и unwrap
- [ ] Состояние UI НЕ ломается при ошибке (идемпотентность)
- [ ] Логирование через `tracing` фиксирует запуск процесса, поиск окна, результат
- [ ] COM-вызовы изолированы через `tokio::task::spawn_blocking`
- [ ] Валидация происходит ДО вызова бэкенда
- [ ] Процесс запускается через `std::process::Command` (не через uiautomation)
- [ ] Поиск окна происходит через `uiautomation` crate (по заголовку/классу)
- [ ] `RuntimeSession` поддерживает методы инструментов (например, `ClickTool::click()` с передачей `main_element`)
- [ ] `MatchMode` enum поддерживает `Exact`, `Contains`, `Regex`

---
## 🗓️ План реализации (для `/plan`)
- [ ] Создать файлы: `src/core/automation_session.rs`, `src/runtime/backends/windows/automation_session.rs`
- [ ] Реализовать типы: `SessionConfig`, `SessionLaunchConfig`, `AutomationError`, `SessionState`, `MatchMode`, `RuntimeSession`
- [ ] Реализовать функции `validate_session_config()`, `validate_title_filter()`, `validate_regex()`, `launch_process()` в core (валидация ДО бэкенда)
- [ ] Реализовать `SessionBackendWindows::attach_by_title()`, `SessionBackendWindows::attach_by_process_id()` через uiautomation с spawn_blocking
- [ ] Реализовать методы `RuntimeSession`: `click()`, `type_text()`, `close()`, `find_element()`
- [ ] Реализовать `SessionBackend` trait и `SessionBackendWindows` implementation
- [ ] Добавить тесты: позитивный (launch + attach), граничный (timeout=0), негативный (окно не найдено)
- [ ] Обновить документацию: `ARCHITECTURE.md` добавить раздел AutomationSession
- [ ] Проверки: `cargo test`, `cargo clippy -- -D warnings`
