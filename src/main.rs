use std::env;

use clap::Command;
use colored::Colorize;

mod commands;
use commands::add_command;

use crate::commands::{
    docs_command, init_command, install_command, list_command, remove_command, uninstall_command,
    update_command, upgrade_command,
};

fn main() {
    const QIPI_VERSION: &str = env!("CARGO_PKG_VERSION");

    let mut app = Command::new(env!("CARGO_PKG_NAME"))
        .version(QIPI_VERSION)
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .alias("qp")
        .help_template(format!(
            "
{about}
{alias} qp || {version} {QIPI_VERSION}

{usage} qipi <command> [flags] [arguments]
{example} qipi add --js react {example_comment}

{commands}
add - {add_description} ¦ qipi add --js react
remove - {remove_description} ¦ qipi remove --js react
list - {list_description} ¦ qipi list
update - {update_description} ¦ qipi update

install - {install_description} ¦ qipi install cli
uninstall - {uninstall_description} ¦ qipi uninstall cli

init - {init_description} ¦ qipi init

upgrade - {upgrade_description} ¦ qipi upgrade
docs - {docs_description} ¦ qipi docs --local

Use {command_help} for more information about a command.
        ",
            about = "🦉 Qipi is a fast and modern universal package manager.".underline(),
            alias = "Alias:".blue().bold(),
            version = "Version:".blue().bold(),
            usage = "Usage:".blue().bold(),
            example = "Example:".blue().bold(),
            example_comment = "# Adds the package in the JavaScript proyect.\n".magenta(),
            commands = "💻 Commands:".blue().bold(),
            add_description = "Adds packages to proyect.".magenta(),
            remove_description = "Removes packages from proyect.".magenta(),
            list_description = "Lists packages in proyect.".magenta(),
            update_description = "Updates packages in proyect.".magenta(),
            install_description = "Install global package/binary program.".magenta(),
            uninstall_description = "Uninstall global package/binary program.".magenta(),
            init_description = "Initialize proyect with blank template.".magenta(),
            upgrade_description = "Upgrade Qipi version.".magenta(),
            docs_description = "View Qipi documentation.".magenta(),
            command_help = "qipi <command> --help".blue().bold()
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

    app.print_help().unwrap();
}