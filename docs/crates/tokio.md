# tokio 1.39.0

**Source**: [docs.rs](https://docs.rs/tokio/1.39.0/)

## 📚 Overview

`tokio` — асинхронный runtime для написания надёжных сетевых приложений с высокой производительностью на Rust.  
Архитектура: event-driven, non-blocking I/O, использует операционные системные механизмы (epoll, kqueue, IOCP и др.).

> **Note**: Версия 1.39.0 — **yanked** (отозвана). Рекомендуется использовать последнюю версию.

## 🔑 Key Types

| Type | Description |
|------|-------------|
| `tokio::spawn`, `JoinHandle` | Асинхронное запуск/ожидание задач |
| `tokio::sync` | Синхронизация: `Mutex`, `mpsc`, `watch`, `broadcast`, `Barrier` |
| `tokio::time` | Таймауты, `sleep`, `interval`, `timeout` |
| `tokio::net` | TCP, UDP, Unix Domain Sockets (`TcpStream`, `TcpListener`, `UdpSocket`) |
| `tokio::fs` | Асинхронные операции с файловой системой |
| `tokio::process` | Асинхронное управление процессами |
| `tokio::signal` | Асинхронная обработка сигналов (Unix/Windows) |
| `tokio::io` | Трейты `AsyncRead`, `AsyncWrite`, `AsyncBufRead`, утилиты I/O |
| `tokio::runtime` | Настройка и управление рантаймом (Builder, `handle()`, `Runtime`) |
| `tokio::task` | `spawn_blocking`, `LocalKey`, `Id`, `Builder`, `JoinSet`, метрики |

## 🔧 Key Methods

- `#[tokio::main]` — помечает `async fn main()` для запуска в рантайме
- `#[tokio::test]` — маркер асинхронных тестов
- `tokio::spawn(async { ... })` — запуск задачи в рантайме
- `tokio::task::spawn_blocking(|| { ... })` — запуск блокирующего кода в отдельном пуле потоков
- `tokio::time::timeout(Duration, fut)` — ограничение времени выполнения асинхронной операции
- `tokio::net::TcpListener::bind(addr)` — привязка TCP-сервера
- `AsyncReadExt::read_to_end(&mut buf)` / `AsyncWriteExt::write_all(&buf)` — асинхронное чтение/запись
- `tokio::sync::mpsc::channel()` — многопоточный асинхронный канал
- `tokio::fs::File::open("file.txt")` — асинхронное открытие файла

## ⚠️ COM Safety Rules (for smith-windows)

**Project-Specific Requirements:**
- `tokio` **не предоставляет** прямой поддержки COM (например, `IUnknown`, `IDispatch`, `IClassFactory` и т.д.)
- Если требуется работа с COM-объектами (например, через `windows` crate), пользователь должен **вручную обеспечить корректный вызов `CoInitializeEx` и `CoUninitialize`**
- `spawn_blocking` может быть полезен при выполнении блокирующих COM-вызовов

**Rationale:**
Крейт `tokio` не управляет COM-объектами напрямую. При интеграции с `uiautomation` или другими COM-зависимыми крейтами необходимо изолировать COM-вызовы в `spawn_blocking`.

## 🎯 Usage Pattern

```rust
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    
    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buf = [0; 1024];
            loop {
                let n = match socket.read(&mut buf).await {
                    Ok(0) => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("read error: {:?}", e);
                        return;
                    }
                };
                if let Err(e) = socket.write_all(&buf[..n]).await {
                    eprintln!("write error: {:?}", e);
                    return;
                }
            }
        });
    }
}
```

## 🔗 Additional Resources

- [docs.rs API](https://docs.rs/tokio/1.39.0/)
- [crates.io](https://crates.io/crates/tokio)
- [GitHub Repository](https://github.com/tokio-rs/tokio)
