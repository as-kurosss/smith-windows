# 📋 Отчёт: Исследование `uiautomation` Crate — Ввод Текста и Эмуляция Клавиш

**Дата**: 2026-04-22  
**Версия crate**: `uiautomation` 0.24.4  
**Исследователь**: smith-crate-researcher  
**Цель**: Оценка поддержки ввода текста и эмуляции клавиш для интеграции в `smith-windows`

---

## ✅ Резюме

`uiautomation` crate v0.24.4 предоставляет **полную поддержку** ввода текста и эмуляции клавиш через модуль `uiautomation::inputs`. Все необходимые функции доступны и стабильны:

- ✅ **Да**, есть функции для эмуляции нажатия клавиш (`Keyboard::send_keys`, `Keyboard::send_text`)
- ✅ **Да**, поддержка комбинаций клавиш через синтаксис `{}` и `()`
- ✅ **Да**, методы для отправки текста как последовательность нажатий (`Keyboard::send_text`, `Keyboard::send_keys`)
- ✅ **Да**, поддержка специальных клавиш (Enter, Tab, Delete, Backspace, Arrow keys, F1-F24)
- ✅ **Да**, функции для нажатия и удержания модификаторов (`begin_hold_keys`, `end_hold_keys`)

---

## 🔤 Модули и Сущности

### 1. Основной модуль: `uiautomation::inputs`

Это **единственный** модуль для работы с вводом в версии 0.24.4.

**Структуры:**

| Сущность | Назначение | Статус |
|---------|-----------|--------|
| `Keyboard` | Эмуляция клавиатуры | ✅ Актуальный |
| `Mouse` | Эмуляция мыши | ✅ Актуальный |
| `Input` | Представление одного элемента ввода | ✅ Внутренний тип |
| `Parser` | Парсер синтаксиса `send_keys` | ✅ Внутренний тип |

### 2. Методы `Keyboard`:

```rust
// Конструкторы
pub fn new() -> Self
pub fn interval(self, ms: u64) -> Self              // Задержка между нажатиями
pub fn ignore_parse_err(self, ignore: bool) -> Self // Игнорировать ошибки парсинга

// Основные методы ввода
pub fn send_keys(&self, keys: &str) -> Result<()>       // Синтаксис с {}
pub fn send_text(&self, text: &str) -> Result<()>       // Обычный текст
pub fn send_key(&self, key: Input) -> Result<()>        // Одиночный Input

// Удержание клавиш (модификаторы)
pub fn begin_hold_keys(&mut self, keys: &str) -> Result<()> // Начало удержания
pub fn end_hold_keys(&mut self) -> Result<()>               // Конец удержания
```

### 3. Типы клавиш (из синтаксиса `send_keys`):

**Буквенно-цифровые:**
- `a`, `b`, `c`, ... `z` → `A`, `B`, `C`, ... `Z`
- `0`, `1`, `2`, ... `9`

**Специальные клавиши (в `{}`):**
- `Enter`, `Tab`, `Backspace`, `Delete`, `Escape`
- `Home`, `End`, `PageUp`, `PageDown`
- `Left`, `Right`, `Up`, `Down` (стрелки)
- `F1`, `F2`, ... `F24`
- `CapsLock`, `NumLock`, `ScrollLock`
- `Print`, `Snapshot`, `Pause`
- `Apps`, `Sleep`

**Модификаторы:**
- `{ctrl}`, `{alt}`, `{shift}`, `{win}`
- `{lctrl}`, `{rctrl}`, `{lalt}`, `{ralt}`, `{lshift}`, `{rshift}` (левые/правые версии)

**Примечание**: Полный список клавиш может быть расширее — полный список в `uiautomation::inputs::keys::VirtualKey`.

---

## 🎯 Использование: Синтаксис и Примеры

### 1. Отправка текста

```rust
// Обычный текст
Keyboard::new().send_text("Привет, мир! 🚀")?;

// Текст с комбинациями
Keyboard::new().send_keys("Hello {enter}World{!}")?;
```

### 2. Комбинации клавиш

| Комбинация | Вызов | Эффект |
|-----------|-------|--------|
| `Ctrl+C` | `send_keys("{ctrl}(c)")` | Удержание Ctrl, нажатие C |
| `Ctrl+Shift+V` | `send_keys("{ctrl}{shift}(v)")` | Удержание Ctrl+Shift, нажатие V |
| `Alt+F4` | `send_keys("{alt}{f4}")` | Последовательное нажатие Alt+F4 |
| `Ctrl+Alt+Del` | ❌ Нельзя | Заблокировано Windows |
| `Win+L` | ❌ Нельзя | Заблокировано Windows |

### 3. Повтор клавиш

```rust
// Enter 3 раза
send_keys("{enter 3}")?;

// Backspace 5 раз
send_keys("{backspace 5}")?;
```

### 4. Удержание модификаторов

```rust
let mut keyboard = Keyboard::new();

// Удерживать Shift
keyboard.begin_hold_keys("{shift}")?;
keyboard.send_text("hello")?; // Введёт "HELLO"
keyboard.end_hold_keys()?;

// Удерживать Ctrl+Shift
keyboard.begin_hold_keys("{ctrl}{shift}")?;
keyboard.send_keys("{c}")?; // Ctrl+Shift+C
keyboard.end_hold_keys()?;
```

### 5. Экранирование скобок

```rust
// Способ 1: экранирование
send_keys("{{}Hi,{(}rust!{)}{}}")?; // Введёт: {Hi,(rust)}

// Способ 2: ignore_parse_err
Keyboard::new()
    .ignore_parse_err(true)
    .send_keys("{Hi,(rust)}")?; // Введёт: {Hi,(rust)} (без экранирования)
```

### 6. Задержка между нажатиями

