# tracing 0.1.40

**Source**: [docs.rs](https://docs.rs/tracing/0.1.40/)

## 📚 Overview

`tracing` — фреймворк для **структурированного логирования и диагностики**, ориентированный на асинхронные и многопоточные приложения.  
Основное отличие от традиционного логирования — поддержка:
- **Способов отслеживания временных промежутков** (spans), включая вложенность и иерархию
- **Структурированной информации** (ключ-значение)
- **Контекстной привязки событий** к spans

**Требуется rustc ≥ 1.56**.

## 🔑 Key Types

| Type | Description |
|------|-------------|
| `Span` | Глубинная структура — период времени с началом и окончанием |
| `Event` | Единичный момент времени (событие), может быть привязан к `Span` |
| `Level` | Уровень детализации: `TRACE`, `DEBUG`, `INFO`, `WARN`, `ERROR` |
| `Metadata` | Метаданные спана/события (имя, уровень, целевой модуль и т.д.) |
| `Dispatch` | Объект, маршрутизирующий данные трассировки в `Subscriber` |
| `Subscriber` | Трейт, реализующий сбор и обработку событий |

## 🔧 Key Methods

#### Методы `Span`:
- `span!(Level, name, fields...)` — создаёт спан
- `span.enter()` — входит в спан, возвращает RAII-guard
- `span.in_scope(f)` — обёртывает синхронный вызов в спан
- `span.record(field, value)` — добавляет поля позже
- `Span::current()` — текущий активный спан потока

#### Методы `Event`:
- `event!(Level, fields..., "message")` — создаёт событие

#### Функции:
- `tracing::subscriber::set_global_default(subscriber)` — установка глобального сабскрайбера
- `tracing::subscriber::with_default(subscriber, || { ... })` — контекстная установка
- `enabled!(...)`, `span_enabled!(...)`, `event_enabled!(...)` — проверка включения

## ⚠️ COM Safety Rules (for smith-windows)

- Крейт **не содержит COM-зависимостей** напрямую
- На Windows может использоваться в COM-совместимых средах (например, с `tracing-etw` для записи в Windows ETW)
- Сам по себе `tracing` **не требует и не использует COM**

## 🎯 Usage Pattern

```rust
use tracing::{span, Level};

let span = span!(Level::TRACE, "my_span");
let _enter = span.enter();
// код внутри спана
```

```rust
use tracing::{instrument, Level, event};

#[instrument]
fn process(y: usize) -> Result<(), String> {
    event!(Level::DEBUG, "processing {}", y);
    if y == 0 { return Err("zero".into()); }
    Ok(())
}
```

## 🔗 Additional Resources

- [docs.rs API](https://docs.rs/tracing/0.1.40/)
- [crates.io](https://crates.io/crates/tracing)
- [GitHub Repository](https://github.com/tokio-rs/tracing)
