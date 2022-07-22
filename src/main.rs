//! # Initiates the CLI
//! Thats the entry point for the CLI, and is the first function to be executed.
//!
//! # Subcommands
//! - `link`: Link the app to the builder.
//!
//! # Examples
//! Base:
//! ```
//! quix link
//! ```
//!
//! Clean:
//! ```
//! quix link --clean
//! # OR
//! quix link -c
//! ```
//! # Panics
//! This function will panic if the **VTEX CLI** is not properly installed and if your login and authentication token are not set.
//!
//! This is because the CLI will not be able to authenticate with the **VTEX API**.

// * General modules, functions, and structs.
mod cli; // CLI config, and CLI commands.
mod clients; // Clients, like the VTEX API.
mod commands; // Commands, like the link command.
mod configs; // Configs, like the VTEX account and the session token.
mod constants; // Constants, like the routes.
mod utils; // Utility functions, like the JSON parser.

// * Widely used modules, come in handy exporting them this way.
#[path = "./utils/json/mod.rs"]
mod json; // Exports the JSON parser, for an easier use.

// * Import only the used functions.
use cli::args;

// * General utils
use human_panic::setup_panic; // Human panic, for a better error handling.

// * Better logging.
use log4rs;

// * Extends log macros for all files.
#[macro_use]
extern crate log;

/// # Main function.
/// Here we start the CLI, and parse the arguments.
fn main() {
    // !!! Setup the panic handler.
    setup_panic!(Metadata {
        name: env!("CARGO_PKG_NAME").into(),
        version: env!("CARGO_PKG_VERSION").into(),
        authors: env!("CARGO_PKG_AUTHORS").into(),
        homepage: "https://github.com/rafaelrcamargo/quix".into(),
    });

    // ? Setup the logger.
    log4rs::init_file("logger.yml", Default::default()).unwrap();

    // ? Argument parser, returns a `ArgMatches` struct.
    let matches = args::matches();

    // * Parse the arguments.
    match matches.subcommand() {
        Some(("link", args)) => commands::link(args), // Link subcommand.
        _ => unreachable!("Invalid entry."), // Shouldn't happen, but just in case, who knows?
    }
}
