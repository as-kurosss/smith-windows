# global-hotkey 0.7.0

**Source**: [docs.rs](https://docs.rs/global-hotkey/0.7.0/global_hotkey/)

## 📚 Overview

`global-hotkey` — кросс-платформенная библиотека для регистрации и обработки глобальных горячих клавиш (hotkeys) в Windows, macOS и Linux (X11).

**Статус документации**: ~53.49% кода задокументировано

**Лицензия**: Apache-2.0 OR MIT

**Репозиторий**: https://github.com/tauri-apps/global-hotkey

## 🔑 Key Types

| Type | Description |
|------|-------------|
| `GlobalHotKeyManager` | Управление глобальными хоткеями: `new()`, `register()`, `unregister()` |
| `GlobalHotKeyEvent` | Событие нажатия/отпускания клавиши (содержит ID, состояние, клавишу) |
| `HotKey` | Горячая клавиша: модификаторы + код клавиши |
| `Modifiers` | `SHIFT`, `CTRL`, `ALT`, `META` и пр. (битовая маска) |
| `Code` | `KeyA`, `KeyD`, `Space`, `F1`, `Enter` и пр. (коды клавиш) |
| `HotKeyState` | `Registered`, `NotRegistered` (состояние регистрации) |
| `Error` | `RegisterFailed(String)`, `UnregisterFailed(String)` |
| `Result` | `Result<T, Error>` |

## 🔧 Key Methods

- `GlobalHotKeyManager::new()` — создать менеджер хоткеев (требует активный Win32 event loop)
- `GlobalHotKeyManager::register(hotkey)` — зарегистрировать хоткей, возвращает `u32` ID
- `GlobalHotKeyManager::unregister(id)` — отменить регистрацию хоткея по ID
- `HotKey::new(opt_modifiers, code)` — создать горячую клавишу с опциональными модификаторами
- `GlobalHotKeyEvent::receiver()` — получить **синхронный** `crossbeam_channel::Receiver` для событий
- `GlobalHotKeyEvent::try_recv()` — получить событие без блокировки

## ⚠️ COM Safety Rules (for smith-windows)

**Project-Specific Requirements:**
- ❌ **НЕ рекомендуется** использовать `global-hotkey` напрямую в `smith-windows`
- ❌ Библиотека **не async-совместима** и **не интегрируется с tokio** напрямую
- ⚠️ Требует наличия активного Win32 event loop **на том же потоке**, где создаётся `GlobalHotKeyManager`
- ⚠️ На Windows обязательно наличие оконного цикла сообщений (message loop)

**Rationale:**
`smith-windows` построен вокруг `tokio` async runtime и `uiautomation` crate. Прямое использование `global-hotkey` нарушит принципы:
1. **COM Isolation**: все вызовы WinAPI должны изолироваться через `tokio::task::spawn_blocking`
2. **UIAutomation-First**: все UI операции через `uiautomation`, а не через WinAPI hooks
3. **Modularity**: отсутствие привязки к конкретному event loop

## 🎯 Usage Pattern

### Базовая регистрация хоткея:

```rust
use global_hotkey::{GlobalHotKeyManager, hotkey::{HotKey, Modifiers, Code}};

// Создать менеджер (должен быть на потоке с Win32 event loop)
let manager = GlobalHotKeyManager::new()?;

// Создать хоткей: Ctrl+Shift+I
let hotkey = HotKey::new(
    Some(Modifiers::CTRL | Modifiers::SHIFT),
    Code::KeyI
);

// Зарегистрировать — вернёт id (u32)
let id = manager.register(hotkey)?;
```

### Обработка событий (синхронный цикл):

```rust
use global_hotkey::GlobalHotKeyEvent;

// Получение события без блокировки
if let Ok(event) = GlobalHotKeyEvent::receiver().try_recv() {
    if event.pressed && event.id == id {
        println!("HotKey pressed!");
    }
}
```

### Интеграция с tokio (workaround):

```rust
// Запустить event loop в отдельном потоке
let hotkey_manager = std::thread::spawn(move || {
    let manager = GlobalHotKeyManager::new().unwrap();
    let hotkey = HotKey::new(Some(Modifiers::CTRL), Code::KeyI);
    let id = manager.register(hotkey).unwrap();

    // Синхронно опрашивать события
    loop {
        if let Ok(event) = GlobalHotKeyEvent::receiver().try_recv() {
            // Отправить событие в tokio-канал
            // async_sender.send(event).await.unwrap()
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
});
```

## 🔗 Additional Resources

- [docs.rs API](https://docs.rs/global-hotkey/0.7.0/global_hotkey/)
- [GitHub Repository](https://github.com/tauri-apps/global-hotkey)
- [Crates.io](https://crates.io/crates/global-hotkey)

## 📋 Integration Notes for smith-windows

### ❌ Прямая интеграция не рекомендуется

`global-hotkey` требует привязки к OS event loop, что противоречит архитектуре `smith-windows`:
- `smith-windows` использует `tokio::task::spawn_blocking` для всех WinAPI вызовов
- `global-hotkey` требует активного Win32 message loop на том же потоке
- `global-hotkey` синхронный, а `smith-windows` — async-first

### ✅ Рекомендуемый подход

Для реализации "Ctrl + hover over element" в `smith-windows`:

1. **Использовать `uiautomation` для мониторинга элементов**:
   - Получать координаты элементов через `CurrentBoundingRectangle`
   - Использовать `uiautomation::input` для симуляции ввода

2. **Реализовать хоткеи через нативный WinAPI** (если критично):
   - `RegisterHotKey` + `GetMessage` в отдельном потоке
   - Все вызовы через `tokio::task::spawn_blocking`
   - Синхронизация через каналы (`tokio::sync::mpsc`)

3. **Рассмотреть альтернативы**:
   - GUI-панель с кнопкой (используя `winit` в отдельном модуле)
   - Встроенная команда в UI Automation сессии
   - Горячая клавиша на уровне приложения (не глобальная)

### 🚧 Возможное расширение (для будущих модулей)

Если горячие клавиши критичны для MVP:

1. Создать новый модуль `src/core/hotkey.rs` с трейтом `HotkeyBackend`
2. Windows-реализация через `RegisterHotKey` + `spawn_blocking`
3. Протестировать idempotency и отмену операций
4. Документировать в `docs/design/hotkey-tool/`

**Важно**: Всегда следовать принципу "Contracts First" — спецификация → контракт → тест-план → brief → код.
