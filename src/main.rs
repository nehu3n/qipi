use anyhow::{Context, Result};

mod cli;
mod config;
mod core;

#[tokio::main]
async fn main() -> Result<()> {
    cli::app::init().await.context("Could not initialize CLI")?;

    Ok(())
}
