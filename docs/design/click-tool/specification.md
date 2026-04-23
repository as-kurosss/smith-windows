## 📐 Спека: ClickTool | smith-windows

**🎯 Цель:** Выполнение клика по элементу UI через UI Automation API с полной обработкой всех сценариев, включая offscreen, disabled, timeout и отмену операции.

**📥 Вход:**
- `element: &UIElement` | валидный UIA элемент, полученный через uiautomation | элемент кнопки "OK" в диалоговом окне
- `config: ClickConfig` | валидная конфигурация клика | `ClickConfig { click_type: ClickType::LeftSingle, timeout: Duration::from_secs(5), cancellation: CancellationToken }`
- `cancellation: CancellationToken` | токен отмены операции | созданный через `tokio_util::sync::CancellationToken`

**📤 Выход:**
- `Result<(), ClickError>` | успешное завершение или конкретная ошибка
- При успехе: клик выполнен, состояние UI изменилось (если элемент реагирует на клик)
- При ошибке: состояние UI не изменилось, фиксируется лог/событие с причиной сбоя

**🔄 ClickType (Тип клика):**
- `ClickType::LeftSingle` | Одинарный левый клик | `element.click()` | Клик по кнопке
- `ClickType::RightSingle` | Одинарный правый клик | `element.right_click()` | Открытие контекстного меню
- `ClickType::LeftDouble` | Двойной левый клик | `element.double_click()` | Открытие файла/приложения

**⚠️ Границы:**
- Элемент offscreen (не отображается): возврат `ClickError::ElementOffscreen`
- Элемент disabled (неактивен): возврат `ClickError::ElementNotEnabled`
- Timeout: если клик не завершён за `config.timeout` → `ClickError::Timeout`
- Отмена: при срабатывании `cancellation` → `ClickError::Cancelled`
- Валидация конфига: `timeout <= Duration::ZERO` или `timeout > 1 час` → `ClickError::InvalidConfig`
- Валидация элемента: `element` не является валидным UIA элементом → `ClickError::ElementNotFound`

**✅ Критерии успеха:**
- [ ] Все сценарии из «Границ» обработаны без паник и unwrap
- [ ] Состояние UI НЕ ломается при ошибке (идемпотентность)
- [ ] Логирование через `tracing` фиксирует результат или причину сбоя
- [ ] COM-вызовы выполняются на том же потоке (UIA требует STA affinity)
- [ ] Валидация происходит ДО вызова бэкенда

---
## 🗓️ План реализации (для `/plan`)
- [x] Добавить `ClickType` enum с вариантами: `LeftSingle`, `RightSingle`, `LeftDouble`
- [x] Обновить `ClickConfig` с полем `click_type: ClickType`
- [x] Обновить `ClickBackend::click(element, click_type)` - добавить параметр типа клика
- [x] Реализовать `ClickBackend::click()` через uiautomation: `click()`, `right_click()`, `double_click()`
- [x] Переделать `RightClickTool` в обёртку вокруг `ClickTool` с `RightSingle`
- [x] Добавить тесты для всех типов кликов: позитивный, негативный, граничный
- [x] Обновить примеры: click_example.rs, right_click_example.rs, notepad_click.rs
- [x] Проверки: `cargo test`, `cargo clippy -- -D warnings`
