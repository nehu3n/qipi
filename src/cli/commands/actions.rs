use crate::manager::js::obtain::obtain_package;
use std::{collections::HashMap, env};

use colored::Colorize;
use inquire::{Confirm, Select};
use spinoff::{spinners, Color, Spinner};

use crate::cli::detector::{
    count_extensions, detect_cross_lockfile, detect_manifiest_files, get_language_from_extension,
    is_known_extension,
};

pub async fn add_command_action(
    js: bool,
    py: bool,
    rb: bool,
    lock: bool,
    lock_cross: bool,
    packages: Vec<String>,
) {
    if lock && lock_cross {
        println!(
            "{} {}",
            "✗".red(),
            "Locking and cross locking cannot be specified at the same time".bold()
        );

        return;
    }

    let current_dir = env::current_dir().unwrap();

    if lock {
        // qp.lock
        return;
    }

    if lock_cross {
        if js {
            let mut spinner =
                Spinner::new(spinners::Dots, "Detecting cross lockfiles...", Color::Blue);

            let adm_lock_file = detect_cross_lockfile(current_dir.to_str().unwrap(), "js");

            if adm_lock_file == "" {
                return spinner.fail(
                    &"No cross-lockfile was automatically found in the current directory.".bold(),
                );
            } else {
                spinner.success(
                    format!(
                        "{} {}",
                        "Cross-lockfile detected:".bold(),
                        adm_lock_file.cyan()
                    )
                    .as_str(),
                );

                let ans_cross_lockfile = Confirm::new("Is the detected cross-lockfile correct?")
                    .with_default(true)
                    .prompt()
                    .unwrap();

                if ans_cross_lockfile {
                    todo!("Add packages from cross-lockfile");
                } else {
                    let options: Vec<&str> = vec![
                        "pnpm-lock.yaml (pnpm)",
                        "yarn.lock (yarn)",
                        "package-lock.json (npm)",
                    ];

                    let sel_lockfile = Select::new("Select the lockfile.", options)
                        .prompt()
                        .unwrap();

                    if sel_lockfile == "pnpm-lock.yaml (pnpm)" {
                        todo!("Add packages from pnpm cross-lockfile");
                    } else if sel_lockfile == "yarn.lock (yarn)" {
                        todo!("Add packages from yarn cross-lockfile");
                    } else if sel_lockfile == "package-lock.json (npm)" {
                        todo!("Add packages from npm cross-lockfile");
                    }
                }
            }
        } else if py {
            todo!("Python cross-lockfile detection");
        } else if rb {
            todo!("Ruby cross-lockfile detection");
        } else {
            let mut spinner = Spinner::new(spinners::Dots, "Detecting language...", Color::Blue);

            let mut max_language: String = detect_manifiest_files(current_dir.to_str().unwrap());

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
                spinner
                    .success(format!("{} {}", "Language detected:".bold(), max_language).as_str());

                let ans_language = Confirm::new("Is the detected language correct?")
                    .with_default(true)
                    .prompt()
                    .unwrap();

                if ans_language {
                    if max_language == "JavaScript" || max_language == "TypeScript" {
                        let mut spinner = Spinner::new(
                            spinners::Dots,
                            "Detecting cross lockfile...",
                            Color::Blue,
                        );

                        let adm_lock_file =
                            detect_cross_lockfile(current_dir.to_str().unwrap(), "js");

                        if adm_lock_file == "" {
                            return spinner.fail(
                    &"No cross-lockfile was automatically found in the current directory.".bold(),
                );
                        } else {
                            spinner.success(
                                format!(
                                    "{} {}",
                                    "Cross-lockfile detected:".bold(),
                                    adm_lock_file.cyan()
                                )
                                .as_str(),
                            );

                            let ans_cross_lockfile =
                                Confirm::new("Is the detected cross-lockfile correct?")
                                    .with_default(true)
                                    .prompt()
                                    .unwrap();

                            if ans_cross_lockfile {
                                todo!("Add packages from cross-lockfile");
                            } else {
                                let options: Vec<&str> = vec![
                                    "pnpm-lock.yaml (pnpm)",
                                    "yarn.lock (yarn)",
                                    "package-lock.json (npm)",
                                ];

                                let sel_lockfile = Select::new("Select the lockfile.", options)
                                    .prompt()
                                    .unwrap();

                                if sel_lockfile == "pnpm-lock.yaml (pnpm)" {
                                    todo!("Add packages from pnpm cross-lockfile");
                                } else if sel_lockfile == "yarn.lock (yarn)" {
                                    todo!("Add packages from yarn cross-lockfile");
                                } else if sel_lockfile == "package-lock.json (npm)" {
                                    todo!("Add packages from npm cross-lockfile");
                                }
                            }
                        }
                    } else if max_language == "Python" {
                        todo!("Add packages from Python lockfile");
                    } else if max_language == "Ruby" {
                        todo!("Add packages from Ruby lockfile");
                    }
                } else {
                    let options: Vec<&str> = vec!["JavaScript/TypeScript", "Python", "Ruby"];

                    let sel_language = Select::new("Select the language.", options)
                        .prompt()
                        .unwrap();

                    if sel_language == "JavaScript/TypeScript" {
                        let mut spinner = Spinner::new(
                            spinners::Dots,
                            "Detecting cross lockfile...",
                            Color::Blue,
                        );

                        let adm_lock_file =
                            detect_cross_lockfile(current_dir.to_str().unwrap(), "js");

                        if adm_lock_file == "" {
                            return spinner.fail(
                    &"No cross-lockfile was automatically found in the current directory.".bold(),
                );
                        } else {
                            spinner.success(
                                format!(
                                    "{} {}",
                                    "Cross-lockfile detected:".bold(),
                                    adm_lock_file.cyan()
                                )
                                .as_str(),
                            );

                            let ans_cross_lockfile =
                                Confirm::new("Is the detected cross-lockfile correct?")
                                    .with_default(true)
                                    .prompt()
                                    .unwrap();

                            if ans_cross_lockfile {
                                todo!("Add packages from cross-lockfile");
                            } else {
                                let options: Vec<&str> = vec![
                                    "pnpm-lock.yaml (pnpm)",
                                    "yarn.lock (yarn)",
                                    "package-lock.json (npm)",
                                ];

                                let sel_lockfile = Select::new("Select the lockfile.", options)
                                    .prompt()
                                    .unwrap();

                                if sel_lockfile == "pnpm-lock.yaml (pnpm)" {
                                    todo!("Add packages from pnpm cross-lockfile");
                                } else if sel_lockfile == "yarn.lock (yarn)" {
                                    todo!("Add packages from yarn cross-lockfile");
                                } else if sel_lockfile == "package-lock.json (npm)" {
                                    todo!("Add packages from npm cross-lockfile");
                                }
                            }
                        }
                    } else if sel_language == "Python" {
                        todo!("Add packages from pnpm cross-lockfile");
                    } else if sel_language == "Ruby" {
                        todo!("Add packages from pnpm cross-lockfile");
                    }
                }
            }
        }

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

    if packages.len() == 0 {
        println!(
            "{} {}",
            "✗".red(),
            "You need to specify a minimum of one package".bold()
        );
        return;
    }

    if !js && !py && !rb {
        let mut spinner = Spinner::new(spinners::Dots, "Detecting language...", Color::Blue);

        let mut max_language: String = detect_manifiest_files(current_dir.to_str().unwrap());

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
                .with_default(true)
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
            // println!("{} {}", "📦", package.to_string().green());
            let (name, version) = package.split_once('@').unwrap_or((&package, "latest"));

            let package = obtain_package(name, version).await.unwrap();
            println!("{:?}", package);
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
