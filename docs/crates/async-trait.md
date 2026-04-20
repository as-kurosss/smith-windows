# async-trait 0.1.83

**Source**: [docs.rs](https://docs.rs/async-trait/0.1.83/)

## 📚 Overview

`async-trait` — позволяет использовать `async fn` в трейтах и делать из таких трейтов объекты (`dyn Trait`).  
Библиотека макросов-производных, преобразующих `async fn` в синхронные методы, возвращающие `Pin<Box<dyn Future + Send + 'async_trait>>`.

**Требуется rustc ≥ 1.49** (для поддержки `async fn in trait` в стабильной версии).

## 🔑 Key Types

- **`#[async_trait]`** — единственный публичный элемент (макрос-атрибут)

## 🔧 Key Methods

Атрибут `#[async_trait]` применяется к:
- трейтам, содержащим `async fn`
- реализациям (`impl`) таких трейтов

**Поддерживаемые возможности**:
- `self`: `self`, `&self`, `&mut self`, отсутствие `self`
- любое количество и типы параметров
- обобщённые параметры (`T`, `&'a T`, `const`)
- ассоциированные типы
- смешанные (асинхронные и синхронные) методы
- реализации по умолчанию
- элидированные lifetime (для `&`/`&mut`)

## ⚠️ COM Safety Rules (for smith-windows)

**Not applicable** — `async-trait` — чисто Rust-крейт, не связан с COM или Windows-специфичными механизмами.

## 🎯 Usage Pattern

```rust
use async_trait::async_trait;

#[async_trait]
trait Advertisement {
    async fn run(&self);
}

struct Modal;

#[async_trait]
impl Advertisement for Modal {
    async fn run(&self) {
        self.render_fullscreen().await;
        remind_user_to_join_mailing_list().await;
        self.hide_for_now().await;
    }
}

// Можно создавать trait objects:
let ads: Vec<Box<dyn Advertisement + Sync>> = vec![Box::new(Modal)];
```

## 🔗 Additional Resources

- [docs.rs API](https://docs.rs/async-trait/0.1.83/)
- [crates.io](https://crates.io/crates/async-trait)
- [GitHub Repository](https://github.com/dtolnay/async-trait)
