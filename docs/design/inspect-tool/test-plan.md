## 🧪 План тестов: InspectTool | smith-windows

**Полная реализация:** Модуль InspectTool теперь использует `UITreeWalker` для построения полного пути от `head_window` до `element` через обход дерева вверх. Проверка иерархии выполняется через `UIAutomation::compare_elements()`.

---

**✅ Позитивные:**
- Валидный элемент → `Ok("Window->Button->CheckBox{Name}")` (полный путь от head_window до element)
- Валидный элемент с timeout=10s → `Ok(String)` в пределах таймаута
- Элемент с пустым Name → `Ok("Window->Button")` (только ControlType без имени)
- Отмена после завершения → `Ok(String)` (отмена не влияет на уже завершённую операцию)
- Иерархическая проверка пройдена: `element` является потомком `head_window`

**🔄 Граничные:**
- `timeout = Duration::ZERO` → `InspectError::InvalidConfig` (валидация)
- `timeout = Duration::from_millis(1)` → `Ok(String)` или `Timeout` в зависимости от скорости элемента
- `timeout > 1 час` (например, `Duration::from_secs(3601)`) → `InspectError::InvalidConfig` (защита)
- Отрицательный timeout через `Duration::from_millis(i64::MIN)` → `InspectError::InvalidConfig`
- Пустой элемент (nil/invalid) → `InspectError::ElementNotFound`
- Элемент offscreen (скрыт за экраном) → `InspectError::ElementOffscreen`
- Элемент disabled (неактивен) → `InspectError::ElementNotEnabled`
- **Путь глубиной > 256 элементов** → `InspectError::InvalidSelector` (максимальная глубина проверяется в коде через UITreeWalker)
- **element не является потомком head_window** → `InspectError::InvalidSelector` (проверка иерархии через UITreeWalker)

**❌ Негативные:**
- Элемент `ElementNotEnabled` (инспекция неактивного элемента) → `InspectError::ElementNotEnabled`
- Элемент `ElementOffscreen` (не виден) → `InspectError::ElementOffscreen`
- Timeout при сборе пути (элемент не отвечает) → `InspectError::Timeout`
- Отмена операции (`cancellation.cancel()`) → `InspectError::Cancelled`
- Невалидный config (`timeout < 0`) → `InspectError::InvalidConfig`
- COM-ошибка при сборе пути → `InspectError::ComError(String)`
- **Элемент не в иерархии head_window** → `InspectError::InvalidSelector` (проверка иерархии через UITreeWalker)
- **Путь глубиной > 256** → `InspectError::InvalidSelector` (ограничение глубины через UITreeWalker traversal)
- **head_window == element** → `InspectError::InvalidSelector` (пустой путь)

**🔍 Обязательные проверки:**
- [ ] При `Err` путь НЕ создан и состояние UI не изменилось (проверка через повторный вызов с теми же данными)
- [ ] Нет дублей событий/логов (проверка через `cargo test -- --nocapture`)
- [ ] Нет `unwrap()`, `panic!`, блокировок в async (cargo clippy -- -D warnings)
- [ ] Валидация `timeout` происходит ДО вызова бэкенда (проверка в unit-тестах)
- [ ] Иерархическая проверка: элемент должен быть потомком head_window (через UITreeWalker)
- [ ] Путь не создаётся при отмене или сбое валидации (проверка через мок-бэкенд)
- [ ] Максимальная глубина 256 элементов (через UITreeWalker traversal)
- [ ] Отмена проверяется через `cancellation.is_cancelled()` перед и во время выполнения

---

## 🗓️ Для `/plan`: тесты как шаги
- [ ] Создать `mod tests` внутри `src/core/inspect.rs` и `tests/integration/inspect_tests.rs`
- [ ] Реализовать тест `test_inspect_success`: валидный элемент → `Ok(String)` (с проверкой полного формата пути через UITreeWalker)
- [ ] Реализовать тест `test_inspect_timeout`: элемент с долгим откликом → `InspectError::Timeout`
- [ ] Реализовать тест `test_inspect_offscreen`: offscreen элемент → `InspectError::ElementOffscreen`
- [ ] Реализовать тест `test_inspect_disabled`: disabled элемент → `InspectError::ElementNotEnabled`
- [ ] Реализовать тест `test_inspect_invalid_config_zero`: timeout=0 → `InspectError::InvalidConfig`
- [ ] Реализовать тест `test_inspect_invalid_config_large`: timeout>1h → `InspectError::InvalidConfig`
- [ ] Реализовать тест `test_inspect_cancelled`: отмена → `InspectError::Cancelled`
- [ ] Реализовать тест `test_inspect_not_in_hierarchy`: element не является потомком head_window → `InspectError::InvalidSelector` (через UITreeWalker)
- [ ] Реализовать тест `test_inspect_max_depth`: путь > 256 элементов → `InspectError::InvalidSelector` (через UITreeWalker)
- [ ] Реализовать тест `test_inspect_empty_path`: head_window == element → `InspectError::InvalidSelector`
- [ ] Запустить `cargo test -- --nocapture` для проверки
- [ ] Запустить `cargo clippy -- -D warnings` для проверки качества кода
