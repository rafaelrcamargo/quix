#[path = "./utils/send/dir/mod.rs"]
mod dir;

/*
#[path = "./cli/mod.rs"]
mod cli;
#[path = "./auth/session/mod.rs"]
mod session;
// ? Importing just the used functions from the mods.
use cli::{args, commands}; // CLI Argument + Commands
use session::Session; // Session struct
*/

/// Main function.
fn main() {
    /* // ? Get user session data
    let _session: Session = Session::new();

    // ? CLI
    let matches = args::matches(); // Argument parser
    match matches.subcommand() {
        Some(("link", args)) => commands::link(args), // Link command
        _ => unreachable!(),                          // Unreachable command
    } */

    match dir::zip("example\\") {
        Ok(_) => println!("done writting zip file"),
        Err(e) => println!("Error: {:?}", e),
    }
}
