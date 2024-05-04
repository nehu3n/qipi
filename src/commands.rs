use clap::Command;
use colored::Colorize;

pub fn add_command() -> Command {
    Command::new("add")
        .about("Adds packages to proyect.")
        .alias("a")
        .help_template(format!(
            "
{about}

{examples} 
qipi add --js react {example1_comment}
qipi add react {example2_comment}

{flags}
--js - {js_flag_description} ¦ qipi add --js react ¦ qipi add react
--py - {py_flag_description} ¦ qipi add --py django ¦ qipi add django
--rb - {rb_flag_description} ¦ qipi add --rb rails ¦ qipi add rails
",
            about = "Adds packages to proyect.".underline(),
            examples = "Examples:".blue().bold(),
            example1_comment = "# Adds the package in the JavaScript proyect.".magenta(),
            example2_comment = "# Detect automatically the language to the proyect.".magenta(),
            flags = "Flags:".blue().bold(),
            js_flag_description = "Adds the package in the JavaScript proyect.".magenta(),
            py_flag_description = "Adds the package in the Python proyect.".magenta(),
            rb_flag_description = "Adds the package in the Ruby proyect.".magenta()
        ))
}

pub fn remove_command() -> Command {
    Command::new("remove")
        .about("Removes packages from proyect.")
        .alias("r")
}

pub fn list_command() -> Command {
    Command::new("list")
        .about("Lists packages in proyect.")
        .alias("l")
}

pub fn update_command() -> Command {
    Command::new("update")
        .about("Updates packages in proyect.")
        .alias("u")
}

pub fn install_command() -> Command {
    Command::new("install")
        .about("Install global package/binary program.")
        .alias("i")
}

pub fn uninstall_command() -> Command {
    Command::new("uninstall")
        .about("Uninstall global package/binary program.")
        .alias("un")
}

pub fn init_command() -> Command {
    Command::new("init")
        .about("Initialize proyect with blank template.")
        .alias("in")
}

pub fn upgrade_command() -> Command {
    Command::new("upgrade")
        .about("Upgrade Qipi version.")
        .alias("up")
}

pub fn docs_command() -> Command {
    Command::new("docs")
        .about("View Qipi documentation.")
        .alias("d")
}
