use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct QipiCLI {
    #[clap(subcommand)]
    pub cmds: Option<Commands>,
}

#[derive(Subcommand, Clone)]
pub enum Commands {
    /// Add packages to a project.
    Add { packages: Vec<String> },

    /// Remove packages from a project.
    Remove { packages: Vec<String> },

    /// Install all the packages of a project.
    Install,
}
