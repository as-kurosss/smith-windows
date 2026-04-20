# clipboard 0.5.0

**Source**: [docs.rs](https://docs.rs/clipboard/0.5.0/)

## 📚 Overview

`clipboard` — кроссплатформенная библиотека для получения и установки содержимого системного буфера обмена.  
Применяется в Mozilla Servo.  
Лицензия: MIT / Apache 2.0.

**Размер**: ~34.62 кБ  
**Последний релиз**: 2018-09-22

## 🔑 Key Types

| Type | Description |
|------|-------------|
| `ClipboardProvider` | Трейт, предоставляющий базовый API для работы с буфером обмена |
| `ClipboardContext` | Тип-алиас для платформозависимой реализации |
| `WindowsClipboardContext` | Реализация для Windows |
| `OSXClipboardContext` | Реализация для macOS |
| `X11ClipboardContext` | Реализация для Linux (X11) |
| `NopClipboardContext` | «Пустая» реализация для неподдерживаемых платформ |

## 🔧 Key Methods

```rust
fn new() -> Result<Self, Box<Error>>;
fn get_contents(&mut self) -> Result<String, Box<Error>>;
fn set_contents(&mut self, String) -> Result<(), Box<Error>>;
```

## ⚠️ COM Safety Rules (for smith-windows)

**Platform-specific notes for Windows:**
- Использует `clipboard-win ^2.1`, который не является «чистым» COM-интерфейсом
- **Документации по COM-безопасности** в описании крейта **не приведено**
- При использовании в средах с активной COM-сессией (например, COM-серверы, Excel add-ins) требуется дополнительная осторожность
- Вызов `ClipboardProvider::new()` и методов может блокировать поток или конфликтовать с уже запущенным COM

**Recommendation**: Использовать в `tokio::task::spawn_blocking` при работе с COM-совместимыми приложениями.

## 🎯 Usage Pattern

```rust
use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;

fn example() {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    println!("{:?}", ctx.get_contents());
    ctx.set_contents("some string".to_owned()).unwrap();
}
```

## 🔗 Additional Resources

- [docs.rs API](https://docs.rs/clipboard/0.5.0/)
- [crates.io](https://crates.io/crates/clipboard)
- [GitHub Repository](https://github.com/aweinstock314/rust-clipboard)
