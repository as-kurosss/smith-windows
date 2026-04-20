# regex 1.12.3

**Source**: [docs.rs](https://docs.rs/regex/1.12.3/)

## 📚 Overview

`regex` — библиотека для поиска подстрок в строках по регулярным выражениям.  
**Гарантии**: все поиски имеют **worst-case временную сложность O(m * n)**, где:
- `m` — размер скомпилированного регулярного выражения
- `n` — размер строки (haystack) в байтах

**Особенности**:
- По умолчанию **полная поддержка Unicode** (уровень 1 UTS#18)
- **Не поддерживает look-around и backreferences** из-за сложности их эффективной реализации
- Подходит для работы с **ненадёжными входными данными** (защита от ReDoS)

## 🔑 Key Types

| Type | Description |
|------|-------------|
| `Regex` | Скомпилированное регулярное выражение для поиска в `&str` |
| `RegexBuilder` | Настройка компиляции `Regex` (флаги, ограничения) |
| `RegexSet`, `RegexSetBuilder` | Поиск нескольких regex одновременно |
| `Captures` | Результат одного поиска с группами захвата |
| `Match` | Результат одного поиска без групп |
| `CapturesIter`, `MatchesIter` | Итераторы по всем совпадениям |
| `Split`, `SplitN` | Итераторы для разбиения строки |
| `Replacer`, `NoExpand` | Трейт и хелпер для замены совпадений |
| `Error` | Ошибка при парсинге/компиляции |

## 🔧 Key Methods

| Method | Description |
|--------|-------------|
| `Regex::new(pattern)` | Компиляция регулярного выражения |
| `Regex::is_match(haystack)` | Проверка на наличие совпадения |
| `Regex::find(haystack)` | Получение первого совпадения |
| `Regex::find_iter(haystack)` | Итератор по всем совпадениям |
| `Regex::captures(haystack)` | Получение группы захвата для первого совпадения |
| `Regex::captures_iter(haystack)` | Итератор по всем совпадениям с группами |
| `Regex::replace_all(haystack, replacement)` | Замена всех совпадений |
| `Regex::split(haystack)`, `Regex::splitn(haystack, n)` | Разбиение строки |

## ⚠️ COM Safety Rules (for smith-windows)

**Not applicable** — `regex` — чисто Rust-крейт без взаимодействия с Windows COM API.

## 🎯 Usage Pattern

```rust
// Поиск middle initial
let re = Regex::new(r"Homer (.)\. Simpson").unwrap();
let hay = "Homer J. Simpson";
let Some(caps) = re.captures(hay) else { return };
assert_eq!("J", &caps[1]);

// Извлечение по именованным группам
let re = Regex::new(r"(?<y>\d{4})-(?<m>\d{2})-(?<d>\d{2})").unwrap();
let caps = re.captures("1973-01-05").unwrap();
assert_eq!("01", &caps["m"]);
```

## 🔗 Additional Resources

- [docs.rs API](https://docs.rs/regex/1.12.3/)
- [crates.io](https://crates.io/crates/regex)
- [GitHub Repository](https://github.com/rust-lang/regex)
