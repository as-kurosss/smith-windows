# serde 1.0.215

**Source**: [docs.rs](https://docs.rs/serde/1.0.215/)

## 📚 Overview

`serde` — фреймворк для сериализации и десериализации данных в Rust.  
Основан на **trait-ах**, а не на runtime-рефлексии. Отсутствуют накладные расходы на reflection.

## 🔑 Key Types

| Type | Description |
|------|-------------|
| `Serialize` | Trait для структур, которые можно сериализовать |
| `Deserialize` | Trait для структур, которые можно десериализовать |
| `Serializer` | Trait для форматов, умеющих сериализовать |
| `Deserializer` | Trait для форматов, умеющих десериализовать |

## 🔧 Key Methods

- **`#[derive(Serialize)]`** — авто-генерирует реализацию `Serialize`
- **`#[derive(Deserialize)]`** — авто-генерирует реализацию `Deserialize` (требует фичу `derive`)
- **`forward_to_deserialize_any!`** — вспомогательный макрос для реализации `Deserializer`

## ⚠️ COM Safety Rules (for smith-windows)

**Not directly applicable** — `serde` — крейт чистого Rust, **не взаимодействует напрямую с COM**.  
Однако при использовании для сериализации структур, передаваемых в COM-объекты (например, через `serde_json` + `windows`), нужно учитывать COM-требования отдельно.

## 🎯 Usage Pattern

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: f64,
    y: f64,
}

fn main() {
    let point = Point { x: 1.0, y: 2.0 };

    // Сериализация
    let json = serde_json::to_string(&point).unwrap();
    println!("Сериализовано: {}", json); // {"x":1.0,"y":2.0}

    // Десериализация
    let deserialized: Point = serde_json::from_str(&json).unwrap();
    println!("Десериализовано: {:?}", deserialized);
}
```

## 🔗 Additional Resources

- [docs.rs API](https://docs.rs/serde/1.0.215/)
- [crates.io](https://crates.io/crates/serde)
- [GitHub Repository](https://github.com/serde-rs/serde)
- [Homepage](https://serde.rs/)
