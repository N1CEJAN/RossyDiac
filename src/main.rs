use crate::api::cli;

mod api;
mod business;
mod core;

fn main() {
    env_logger::init();
    cli::run();
}
