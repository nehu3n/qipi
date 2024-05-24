use colored::Colorize;

use crate::cli::init::LockCross;

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
            "🗙".red(),
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
            "🗙".red(),
            "Only language support can be added at the same time".bold()
        );
        return;
    }

    for package in packages {
        println!("{} {}", "📦", package.to_string().green());
    }
}

pub fn remove_command_action(js: bool, py: bool, rb: bool, packages: Vec<String>) {
    if (js && py) || (js && rb) || (py && rb) {
        println!(
            "{} {}",
            "🗙".red(),
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
            "🗙".red(),
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
