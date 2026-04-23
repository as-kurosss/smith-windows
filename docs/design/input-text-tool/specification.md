## 📐 Спека: InputTextTool | smith-windows

**🎯 Цель:** Эмуляция ввода текста с клавиатуры (как будто это делает человек) через UI Automation API, включая поддержку комбинаций клавиш (Ctrl+C, Ctrl+V и т.д.) и специальных клавиш (Enter, Tab, F1..F24, стрелки).

**Важное отличие от SetTextTool:**
- **SetTextTool**: Программная установка текста через `UIValuePattern.set_value()` — прямая замена значения элемента
- **InputTextTool**: Посимвольная эмуляция ввода через `Keyboard::send_keys()` / `Keyboard::send_text()` — нажатия клавиш, как делает человек

**📥 Вход:**
- `selector: &str` | валидный CSS-селектор или XPath для поиска элемента | `"#editbox"` или `"//EditText[@name='Input']"`
- `config: InputTextConfig` | валидная конфигурация ввода | `InputTextConfig { text: "Hello {enter}World", timeout: Duration::from_secs(5), cancellation: CancellationToken }`

**📤 Выход:**
- `Result<(), InputTextError>` | успешное завершение или конкретная ошибка
- При успехе: текст введён в элемент через эмуляцию клавиатуры (как будто человек нажимает клавиши)
- При ошибке: состояние UI не изменилось, фиксируется лог/событие с причиной сбоя

**⚠️ Границы:**
- Пустой селектор: возврат `InputTextError::InputSelectorError("selector cannot be empty")`
- Слишком длинный селектор (>2048 символов): возврат `InputTextError::InputSelectorError("selector too long")`
- Элемент не найден: возврат `InputTextError::ElementNotFoundError`
- Элемент offscreen (не отображается): возврат `InputTextError::ElementOffscreen`
- Элемент disabled (неактивен): возврат `InputTextError::ElementNotEnabled`
- Элемент read-only (только для чтения): возврат `InputTextError::ElementReadOnly`
- Timeout: если ввод не завершён за `config.timeout` → `InputTextError::Timeout`
- Отмена: при срабатывании `cancellation` → `InputTextError::Cancelled`
- Валидация конфига: `timeout <= Duration::ZERO` → `InputTextError::InvalidConfig`
- Валидация конфига: `timeout > 1 час` → `InputTextError::InvalidConfig`
- Валидация текста: `text.is_empty()` → `InputTextError::InvalidConfig`
- Валидация текста: `text.len() > 65536` → `InputTextError::InvalidConfig` (защита от переполнения)
- Синтаксис `{}`: `{enter}` → Enter, `{ctrl}(c)` → Ctrl+C, `{ctrl}{shift}(v)` → Ctrl+Shift+V
- Специальные клавиши: `{tab}`, `{delete}`, `{backspace}`, `{left}`, `{right}`, `{f1}..{f24}`
- Повторы: `{enter 3}` → Enter 3 раза
- Unicode: `Привет, мир! 👋` → вводится корректно
- Комбинации с Unicode: `"{ctrl}(a){ctrl}(c)Привет {ctrl}(v)"` → копирование и вставка

**✅ Критерии успеха:**
- [ ] Все сценарии из «Границ» обработаны без паник и unwrap
- [ ] Состояние UI НЕ ломается при ошибке (идемпотентность)
- [ ] Логирование через `tracing` фиксирует результат или причину сбоя
- [ ] COM-вызовы выполняются на правильном потоке (UIElement !Send, !Sync)
- [ ] Валидация происходит ДО вызова бэкенда
- [ ] Использование `Keyboard::send_keys()` / `Keyboard::send_text()` (не clipboard + paste!)

**🔧 Синтаксис `uiautomation::inputs::Keyboard::send_keys()`:**
- Обычный текст: `"Hello"` → отправляется как `send_text("Hello")`
- Специальные клавиши в скобках: `"{enter}"`, `"{tab}"`, `"{f1}"`, `"{left}"`
- Модификаторы: `{ctrl}`, `{shift}`, `{alt}`, `{lctrl}`, `{rctrl}`, `{lshift}`, `{rshift}`, `{lalt}`, `{ralt}`
- Комбинации: `"{ctrl}(c)"` = Ctrl+C, `"{ctrl}{shift}(v)"` = Ctrl+Shift+V
- Повторы: `"{enter 3}"` = Enter 3 раза
- Удержание: `begin_hold_keys()` / `end_hold_keys()` (не в текущей версии, только send_keys)
