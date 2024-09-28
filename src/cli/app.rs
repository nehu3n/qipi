use clap::Parser;

use super::r#struct::{Commands, QipiCLI};

pub fn init() {
    let cli = QipiCLI::parse();

    match cli.cmds {
        Some(Commands::Add { packages }) => (),
        Some(Commands::Remove { packages }) => (),

        Some(Commands::Install) => (),
        
        None => (),
    }
}
