## 🤖 Инструкция для агента: Critical Fixes & Consistency | smith-windows

**📁 Источники (читать в приоритетном порядке):**
1. `AGENTS.md` — строгие запреты (`unwrap`/`expect`/`panic!`, COM Safety)
2. `ARCHITECTURE.md` — текущая структура модулей
3. `src/core/mod.rs`, `src/lib.rs`, `src/runtime/backends/windows/mod.rs` — экспорты
4. `src/core/click.rs`, `src/core/type.rs` — TODO с таймаутами для `!Send`
5. `src/core/*_rs` — мок-бэкенды с `.expect()`
6. `README.md`, `CHANGELOG.md` — документация

**🎯 Задача:**
Устранить критические нарушения стандартов проекта, исправить обход таймаутов в `!Send` трейтах, заменить запрещённые `.expect()`/`.unwrap()`, синхронизировать экспорты модулей и обновить документацию.

**📋 Формат вывода (строго):**
`// 📄 [file_path]` → исправленный код / патч
`// 📄 README.md` / `CHANGELOG.md` / `ARCHITECTURE.md` → обновлённые секции
`✅ Compliance Checklist` → подтверждение соответствия `AGENTS.md`

**✅ Обязательные исправления:**

### 🔴 Priority 1: Запрещённые `.expect()` / `.unwrap()` в моках
**Проблема:** Все `Mock*Backend::get_state()` используют `.expect("Mock state mutex poisoned")`, что нарушает `AGENTS.md`.
**Фикс:** Заменить на `.map_err(|e| ErrorType::ComError(e.to_string()))`.
**Затронутые файлы:** `src/core/click.rs`, `src/core/type.rs`, `src/core/read.rs`, `src/core/set_text.rs`, `src/core/wait.rs`, `src/core/inspect.rs`, `src/core/screenshot.rs`, `src/core/input.rs`, `src/core/input_text.rs`, `src/core/automation_session.rs`.

### 🟠 Priority 2: Обход таймаутов в `!Send` трейтах
**Проблема:** `ClickTool` и `TypeTool` содержат комментарии: `// For now, return the direct result - timeout should be handled at a higher level`. Таймаут фактически не работает из-за `!Send` ограничения `tokio::time::timeout`.
**Фикс:** Реализовать ручную проверку через `tokio::select!`:
```rust
let deadline = tokio::time::Instant::now() + config.timeout;
loop {
    if config.cancellation.is_cancelled() { return Err(Error::Cancelled); }
    if tokio::time::Instant::now() >= deadline { return Err(Error::Timeout); }
    // Прямой вызов бэкенда (он не блокирует runtime, т.к. UIA синхронен)
    match backend.call(element).await {
        Ok(res) => return Ok(res),
        Err(e) => return Err(e),
    }
}
```
*Альтернатива:* Если UIA-вызовы мгновенны, явно задокументировать, что таймаут контролируется через `CancellationToken`, и убрать `TODO`.

### 🟡 Priority 3: Рассинхронизация экспортов модулей
**Проблема:** `src/core/mod.rs` не содержит `pub mod read;`, хотя `src/core/read.rs` существует и экспортируется в `lib.rs`.
**Фикс:** Добавить `pub mod read;` в `src/core/mod.rs`. Проверить, что все существующие модули (`wait`, `input_text`, `screenshot`, `set_text`, `inspect`) присутствуют в `mod.rs`.

### 🟢 Priority 4: Актуализация документации
**Проблема:** `ARCHITECTURE.md` и `README.md` не отражают добавленные `WaitTool`, `InputTextTool`, `ReadTool`, `ScreenshotTool`.
**Фикс:** Обновить таблицы модулей, диаграммы зависимостей и секцию `Key Features` в `README.md`. Добавить секции `[Unreleased]` в `CHANGELOG.md` для недостающих модулей.

**🚫 Запреты:**
- ❌ Оставлять любые `.expect()`, `.unwrap()`, `panic!` в `src/` и `tests/`
- ❌ Использовать `tokio::time::timeout` для `!Send` future (код не скомпилируется)
- ❌ Менять публичные сигнатуры без явного `[REQUIRES APPROVAL]`
- ❌ Пропускать `cargo clippy -- -D warnings`
- ❌ Обновлять `context_bundle.md` до полного прохождения CI

**🔄 Процесс:**
1. Исправить `.expect()` → `.map_err()` во всех мок-бэкендах.
2. Внедрить корректную обработку таймаутов/отмены в `ClickTool` и `TypeTool`.
3. Синхронизировать `src/core/mod.rs` с реальными файлами.
4. Обновить `README.md`, `CHANGELOG.md`, `ARCHITECTURE.md`.
5. Запустить `cargo test && cargo clippy -- -D warnings && cargo fmt --check`.
6. Перегенерировать `context_bundle.md` командой `cargo run --bin bundle_context`.
7. Вывести отчёт по формату.

**📝 Метаданные:**
- **Автор**: smith-compliance / smith-architect
- **Дата**: 2026-04-23
- **Статус**: `draft` → `approved` после прохождения CI
- **Целевой агент**: `smith-coder` / `smith-debugger`