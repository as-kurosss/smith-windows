# uiautomation crate: Keyboard Input & Key Simulation

**Source**: [docs.rs](https://docs.rs/uiautomation/0.24.4/uiautomation/)

## 📚 Overview

Модуль `uiautomation::inputs` предоставляет полную поддержку эмуляции ввода текста и нажатий клавиш через Windows Win32 API. Все функции работают на уровне операционной системы и не зависят от конкретного UI-элемента.

**Основные структуры:**
- `Keyboard` — ввод клавиш и текста
- `Mouse` — эмуляция событий мыши
- `Input` — представление одного элемента ввода

**Типы ошибок:**
- Все методы возвращают `Result<()>`
- Ошибки связаны с Win32 API (`SendInput`)

---

## 🔑 Key Types & Enums

### `Keyboard`

Основная структура для эмуляции ввода с клавиатуры.

| Метод | Описание |
|-------|----------|
| `new() -> Self` | Создание экземпляра |
| `interval(self, ms: u64) -> Self` | Установка задержки между нажатиями (по умолчанию 50ms) |
| `ignore_parse_err(self, ignore: bool) -> Self` | Игнорировать ошибки парсинга (вводить `{` как символ) |
| `send_keys(&self, keys: &str) -> Result<()>` | Отправка строки с синтаксисом `{}` |
| `send_text(&self, text: &str) -> Result<()>` | Отправка обычного текста (без обработки спецсимволов) |
| `send_key(&self, key: Input) -> Result<()>` | Отправка одного Input-элемента |
| `begin_hold_keys(&mut self, keys: &str) -> Result<()>` | Удержание клавиш (начало) |
| `end_hold_keys(&mut self) -> Result<()>` | Отпускание удерживаемых клавиш |

### `Mouse`

Основная структура для эмуляции событий мыши.

| Метод | Описание |
|-------|----------|
| `new() -> Self` | Создание экземпляра |
| `move_to(&self, target: &Point) -> Result<()>` | Перемещение курсора в точку |
| `click(&self, pos: &Point) -> Result<()>` | Клик по точке |
| `double_click(&self, pos: &Point) -> Result<()>` | Двойной клик по точке |

**Важно**: Метода `wheel()` (прокрутка колесика) **нет** в текущей версии.

### `Input`

Содержит следующие варианты (из синтаксиса `send_keys`):

| Тип | Описание |
|-----|----------|
| `Char(char)` | Символ ('a', 'B', '1', '!', 'й', '🚀') |
| `VirtualKey(VK)` | Виртуальный код клавиши (VK_RETURN, VK_CONTROL, VK_SHIFT) |
| `HoldKey(HoldKey)` | Клавиша для удержания (с `begin_hold_keys`/`end_hold_keys`) |

---

## 🔧 Key Methods

### `send_keys` — отправка клавиш и комбинаций

```rust
Keyboard::new().send_keys("{ctrl}{alt}{delete}")?;
```

**Синтаксис:**
- `{key}` — специальная клавиша (enter, ctrl, alt, shift, delete, tab, home, end, pgup, pgdn, left, right, up, down, f1..f24, capslock, numlock, scrolllock)
- `{key N}` — повтор клавиши N раз (`{enter 3}` → нажатие Enter 3 раза)
- `{mod}(text)` — удержание модификатора при вводе текста (`{ctrl}(AB)` → Ctrl+A+B)
- `{{}}` — экранирование фигурных скобок (`{{}Hi}` → `{Hi`)
- `()` внутри `{}` — группировка (`{shift}(abc)` → Shift+abc)

**Примеры:**

| Вызов | Эффект |
|-------|--------|
| `{enter}` | Нажатие Enter |
| `{backspace 5}` | Нажатие Backspace 5 раз |
| `{ctrl}(AB)` | Удержание Ctrl, ввод A, затем B (как Ctrl+A+B) |
| `{shift}(Hello)` | Удержание Shift, ввод "hello" как "HELLO" |
| `{{}Hi,{(}rust!{)}{}}` | Ввод текста как есть: `{Hi,(rust)}` |
| `{tab}{tab}{enter}` | Последовательное нажатие Tab, Tab, Enter |

### `send_text` — отправка обычного текста

```rust
Keyboard::new().send_text("Привет, мир! 🚀")?;
```

- Вводит **только литеральные символы**
- **Не обрабатывает** `{}`, `()`, `enter`, `ctrl` и т.п.
- Для ввода `{`, `}` как символов используйте `send_text` или экранируйте в `send_keys`

### `begin_hold_keys` / `end_hold_keys` — удержание модификаторов

```rust
let mut keyboard = Keyboard::new();

// Удерживать Shift при вводе текста
keyboard.begin_hold_keys("{shift}")?;
keyboard.send_text("hello")?;
keyboard.end_hold_keys()?;

// Удерживать Ctrl+Shift
keyboard.begin_hold_keys("{ctrl}{shift}")?;
keyboard.send_keys("{c}")?;  // Ctrl+Shift+C
keyboard.end_hold_keys()?;
```

**Важно:** 
- Удержание активно только между `begin_hold_keys` и `end_hold_keys`
- Все удерживаемые клавиши отпускаются в `end_hold_keys`

---

## ⚠️ COM Safety Rules (for smith-windows)

**Project-Specific Requirements:**
- ✅ **ВСЕ** вызовы `Keyboard::send_keys`, `Keyboard::send_text` изолируются через `tokio::task::spawn_blocking`
- ✅ **НЕТ** необходимости вызывать `CoInitializeEx` напрямую — `uiautomation` делает это внутри
- ✅ **НЕТ** ограничений на потоки — unlike `uiautomation::UIAutomation`, `Keyboard` может использоваться в любом потоке
- ✅ **ВСЕ** вызовы синхронные и не блокируют async runtime напрямую

**Rationale:**
- `Keyboard` и `Mouse` — это обёртки вокруг Win32 API `SendInput`, которые НЕ используют COM
- `SendInput` — синхронная системная функция, которая симулирует ввод на уровне драйвера
- В отличие от `UIAutomation`/`UIElement`, `Keyboard` не имеет `!Send`/`!Sync` ограничений
- Однако, для единообразия и безопасности (чтобы избежать случайных блокировок UI-потока) все вызовы должны быть изолированы через `spawn_blocking`

---

## 🎯 Usage Pattern

### Ввод текста в поле ввода

```rust
use uiautomation::inputs::Keyboard;

// Способ 1: send_text для обычного текста
Keyboard::new().send_text("Привет, мир! 🚀")?;

// Способ 2: send_keys для текста с комбинациями
Keyboard::new().send_keys("Hello {enter}World{!}")?;
```

### Эмуляция комбинаций клавиш

```rust
use uiautomation::inputs::Keyboard;

// Ctrl+C (копировать)
Keyboard::new().send_keys("{ctrl}(c)")?;

// Ctrl+Shift+V (вставить как обычный текст)
Keyboard::new().send_keys("{ctrl}{shift}(v)")?;

// Alt+F4 (закрыть окно)
Keyboard::new().send_keys("{alt}{f4}")?;

// Ctrl+Alt+Del (НЕ работает - заблокировано Windows)
// Keyboard::new().send_keys("{ctrl}{alt}{delete}")?; // ❌
```

### Удержание модификаторов

```rust
use uiautomation::inputs::Keyboard;

let mut keyboard = Keyboard::new();

// Удерживать Shift при вводе заглавных букв
keyboard.begin_hold_keys("{shift}")?;
keyboard.send_text("hello")?; // Введёт "HELLO"
keyboard.end_hold_keys()?;

// Удерживать Ctrl+Shift для комбинации
keyboard.begin_hold_keys("{ctrl}{shift}")?;
keyboard.send_keys("{c}")?; // Ctrl+Shift+C
keyboard.end_hold_keys()?;
```

### Ввод с задержкой

```rust
use uiautomation::inputs::Keyboard;

// Увеличить задержку для стабильности при вводе длинного текста
Keyboard::new()
    .interval(100)  // 100ms между нажатиями
    .send_keys("Очень длинный текст...")?;
```

### Экранирование скобок

```rust
use uiautomation::inputs::Keyboard;

// Способ 1: экранирование
Keyboard::new().send_keys("{{}Hi,{(}rust!{)}{}}")?; // Введёт: {Hi,(rust)}

// Способ 2: ignore_parse_err
Keyboard::new()
    .ignore_parse_err(true)
    .send_keys("{Hi,(rust)}")?; // Введёт: {Hi,(rust)} (без экранирования)
```

### Повтор клавиш

```rust
use uiautomation::inputs::Keyboard;

// Нажать Enter 3 раза
Keyboard::new().send_keys("{enter 3}")?;

// Нажать Backspace 5 раз
Keyboard::new().send_keys("{backspace 5}")?;
```

---

## 🔗 Additional Resources

- **docs.rs API**: [uiautomation::inputs](https://docs.rs/uiautomation/0.24.4/uiautomation/inputs/)
- **Keyboard struct**: [Keyboard docs](https://docs.rs/uiautomation/latest/uiautomation/inputs/struct.Keyboard.html)
- **Mouse struct**: [Mouse docs](https://docs.rs/uiautomation/latest/uiautomation/inputs/struct.Mouse.html)
- **Cargo.toml on crates.io**: [uiautomation](https://crates.io/crates/uiautomation)

---

## 📝 Итоговая таблица API

| Функция | Тип | Комбинации | UTF-8 | Устаревший | COM-безопасный |
|--------|-----|-------------|-------|------------|----------------|
| `Keyboard::send_text(&str)` | Текст | ❌ | ✅ | ❌ | ✅ (spawn_blocking) |
| `Keyboard::send_keys(&str)` | Клавиши | ✅ | ❌ (только клавиши) | ❌ | ✅ (spawn_blocking) |
| `Keyboard::begin_hold_keys` | Удержание | ✅ | ❌ | ❌ | ✅ (spawn_blocking) |
| `Mouse::move_to` | Мышь | ❌ | ❌ | ❌ | ✅ (spawn_blocking) |
| `Mouse::click` | Мышь | ❌ | ❌ | ❌ | ✅ (spawn_blocking) |

---

## ⚠️ Известные ограничения

1. **Win-клавиши заблокированы**: `{win}` не работает (безопасная функция Windows)
2. **Ctrl+Alt+Del заблокирован**: нельзя эмулировать через `send_keys`
3. **Нет встроенного `wheel`**: прокрутка колесика не поддерживается (требуется Win32 API напрямую)
4. **Требует фокуса**: ввод работает только на активном окне (обычно)
5. **Нет гарантии синхронности**: `send_keys` может завершиться раньше, чем ввод в приложение
6. **Скорость ввода**: по умолчанию 50ms между нажатиями — для длинного текста увеличьте через `interval()`

---

## 🆚 Альтернативы

### Использование `windows` crate напрямую

Для кастомных сценариев можно использовать Win32 API напрямую:

```rust
use windows::Win32::UI::Input::KeyboardAndMouse::{SendInput, INPUT, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, VK_KEYDOWN};
use windows::Win32::Foundation::HWND;

// Пример для Ctrl+C (гипотетически)
let inputs = [
    INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: KEYBDINPUT {
            wVk: 0x11, // VK_CONTROL
            wScan: 0,
            dwFlags: 0,
            time: 0,
            dwExtraInfo: 0,
        },
    },
    // ... остальные
];
unsafe {
    SendInput(&inputs[..]);
}
```

**Но**: `uiautomation::inputs` — это уже обёртка над `SendInput`, поэтому прямое использование `windows` редко оправдано.

### Сравнение с `uiautomation::input` (устаревший модуль)

| Модуль | Статус | Методы | Рекомендация |
|--------|--------|--------|--------------|
| `uiautomation::input` (старый) | ❌ Устаревший | `send_text`, `send_keys`, `send_key` | Используйте `uiautomation::inputs` |
| `uiautomation::inputs` (новый) | ✅ Актуальный | `Keyboard::send_keys`, `Keyboard::send_text` | **Используйте этот** |

---

## 🔧 Integration in smith-windows

**Текущее состояние:**
- `src/runtime/backends/windows/input.rs` использует `uiautomation::inputs::Keyboard::send_keys` для `click_key`
- `src/runtime/backends/windows/type.rs` использует clipboard approach (вместо `send_keys`)

**Рекомендации для `TypeBackend`:**
1. Использовать `Keyboard::send_keys("{ctrl}(v)")` для вставки через Ctrl+V
2. Или использовать clipboard approach с `Keyboard::send_keys("{ctrl}(v)")`

**Пример реализации `type_text` через `send_keys`:**

```rust
use uiautomation::inputs::Keyboard;
use clipboard::{ClipboardContext, ClipboardProvider};

pub async fn type_text(element: &uiautomation::UIElement, text: &str) -> Result<(), TypeError> {
    // Установить фокус на элемент
    let _ = element.set_focus();
    
    // Сохранить и заменить clipboard
    let original_clipboard = ClipboardContext::new()
        .ok()
        .and_then(|mut ctx| ctx.get_contents().ok());
    
    if ClipboardContext::new()
        .ok()
        .map(|mut ctx| ctx.set_contents(text.to_string()).ok())
        .flatten()
        .is_none()
    {
        return Err(TypeError::ComError("Failed to set clipboard".to_string()));
    }
    
    // Небольшая задержка для установки clipboard
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    
    // Вставить через Ctrl+V
    Keyboard::new().send_keys("{ctrl}(v)").map_err(|e| {
        TypeError::ComError(format!("Failed to paste: {}", e))
    })?;
    
    // Восстановить clipboard
    if let Some(original) = original_clipboard {
        let _ = ClipboardContext::new()
            .ok()
            .map(|mut ctx| ctx.set_contents(original).ok())
            .flatten();
    }
    
    Ok(())
}
```

---

**Author**: smith-crate-researcher  
**Date**: 2026-04-22  
**Version**: uiautomation 0.24.4
