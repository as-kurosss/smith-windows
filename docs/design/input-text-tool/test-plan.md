## 🧪 Тест-план: InputTextTool | smith-windows

### 📊 Обзор
Покрытие всех сценариев из спецификации и контракта: валидация конфига, валидация текста, валидация селектора, проверка состояния элемента, timeout, отмена, синтаксис `{}`, Unicode, COM-ошибки.

---

### 🧩 Юнит-тесты (src/core/input_text.rs - `mod tests`)

#### 1. Валидация конфигурации

| ID | Сценарий | Вход | Ожидание |
|----|----------|------|----------|
| UT1.1 | Валидный config | `timeout: Duration::from_secs(5)` | `Ok(())` |
| UT1.2 | Нулевой timeout | `timeout: Duration::ZERO` | `Err(InputTextError::InvalidConfig)` |
| UT1.3 | Отрицательный timeout | `timeout: Duration::from_millis(0)` | `Err(InputTextError::InvalidConfig)` |
| UT1.4 | Large timeout (>1ч) | `timeout: Duration::from_secs(3601)` | `Err(InputTextError::InvalidConfig)` |
| UT1.5 | Пустой текст | `text: ""` | `Err(InputTextError::InvalidConfig)` |
| UT1.6 | Unicode текст | `text: "Привет, мир! 👋"` | `Ok(())` после валидации config |
| UT1.7 | Слишком длинный текст | `text: "a".repeat(65537)` | `Err(InputTextError::InvalidConfig)` |

#### 2. Валидация селектора

| ID | Сценарий | Вход | Ожидание |
|----|----------|------|----------|
| UT2.1 | Валидный selector | `selector: "#editbox"` | `Ok(())` |
| UT2.2 | Пустой selector | `selector: ""` | `Err(InputTextError::InputSelectorError)` |
| UT2.3 | Слишком длинный selector | `selector: "a".repeat(2049)` | `Err(InputTextError::InputSelectorError)` |
| UT2.4 | Unicode selector | `selector: "#поле_ввода"` | `Ok(())` |

#### 3. Mock backend

| ID | Сценарий | State | Ожидание |
|----|----------|-------|----------|
| UT3.1 | Успешный ввод | `should_succeed: true` | `Ok(())`, `call_count = 1`, `last_keys = "Hello"` |
| UT3.2 | Ошибка ElementNotFoundError | `should_succeed: false`, `last_error: Some(ElementNotFoundError)` | `Err(ElementNotFoundError)`, `call_count = 1` |
| UT3.3 | Ошибка ElementNotEnabled | `should_succeed: false`, `last_error: Some(ElementNotEnabled)` | `Err(ElementNotEnabled)`, `call_count = 1` |
| UT3.4 | Ошибка ElementOffscreen | `should_succeed: false`, `last_error: Some(ElementOffscreen)` | `Err(ElementOffscreen)`, `call_count = 1` |
| UT3.5 | Ошибка ElementReadOnly | `should_succeed: false`, `last_error: Some(ElementReadOnly)` | `Err(ElementReadOnly)`, `call_count = 1` |
| UT3.6 | Ошибка ввода | `should_succeed: false`, `last_error: Some(InputExecutionError)` | `Err(InputExecutionError)` |
| UT3.7 | Идемпотентность при Err | `should_succeed: false` | Повторный вызов возвращает ту же ошибку, состояние не меняется |

#### 4. Синтаксис `send_keys()`

| ID | Сценарий | Вход | Ожидание |
|----|----------|------|----------|
| UT4.1 | Обычный текст | `text: "Hello"` | `Keyboard::send_text("Hello")` или `send_keys("Hello")` |
| UT4.2 | Специальная клавиша | `text: "{enter}"` | `Keyboard::send_keys("{enter}")` |
| UT4.3 | Комбинация Ctrl+C | `text: "{ctrl}(c)"` | `Keyboard::send_keys("{ctrl}(c)")` |
| UT4.4 | Комбинация Ctrl+Shift+V | `text: "{ctrl}{shift}(v)"` | `Keyboard::send_keys("{ctrl}{shift}(v)")` |
| UT4.5 | Повтор клавиши | `text: "{enter 3}"` | `Keyboard::send_keys("{enter 3}")` |
| UT4.6 | Модификаторы | `text: "{lctrl}{rshift}(a)"` | `Keyboard::send_keys("{lctrl}{rshift}(a)")` |
| UT4.7 | Unicode текст | `text: "Привет"` | `Keyboard::send_text("Привет")` |
| UT4.8 | Смешанный ввод | `text: "Hello {enter}World {ctrl}(a)"` | `Keyboard::send_keys("Hello {enter}World {ctrl}(a)")` |

#### 5. Timeout и отмена

| ID | Сценарий | Конфиг | Ожидание |
|----|----------|--------|----------|
| UT5.1 | Timeout срабатывание | `timeout: Duration::ZERO` | `Err(InputTextError::Timeout)` (в `input_text_with_config`) |
| UT5.2 | Отмена до начала | `cancellation.cancel()` до вызова | `Err(InputTextError::Cancelled)` |
| UT5.3 | Отмена во время выполнения | Отмена в процессе `input_text()` | `Err(InputTextError::Cancelled)` |

---

### 🔗 Интеграционные тесты (tests/integration/input_text_tests.rs)

