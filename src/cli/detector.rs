use std::collections::HashMap;
use std::fs;
use std::path::{PathBuf, MAIN_SEPARATOR_STR};

pub fn detect_manifiest_files(dir_path: &str) -> String {
    let manifiest_files: [&str; 3] = ["package.json", "Gemfile", "requirements.txt"];

    for file in manifiest_files.iter() {
        let path = format!(
            "{}{os_separator}{}",
            dir_path,
            file,
            os_separator = MAIN_SEPARATOR_STR
        );
        if fs::metadata(&path).is_ok() {
            match file {
                &"package.json" => return "JavaScript".to_string(),
                &"requirements.txt" => return "Python".to_string(),
                &"Gemfile" => return "Ruby".to_string(),
                _ => (),
            }
        }
    }

    "".to_string()
}

pub fn detect_cross_lockfile(dir_path: &str, language: &str) -> String {
    if language == "js" {
        let lock_files: [&str; 3] = ["pnpm-lock.yaml", "yarn.lock", "package-lock.json"];

        for file in lock_files.iter() {
            let path = format!(
                "{}{os_separator}{}",
                dir_path,
                file,
                os_separator = MAIN_SEPARATOR_STR
            );
            if fs::metadata(&path).is_ok() {
                return file.to_string();
            }
        }
    } else if language == "py" {
        todo!("Detect cross lockfile for Python");
    } else if language == "rb" {
        todo!("Detect cross lockfile for Ruby");
    }
    "".to_string()
}

pub fn count_extensions(dir: &PathBuf, extension_counts: &mut HashMap<String, usize>) {
    if dir.is_dir() {
        let entries = fs::read_dir(dir).unwrap().into_iter().collect::<Vec<_>>();
        for entry in entries {
            let path = entry.unwrap().path();
            if path.is_dir() {
                count_extensions(&path, extension_counts);
            } else {
                let extension = path
                    .extension()
                    .and_then(|ext| ext.to_str().map(|s| s.to_string()))
                    .unwrap_or_else(|| "".to_string());
                if extension != "" {
                    *extension_counts.entry(extension).or_insert(0) += 1;
                }
            }
        }
    }
}

pub fn get_language_from_extension(extension: &str) -> String {
    match extension {
        "py" => "Python".to_string(),
        "js" => "JavaScript".to_string(),
        "ts" => "TypeScript".to_string(),
        "go" => "Go".to_string(),
        "java" => "Java".to_string(),
        "cpp" => "C++".to_string(),
        "c" => "C".to_string(),
        "rb" => "Ruby".to_string(),
        "php" => "PHP".to_string(),
        "swift" => "Swift".to_string(),
        "kt" => "Kotlin".to_string(),
        "cs" => "C#".to_string(),
        _ => "Unknown".to_string(),
    }
}

pub fn is_known_extension(extension: &str) -> bool {
    matches!(
        extension,
        "py" | "js" | "ts" | "go" | "java" | "cpp" | "c" | "rb" | "php" | "swift" | "kt" | "cs"
    )
}
