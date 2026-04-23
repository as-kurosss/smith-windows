## 📐 Спека: SetTextTool | smith-windows

**🎯 Цель:** Программная установка текста элемента UI через UI Automation API (NOT keyboard typing!) с полной обработкой всех сценариев, включая offscreen, disabled, read-only, timeout и отмену операции.

**📥 Вход:**
- `element: &UIElement` | валидный UIA элемент, полученный через uiautomation | элемент EditText в диалоговом окне
- `text: &str` | строка для установки | "Hello, World!"
- `config: SetTextConfig` | валидная конфигурация установки текста | `SetTextConfig { timeout: Duration::from_secs(5), cancellation: CancellationToken }`

**📤 Выход:**
- `Result<(), SetTextError>` | успешное завершение или конкретная ошибка
- При успехе: текст элемента установлен через UI Automation API (element.set_value() или аналог)
- При ошибке: состояние UI не изменилось, фиксируется лог/событие с причиной сбоя

**⚠️ Границы:**
- Элемент offscreen (не отображается): возврат `SetTextError::ElementOffscreen`
- Элемент disabled (неактивен): возврат `SetTextError::ElementNotEnabled`
- Элемент read-only (только для чтения): возврат `SetTextError::ElementNotWritable`
- Timeout: если установка текста не завершена за `config.timeout` → `SetTextError::Timeout`
- Отмена: при срабатывании `cancellation` → `SetTextError::Cancelled`
- Валидация конфига: `timeout <= Duration::ZERO` → `SetTextError::InvalidConfig`
- Валидация текста: `text.is_empty()` → `SetTextError::InvalidConfig`
- Валидация элемента: `element` не является валидным UIA элементом → `SetTextError::ElementNotFound`
- Timeout > 1 часа: возврат `SetTextError::InvalidConfig` (защита от переполнения)
- Отрицательный timeout (в мс): через `Duration::from_millis(0)` → `SetTextError::InvalidConfig`

**✅ Критерии успеха:**
- [ ] Все сценарии из «Границ» обработаны без паник и unwrap
- [ ] Состояние UI НЕ ломается при ошибке (идемпотентность)
- [ ] Логирование через `tracing` фиксирует результат или причину сбоя
- [ ] COM-вызовы выполняются на правильном потоке (UIElement !Send, !Sync)
- [ ] Валидация происходит ДО вызова бэкенда
- [ ] Использование `element.set_value()` или аналогичного метода UI Automation (не clipboard + paste!)
