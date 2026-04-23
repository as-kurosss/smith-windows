## 🧪 Test Plan: ReadTool | smith-windows

**✅ Positive:** валидный элемент с текстом → `Ok("expected text".to_string())`
- [ ] Тест: получение текста из поля ввода (Edit control)
- [ ] Тест: получение текста из лейбла (Text control)
- [ ] Тест: получение текста из заголовка окна (Title bar)

**🔄 Boundary:** граничные случаи → ожидаемое поведение
- [ ] `timeout = Duration::ZERO` → `Err(ReadError::InvalidConfig)`
- [ ] `timeout = Duration::from_secs(3601)` (> 1 час) → `Err(ReadError::InvalidConfig)`
- [ ] `text = ""` (пустой текст в элементе) → `Ok("".to_string())`
- [ ] `element.is_enabled() == false` → `Err(ReadError::ElementNotEnabled)`
- [ ] `element.is_offscreen() == true` → `Err(ReadError::ElementOffscreen)`

**❌ Negative:** невалидный ввод → ожидаемая ошибка
- [ ] Невалидный элемент (удалён из дерева) → `Err(ReadError::ElementNotFound)`
- [ ] Элемент без TextPattern и Name → `Err(ReadError::ElementNotWritable)`
- [ ] `CancellationToken` отменён до начала операции → `Err(ReadError::Cancelled)`
- [ ] `CancellationToken` отменён во время операции → `Err(ReadError::Cancelled)`
- [ ] `timeout = Duration::ZERO` → `Err(ReadError::InvalidConfig)`

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

- [ ] Создать файл: `src/core/read.rs` — добавить `mod tests` внутри
- [ ] Implement tests: base, boundary, negative (внутри `mod tests`)
- [ ] Создать файл: `tests/integration/read_tool.rs` — интеграционные тесты
- [ ] Implement integration tests: полный lifecycle, отмена, таймаут
- [ ] Run: `cargo test -- --nocapture`
- [ ] Run: `cargo test --test read_tool -- --nocapture`
- [ ] Run: `cargo clippy -- -D warnings`
- [ ] Run: `cargo fmt --check`
