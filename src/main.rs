extern crate clap;
extern crate reqwest;
extern crate serde;

mod cli;
mod core;

fn main() {
    cli::app::init();
}
