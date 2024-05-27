mod cli {
    pub mod commands {
        pub mod actions;
    }
    pub mod detector;
    pub mod init;
}
mod manager {
    pub mod js {
        pub mod lockfile {
            pub mod qp;
        }
        pub mod obtain;
    }
}

#[tokio::main]
async fn main() {
    cli::init::init_cli().await;
}
