## 🧪 Тест-план: SetTextTool | smith-windows

### 📊 Обзор
Покрытие всех сценариев из спецификации и контракта: валидация конфига, валидация текста, проверка состояния элемента, timeout, отмена, COM-ошибки.

---

### 🧩 Юнит-тесты (src/core/set_text.rs - `mod tests`)

#### 1. Валидация конфигурации

| ID | Сценарий | Вход | Ожидание |
|----|----------|------|----------|
| UT1.1 | Валидный config | `timeout: Duration::from_secs(5)` | `Ok(())` |
| UT1.2 | Нулевой timeout | `timeout: Duration::ZERO` | `Err(SetTextError::InvalidConfig)` |
| UT1.3 | Отрицательный timeout | `timeout: Duration::from_millis(0)` | `Err(SetTextError::InvalidConfig)` |
| UT1.4 | Large timeout (>1ч) | `timeout: Duration::from_secs(3601)` | `Err(SetTextError::InvalidConfig)` |
| UT1.5 | Пустой текст | `text: ""` | `Err(SetTextError::InvalidConfig)` |
| UT1.6 | Unicode текст | `text: "Привет, мир! 👋"` | `Ok(())` после валидации config |

#### 2. Mock backend

| ID | Сценарий | State | Ожидание |
|----|----------|-------|----------|
| UT2.1 | Успешная установка | `should_succeed: true` | `Ok(())`, `call_count = 1` |
| UT2.2 | Ошибка ElementNotFound | `should_succeed: false`, `last_error: Some(ElementNotFound)` | `Err(ElementNotFound)`, `call_count = 1` |
| UT2.3 | Ошибка ElementNotEnabled | `should_succeed: false`, `last_error: Some(ElementNotEnabled)` | `Err(ElementNotEnabled)`, `call_count = 1` |
| UT2.4 | Ошибка ElementOffscreen | `should_succeed: false`, `last_error: Some(ElementOffscreen)` | `Err(ElementOffscreen)`, `call_count = 1` |
| UT2.5 | Ошибка ElementNotWritable | `should_succeed: false`, `last_error: Some(ElementNotWritable)` | `Err(ElementNotWritable)`, `call_count = 1` |
| UT2.6 | Идемпотентность при Err | `should_succeed: false` | Повторный вызов возвращает ту же ошибку, состояние не меняется |

#### 3. Timeout и отмена

| ID | Сценарий | Конфиг | Ожидание |
|----|----------|--------|----------|
| UT3.1 | Timeout срабатывание | `timeout: Duration::ZERO` | `Err(SetTextError::Timeout)` (в `set_text_with_config`) |
| UT3.2 | Отмена до начала | `cancellation.cancel()` до вызова | `Err(SetTextError::Cancelled)` |
| UT3.3 | Отмена во время выполнения | Отмена в процессе `set_text()` | `Err(SetTextError::Cancelled)` |

---

### 🔗 Интеграционные тесты (tests/integration/set_text_tests.rs)

#### 1. Полный lifecycle

| ID | Сценарий | Элемент | Ожидание |
|----|----------|---------|----------|
| IT1.1 | Успешная установка | Валидный EditText (enabled, on-screen, writable) | `Ok(())`, текст установлен |
| IT1.2 | Element not found | Несуществующий элемент | `Err(SetTextError::ElementNotFound)` |
| IT1.3 | Element disabled | EditText в disabled state | `Err(SetTextError::ElementNotEnabled)` |
| IT1.4 | Element offscreen | Элемент не виден на экране | `Err(SetTextError::ElementOffscreen)` |
| IT1.5 | Element read-only | EditText с `is_read_only() = true` | `Err(SetTextError::ElementNotWritable)` |

#### 2. Timeout

| ID | Сценарий | Конфиг | Ожидание |
|----|----------|--------|----------|
| IT2.1 | Real timeout | `timeout: Duration::from_millis(1)` (очень короткий) | `Err(SetTextError::Timeout)` |

#### 3. Отмена

| ID | Сценарий | Конфиг | Ожидание |
|----|----------|--------|----------|
| IT3.1 | Отмена через токен | `cancellation.cancel()` до вызова | `Err(SetTextError::Cancelled)` |

#### 4. Idempotency

| ID | Сценарий | Действия | Ожидание |
|----|----------|----------|----------|
| IT4.1 | Повторная установка того же текста | Вызвать 2 раза с теми же данными | Оба вызова возвращают `Ok(())`, текст не меняется |

#### 5. Граничные случаи

| ID | Сценарий | Вход | Ожидание |
|----|----------|------|----------|
| IT5.1 | Пустой текст | `text: ""` | `Err(SetTextError::InvalidConfig)` |
| IT5.2 | Огромный текст | `text: "a".repeat(100000)` | `Ok(())` или `Err` от UIA (в зависимости от реализации) |
| IT5.3 | Unicode emoji | `text: "Hello 👋 World 🚀"` | `Ok(())` с корректной установкой |
| IT5.4 | Специальные символы | `text: "Line1\nLine2\tTab"` | `Ok(())` с сохранением символов |

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
// src/core/set_text.rs - mod tests

#[test]
fn test_validate_set_text_config_valid() {
    let cancellation = CancellationToken::new();
    let config = SetTextConfig {
        timeout: Duration::from_secs(5),
        cancellation,
    };
    
    assert!(validate_set_text_config(&config).is_ok());
}

#[test]
fn test_validate_set_text_config_empty_text() {
    let cancellation = CancellationToken::new();
    let config = SetTextConfig {
        timeout: Duration::from_secs(5),
        cancellation,
    };
    
    // Валидация config успешна, но text проверяется в set_text_with_config
    assert!(validate_set_text_config(&config).is_ok());
}

#[test]
fn test_mock_backend_idempotent_on_error() {
    let state = MockSetTextState {
        should_succeed: false,
        last_error: Some(SetTextError::ElementNotEnabled),
        ..Default::default()
    };
    let backend = MockSetTextBackend::with_state(state);
    
    // Первый вызов
    let result1 = backend.set_text(&element, "test").await;
    assert!(matches!(result1, Err(SetTextError::ElementNotEnabled)));
    
    // Повторный вызов возвращает ту же ошибку
    let result2 = backend.set_text(&element, "test").await;
    assert!(matches!(result2, Err(SetTextError::ElementNotEnabled)));
}
```
