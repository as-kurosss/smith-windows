## 📐 Спека: WindowControlTool | smith-windows

**🎯 Цель:** Управление состоянием окна Windows (развернуть, восстановить, свернуть) через UI Automation API. Решает проблему: "Как управлять состоянием окна (maximize/restore/minimize) программно через UIA?"

**📥 Вход:**
- `element: &UIElement` | валидный UIA элемент окна (должен иметь `WindowPattern`) | элемент главного окна приложения Notepad
- `config: WindowControlConfig` | валидная конфигурация с таймаутом и токеном отмены | `WindowControlConfig { timeout: Duration::from_secs(5), cancellation: CancellationToken }`
- `action: WindowControlAction` | действие над окном: `Maximize`, `Restore`, `Minimize` | `WindowControlAction::Maximize`

**📤 Выход:**
- `Result<(), WindowControlError>` | успешное завершение или конкретная ошибка
- При успехе: состояние окна изменено (развернуто/восстановлено/свернуто)
- При ошибке: состояние окна НЕ изменено, фиксируется лог с причиной сбоя

**🔄 WindowControlAction (Тип действия):**
- `WindowControlAction::Maximize` | Развернуть окно на весь экран | `window_pattern.set_show_window(WindowShow::Maximize)` | Максимизация окна
- `WindowControlAction::Restore` | Восстановить окно (из maximize/minimize) | `window_pattern.set_show_window(WindowShow::Restore)` | Восстановление окна
- `WindowControlAction::Minimize` | Свернуть окно в панель задач | `window_pattern.set_show_window(WindowShow::Minimize)` | Сворачивание окна

**⚠️ Границы:**
- Окно отключено (disabled): возврат `WindowControlError::WindowNotEnabled`
- Окно offscreen: возврат `WindowControlError::WindowOffscreen`
- Паттерн WindowPattern недоступен: возврат `WindowControlError::WindowPatternNotAvailable`
- Timeout: если операция не завершена за `config.timeout` → `WindowControlError::Timeout`
- Отмена: при срабатывании `cancellation` → `WindowControlError::Cancelled`
- Недопустимый config: `timeout <= Duration::ZERO` или `timeout > 1 час` → `WindowControlError::InvalidConfig`
- Элемент не является окном: элемент не имеет `WindowPattern` → `WindowControlError::WindowPatternNotAvailable`
- Окно уже в целевом состоянии: возврат `Ok(())` (идемпотентность)

**✅ Критерии успеха:**
- [ ] Все сценарии из «Границ» обработаны без паник и unwrap
- [ ] Состояние окна НЕ меняется при ошибке (идемпотентность)
- [ ] Логирование через `tracing` фиксирует результат или причину сбоя
- [ ] COM-вызовы выполняются на том же потоке (UIA требует STA affinity)
- [ ] Валидация происходит ДО вызова бэкенда
- [ ] Операции идемпотентны (повторный вызов не ломает состояние)

---
## 🗓️ План реализации (для `/plan`)
- [ ] Создать `src/core/window_control.rs`: `WindowControlConfig`, `WindowControlAction`, `WindowControlError`, валидация
- [ ] Создать `src/runtime/backends/windows/window_control.rs`: Windows реализация через `uiautomation::patterns::UIWindowPattern`
- [ ] Экспортировать `WindowControlTool` из `src/lib.rs`
- [ ] Добавить unit тесты: валидация, мок бэкенд, все типы действий
- [ ] Добавить integration тесты: end-to-end сценарии
- [ ] Обновить `README.md`, `CHANGELOG.md`, запустить `context_bundle`
- [ ] Проверки: `cargo test`, `cargo clippy -- -D warnings`
