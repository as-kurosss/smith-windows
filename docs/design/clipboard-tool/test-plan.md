## 🧪 Test Plan: ClipboardTool | smith-windows

**✅ Positive:** валидные данные → ожидаемый результат
- [ ] Тест: `get_text()` → успешное чтение текста
- [ ] Тест: `set_text(text)` → успешная установка текста
- [ ] Тест: `has_text()` → `true` при наличии текста
- [ ] Тест: `has_text()` → `false` при пустом буфере

**🔄 Boundary:** граничные случаи → ожидаемое поведение
- [ ] `timeout = Duration::ZERO` → `Err(ClipboardError::InvalidConfig)`
- [ ] `timeout = Duration::from_secs(3601)` (> 1 час) → `Err(ClipboardError::InvalidConfig)`
- [ ] `text = ""` (пустой текст для установки) → `Err(ClipboardError::TextEmpty)`
- [ ] `get_text()` при пустом буфере → `Err(ClipboardError::ClipboardEmpty)`
- [ ] `get_text()` при недоступном буфере → `Err(ClipboardError::ClipboardAccessDenied)`

**❌ Negative:** невалидный ввод → ожидаемая ошибка
- [ ] `CancellationToken` отменён до начала операции → `Err(ClipboardError::Cancelled)`
- [ ] `CancellationToken` отменён во время операции → `Err(ClipboardError::Cancelled)`
- [ ] `timeout = Duration::ZERO` → `Err(ClipboardError::InvalidConfig)`
- [ ] `text = ""` для `set_text()` → `Err(ClipboardError::TextEmpty)`

**🔍 Mandatory checks:**
- [ ] On `Err`, состояние системы НЕ изменилось (идемпотентность)
- [ ] Нет дублирующихся событий/логов
- [ ] Нет `unwrap()`, `panic!`, `expect()` в `src/core/` и `src/runtime/`
- [ ] Тесты отмены через `CancellationToken`
- [ ] Тесты таймаута (`Duration::ZERO` → `InvalidConfig`)
- [ ] Тесты мокового backend для изоляции состояния
- [ ] Тесты проверки идемпотентности при повторном вызове с `Err`

---
## 🗓️ For `/plan`: tests as steps

- [ ] Создать файл: `src/core/clipboard.rs` — добавить `mod tests` внутри
- [ ] Implement tests: base, boundary, negative (внутри `mod tests`)
- [ ] Создать файл: `tests/integration/clipboard_tool.rs` — интеграционные тесты
- [ ] Implement integration tests: полный lifecycle, отмена, таймаут
- [ ] Run: `cargo test -- --nocapture`
- [ ] Run: `cargo test --test clipboard_tool -- --nocapture`
- [ ] Run: `cargo clippy -- -D warnings`
- [ ] Run: `cargo fmt --check`
