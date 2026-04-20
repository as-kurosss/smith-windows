# tracing-subscriber 0.3.18

**Source**: [docs.rs](https://docs.rs/tracing-subscriber/0.3.18/)

## 📚 Overview

`tracing-subscriber` — вспомогательная библиотека для `tracing`, предоставляющая инструменты для создания и композиции подписантов (`Subscriber`) и слоёв (`Layer`).  
Основное назначение: композиция поведения трассировки через `Layer` и фильтрация через `Filter`.

**Требуемая версия Rust**: `1.63+`.

## 🔑 Key Types

| Type | Description |
|------|-------------|
| `fmt::fmt` / `fmt::Subscriber` (`FmtSubscriber`) | Подписант для форматирования и логирования трассировочных данных |
| `filter::EnvFilter` | Фильтр, имитирующий поведение `env_logger` |
| `layer::Layer` | Трейт для компонуемых слоёв поведения |
| `registry::Registry` | Хранилище данных span-ов, общее для нескольких слоёв |

## 🔧 Key Methods

- **Композиция через `Layer`** — комбинировать слои (фильтрация + форматирование + запись)
- **Фильтрация** — `Filter` настраивает, какие `span`s и `event`s будут обрабатываться
- **Перезагрузка слоёв** — модуль `reload` для динамической перезагрузки
- **Поддержка JSON** — `fmt::Subscriber` может выводить JSON (с флагом `json`)
- **Локальное время** — опциональная поддержка через `time` + `local-time`

## ⚠️ COM Safety Rules (for smith-windows)

- **COM**: Крейт не имеет отношения к COM, так как это чисто Rust-библиотека трассировки
- **Платформенные ограничения**: `i686-pc-windows-msvc`, `i686-unknown-linux-gnu`, `x86_64-apple-darwin`, `x86_64-pc-windows-msvc`, `x86_64-unknown-linux-gnu`
- **`no_std`**: Работает без `std`, но `fmt`, `EnvFilter`, `Registry`, `reload` отключены

## 🎯 Usage Pattern

```rust
use tracing_subscriber::fmt;

fn main() {
    fmt::init();
    tracing::info!("Hello, world!");
}
```

```rust
use tracing_subscriber::{fmt, filter::EnvFilter};

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
}
```

## 🔗 Additional Resources

- [docs.rs API](https://docs.rs/tracing-subscriber/0.3.18/)
- [crates.io](https://crates.io/crates/tracing-subscriber)
- [GitHub Repository](https://github.com/tokio-rs/tracing)