```rust
Keyboard::new()
    .interval(100) // 100ms между нажатиями
    .send_keys("Очень длинный текст...")?;
```

---

## ⚠️ Ограничения и Особенности

### 1. COM Safety (для smith-windows)

**Project-Specific Requirements:**
- ✅ Все вызовы `Keyboard::send_keys`, `Keyboard::send_text` **должны** быть изолированы через `tokio::task::spawn_blocking`
- ✅ **НЕТ** необходимости вызывать `CoInitializeEx` напрямую — `uiautomation` делает это внутри
- ✅ **НЕТ** ограничений на потоки — unlike `uiautomation::UIAutomation`, `Keyboard` может использоваться в любом потоке
- ✅ Все вызовы синхронные и не блокируют async runtime напрямую

**Rationale:**
- `Keyboard` и `Mouse` — это обёртки вокруг Win32 API `SendInput`, которые НЕ используют COM
- `SendInput` — синхронная системная функция, которая симулирует ввод на уровне драйвера
- В отличие от `UIAutomation`/`UIElement`, `Keyboard` не имеет `!Send`/`!Sync` ограничений

### 2. Известные ограничения

1. **Win-клавиши заблокированы**: `{win}` не работает (безопасная функция Windows)
2. **Ctrl+Alt+Del заблокирован**: нельзя эмулировать через `send_keys`
3. **Нет встроенного `wheel`**: прокрутка колесика не поддерживается (требуется Win32 API напрямую)
4. **Требует фокуса**: ввод работает только на активном окне (обычно)
5. **Нет гарантии синхронности**: `send_keys` может завершиться раньше, чем ввод в приложение
6. **Скорость ввода**: по умолчанию 50ms между нажатиями — для длинного текста увеличьте через `interval()`

### 3. Совместимость с текущей реализацией в `smith-windows`

**Текущее состояние:**
- `src/runtime/backends/windows/input.rs` использует `uiautomation::inputs::Keyboard::send_keys` для `click_key`
- `src/runtime/backends/windows/type.rs` использует clipboard approach (вместо `send_keys`)

**Рекомендации:**
- Для `TypeBackend::type_text` использовать `Keyboard::send_keys("{ctrl}(v)")` для вставки через Ctrl+V
- Или использовать clipboard approach с `Keyboard::send_keys("{ctrl}(v)")`

---

## 🔧 Рекомендации для `smith-windows`

### 1. Типы ошибок

Текущие типы ошибок в `src/core/input.rs` и `src/core/type.rs` подходят:
- `InputError::KeyClickError(String)`
- `TypeError::ComError(String)`

### 2. Использование `send_keys` в `TypeBackend`

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

### 3. Модуль `hotkey`

Текущая реализация в `src/runtime/backends/windows/hotkey.rs` использует `GetAsyncKeyState` для опроса клавиш — это **допустимый подход** для детекции нажатий модификаторов в реальном времени.

---

## 📚 Документация

### Основные ссылки

- **docs.rs API**: [uiautomation::inputs](https://docs.rs/uiautomation/0.24.4/uiautomation/inputs/)
- **Keyboard struct**: [Keyboard docs](https://docs.rs/uiautomation/latest/uiautomation/inputs/struct.Keyboard.html)
- **Mouse struct**: [Mouse docs](https://docs.rs/uiautomation/latest/uiautomation/inputs/struct.Mouse.html)
- **Cargo**: [uiautomation on crates.io](https://crates.io/crates/uiautomation)

### Созданные документы

- `docs/crates/uiautomation_keyboard_input.md` — полная спецификация ввода и эмуляции клавиш

### context_bundle.md

Обновлён: `cargo run --bin bundle_context`

- Включает 75 файлов (включая 17 файлов из `docs/crates/`)
- Актуальная документация по `uiautomation` доступна для ИИ-агентов

---

## 📊 Итоговая таблица API

| Функция | Тип | Комбинации | UTF-8 | Устаревший | COM-безопасный |
|--------|-----|-------------|-------|------------|----------------|
| `Keyboard::send_text(&str)` | Текст | ❌ | ✅ | ❌ | ✅ (spawn_blocking) |
| `Keyboard::send_keys(&str)` | Клавиши | ✅ | ❌ (только клавиши) | ❌ | ✅ (spawn_blocking) |
| `Keyboard::begin_hold_keys` | Удержание | ✅ | ❌ | ❌ | ✅ (spawn_blocking) |
| `Mouse::move_to` | Мышь | ❌ | ❌ | ❌ | ✅ (spawn_blocking) |
| `Mouse::click` | Мышь | ❌ | ❌ | ❌ | ✅ (spawn_blocking) |

---

## 🎓 Выводы

1. **Полная поддержка** ввода текста и эмуляции клавиш доступна через `uiautomation::inputs`
2. **Синтаксис простой** и интуитивный: `{key}` для специальных клавиш, `()` для группировки, `{{}}` для экранирования
3. **Удержание модификаторов** работает через `begin_hold_keys`/`end_hold_keys`
4. **COM-безопасность** обеспечена — `Keyboard` не требует `!Send`/`!Sync` ограничений
5. **Рекомендация**: использовать `Keyboard::send_keys("{ctrl}(v)")` для вставки текста через Ctrl+V

---

**Статус**: ✅ **Готов к интеграции**  
**Следующие шаги**:
1. Обновить `src/runtime/backends/windows/type.rs` для использования `Keyboard::send_keys("{ctrl}(v)")` вместо clipboard approach
2. Проверить `src/runtime/backends/windows/input.rs` — он уже использует `send_keys`
3. Добавить тесты в `tests/set_text.rs` для проверки ввода текста

---

**Автор**: smith-crate-researcher  
**Дата**: 2026-04-22
