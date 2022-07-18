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
use cli::args; // CLI Argument + Commands

/// # Main function.
/// Here we start the CLI, and parse the arguments.
fn main() {
    let matches = args::matches(); // Argument parser, returns a `ArgMatches` struct.

    match matches.subcommand() {
        Some(("link", args)) => commands::link(args), // Link subcommand.
        _ => unreachable!("Invalid entry."), // Shouldn't happen, but just in case, who knows?
    }
}
