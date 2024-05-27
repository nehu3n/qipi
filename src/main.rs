mod cli {
    pub mod commands {
        pub mod actions;
    }
    pub mod detector;
    pub mod init;
}
mod manager {
    pub mod js {
        pub mod packages {
            pub mod cache;
        }
        pub mod lockfile {
            pub mod cross;
            pub mod qp;
        }
        pub mod obtain;
    }
}

#[tokio::main]
async fn main() {
    cli::init::init_cli().await;
}
