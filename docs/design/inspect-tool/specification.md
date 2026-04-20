## 📐 Спека: InspectTool | smith-windows

**🎯 Цель:** Осуществление режима инспекции UI-элементов — запуск интерактивного режима, где при наведении курсора и нажатии Ctrl сохраняется путь автоматизации от головного окна до целевого элемента в виде строкового селектора.

**📥 Вход:**
- `element: &UIElement` | валидный UIA элемент, полученный через uiautomation | элемент, полученный в режиме инспекции
- `head_window: &UIElement` | корневой элемент (головное окно) | элемент главного окна приложения
- `config: InspectConfig` | валидная конфигурация инспекции | `InspectConfig { timeout: Duration::from_secs(10), cancellation: CancellationToken }`
- `cancellation: CancellationToken` | токен отмены операции | созданный через `tokio_util::sync::CancellationToken`

**📤 Выход:**
- `Result<String, InspectError>` | строковый путь-селектор от головного окна до элемента или конкретная ошибка
- При успехе: путь в формате `Window->Button->CheckBox{Name}` (полная иерархия от head_window до element)
- При ошибке: путь НЕ создается, состояние UI и курсора не изменилось, фиксируется лог/событие с причиной сбоя

**⚠️ Границы:**
- Элемент offscreen (не отображается): возврат `InspectError::ElementOffscreen`, путь НЕ создаётся
- Элемент disabled (неактивен): возврат `InspectError::ElementNotEnabled`, путь НЕ создаётся
- Timeout: если путь не собран за `config.timeout` → `InspectError::Timeout`, путь НЕ создаётся
- Отмена: при срабатывании `cancellation` → `InspectError::Cancelled`, путь НЕ создаётся
- Валидация конфига: `timeout <= Duration::ZERO` или `timeout > 1 час` → `InspectError::InvalidConfig`
- Валидация элемента: `element` не является валидным UIA элементом → `InspectError::ElementNotFound`
- Элемент null/nil: возврат `InspectError::ElementNotFound`
- Пустой путь (head_window == element): возврат `InspectError::InvalidSelector` (некорректная иерархия)
- Рекурсия: максимальная глубина пути — 256 элементов (защита от бесконечных циклов)
- Иерархическая проверка: `element` должен быть потомком `head_window`, иначе `InspectError::InvalidSelector`

**✅ Критерии успеха:**
- [ ] Все сценарии из «Границ» обработаны без паник и unwrap
- [ ] Путь генерируется в формате `ElementControlType{Name}` или `ElementControlType` (если Name пуст)
- [ ] Полный путь строится от head_window до element через UITreeWalker
- [ ] Состояние UI и курсора НЕ ломается при ошибке (идемпотентность)
- [ ] Логирование через `tracing` фиксирует результат или причину сбоя
- [ ] COM-вызовы изолированы через `tokio::task::spawn_blocking`
- [ ] Валидация происходит ДО вызова бэкенда
- [ ] Путь не создаётся при отмене или сбое валидации

---
## ✅ Полная реализация

### Что работает
- ✅ Полный путь иерархии строится через `UITreeWalker`, который получается из `UIAutomation::create_tree_walker()`
- ✅ Метод `get_parent()` позволяет пройти от элемента к `head_window`, собирая все промежуточные элементы в путь
- ✅ Сравнение элементов через `UIAutomation::compare_elements()` (так как `UIElement` не реализует `PartialEq`)
- ✅ Валидация элементов: `is_enabled()`, `is_offscreen()`
- ✅ Получение свойств элемента: `ControlType`, `Name`
- ✅ Построение полного пути в формате `Window->Button->CheckBox{Name}` (все родительские элементы)
- ✅ Иерархическая проверка: подтверждение, что `element` является потомком `head_window`
- ✅ Обработка всех ошибок через `InspectError` (элемент не найден, отключен, offscreen, timeout, отмена, невалидная иерархия)
- ✅ Таймауты и отмена операций через `CancellationToken`
- ✅ Максимальная глубина пути: 256 элементов (защита от бесконечных циклов)
- ✅ COM-вызовы изолированы через `tokio::task::spawn_blocking`

### Реализация через UITreeWalker
Полный путь строится следующим образом:

1. Получение `UITreeWalker` через `UIAutomation::create_tree_walker()`
2. Начиная от `element`, метод `get_parent()` вызывается рекурсивно до достижения `head_window`
3. На каждом шаге элемент добавляется в путь с проверкой на `Null` и максимальную глубину
4. Сравнение элементов выполняется через `UIAutomation::compare_elements(&parent, &head_window)`
5. При достижении `head_window` путь реверсируется и форматируется как `Window->Button->CheckBox{Name}`

**Ключевые API `uiautomation` v0.24.4:**
- `UIAutomation::create_tree_walker()` → возвращает `UITreeWalker` для обхода дерева
- `UITreeWalker::get_parent(&element)` → возвращает родительский элемент или `Null` (для head_window)
- `UIAutomation::compare_elements(&e1, &e2)` → сравнивает два элемента на идентичность

---
