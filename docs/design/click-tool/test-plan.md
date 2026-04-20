## 🧪 План тестов: ClickTool | smith-windows

**✅ Позитивные:** 
- Валидный элемент, валидный config → `Ok(())` (клик выполнен через uiautomation)
- Валидный элемент с timeout=5s → `Ok(())` в пределах таймаута
- Валидный элемент, отмена после клика → `Ok(())` (отмена не влияет на уже завершённый клик)

**🔄 Граничные:**
- `timeout = Duration::ZERO` → `ClickError::InvalidConfig` (валидация)
- `timeout = Duration::from_millis(1)` → `Ok(())` или `Timeout` в зависимости от скорости элемента
- `timeout > 1 час` (например, `Duration::from_secs(3601)`) → `ClickError::InvalidConfig` (защита)
- Отрицательный timeout через `Duration::from_millis(i64::MIN)` → `ClickError::InvalidConfig`
- Пустой элемент (nil/invalid) → `ClickError::ElementNotFound`
- Элемент offscreen (скрыт за экраном) → `ClickError::ElementOffscreen`
- Элемент disabled (неактивен) → `ClickError::ElementNotEnabled`

**❌ Негативные:**
- Элемент `ElementNotEnabled` (нажатие на неактивную кнопку) → `ClickError::ElementNotEnabled`
- Элемент `ElementOffscreen` (не виден) → `ClickError::ElementOffscreen`
- Timeout при ожидании (элемент не отвечает) → `ClickError::Timeout`
- Отмена операции (`cancellation.cancel()`) → `ClickError::Cancelled`
- Невалидный config (`timeout < 0`) → `ClickError::InvalidConfig`
- COM-ошибка при клике → `ClickError::ComError(String)`

**🔍 Обязательные проверки:**
- [ ] При `Err` состояние UI не изменилось (проверка через повторный вызов с теми же данными)
- [ ] Нет дублей событий/логов (проверка через `cargo test -- --nocapture`)
- [ ] Нет `unwrap()`, `panic!`, блокировок в async (cargo clippy -- -D warnings)
- [ ] Валидация `timeout` происходит ДО вызова бэкенда (проверка в unit-тестах)
- [ ] Offscreen и disabled проверяются через `uiautomation` crate методы
- [ ] Отмена проверяется через `cancellation.is_cancelled()` перед и во время выполнения

---
## 🗓️ Для `/plan`: тесты как шаги
- [ ] Создать `mod tests` внутри `src/core/click.rs` и `tests/integration/click_tests.rs`
- [ ] Реализовать тест `test_click_success`: валидный элемент → `Ok(())`
- [ ] Реализовать тест `test_click_timeout`: элемент с долгим откликом → `ClickError::Timeout`
- [ ] Реализовать тест `test_click_offscreen`: offscreen элемент → `ClickError::ElementOffscreen`
- [ ] Реализовать тест `test_click_disabled`: disabled элемент → `ClickError::ElementNotEnabled`
- [ ] Реализовать тест `test_click_invalid_config_zero`: timeout=0 → `ClickError::InvalidConfig`
- [ ] Реализовать тест `test_click_invalid_config_large`: timeout>1h → `ClickError::InvalidConfig`
- [ ] Реализовать тест `test_click_cancelled`: отмена → `ClickError::Cancelled`
- [ ] Запустить `cargo test -- --nocapture` для проверки
- [ ] Запустить `cargo clippy -- -D warnings` для проверки качества кода
