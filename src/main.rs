// ? CLI
mod args;
mod auth;
mod commands;
mod tools;

// ? Home path
use home;

/// * Main function.
fn main() {
    tools::zip();

    // ? Get the home directory
    match home::home_dir() {
        Some(path) => {
            let _session = auth::session::get_session(path);
        }
        None => println!("No home directory found."),
    }

    // ? CLI Argument parser
    let matches = args::arg_matches();

    // ? CLI Commands handling
    match matches.subcommand() {
        // ? Link command
        Some(("link", link_matches)) => commands::link(link_matches),
        _ => unreachable!(),
    }
}
