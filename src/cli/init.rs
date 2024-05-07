use std::env;

use clap::Command;
use colored::Colorize;

use crate::cli::commands::skeletons::{
    add_command, docs_command, init_command, install_command, list_command, remove_command,
    uninstall_command, update_command, upgrade_command,
};

use super::commands::actions::{
    add_command_action, docs_command_action, init_command_action, install_command_action,
    list_command_action, remove_command_action, uninstall_command_action, update_command_action,
    upgrade_command_action,
};

pub fn init_cli() {
    const QIPI_VERSION: &str = env!("CARGO_PKG_VERSION");

    let app = Command::new(env!("CARGO_PKG_NAME"))
        .version(QIPI_VERSION)
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .alias("qp")
        .help_template(format!(
            "
🦉 {about}
{alias} qp || {version} {QIPI_VERSION}

{usage} qipi <command> [flags] [arguments]
{example} qipi add --js react {example_comment}

💻 {commands}
add ➜ {add_description} -  qipi add --js react
remove ➜ {remove_description} -  qipi remove --js react
list ➜ {list_description} -  qipi list
update ➜ {update_description} -  qipi update

install ➜ {install_description} -  qipi install cli
uninstall ➜ {uninstall_description} -  qipi uninstall cli

init ➜ {init_description} -  qipi init

upgrade ➜ {upgrade_description} -  qipi upgrade
docs ➜ {docs_description} -  qipi docs --local

Use {command_help} for more information about a command.
",
            about = "Qipi is a fast and modern universal package manager."
                .bold()
                .green(),
            alias = "Alias:".bold().blue(),
            version = "Version:".bold().blue(),
            usage = "Usage:".bold().blue(),
            example = "Example:".bold().blue(),
            example_comment = "# Adds the package in the JavaScript project.\n".magenta(),
            commands = "Commands:".bold().blue(),
            add_description = " Adds packages to project.".magenta(),
            remove_description = " Removes packages from project.".magenta(),
            list_description = " Lists packages in project.".magenta(),
            update_description = " Updates packages in project.".magenta(),
            install_description = " Install global package/binary program.".magenta(),
            uninstall_description = " Uninstall global package/binary program.".magenta(),
            init_description = " Initialize project with blank template.".magenta(),
            upgrade_description = " Upgrade Qipi version.".magenta(),
            docs_description = " View Qipi documentation.".magenta(),
            command_help = "qipi <command> --help".bold().blue()
        ))
        .subcommand(add_command())
        .subcommand(remove_command())
        .subcommand(list_command())
        .subcommand(update_command())
        .subcommand(install_command())
        .subcommand(uninstall_command())
        .subcommand(init_command())
        .subcommand(upgrade_command())
        .subcommand(docs_command());

    process_subcommands(app)
}

fn process_subcommands(app: Command) {
    let matches = app.get_matches();

    if let Some((command_name, _command_matches)) = matches.subcommand() {
        match command_name {
            "add" => add_command_action(),
            "remove" => remove_command_action(),
            "list" => list_command_action(),
            "update" => update_command_action(),
            "install" => install_command_action(),
            "uninstall" => uninstall_command_action(),
            "init" => init_command_action(),
            "upgrade" => upgrade_command_action(),
            "docs" => docs_command_action(),
            _ => (),
        }
    }
}