#### 1. Полный lifecycle

| ID | Сценарий | Элемент | Ожидание |
|----|----------|---------|----------|
| IT1.1 | Успешный ввод | Валидный EditText (enabled, on-screen, writable) | `Ok(())`, текст введён |
| IT1.2 | Element not found | Несуществующий элемент | `Err(InputTextError::ElementNotFoundError)` |
| IT1.3 | Element disabled | EditText в disabled state | `Err(InputTextError::ElementNotEnabled)` |
| IT1.4 | Element offscreen | Элемент не виден на экране | `Err(InputTextError::ElementOffscreen)` |
| IT1.5 | Element read-only | EditText с `is_read_only() = true` | `Err(InputTextError::ElementReadOnly)` |

#### 2. Timeout

| ID | Сценарий | Конфиг | Ожидание |
|----|----------|--------|----------|
| IT2.1 | Real timeout | `timeout: Duration::from_millis(1)` (очень короткий) | `Err(InputTextError::Timeout)` |

#### 3. Отмена

| ID | Сценарий | Конфиг | Ожидание |
|----|----------|--------|----------|
| IT3.1 | Отмена через токен | `cancellation.cancel()` до вызова | `Err(InputTextError::Cancelled)` |

#### 4. Idempotency

| ID | Сценарий | Действия | Ожидание |
|----|----------|----------|----------|
| IT4.1 | Повторный ввод того же текста | Вызвать 2 раза с теми же данными | Оба вызова возвращают `Ok(())`, текст не меняется |

#### 5. Синтаксис `send_keys()`

| ID | Сценарий | Вход | Ожидание |
|----|----------|------|----------|
| IT5.1 | Обычный текст | `text: "Hello"` | `Ok(())`, введён "Hello" |
| IT5.2 | Специальная клавиша | `text: "{enter}"` | `Ok(())`, введён Enter |
| IT5.3 | Комбинация Ctrl+C | `text: "{ctrl}(c)"` | `Ok(())`, выполнено Ctrl+C |
| IT5.4 | Повтор | `text: "{tab 2}"` | `Ok(())`, 2 нажатия Tab |
| IT5.5 | Unicode | `text: "Привет, мир!"` | `Ok(())`, введён "Привет, мир!" |
| IT5.6 | Смешанный | `text: "Hello {enter}World {ctrl}(a)"` | `Ok(())`, введён текст + комбинации |

#### 6. Граничные случаи

| ID | Сценарий | Вход | Ожидание |
|----|----------|------|----------|
| IT6.1 | Пустой текст | `text: ""` | `Err(InputTextError::InvalidConfig)` |
| IT6.2 | Пустой selector | `selector: ""` | `Err(InputTextError::InputSelectorError)` |
| IT6.3 | Огромный текст | `text: "a".repeat(65536)` | `Ok(())` или `Err` от UIA (в зависимости от реализации) |
| IT6.4 | Максимальный selector | `selector: "a".repeat(2048)` | `Ok(())` |
| IT6.5 | Unicode emoji | `text: "Hello 👋 World 🚀"` | `Ok(())` с корректной отправкой |

---

### ✅ Критерии успеха тестов

- [ ] Все юнит-тесты проходят: `cargo test --lib`
- [ ] Все интеграционные тесты проходят: `cargo test --test integration`
- [ ] `cargo clippy -- -D warnings` без предупреждений
- [ ] Покрытие: 100% сценариев из спецификации
- [ ] Идемпотентность: повторные вызовы с `Err` не меняют состояние
- [ ] COM-безопасность: тесты не вызывают race conditions

---

### 📝 Примеры тестов

```rust
// src/core/input_text.rs - mod tests

#[test]
fn test_validate_input_text_config_valid() {
    let cancellation = CancellationToken::new();
    let config = InputTextConfig {
        text: "Hello".to_string(),
        timeout: Duration::from_secs(5),
        cancellation,
    };

    assert!(validate_input_text_config(&config).is_ok());
}

#[test]
fn test_validate_input_text_config_empty_text() {
    let cancellation = CancellationToken::new();
    let config = InputTextConfig {
        text: "".to_string(), // пустой текст
        timeout: Duration::from_secs(5),
        cancellation,
    };

    assert!(matches!(
        validate_input_text_config(&config),
        Err(InputTextError::InvalidConfig(_))
    ));
}

#[test]
fn test_validate_input_selector_valid() {
    assert!(validate_input_selector("#editbox").is_ok());
}

#[test]
fn test_validate_input_selector_empty() {
    assert!(matches!(
        validate_input_selector(""),
        Err(InputTextError::InputSelectorError(_))
    ));
}

#[test]
fn test_mock_backend_idempotent_on_error() {
    let state = MockInputTextState {
        should_succeed: false,
        last_error: Some(InputTextError::ElementNotEnabled),
        ..Default::default()
    };
    let backend = MockInputTextBackend::with_state(state);

    // Первый вызов
    let result1 = backend.input_text(&element, "test").await;
    assert!(matches!(result1, Err(InputTextError::ElementNotEnabled)));

    // Повторный вызов возвращает ту же ошибку
    let result2 = backend.input_text(&element, "test").await;
    assert!(matches!(result2, Err(InputTextError::ElementNotEnabled)));
}
```
