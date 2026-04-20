# tokio-util 0.7.12

**Source**: [docs.rs](https://docs.rs/tokio-util/0.7.12/)

## 📚 Overview

`tokio-util` содержит вспомогательные утилиты и абстракции для удобной работы с `Tokio`.  
Входит в экосистему Tokio и не привязан строго к конкретной версии, но использует `tokio ^1.28.0` как основную зависимость.

## 🔑 Key Types

| Type | Description |
|------|-------------|
| `codec::Framed`, `FramedRead`, `FramedWrite` | Обёртки для `AsyncRead`/`AsyncWrite` с буферизацией и кодеками |
| `either::Either` | Комбинатор двух типов (`Either<A, B>`) |
| `context::Context` | Позволяет «привязать» значение к текущему контексту выполнения Tokio |
| `task::JoinSet` | Коллекция задач, позволяющая безопасно добавлять/извлекать задачи |
| `sync::Watch` | Однопоточная реализация watch-синхронизации |
| `time::DelayQueue` | Очередь задержек для выполнения задач по времени |

## 🔧 Key Methods

- `Framed::new(stream, codec)` — обёртка с кодеком для потока
- `Either::left(value)` / `Either::right(value)` — создание значения
- `Context::new(future, ctx)` — привязка контекста к будущему
- `JoinSet::spawn(task)` — добавление задачи в коллекцию
- `DelayQueue::insert(key, value, duration)` — добавление задачи в очередь

## ⚠️ COM Safety Rules (for smith-windows)

**Not applicable** — `tokio-util` — чисто Rust-крейт, ориентированный на асинхронное выполнение в Tokio.  
**COM (Component Object Model)** — Windows-специфичный интерфейс, а `tokio-util` не предоставляет и не поддерживает взаимодействие с COM.

## 🎯 Usage Pattern

```rust
use tokio::fs::File;
use tokio_util::codec::{Framed, LinesCodec};
use tokio_util::io::AsyncReadExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input.txt").await?;
    let mut framed = Framed::new(file, LinesCodec::new());
    while let Some(line) = framed.next().await {
        println!("Got line: {:?}", line?);
    }
    Ok(())
}
```

## 🔗 Additional Resources

- [docs.rs API](https://docs.rs/tokio-util/0.7.12/)
- [crates.io](https://crates.io/crates/tokio-util)
- [GitHub Repository](https://github.com/tokio-rs/tokio)
