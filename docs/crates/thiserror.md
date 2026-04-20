# thiserror 1.0.65

**Source**: [docs.rs](https://docs.rs/thiserror/1.0.65/)

## 📚 Overview

`thiserror` — предоставляет удобную макрос-производную (`derive`) для реализации трейта `std::error::Error`.  
Не попадает в публичный API (реализация эквивалентна ручной реализации `Error`).  
100% покрыто документацией.

## 🔑 Key Types

- **`#[derive(Error)]`** — макрос-производная для типов ошибок

## 🔧 Key Methods

| Attribute | Description |
|-----------|-------------|
| `#[error("...")]` | Генерирует реализацию `Display` для ошибки. Поддерживает интерполяцию: `{var}`, `{0}`, `{var:?}` |
| `#[from]` | Автоматически генерирует `From<T>`-реализацию |
| `#[source]` или поле `source` | Используется для реализации метода `source()` трейта `Error` |
| `backtrace: Backtrace` | Автоматически обеспечивает реализацию `provide()` для `Backtrace` (nightly ≥ 1.73) |
| `#[backtrace]` | Позволяет передавать `Backtrace` через `provide()` (nightly ≥ 1.73) |
| `#[error(transparent)]` | Проксирует `source()` и `Display` на подleying-ошибку без добавления собственного сообщения |

## ⚠️ COM Safety Rules (for smith-windows)

**Not applicable** — `thiserror` — чисто Rust-крейт, не взаимодействует с COM.  
Отсутствуют упоминания платформ Windows-specific API.

## 🎯 Usage Pattern

```rust
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataStoreError {
    #[error("data store disconnected")]
    Disconnect(#[from] io::Error),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader {
        expected: String,
        found: String,
    },
    #[error("unknown data store error")]
    Unknown,
}
```

## 🔗 Additional Resources

- [docs.rs API](https://docs.rs/thiserror/1.0.65/)
- [crates.io](https://crates.io/crates/thiserror)
- [GitHub Repository](https://github.com/dtolnay/thiserror)
