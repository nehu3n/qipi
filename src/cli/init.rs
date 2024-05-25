use clap::{arg, Parser, Subcommand, ValueEnum};

use super::commands::actions::{
    add_command_action, docs_command_action, init_command_action, install_command_action,
    list_command_action, remove_command_action, uninstall_command_action, update_command_action,
    upgrade_command_action,
};

#[derive(Clone, ValueEnum, PartialEq, Eq, Debug)]
pub enum LockCross {
    Pnpm,
    Npm,
    Yarn,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Add packages to the project
    Add {
        #[arg(short, long)]
        js: bool,
        #[arg(short, long)]
        py: bool,
        #[arg(short, long)]
        rb: bool,

        #[arg(long)]
        lock: bool,

        #[arg(long, value_enum)]
        lock_cross: bool,

        packages: Vec<String>,
    },

    /// Remove packages from the project
    Remove {
        #[arg(short, long)]
        js: bool,
        #[arg(short, long)]
        py: bool,
        #[arg(short, long)]
        rb: bool,

        packages: Vec<String>,
    },

    /// Install a package/binary globally.
    Install { packages: Vec<String> },

    /// Uninstall a package/binary globally.
    Uninstall { packages: Vec<String> },

    /// Update packages in the project
    Update {
        #[arg(short, long)]
        js: bool,
        #[arg(short, long)]
        py: bool,
        #[arg(short, long)]
        rb: bool,

        #[arg(value_name = "VERSION")]
        version: Vec<String>,
    },

    /// Upgrade Qipi
    Upgrade {
        #[arg(value_name = "VERSION")]
        version: Option<String>,
    },

    /// See or download the Qipi documentation
    Docs {
        #[arg(long)]
        local: bool,
    },

    /// List installed packages in the project or globally
    List,

    /// Initialize the project template
    Init,
}

pub fn init_cli() {
    let args = Cli::parse();

    match args.command {
        Commands::Add {
            js,
            py,
            rb,
            lock,
            lock_cross,
            packages,
        } => add_command_action(js, py, rb, lock, lock_cross, packages),
        Commands::Remove {
            js,
            py,
            rb,
            packages,
        } => remove_command_action(js, py, rb, packages),
        Commands::Install { packages } => install_command_action(packages),
        Commands::Uninstall { packages } => uninstall_command_action(packages),
        Commands::Update {
            js,
            py,
            rb,
            version,
        } => update_command_action(js, py, rb, version),
        Commands::Upgrade { version } => upgrade_command_action(version),
        Commands::Docs { local } => docs_command_action(local),
        Commands::List => list_command_action(),
        Commands::Init => init_command_action(),
    }
}
