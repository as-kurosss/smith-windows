# 📄 Contract: SelectorStorage

## 🎯 Requirements

1. **Storage Location**: По умолчанию `std::env::temp_dir()` (используется системный temp)
2. **File Format**: JSON с pretty printing (`.json`)
3. **Naming**: `{sanitized_id}.json`

## 🛡️ Guarantees

1. **Path Traversal Protection**:
   - `sanitize_id()` блокирует `..`, `/`, `\`
   - Удаляет control characters и специальные символы
   - Возвращает error если sanitized ID пуст

2. **Validation**:
   - Maximum depth: 256 шагов
   - Maximum storage size: 100MB
   - Maximum selectors: 1000
   - Каждый шаг должен иметь хотя бы одно идентифицирующее свойство

3. **Error Handling**:
   - Все функции возвращают `Result<_, StorageError>`
   - `StorageError` — расширяемый enum через `thiserror`
   - Idempotent: повторный вызов `Err` не меняет состояние

## 🚫 Prohibitions

1. ❌ **No spawn_blocking for tokio::fs** (уже async)
2. ❌ **No raw ControlType serialization** — маппинг в String обязательный
3. ❌ **No global mutable state** без Arc + explicit context
4. ❌ **No unwrap()/expect()/panic!** в `src/`
5. ❌ **No direct COM calls** (CoInitializeEx, CoCreateInstance, BSTR, VARIANT)

## 🔥 Failure Modes

| Error Type | When | Recovery |
|------------|------|----------|
| `InvalidSelectorId` | Пустой ID, слишком длинный, invalid chars | Пользовательская валидация перед вызовом |
| `SelectorNotFound` | ID не найден | Создать новый ID |
| `SelectorAlreadyExists` | Повторный save с тем же ID | Использовать `update_selector()` (нет) или `delete_selector()` + `save_selector()` |
| `IoError` | Файловые ошибки | Проверить права, место на диске |
| `SerializationError` | JSON ошибки | Проверить данные selector'а |
| `PathTraversalDetected` | Попытка path traversal | Отклонить ID |
| `StorageSizeLimitExceeded` | Превышен лимит размера | Удалить старые selector'ы |
| `TooManySelectors` | Превышен лимит кол-ва | Удалить старые selector'ы |
| `InvalidControlType` | Неизвестный control type в stored data | Удалить corrupted файл |
| `InvalidSelectorData` | Пустые steps, отсутствие идентифицирующих свойств | Валидировать на входе |

## 📝 API

```rust
pub struct SelectorStorageConfig {
    pub storage_dir: PathBuf,
    pub max_storage_size: u64,
    pub max_selectors: usize,
}

pub struct SelectorStorage {
    config: SelectorStorageConfig,
}

impl SelectorStorage {
    pub fn new() -> Self;
    pub fn with_config(config: SelectorStorageConfig) -> Self;
    pub async fn save_selector(&self, id: &str, recorded: &RecordedSelector) -> Result<(), StorageError>;
    pub async fn load_selector(&self, id: &str) -> Result<RecordedSelector, StorageError>;
    pub async fn list_selectors(&self) -> Result<Vec<String>, StorageError>;
    pub async fn delete_selector(&self, id: &str) -> Result<(), StorageError>;
    pub fn sanitize_id(id: &str) -> Result<String, StorageError>;
}

#[derive(Error, Debug)]
pub enum StorageError {
    InvalidSelectorId(String),
    SelectorNotFound(String),
    SelectorAlreadyExists(String),
    InvalidConfig(String),
    IoError(String),
    SerializationError(String),
    PathTraversalDetected,
    StorageSizeLimitExceeded,
    TooManySelectors,
    InvalidControlType(String),
    InvalidSelectorData(String),
}
```
