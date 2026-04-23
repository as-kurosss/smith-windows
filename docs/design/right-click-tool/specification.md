## 📐 Спека: RightClickTool | smith-windows

**🎯 Цель:** Выполнение правого клика по элементу UI через UI Automation API с полной обработкой всех сценариев, включая offscreen, disabled, timeout и отмену операции.

**📥 Вход:**
- `element: &UIElement` | валидный UIA элемент, полученный через uiautomation | элемент кнопки "OK" в диалоговом окне
- `config: RightClickConfig` | валидная конфигурация правого клика | `RightClickConfig { timeout: Duration::from_secs(5), cancellation: CancellationToken }`
- `cancellation: CancellationToken` | токен отмены операции | созданный через `tokio_util::sync::CancellationToken`

**📤 Выход:**
- `Result<(), RightClickError>` | успешное завершение или конкретная ошибка
- При успехе: правый клик выполнен, контекстное меню открылось (если элемент реагирует на ПКМ)
- При ошибке: состояние UI не изменилось, фиксируется лог/событие с причиной сбоя

**⚠️ Границы:**
- Элемент offscreen (не отображается): возврат `RightClickError::ElementOffscreen`
- Элемент disabled (неактивен): возврат `RightClickError::ElementNotEnabled`
- Timeout: если правый клик не завершён за `config.timeout` → `RightClickError::Timeout`
- Отмена: при срабатывании `cancellation` → `RightClickError::Cancelled`
- Валидация конфига: `timeout <= Duration::ZERO` → `RightClickError::InvalidConfig`
- Валидация элемента: `element` не является валидным UIA элементом → `RightClickError::ElementNotFound`
- Timeout > 1 часа: возврат `RightClickError::InvalidConfig` (защита от переполнения)
- Отрицательный timeout (в мс): через `Duration::from_millis(0)` → `RightClickError::InvalidConfig`

**✅ Критерии успеха:**
- [ ] Все сценарии из «Границ» обработаны без паник и unwrap
- [ ] Состояние UI НЕ ломается при ошибке (идемпотентность)
- [ ] Логирование через `tracing` фиксирует результат или причину сбоя
- [ ] COM-вызовы изолированы через `tokio::task::spawn_blocking`
- [ ] Валидация происходит ДО вызова бэкенда

---
## 🗓️ План реализации (для `/plan`)
- [ ] Создать файлы: `src/core/right_click.rs`, `src/runtime/backends/windows/right_click.rs`
- [ ] Реализовать типы: `RightClickConfig`, `RightClickError`, `RightClickBackend`
- [ ] Реализовать функцию `validate_right_click_config()` в core
- [ ] Реализовать `RightClickBackend::right_click()` через uiautomation Mouse::right_click()
- [ ] Добавить тесты: позитивный (валидный элемент), граничный (timeout=0), негативный (offscreen)
- [ ] Обновить документацию: `ARCHITECTURE.md` добавить раздел RightClickTool
- [ ] Проверки: `cargo test`, `cargo clippy -- -D warnings`
