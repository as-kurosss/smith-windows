## 📐 Спека: ClickTool | smith-windows

**🎯 Цель:** Выполнение клика по элементу UI через UI Automation API с полной обработкой всех сценариев, включая offscreen, disabled, timeout и отмену операции.

**📥 Вход:**
- `element: &UIElement` | валидный UIA элемент, полученный через uiautomation | элемент кнопки "OK" в диалоговом окне
- `config: ClickConfig` | валидная конфигурация клика | `ClickConfig { timeout: Duration::from_secs(5), cancellation: CancellationToken }`
- `cancellation: CancellationToken` | токен отмены операции | созданный через `tokio_util::sync::CancellationToken`

**📤 Выход:**
- `Result<(), ClickError>` | успешное завершение или конкретная ошибка
- При успехе: клик выполнен, состояние UI изменилось (если элемент реагирует на клик)
- При ошибке: состояние UI не изменилось, фиксируется лог/событие с причиной сбоя

**⚠️ Границы:**
- Элемент offscreen (не отображается): возврат `ClickError::ElementOffscreen`
- Элемент disabled (неактивен): возврат `ClickError::ElementNotEnabled`
- Timeout: если клик не завершён за `config.timeout` → `ClickError::Timeout`
- Отмена: при срабатывании `cancellation` → `ClickError::Cancelled`
- Валидация конфига: `timeout <= Duration::ZERO` → `ClickError::InvalidConfig`
- Валидация элемента: `element` не является валидным UIA элементом → `ClickError::ElementNotFound`
- Timeout > 1 часа: возврат `ClickError::InvalidConfig` (защита от переполнения)
- Отрицательный timeout (в мс): через `Duration::from_millis(0)` → `ClickError::InvalidConfig`

**✅ Критерии успеха:**
- [ ] Все сценарии из «Границ» обработаны без паник и unwrap
- [ ] Состояние UI НЕ ломается при ошибке (идемпотентность)
- [ ] Логирование через `tracing` фиксирует результат или причину сбоя
- [ ] COM-вызовы изолированы через `tokio::task::spawn_blocking`
- [ ] Валидация происходит ДО вызова бэкенда

---
## 🗓️ План реализации (для `/plan`)
- [ ] Создать файлы: `src/core/click.rs`, `src/runtime/backends/windows/click.rs`
- [ ] Реализовать типы: `ClickConfig`, `ClickError`, `ClickBackend`
- [ ] Реализовать функцию `validate_click_config()` в core
- [ ] Реализовать `ClickBackend::click()` через uiautomation с spawn_blocking
- [ ] Добавить тесты: позитивный (валидный элемент), граничный (timeout=0), негативный (offscreen)
- [ ] Обновить документацию: `ARCHITECTURE.md` добавить раздел ClickTool
- [ ] Проверки: `cargo test`, `cargo clippy -- -D warnings`
