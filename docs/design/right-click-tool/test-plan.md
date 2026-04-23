## 🧪 Тест-план: RightClickTool | smith-windows

**Цель:** Покрыть все сценарии из контракта, включая позитивные, негативные и граничные случаи.

---

## 📋 Блок 1: Валидация конфигурации

### 1.1 `validate_right_click_config()` — валидный конфиг
**Цель:** Принять валидную конфигурацию

**Данные:**
```rust
let config = RightClickConfig {
    timeout: Duration::from_secs(5),
    cancellation: CancellationToken::new(),
};
```

**Ожидание:** `Ok(())`

---

### 1.2 `validate_right_click_config()` — zero timeout
**Цель:** Отклонить нулевой timeout

**Данные:**
```rust
let config = RightClickConfig {
    timeout: Duration::ZERO,
    cancellation: CancellationToken::new(),
};
```

**Ожидание:** `Err(RightClickError::InvalidConfig(msg))` где `msg` содержит "timeout must be > 0"

---

### 1.3 `validate_right_click_config()` — large timeout (> 1 hour)
**Цель:** Отклонить timeout > 1 часа (защита от переполнения)

**Данные:**
```rust
let config = RightClickConfig {
    timeout: Duration::from_secs(3601), // 1 час + 1 сек
    cancellation: CancellationToken::new(),
};
```

**Ожидание:** `Err(RightClickError::InvalidConfig(msg))` где `msg` содержит "timeout must be <= 1 hour"

---

### 1.4 `validate_right_click_config()` — timeout = 1 час (граничный)
**Цель:** Принять максимальный timeout (1 час = 3600 сек)

**Данные:**
```rust
let config = RightClickConfig {
    timeout: Duration::from_secs(3600),
    cancellation: CancellationToken::new(),
};
```

**Ожидание:** `Ok(())`

---

## 📋 Блок 2: MockBackend — симуляция сценариев

### 2.1 `MockRightClickBackend` — успех
**Цель:** Проверить успешное выполнение правого клика

**Данные:**
```rust
let backend = MockRightClickBackend::new();
let mut state = backend.get_state();
state.should_succeed = true;
```

**Ожидание:** `Ok(())`, `state.call_count == 1`

---

### 2.2 `MockRightClickBackend` — ElementNotFound
**Цель:** Проверить обработку ошибки "элемент не найден"

**Данные:**
```rust
let mut state = backend.get_state();
state.should_succeed = false;
state.last_error = Some(RightClickError::ElementNotFound);
```

**Ожидание:** `Err(RightClickError::ElementNotFound)`

---

### 2.3 `MockRightClickBackend` — ElementNotEnabled
**Цель:** Проверить обработку ошибки "элемент отключен"

**Ожидание:** `Err(RightClickError::ElementNotEnabled)`

---

### 2.4 `MockRightClickBackend` — ElementOffscreen
**Цель:** Проверить обработку ошибки "элемент скрыт"

**Ожидание:** `Err(RightClickError::ElementOffscreen)`

---

### 2.5 `MockRightClickBackend` — Timeout
**Цель:** Проверить обработку таймаута

**Ожидание:** `Err(RightClickError::Timeout)`

---

### 2.6 `MockRightClickBackend` — reset()
**Цель:** Проверить сброс состояния

**Данные:**
```rust
let backend = MockRightClickBackend::new();
backend.reset();
```

**Ожидание:** `backend.get_state().call_count == 0`

---

## 📋 Блок 3: Integрационные тесты (с реальным UIA)

### 3.1 `right_click_with_config()` — валидный элемент (Calculator)
**Цель:** Правый клик по реальному элементу (кнопка в Calculator)

**Данные:**
- Открыть Calculator
- Найти кнопку (например, "1")
- Вызвать `right_click_with_config(button, &config)`

**Ожидание:** `Ok(())`, контекстное меню открылось

**Примечание:** Требует наличия Calculator, может быть flaky

---

### 3.2 `right_click_with_config()` — отмена операции
**Цель:** Проверить отмену через CancellationToken

**Данные:**
```rust
let cancellation = CancellationToken::new();
let config = RightClickConfig {
    timeout: Duration::from_secs(10),
    cancellation: cancellation.clone(),
};

// Запустить right_click_with_config в spawn
// Сразу после запуска: cancellation.cancel()
```

**Ожидание:** `Err(RightClickError::Cancelled)`

---

### 3.3 `right_click_with_config()` — offscreen элемент
**Цель:** Проверить обработку offscreen элемента

**Данные:**
- Найти элемент, который скрыт за другым окном
- Вызвать `right_click_with_config()` на этом элементе

**Ожидание:** `Err(RightClickError::ElementOffscreen)`

---

### 3.4 `right_click_with_config()` — disabled элемент
**Цель:** Проверить обработку disabled элемента

**Данные:**
- Найти отключенную кнопку (например, "OK" в диалоге с невалидными данными)
- Вызвать `right_click_with_config()` на этой кнопке

**Ожидание:** `Err(RightClickError::ElementNotEnabled)`

---

## 📋 Блок 4: Комбинированные сценарии

### 4.1 Идемпотентность — повторный вызов с той же конфигурацией
**Цель:** Проверить, что повторный вызов не ломает состояние

**Данные:**
```rust
let backend = MockRightClickBackend::new();
// Вызвать right_click_with_config дважды
```

**Ожидание:** Оба вызова завершаются независимо, состояние не накапливается

---

### 4.2 Порядок вызовов — валидация ДО бэкенда
**Цель:** Проверить, что валидация происходит до вызова бэкенда

**Данные:**
```rust
let config = RightClickConfig {
    timeout: Duration::ZERO,  // Невалидный
    cancellation: CancellationToken::new(),
};
// Вызвать right_click_with_config
```

**Ожидание:** `Err(RightClickError::InvalidConfig)` — бэкенд НЕ вызывается

---

## 📋 Блок 5: Edge cases

### 5.1 `timeout = Duration::from_millis(1)` — минимальный timeout
**Цель:** Проверить минимально допустимый timeout

**Ожидание:** `Ok(())` (если элемент быстро обрабатывается)

---

### 5.2 `timeout = Duration::from_secs(3599)` — почти предел
**Цель:** Проверить timeout почти на пределе (3599 сек = 59 мин 59 сек)

**Ожидание:** `Ok(())`

---

### 5.3 Отмена до начала операции
**Цель:** Проверить отмену до запуска операции

**Данные:**
```rust
let cancellation = CancellationToken::new();
cancellation.cancel();  // Отменить ДО вызова
let config = RightClickConfig { timeout, cancellation };
```

**Ожидание:** `Err(RightClickError::Cancelled)` — валидация пропускает, но отмена срабатывает

---

## 🧪 Запуск тестов

```bash
# Запуск всех тестов RightClickTool
cargo test --lib right_click

# Запуск интеграционных тестов
cargo test --test integration right_click

# Запуск с логированием
RUST_LOG=debug cargo test --lib right_click

# Проверка clippy
cargo clippy -- -D warnings
```

---

## 📊 Критерии успеха

- [ ] Все 5 блоков покрыты тестами
- [ ] Минимум 2 теста на каждый public API (позитивный + негативный)
- [ ] Граничные случаи (0, max, 1 hour) покрыты
- [ ] `cargo test` проходит без ошибок
- [ ] `cargo clippy -- -D warnings` проходит без предупреждений
