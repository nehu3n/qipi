use clap::Command;
use colored::Colorize;

pub fn add_command() -> Command {
    Command::new("add")
        .about("Adds packages to proyect.")
        .alias("a")
        .help_template(format!(
            "
🚀 {about}

💡 {examples}
{example1}{example1_comment}
{example2}{example2_comment}

🛠️ {flags}
{js_flag}{js_description}
{py_flag}{py_description}
{rb_flag}{rb_description}

📘 {additional_info}
- Use 'qipi add <package_name>' to add a package with automatic language detection.
- You can also specify the language explicitly using flags like '--js', '--py', or '--rb'.\n
",
            about = "Add packages to your project.".bold().green(),
            examples = "Examples:".bold().blue(),
            example1 = "qipi add --js react",
            example1_comment = "  ➜  Add package to a JavaScript project.".magenta(),
            example2 = "qipi add react",
            example2_comment = "  ➜  Automatically detect project language.".magenta(),
            flags = "Flags:".bold().blue(),
            js_flag = "--js".bright_cyan(),
            js_description = "  ➜  Add package to a JavaScript project.".magenta(),
            py_flag = "--py".bright_yellow(),
            py_description = "  ➜  Add package to a Python project.".magenta(),
            rb_flag = "--rb".bright_red(),
            rb_description = "  ➜  Add package to a Ruby project.".magenta(),
            additional_info = "Additional Information:".bold().blue()
        ))
}

pub fn remove_command() -> Command {
    Command::new("remove")
        .about("Removes packages from proyect.")
        .alias("r")
        .help_template(format!(
            "
🚀 {about}

💡 {examples}
{example1}{example1_comment}
{example2}{example2_comment}

🛠️ {flags}
{js_flag}{js_description}
{py_flag}{py_description}
{rb_flag}{rb_description}

📘 {additional_info}
- Use 'qipi remove <package_name>' to remove a package with automatic language detection.
- You can also specify the language explicitly using flags like '--js', '--py', or '--rb'.\n
",
            about = "Remove packages from your project.".bold().green(),
            examples = "Examples:".bold().blue(),
            example1 = "qipi remove --js react",
            example1_comment = "  ➜  Remove package from a JavaScript project.".magenta(),
            example2 = "qipi remove react",
            example2_comment = "  ➜  Automatically detect project language.".magenta(),
            flags = "Flags:".bold().blue(),
            js_flag = "--js".bright_cyan(),
            js_description = "  ➜  Remove package from a JavaScript project.".magenta(),
            py_flag = "--py".bright_yellow(),
            py_description = "  ➜  Remove package from a Python project.".magenta(),
            rb_flag = "--rb".bright_red(),
            rb_description = "  ➜  Remove package from a Ruby project.".magenta(),
            additional_info = "Additional Information:".bold().blue()
        ))
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
