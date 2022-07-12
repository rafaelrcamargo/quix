#[path = "./cli/mod.rs"]
mod cli;
#[path = "./auth/session/mod.rs"]
mod session;

// ? Importing just the used functions from the mods.
use cli::{args::matches, commands::link}; // CLI Argument + Commands
use session::Session; // Session struct

/// * Main function.
fn main() {
    // ? Get user session data
    let _session: Session = Session::new();

    // ? CLI
    let matches = matches(); // Argument parser
    match matches.subcommand() {
        Some(("link", args)) => link(args), // Link command
        _ => unreachable!(),                // Unreachable command
    }
}
