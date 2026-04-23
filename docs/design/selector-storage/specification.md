# 📋 Specification: SelectorStorage

## 🎯 Overview

`SelectorStorage` — это модуль для персистентного хранения recorded selector'ов в JSON-файлах на диске. Модуль обеспечивает валидацию, защиту от path traversal атак и управление размером хранилища.

## 📏 Input/Output

### Input
```rust
// Сущности на вход
struct SelectorStorageConfig {
    storage_dir: PathBuf,           // Директория для хранения
    max_storage_size: u64,          // Максимальный размер (100MB default)
    max_selectors: usize,           // Максимальное кол-во селекторов (1000 default)
}

struct RecordedSelector {
    steps: Vec<SelectorStep>,
    depth: usize,
}

// ID должен быть валидным (без path traversal)
let id: &str = "my_selector";
```

### Output
```rust
// Результаты операций
type StorageResult<T> = Result<T, StorageError>;

// Операции
save_selector(id, recorded) -> StorageResult<()>
load_selector(id) -> StorageResult<RecordedSelector>
list_selectors() -> StorageResult<Vec<String>>
delete_selector(id) -> StorageResult<()>
```

## 📐 Boundaries

### What the module does:
- ✅ Сохраняет selector'ы в JSON-файлы
- ✅ Загружает selector'ы из JSON-файлов
- ✅ Валидирует ID и данные selector'ов
- ✅ Защищает от path traversal атак
- ✅ Управляет размером хранилища
- ✅ Преобразует ControlType ↔ String для JSON

### What the module does NOT do:
- ❌ Не использует `spawn_blocking` для tokio::fs (уже async)
- ❌ Не взаимодействует с UI Automation API
- ❌ Не снимает скриншоты
- ❌ Не делает клики

## 🧪 Test Scenarios

### Positive:
1. ✅ Сохранение и загрузка selector'а
2. ✅ Список всех сохранённых selector'ов
3. ✅ Удаление selector'а
4. ✅ Path traversal заблокирован

### Negative:
1. ❌ Ошибка при сохранении существующего ID
2. ❌ Ошибка при загрузке несуществующего ID
3. ❌ Ошибка при path traversal
4. ❌ Ошибка при превышении размера хранилища
5. ❌ Ошибка при превышении кол-ва selector'ов

### Boundary:
- 1. Пустой ID → error
- 2. Один шаг в selector'е → ok
- 3. Максимальная глубина (256) → ok
- 4. Более 256 шагов → error
