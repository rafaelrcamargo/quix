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
mod connections; // Connections, like the VTEX Builder API.
mod constants; // Constants, like the routes.
mod utils; // Utility functions, like the JSON parser.

// * Widely used modules, come in handy exporting them this way.
#[path = "./utils/json/mod.rs"]
mod json; // Exports the JSON parser, for an easier use.

// * Import only the used functions.
use cli::args;

// * General utils
use human_panic::setup_panic; // Human panic, for a better error handling.

#[macro_use]
extern crate loggy;

/// # Main function.
/// Here we start the CLI, and parse the arguments.
fn main() {
    // * Setups
    setup_panic!(Metadata {
        name: env!("CARGO_PKG_NAME").into(),
        version: env!("CARGO_PKG_VERSION").into(),
        authors: env!("CARGO_PKG_AUTHORS").into(),
        homepage: "https://github.com/rafaelrcamargo/quix".into(),
    }); // !!! Setup the panic handler.

    // * Dialogs
    // Some greetings from the loggy.
    let dialogs = [
        (
            "👋 HEY!".to_string(),
            "Welcome to QUIX! VTEX like you never seen before. 🤠".to_string(),
        ),
        (
            "🧭 GOAL".to_string(),
            "We're working to speed up VTEX's development process, and make it more user friendly. ✨".to_string(),
        ),
        (
            "🤝 COMM".to_string(),
            "Feel free to use and contribute to this project, and help us to improve it. ✍️".to_string(),
        ),
    ];

    // iterate over the dialogs, and print them, using custom! macro.
    for dialog in dialogs {
        let (title, text) = dialog;
        custom!(title, text);
    }

    // Warn the user about the beta version.
    warn!("This is a beta version of the CLI, and may not be stable. 😬\n");
    trace!("We're about to go fast, fasten your seat belts. 🚀\n");

    // * Main function
    // ? Argument parser
    let matches = args::matches(); // Return a ArgMatches object, with the arguments.
    match matches.subcommand() {
        Some(("link", args)) => commands::link(args),
        _ => unreachable!("Invalid entry."), // !!! Shouldn't happen, but just in case, who knows?
    }
}
