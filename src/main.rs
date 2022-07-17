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
//! # 🛑 Panics
//! This function will panic if the **VTEX CLI** is not properly installed and if your login and authentication token are not set.
//!
//! This is because the CLI will not be able to authenticate with the **VTEX API**.

#[path = "./cli/mod.rs"]
mod cli;
#[path = "./utils/send/mod.rs"]
mod send;

// ? Importing just the used functions from the mods.
use cli::{args, commands}; // CLI Argument + Commands

/// # Main function.
/// Here we start the CLI, and parse the arguments.
fn main() {
    let matches = args::matches(); // Argument parser
    match matches.subcommand() {
        Some(("link", args)) => commands::link(args), // Link command
        _ => unreachable!(),                          // Unreachable command
    }
}
