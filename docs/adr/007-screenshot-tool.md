# ADR 007: ScreenshotTool Module

**Date:** 2026-04-22
**Status:** [approved]
**Author:** smith-architect
**Module:** ScreenshotTool

## Context

Проект smith-windows требует модуль для **захвата скриншотов экрана, окна или региона** в PNG формате через Windows GDI/GDI+ API. Это основная функциональность для автоматической генерации документации, тестовой визуализации и отладки UI automation сценариев.

**Сценарии использования ScreenshotTool:**
- Генерация скриншотов для документации (автоматическая вставка в README/Docs)
- Визуальная инспекция состояния приложения после операций (debugging)
- Capture failures — скриншот ошибок для отладки тестов
- Regression testing — сравнение скриншотов "before/after" изменений
- Таймлайны — последовательность скриншотов для анимации процесса
- Отчётность — скриншоты в отчётах о тестировании

**Существующие решения:**
- `AutomationSession`: управление сессиями UI Automation, но НЕ захват скриншотов
- `ClickTool`, `TypeTool`, `InputTextTool`, `SetTextTool`, `WaitTool`, `InspectTool` — все работают с UI элементами, но НЕ захватывают экран
- Отсутствует модуль для захвата "глобального" состояния экрана (не привязанного к UI элементу)

**Проблема:** Нет способа получить визуальное представление экрана/окна для документации и отладки. UI Automation работает с элементами, но не с "пикселями экрана".

## Decision

Добавить новый модуль `ScreenshotTool` в структуру smith-windows как вспомогательную операцию:

- **Файлы:**
  - `src/core/screenshot.rs` — типы, трейты, валидация, режимы захвата
  - `src/runtime/backends/windows/screenshot.rs` — Windows-реализация через GDI/GDI+
  - `src/runtime/backends/unsupported.rs` — stub для non-Windows платформ
  - `tests/screenshot_tests.rs` — интеграционные тесты (в корне `tests/`)

- **Типы:**
  - `ScreenshotMode` — enum: `Screen` (весь экран) | `Window(UIElement)` (окно) | `Region{x: i32, y: i32, width: u32, height: u32}` (регион)
  - `ScreenshotConfig { timeout: Duration, cancellation: CancellationToken }`
  - `ScreenshotError` — enum с `thiserror`:
    - `InvalidRegion(String)` — неверный регион (отрицательные координаты или нулевые размеры)
    - `InvalidConfig(String)` — неверный конфиг (timeout <= 0 или timeout > 1 час)
    - `ElementNotFound` — неверный UIElement для window mode
    - `Timeout` — превышен таймаут захвата
    - `Cancelled` — операция отменена
    - `CaptureFailed(String)` — GDI/GDI+ ошибка с `GetLastError()`
    - `UnsupportedPlatform` — stub для non-Windows
  - `ScreenshotBackend` — трейт с методом `capture(mode: &ScreenshotMode) -> Result<Vec<u8>, ScreenshotError>`
  - `MockScreenshotBackend` — мок с `Arc<Mutex<MockScreenshotState>>`

- **Функции:**
  - `validate_screenshot_config()` — валидация конфига (timeout > 0, timeout <= 1 hour)
  - `validate_screenshot_mode()` — валидация режима (x >= 0, y >= 0, width > 0, height > 0)
  - `screenshot_with_config()` — основная функция с timeout/cancellation, возвращает `Result<Vec<u8>, ScreenshotError>`
  - `ScreenshotBackendWindows::capture()` — Windows-реализация через GDI/GDI+ API

- **Алгоритм GDI/GDI+ (Windows):**
  ```
  1. Determine target rect based on mode:
     - Screen: GetDesktopWindow() + GetWindowRect()
     - Window: UIElement -> window rect via AutomationProperties or GetWindowRect
     - Region: use provided x, y, width, height

  2. Create compatible DC and bitmap:
     - hdc = GetWindowDC(desktop_window) or GetDC(NULL)
     - hdc_mem = CreateCompatibleDC(hdc)
     - hbm = CreateCompatibleBitmap(hdc, width, height)
     - SelectObject(hdc_mem, hbm)

  3. Capture via BitBlt/StretchBlt:
     - BitBlt(hdc_mem, 0, 0, width, height, hdc, x, y, SRCCOPY)

  4. Convert to PNG:
     - GetDIBits() to get raw pixels
     - Encode to PNG via image crate or manual DIB->PNG conversion

  5. Cleanup:
     - DeleteObject(hbm)
     - DeleteDC(hdc_mem)
     - ReleaseDC(...)
  ```

- **Windows-реализация:**
  - Использует `tokio::task::spawn_blocking` для изоляции GDI/GDI+ вызовов
  - Нет COM инициализации требуется для GDI/GDI+ в этом паттерне
  - Ошибки: `GetLastError()` преобразуется в `ScreenshotError::CaptureFailed(String)`
  - PNG encoding: используется `image` crate или manual DIB conversion

