## 📐 Specification: ClipboardTool | smith-windows

**🎯 Purpose:** Работа с системным буфером обмена Windows: получение текста, установка текста, проверка наличия текста.

**📥 Input:**
- `action` (*ClipboardAction*) | тип операции: GetText, SetText, HasText
- `config` (*ClipboardConfig*) | валидная конфигурация с timeout > 0 и <= 1 час
- `params` (*SetTextParams*) | текст для установки (только для SetText)

**📤 Output:**
- `get_text()`: `Result<String, ClipboardError>` | текст из буфера обмена
- `set_text()`: `Result<(), ClipboardError>` | результат установки текста
- `has_text()`: `Result<bool, ClipboardError>` | наличие текста в буфере

**⚠️ Boundaries:**
- Что если `timeout` = `Duration::ZERO`? → `ClipboardError::InvalidConfig`
- Что если `timeout` > 1 час? → `ClipboardError::InvalidConfig`
- Что если буфер пуст (get_text)? → `ClipboardError::ClipboardEmpty`
- Что если буфер недоступен (заблокирован)? → `ClipboardError::ClipboardAccessDenied`
- Что если пустой текст для установки? → `ClipboardError::TextEmpty`
- Что если операция занимает больше времени, чем timeout? → `ClipboardError::Timeout`
- Что если операция отменена через `CancellationToken`? → `ClipboardError::Cancelled`

**✅ Success criteria:**
- [ ] Все граничные случаи обрабатываются без паники (panics)
- [ ] Состояние системы не меняется при ошибках (идемпотентность)
- [ ] Все ошибки возвращаются через `Result<T, ClipboardError>`
- [ ] Логирование: `info!` при успехе, `error!` при ошибках
- [ ] Валидация конфигурации происходит БЕЗ вызова backend

---
## 🗓️ Implementation plan (для `/plan`)

- [ ] Создать файл: `src/core/clipboard.rs` — трейты, типы, валидация, моки
- [ ] Создать файл: `src/runtime/backends/windows/clipboard.rs` — Windows-реализация
- [ ] Обновить `src/runtime/backends/windows/mod.rs` — добавить `pub mod clipboard`
- [ ] Обновить `src/core/mod.rs` — добавить `pub mod clipboard`
- [ ] Добавить dependency: `clipboard = "0.5"` в `Cargo.toml`
- [ ] Добавить тесты: `#[cfg(test)] mod tests` внутри `src/core/clipboard.rs`
- [ ] Добавить интеграционные тесты: `tests/integration/clipboard_tool.rs`
- [ ] Обновить `ARCHITECTURE.md` — добавить ClipboardTool в список инструментов
- [ ] Создать `docs/adr/XXX-clipboard-tool.md` — запись архитектурного решения
- [ ] Проверки: `cargo test`, `cargo clippy -- -D warnings`
