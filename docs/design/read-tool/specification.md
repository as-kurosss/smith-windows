## 📐 Specification: ReadTool | smith-windows

**🎯 Purpose:** Получение текстового содержимого из UI-элементов (полей ввода, лейблов, заголовков и т.д.) через UI Automation API.

**📥 Input:**
- `element` (*&uiautomation::UIElement*) | ссылка на валидный UI-элемент | результат поиска по селектору
- `config` (*ReadConfig*) | валидная конфигурация с timeout > 0 и <= 1 час | `ReadConfig { timeout: Duration::from_secs(5), cancellation: CancellationToken::new() }`
- `selector` (*String*) | валидный селектор элемента | `"Name:=Button1;ControlType:=Button"`

**📤 Output:**
- `Result<String, ReadError>` | строка с текстом элемента | `Ok("Click me".to_string())`
- On error: `ReadError` | состояние элемента и системы НЕ изменяется

**⚠️ Boundaries:**
- Что если `element` = `null` / недействительный? → `ReadError::ElementNotFound`
- Что если `timeout` = `Duration::ZERO`? → `ReadError::InvalidConfig`
- Что если `timeout` > 1 час? → `ReadError::InvalidConfig`
- Что если элемент не поддерживает текст (нет TextPattern/Name)? → `ReadError::ElementNotWritable`
- Что если элемент отключён (`is_enabled() == false`)? → `ReadError::ElementNotEnabled`
- Что если элемент скрыт (`is_offscreen() == true`)? → `ReadError::ElementOffscreen`
- Что если операция занимает больше времени, чем timeout? → `ReadError::Timeout`
- Что если операция отменена через `CancellationToken`? → `ReadError::Cancelled`

**✅ Success criteria:**
- [ ] Все граничные случаи обрабатываются без паники (panics)
- [ ] Состояние системы не меняется при ошибках (идемпотентность)
- [ ] Все ошибки возвращаются через `Result<T, ReadError>`
- [ ] Логирование: `info!` при успехе, `error!` при ошибках
- [ ] Валидация конфигурации происходит БЕЗ вызова backend

---
## 🗓️ Implementation plan (для `/plan`)

- [ ] Создать файл: `src/core/read.rs` — трейты, типы, валидация, моки
- [ ] Создать файл: `src/runtime/backends/windows/read.rs` — Windows-реализация
- [ ] Обновить `src/runtime/backends/windows/mod.rs` — добавить `pub mod read`
- [ ] Обновить `src/core/mod.rs` — добавить `pub mod read`
- [ ] Обновить `src/runtime/backends/mod.rs` — добавить `pub mod windows` (если необходимо)
- [ ] Добавить тесты: `#[cfg(test)] mod tests` внутри `src/core/read.rs`
- [ ] Добавить интеграционные тесты: `tests/integration/read_tool.rs`
- [ ] Обновить `ARCHITECTURE.md` — добавить ReadTool в список инструментов
- [ ] Создать `docs/adr/XXX-read-tool.md` — запись архитектурного решения
- [ ] Проверки: `cargo test`, `cargo clippy -- -D warnings`