- **Особенности:**
  - Выход: `Vec<u8>` с PNG данными (magic bytes `89 50 4E 47 0D 0A 1A 0A`)
  - Timeout не является ошибкой — это нормальное поведение (захват не завершился)
  - Все три режима: `Screen`, `Window`, `Region`
  - GDI/GDI+ calls isolated via `spawn_blocking` ( отличие от UIA: нет STA requirement)
  - Идемпотентность: повторный вызов с теми же данными не меняет состояние

## Consequences

### Positive
- ✅ Добавляется возможность захвата визуального состояния экрана (для документации и отладки)
- ✅ Единообразная архитектура с другими инструментами (ClickTool, TypeTool)
- ✅ Чёткие контракты и тесты
- ✅ Поддержка трёх режимов захвата (экран, окно, регион)
- ✅ Proper timeout and cancellation support
- ✅ PNG output for easy integration with other tools

### Negative
- ⚠️ Зависимость от GDI/GDI+ API (только Windows, но это допустимо)
- ⚠️ Дополнительная сложность конфигурации (timeout, режимы захвата)
- ⚠️ Необходимость поддержки дополнительных файлов и тестов
- ⚠️ Блокирующие GDI вызовы изолируются через `spawn_blocking` (но это усложняет threading)

## Alternative Considered

### 1. Использовать UI Automation для захвата (элемент -> image pattern)
**Rejected:** UI Automation не предоставляет прямого доступа к "пикселям окна". Pattern View (ImagePattern) не является частью стандартного UIA и требует специфичной поддержки от приложения.

### 2. Встроить захват скриншота в каждый инструмент как опциональную функцию
**Rejected:** Нарушает принцип DRY и модульности. Каждый инструмент будет дублировать логику захвата. ScreenshotTool должен быть независимым модулем.

### 3. Использовать сторонний crate (например, `screenshot`) для всех операций
**Rejected:** `uiautomation` crate уже используется, добавление ещё одного crate для захвата скриншотов нарушает принцип minimal dependencies. GDI/GDI+ — нативный Windows API, не требует дополнительных зависимостей.

### 4. Встроить захват скриншота в `AutomationSession` как метод `take_screenshot()`
**Rejected:** `AutomationSession` отвечает за запуск и сессию приложения, а не за захват экрана. Разделение ответственности. ScreenshotTool должен быть независимым инструментом.

## Implementation Checklist

- [x] `docs/design/screenshot-tool/specification.md` — создана (с полной спецификацией API)
- [x] `docs/design/screenshot-tool/contract.md` — создана (с описанием требований)
- [x] `docs/design/screenshot-tool/test-plan.md` — создана (с сценариями для всех режимов)
- [x] `docs/design/screenshot-tool/brief.md` — создана (с инструкциями для кодера)
- [x] `docs/adr/007-screenshot-tool.md` — создана (ADR с архитектурным решением)
- [x] `README.md` — обновлена таблица модулей и добавлен пример ScreenshotTool
- [x] `CHANGELOG.md` — добавлен раздел ScreenshotTool (ADR 007)
- [x] `ARCHITECTURE.md` — добавлен раздел ScreenshotTool и GDI/GDI+ safety notes
- [ ] `src/core/screenshot.rs` — создана с типами, трейтом, валидацией и unit тестами
- [ ] `src/runtime/backends/windows/screenshot.rs` — создана с полной реализацией через GDI/GDI+
- [ ] `src/runtime/backends/windows/mod.rs` — добавлен `pub mod screenshot` и экспорт `ScreenshotBackendWindows`
- [ ] `src/runtime/backends/mod.rs` — добавлен экспорт `ScreenshotBackendWindows`
- [ ] `src/runtime/backends/unsupported.rs` — добавлен `ScreenshotBackendUnsupported` stub
- [ ] `src/lib.rs` — добавлен re-export `ScreenshotMode`, `ScreenshotConfig`, `ScreenshotError`, `ScreenshotBackend`, `MockScreenshotBackend`
- [ ] `tests/screenshot_tests.rs` — интеграционные тесты (5-7 тестов с `serial_test`)
- [ ] `context_bundle.md` — обновлён с ScreenshotTool документацией
- [ ] `cargo test` — все интеграционные + unit тесты проходят
- [ ] `cargo clippy -- -D warnings` — без ошибок

## References

- `docs/design/screenshot-tool/` — полный набор документов
- `docs/design/click-tool/` — аналогичная архитектура для референса
- `docs/design/automation-session/` — UIAutomation initialization pattern
- [GDI Documentation on Microsoft Learn](https://learn.microsoft.com/en-us/windows/win32/gdi/windows-gdi)
- [BitBlt function](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-bitblt)
- [CreateDIBSection function](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createdibsection)
- [GetDesktopWindow function](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdesktopwindow)
- [GetWindowDC function](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowdc)
