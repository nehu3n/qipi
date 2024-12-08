use anyhow::{Context, Result};

extern crate clap;
extern crate regex;
extern crate reqwest;
extern crate serde;
extern crate tokio;

mod cli;
mod config;
mod core;

#[tokio::main]
async fn main() -> Result<()> {
    cli::app::init().await.context("Could not initialize CLI")?;

    Ok(())
}
