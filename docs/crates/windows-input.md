# windows-input (windows crate) v0.58+

**Source**: [microsoft.github.io/windows-rs-docs](https://microsoft.github.io/windows-rs/)

## 📚 Overview

`windows-input` — это часть официального `windows` crate от Microsoft, предоставляющая обёртку над Win32 API для работы с вводом (клавиатура, мышь, сенсорные устройства).

**Важно**: Это **не** крейт `windows = "0.58.0"` из старой документации docs.rs — это современный `windows-rs` пакет с полной поддержкой Win32 API.

**Платформа**: Только Windows (x86_64-pc-windows-msvc, aarch64-pc-windows-msvc)

## 🔑 Key Types

| Type | Description |
|------|-------------|
| `SendInput` | Функция для синтеза ввода (клавиатура/мышь/геймпад) |
| `INPUT` | Структура ввода для `SendInput` |
| `KEYBDINPUT` | Клавиатурное событие (KeyDown, KeyUp) |
| `MOUSEINPUT` | Мышье событие (Move, LeftDown, LeftUp, Wheel) |
| `GetCursorPos`, `SetCursorPos` | Получение/установка позиции курсора |
| `POINT` | Структура координат (x, y) |
| `RegisterHotKey`, `UnregisterHotKey` | Регистрация/отмена глобальных хоткеев |
| `SetWindowsHookEx`, `CallNextHookEx` | Установка/вызов хуков сообщений |

## 🔧 Key Methods

#### Ввод и симуляция:
- `SendInput(inputs: &[INPUT], size: usize)` — синтезировать ввод (клавиатура/мышь)
- `keybd_event(bVk: u8, bScan: u8, dwFlags: u32, extraInfo: usize)` — эмуляция нажатия клавиши
- `mouse_event(dwFlags: u32, dx: i32, dy: i32, dwData: u32, extraInfo: usize)` — эмуляция мыши

#### Курсор:
- `GetCursorPos(lpPoint: *mut POINT)` — получить позицию курсора (глобально)
- `SetCursorPos(x: i32, y: i32)` — установить позицию курсора
- `GetPhysicalCursorPos`, `SetPhysicalCursorPos` — физические координаты (HIGH DPI aware)

#### Глобальные хоткеи:
- `RegisterHotKey(hWnd: HWND, id: i32, fsModifiers: u32, vk: u32)` — зарегистрировать хоткей
- `UnregisterHotKey(hWnd: HWND, id: i32)` — отменить регистрацию

#### Хуки:
- `SetWindowsHookExW(idHook: i32, lpfn: HOOKPROC, hMod: HMODULE, dwThreadId: u32)` — установить хук
- `CallNextHookEx(lpPrevHookProc: HHOOK, nCode: i32, wParam: WPARAM, lParam: LPARAM)` — передать дальше

## ⚠️ COM Safety Rules (for smith-windows)

**Project-Specific Requirements:**
- ❌ **НЕ рекомендуется** использовать `windows-input` напрямую в `smith-windows`
- ❌ Все WinAPI вызовы (включая `GetCursorPos`, `RegisterHotKey`) должны изолироваться через `tokio::task::spawn_blocking`
- ❌ Никогда не вызывать `CoInitializeEx`, `CoCreateInstance` напрямую
- ⚠️ `GetCursorPos` требует правильного масштабирования для HIGH DPI (использовать `GetPhysicalCursorPos`)
- ⚠️ `RegisterHotKey` требует `HWND` окна — для глобальных хоткеев нужен фоновый оконный процесс

**Rationale:**
`smith-windows` построен на принципах:
1. **COM Isolation**: все WinAPI вызовы в `spawn_blocking` для предотвращения нарушения COM apartment
2. **UIAutomation-First**: все UI операции через `uiautomation`, а не через низкоуровневый WinAPI
3. **Idempotency**: повторные вызовы не должны менять состояние
4. **Zero Silent Failures**: все ошибки явные через `Result`

## 🎯 Usage Pattern

### Получение позиции курсора (HIGH DPI safe):

```rust
use windows::Win32::UI::WindowsAndMessaging::{GetPhysicalCursorPos, POINT};

#[tokio::main]
async fn get_cursor_position() -> Result<(), Box<dyn std::error::Error>> {
    let position = tokio::task::spawn_blocking(|| {
        let mut point: POINT = POINT::default();
        let result = unsafe { GetPhysicalCursorPos(&mut point) };
        if result.as_bool() {
            Ok((point.x, point.y))
        } else {
            Err("GetPhysicalCursorPos failed".to_string())
        }
    }).await?;

    match position {
        Ok((x, y)) => println!("Cursor position: ({}, {})", x, y),
        Err(e) => eprintln!("Error: {}", e),
    }
    Ok(())
}
```

### Регистрация глобального хоткея (с окном):

```rust
use windows::Win32::UI::WindowsAndMessaging::{RegisterHotKey, UnregisterHotKey, VK_CONTROL, MOD_CONTROL};
use windows::Win32::UI::Controls::HWND;

#[tokio::main]
async fn register_hotkey(hwnd: HWND) -> Result<(), Box<dyn std::error::Error>> {
    let id = tokio::task::spawn_blocking(move || {
        let result = unsafe { RegisterHotKey(hwnd, 1, MOD_CONTROL.0 as u32, VK_CONTROL.0 as u32) };
        if result.as_bool() {
            Ok(1)
        } else {
            Err("RegisterHotKey failed".to_string())
        }
    }).await?;

    match id {
        Ok(hotkey_id) => println!("Hotkey registered with ID: {}", hotkey_id),
        Err(e) => eprintln!("Error: {}", e),
    }
    Ok(())
}
```

### Симуляция ввода:

```rust
use windows::Win32::UI::Input::KeyboardAndMouse::{SendInput, KEYBDINPUT, INPUT, INPUT_0, KEYEVENTF_KEYUP};
use windows::Win32::Foundation::{HWND, WPARAM, LPARAM};

#[tokio::main]
async fn send_key(vk: u8) -> Result<(), Box<dyn std::error::Error>> {
    tokio::task::spawn_blocking(move || {
        let input = INPUT {
            r#type: 1, // INPUT_KEYBOARD
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: vk,
                    wScan: 0,
                    dwFlags: 0,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        };
        
        let input_up = INPUT {
            r#type: 1, // INPUT_KEYBOARD
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: vk,
                    wScan: 0,
                    dwFlags: KEYEVENTF_KEYUP.0,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        };
        
        let size = std::mem::size_of::<INPUT>() as u32;
        unsafe {
            SendInput(&[input], size);
            SendInput(&[input_up], size);
        }
        Ok(())
    }).await?;
    
    Ok(())
}
```

## 🔗 Additional Resources

- [Official Windows-rs Documentation](https://microsoft.github.io/windows-rs/)
- [Win32 API Reference: SendInput](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-sendinput)
- [Win32 API Reference: GetCursorPos](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getcursorpos)
- [Win32 API Reference: RegisterHotKey](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerhotkeyw)

## 📋 Integration Notes for smith-windows

### ❌ Прямая интеграция не рекомендуется

`windows-input` — это **низкоуровневый WinAPI layer**, а `smith-windows` использует **высокоуровневый UI Automation API** через `uiautomation` crate.

**Почему не рекомендуется:**
1. **Дублирование функциональности**: `uiautomation` уже предоставляет клик, ввод текста, чтение свойств
2. **COM Safety**: прямые WinAPI вызовы сложнее интегрировать в `spawn_blocking` из-за зависимости от оконного контекста
3. **Платформозависимость**: `windows-input` только для Windows, `uiautomation` также только для Windows, но с лучшей абстракцией

### ✅ Когда использовать

**Только если критично необходима функциональность, отсутствующая в `uiautomation`:**

1. **Глобальные хоткеи** (если без них нельзя):
   - Создать отдельный модуль `hotkey-tool`
   - Использовать `RegisterHotKey` + `GetMessage` в `spawn_blocking`
   - Синхронизация через `tokio::sync::mpsc`

2. **Низкоуровневая симуляция ввода**:
   - Если `uiautomation::input` не подходит (например, системные хоткеи)
   - Использовать `SendInput` в `spawn_blocking`

3. **Системные хуки** (редко):
   - Если нужно перехватывать ввод глобально (мыши, клавиатура)
   - Использовать `SetWindowsHookEx` в отдельном потоке

### 🚧 Возможное расширение (для будущих модулей)

Если требуется глобальный хоткей "Ctrl+Hover":

1. Создать модуль `src/core/hotkey.rs`:
   ```rust
   #[async_trait::async_trait(?Send)]
   pub trait HotkeyBackend {
       async fn register_hotkey(&self, modifiers: Modifiers, key: Key) -> Result<u32, HotkeyError>;
       async fn unregister_hotkey(&self, id: u32) -> Result<(), HotkeyError>;
   }
   ```

2. Windows-реализация в `src/runtime/backends/windows/hotkey.rs`:
   ```rust
   pub async fn register_hotkey_with_timeout(...) -> Result<u32, HotkeyError> {
       // Создать скрытое окно для RegisterHotKey
       // Использовать spawn_blocking для WinAPI вызовов
   }
   ```

3. Документировать в `docs/design/hotkey-tool/` по принципу "Contracts First"

### ⚠️ Правила безопасности

1. **Все WinAPI вызовы** → `tokio::task::spawn_blocking`
2. **Никаких глобальных мутабельных состояний** без `Arc<Mutex<>>`
3. **Все ошибки** → явные `Result<T, Error>` через `thiserror`
4. **Проверка идемпотентности** — повторные вызовы не должны менять состояние при ошибках
5. **HIGH DPI** — использовать `GetPhysicalCursorPos`, а не `GetCursorPos`
