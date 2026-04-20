use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

// Список путей: можно указывать как файлы, так и папки.
// Для папок будут рекурсивно собраны все .md файлы.
const INCLUDE_PATHS: &[&str] = &[
    "AGENTS.md",
    "ARCHITECTURE.md",
    "docs/crates/",                    // Crate documentation from docs.rs
    "docs/templates/",                 // Папка: соберёт все .md внутри
    "docs/design/click-tool/",         // Папка с модуля ClickTool
    "docs/design/type-tool/",          // Папка с модуля TypeTool
    "docs/design/inspect-tool/",       // Папка с модуля InspectTool
    "docs/design/automation-session/", // Папка модуля AutomationSession
    "docs/adr/",                       // Папка ADR
    "README.md",                       // Основной README с limitations
    "CHANGELOG.md",                    // CHANGELOG с историей изменений
];

fn main() {
    println!("🚀 Smith-Core Context Bundle Generator");
    println!("======================================");

    let mut bundle_content = String::new();
    bundle_content.push_str("# 📦 Smith-Core Context Bundle\n\n");
    bundle_content
        .push_str("Этот файл содержит эталонную документацию и правила для smith-core.\n");
    bundle_content
        .push_str("Используй эту информацию как основу для генерации кода и планирования.\n\n");
    bundle_content.push_str("---\n\n");

    let mut success_count = 0;
    let mut warn_count = 0;

    for path_str in INCLUDE_PATHS {
        let path = Path::new(path_str);

        if path.is_dir() {
            // Рекурсивная обработка папки
            match collect_markdown_files(path) {
                Ok(files) => {
                    // Сохраняем количество до того, как файлы будут обработаны
                    let count = files.len();

                    // Итерируемся по ссылке (&files), чтобы не забирать владение
                    for file_path in &files {
                        match process_file(file_path, &mut bundle_content) {
                            Ok(_) => {
                                success_count += 1;
                            }
                            Err(_) => {
                                warn_count += 1;
                            }
                        }
                    }
                    println!("📁 Processed dir: {} ({} files)", path_str, count);
                }
                Err(e) => {
                    println!("❌ Failed to read dir {}: {}", path_str, e);
                    warn_count += 1;
                }
            }
        } else if path.is_file() {
            // Обработка отдельного файла
            match process_file(path, &mut bundle_content) {
                Ok(_) => {
                    println!("📄 Processed: {}", path_str);
                    success_count += 1;
                }
                Err(_) => {
                    println!("⚠️  Skipped: {}", path_str);
                    warn_count += 1;
                }
            }
        } else {
            println!("⚠️  Not found: {}", path_str);
            warn_count += 1;
        }
    }

    // Сохранение результата
    let output_file = "context_bundle.md";
    match fs::File::create(output_file) {
        Ok(mut file) => {
            if let Err(e) = file.write_all(bundle_content.as_bytes()) {
                eprintln!("❌ Failed to write bundle: {}", e);
            } else {
                println!("\n🎉 Success! Context saved to: {}", output_file);
                println!("   Files included: {}", success_count);
                if warn_count > 0 {
                    println!("   ⚠️  Files skipped: {}", warn_count);
                }
            }
        }
        Err(e) => eprintln!("❌ Failed to create output file: {}", e),
    }
}

/// Рекурсивно собирает все .md файлы из директории
fn collect_markdown_files(dir: &Path) -> std::io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            // Рекурсивный вызов для подпапок
            files.extend(collect_markdown_files(&path)?);
        } else if path.extension().is_some_and(|ext| ext == "md") {
            files.push(path);
        }
    }

    Ok(files)
}

/// Добавляет содержимое файла в бандл с форматированием
fn process_file(path: &Path, bundle: &mut String) -> std::io::Result<()> {
    let content = fs::read_to_string(path)?;
    let path_str = path.to_string_lossy();

    bundle.push_str(&format!("## 📜 Файл: `{}`\n\n", path_str));
    bundle.push_str("```markdown\n");
    bundle.push_str(&content);
    // Гарантируем завершение блока кода, даже если файл не заканчивается переносом
    if !content.ends_with('\n') {
        bundle.push('\n');
    }
    bundle.push_str("```\n\n");
    bundle.push_str("---\n\n");

    Ok(())
}
