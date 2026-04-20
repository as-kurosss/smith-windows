# serde_json 1.0.133

**Source**: [docs.rs](https://docs.rs/serde_json/1.0.133/)

## 📚 Overview

`serde_json` — крейт для работы с JSON в Rust.  
Обеспечивает парсинг JSON-строк, сериализацию Rust-структур в JSON и работу с untyped (`Value`) представлением.  
Поддерживает как `std`, так и `no_std` (с `alloc`) окружениями.

## 🔑 Key Types

| Type | Description |
|------|-------------|
| `Value` | Перечисление, представляющий **любой валидный JSON** (Null, Bool, Number, String, Array, Object) |
| `Map<K, V>` | Алиас для `BTreeMap<String, Value>`, представляет JSON-объект |
| `Number` | Структура, представляющая JSON-число |
| `Error` | Описывает ошибки при сериализации/десериализации |
| `Deserializer`, `Serializer<std>` | Низкоуровневые структуры для настройки процесса |
| `StreamDeserializer<T>` | Итератор для десериализации потока JSON-значений |

## 🔧 Key Methods

| Function | Description |
|----------|-------------|
| `from_str(&str) -> Result<T>` | Парсинг JSON из `&str` |
| `from_slice(&[u8]) -> Result<T>` | Парсинг JSON из байтового среза |
| `from_reader<R: Read>(R) -> Result<T>` | Парсинг JSON из любого `io::Read` |
| `to_string(&T) -> Result<String>` | Сериализация в JSON-строку |
| `to_vec(&T) -> Result<Vec<u8>>` | Сериализация в байтовый вектор |
| `to_writer<W: Write>(&T, W) -> Result<()>` | Сериализация в любой `io::Write` |
| `to_value(&T) -> Result<Value>` | Преобразование Rust-значения в `Value` |
| `json!({ ... })` | Создание `serde_json::Value` синтаксисом, близким к JSON |

## ⚠️ COM Safety Rules (for smith-windows)

**Not applicable** — `serde_json` — pure-Rust крейт, **не взаимодействует с COM**.  
**COM-безопасность не релевантна**.

## 🎯 Usage Pattern

```rust
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
}

let data = r#"{"name":"John","age":43}"#;
let person: Person = serde_json::from_str(data)?;
println!("{} is {} years old", person.name, person.age);
```

## 🔗 Additional Resources

- [docs.rs API](https://docs.rs/serde_json/1.0.133/)
- [crates.io](https://crates.io/crates/serde_json)
- [GitHub Repository](https://github.com/serde-rs/json)
