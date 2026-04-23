## 📜 Контракт: WindowControlTool | smith-windows

**🔹 Требования (ДО вызова):**
- `element` должен быть валидным `UIElement` полученным через `uiautomation` crate
- `element` должен быть окном (иметь `WindowPattern` доступный через `get_pattern::<UIWindowPattern>()`)
- `config.timeout` должен быть `> 0` и `<= Duration::from_secs(3600)` (1 час)
- `config.cancellation` должен быть валидным `CancellationToken`
- `action` должен быть допустимым значением `WindowControlAction`
- Окно должно быть `is_enabled() == true` и `is_offscreen() == false`

**🔸 Гарантии (ПОСЛЕ):**
- Если `Ok(())`: окно изменено в соответствии с действием (maximize/restore/minimize), состояние окна обновлено
- Если `Err(WindowControlError::WindowPatternNotAvailable)`: состояние окна не изменилось, паттерн недоступен
- Если `Err(WindowControlError::WindowNotEnabled)`: состояние окна не изменилось, окно отключено
- Если `Err(WindowControlError::WindowOffscreen)`: состояние окна не изменилось, окно не видно
- Если `Err(WindowControlError::Timeout)`: состояние окна не изменилось, операция превысила таймаут
- Если `Err(WindowControlError::Cancelled)`: текущее выполнение прервано, состояние окна не изменилось
- Если `Err(WindowControlError::InvalidConfig)`: состояние окна не изменилось, ошибка обнаружена на этапе валидации
- Если `Err(WindowControlError::ComError)`: состояние окна не изменилось, COM/WinAPI ошибка

**🔄 Поддерживаемые действия:**
- `WindowControlAction::Maximize`: развернуть окно через `set_show_window(WindowShow::Maximize)`
- `WindowControlAction::Restore`: восстановить окно через `set_show_window(WindowShow::Restore)`
- `WindowControlAction::Minimize`: свернуть окно через `set_show_window(WindowShow::Minimize)`

**🚫 Запреты:**
- ❌ Использование `unwrap()`, `expect()`, `panic!` в любом месте реализации
- ❌ Глобальное мутабельное состояние без `Arc` + `Mutex`/`RwLock`
- ❌ Прямые вызовы WinAPI (CoInitializeEx, CoCreateInstance, BSTR, VARIANT)
- ❌ Изменение конфига внутри функции
- ❌ Модификация UI элемента без явной необходимости (только WindowPattern)
- ❌ Использование `spawn_blocking` для UIA вызовов (UIElement is !Send/!Sync)

**⚡ Сбои:**
- **Timeout**: вызов завершается с `WindowControlError::Timeout` через `tokio::time::timeout()`, состояние не меняется
- **Отмена**: проверка `cancellation.is_cancelled()` перед и во время выполнения, возврат `WindowControlError::Cancelled`
- **Некорректный config**: валидация в `validate_window_control_config()` → `WindowControlError::InvalidConfig` ДО вызова бэкенда
- **WindowPattern недоступен**: проверка `get_pattern::<UIWindowPattern>()` → `WindowControlError::WindowPatternNotAvailable`
- **COM-ошибки**: перехват через `anyhow` → `WindowControlError::ComError(String)`

---
## 🗓️ Для `/plan`: ключевые точки валидации
- [ ] Валидация входа происходит в `validate_config()` и `validate_element_ready()` ДО любых вызовов бэкенда
- [ ] События/логи отправляются через `tracing::info!()` при успехе, `tracing::error!()` при ошибках
- [ ] Ошибки обрабатываются через `WindowControlError` (`thiserror`), не через `panic!` или `unwrap()`
- [ ] COM-вызовы выполняются на том же потоке (UIA требует STA affinity)
- [ ] Проверка WindowPattern выполняется через `get_pattern::<UIWindowPattern>()` метод `uiautomation`
