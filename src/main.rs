extern crate clap;
extern crate regex;
extern crate reqwest;
extern crate serde;
extern crate tokio;

mod cli;
mod core;

#[tokio::main]
async fn main() {
    cli::app::init().await;
}
