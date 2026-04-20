# anyhow 1.0.95

**Source**: [docs.rs](https://docs.rs/anyhow/1.0.95/)

## 📚 Overview

`anyhow` — библиотека для удобной и идиоматичной обработки ошибок в Rust.  
Предоставляет тип `anyhow::Error`, представляющий собой динамический объект-ошибку (`trait object`), пригодный для повсеместного использования в приложениях.

## 🔑 Key Types

| Type | Description |
|------|-------------|
| `anyhow::Error` | Основной тип ошибки, упаковывающий любые типы, реализующие `std::error::Error` |
| `anyhow::Result<T>` | Алиас для `Result<T, Error>` |
| `anyhow::Chain` | Итератор цепочки исходных ошибок |
| `anyhow::Context` | Трейт, расширяющий `Result` методом `.context(...)` |
| `anyhow::format_err!` | Ресэкспорт макроса `anyhow!` (для обратной совместимости) |

## 🔧 Key Methods

- **`.context(...)`** — добавляет контекст к ошибке при пропагации через `?`
- **`.with_context(...)`** — ленивая версия `.context(...)`
- **`downcast_ref::<T>()`, `downcast_mut::<T>()`, `into_inner::<T>()`** — извлечение исходной ошибки
- **`backtrace()`** — доступ к трассировке стека (если включена)

## ⚠️ COM Safety Rules (for smith-windows)

**Not applicable** — `anyhow` — чисто Rust-контейнер ошибок, не взаимодействующий с системными интерфейсами Windows COM.  
**COM-безопасность не применима**.

## 🎯 Usage Pattern

```rust
use anyhow::{Context, Result};

fn get_cluster_info() -> Result<ClusterMap> {
    let config = std::fs::read_to_string("cluster.json")?;
    let map: ClusterMap = serde_json::from_str(&config)?;
    Ok(map)
}

fn main() -> Result<()> {
    let path = "./path/to/instrs.json";
    let content = std::fs::read(path)
        .with_context(|| format!("Failed to read instrs from {}", path))?;

    if content.is_empty() {
        bail!("Empty file");
    }

    ensure!(content.len() > 10, "File too small");

    Ok(())
}
```

## 🔗 Additional Resources

- [docs.rs API](https://docs.rs/anyhow/1.0.95/)
- [crates.io](https://crates.io/crates/anyhow)
- [GitHub Repository](https://github.com/dtolnay/anyhow)
