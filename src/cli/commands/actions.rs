use std::{collections::HashMap, env};

use colored::Colorize;
use inquire::{Confirm, Select};
use spinoff::{spinners, Color, Spinner};

use crate::cli::{
    detector::{
        count_extensions, detect_config_files, get_language_from_extension, is_known_extension,
    },
    init::LockCross,
};

pub fn add_command_action(
    js: bool,
    py: bool,
    rb: bool,
    lock: bool,
    lock_cross: Option<LockCross>,
    packages: Vec<String>,
) {
    if lock && lock_cross.is_some() {
        println!(
            "{} {}",
            "✗".red(),
            "Locking and cross locking cannot be specified at the same time".bold()
        );

        return;
    }

    if lock {
        return;
    } else if lock_cross.is_some() {
        return;
    }

    if (js && py) || (js && rb) || (py && rb) {
        println!(
            "{} {}",
            "✗".red(),
            "Only language support can be added at the same time".bold()
        );
        return;
    }

    if !js && !py && !rb {
        let mut spinner = Spinner::new(spinners::Dots, "Detecting language...", Color::Blue);

        let current_dir = env::current_dir().unwrap();

        let mut max_language: String = detect_config_files(current_dir.to_str().unwrap());

        if max_language == "" {
            let mut extension_counts: HashMap<String, usize> = HashMap::new();

            count_extensions(&current_dir, &mut extension_counts);

            let mut max_count = 0;

            if let Some(count) = extension_counts.get("rs") {
                max_count = *count;
                max_language = "Rust".to_string();
            }

            for (extension, count) in &extension_counts {
                if *count > max_count && is_known_extension(extension) {
                    max_count = *count;
                    max_language = get_language_from_extension(extension);
                }
            }
        }

        if max_language == "Unknown" || max_language == "" {
            return spinner.fail(
                format!(
                    "{}",
                    "No language was automatically found in the current directory.".bold()
                )
                .as_str(),
            );
        } else if max_language != "JavaScript"
            && max_language != "TypeScript"
            && max_language != "Python"
            && max_language != "Ruby"
        {
            return spinner.fail(
                format!(
                    "{} {}",
                    "Unsupported language detected in the current directory:".bold(),
                    max_language.cyan()
                )
                .as_str(),
            );
        } else {
            spinner.success(format!("{} {}", "Language detected:".bold(), max_language).as_str());

            let ans_language = Confirm::new("Is the detected language correct?")
                .with_default(false)
                .prompt()
                .unwrap();

            if ans_language {
                for package in packages.clone() {
                    println!("{} {}", "📦", package.to_string().green());
                }
            } else {
                let options: Vec<&str> = vec!["JavaScript/TypeScript", "Python", "Ruby"];

                let sel_language = Select::new("Select the language.", options)
                    .prompt()
                    .unwrap();

                if sel_language == "JavaScript/TypeScript" {
                    for package in packages.clone() {
                        println!(
                            "{} {}",
                            "📦 JavaScript/TypeScript:".bold(),
                            package.to_string().green()
                        );
                    }
                } else if sel_language == "Python" {
                    for package in packages.clone() {
                        println!("{} {}", "📦 Python:".bold(), package.to_string().green());
                    }
                } else if sel_language == "Ruby" {
                    for package in packages.clone() {
                        println!("{} {}", "📦 Ruby:".bold(), package.to_string().green());
                    }
                }
            }
        }
    } else {
        for package in packages {
            println!("{} {}", "📦", package.to_string().green());
        }
    }
}

pub fn remove_command_action(js: bool, py: bool, rb: bool, packages: Vec<String>) {
    if (js && py) || (js && rb) || (py && rb) {
        println!(
            "{} {}",
            "✗".red(),
            "Only language support can be added at the same time".bold()
        );
        return;
    }

    for package in packages {
        println!("{} {}", "📦", package.to_string().green());
    }
}

pub fn list_command_action() {}

pub fn update_command_action(js: bool, py: bool, rb: bool, versions: Vec<String>) {
    if (js && py) || (js && rb) || (py && rb) {
        println!(
            "{} {}",
            "✗".red(),
            "Only language support can be added at the same time".bold()
        );
        return;
    }

    for version in versions {
        println!("{} {}", "📑", version.green());
    }
}

pub fn install_command_action(packages: Vec<String>) {
    for package in packages {
        println!("{} {}", "📦", package.to_string().green());
    }
}

pub fn uninstall_command_action(packages: Vec<String>) {
    for package in packages {
        println!("{} {}", "📦", package.to_string().green());
    }
}

pub fn init_command_action() {}

pub fn upgrade_command_action(packages: Option<String>) {
    let package = Some(packages).unwrap();

    match package {
        Some(package) => println!("{} {}", "📦", package.green()),
        None => {}
    }
}

pub fn docs_command_action(local: bool) {
    if local {
        println!("{} {}", "🏠", "Local".green());
    }
}
