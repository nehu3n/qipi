mod cli {
    pub mod commands {
        pub mod actions;
    }
    pub mod detector;
    pub mod init;
}

fn main() {
    cli::init::init_cli()
}
