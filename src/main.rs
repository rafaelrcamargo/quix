// ? CLI
mod quix;

/// * Main function.
fn main() {
    // ? CLI Argument parser
    let matches = quix::args::arg_matches();

    // ? CLI Commands handling
    match matches.subcommand() {
        // ? Link command
        Some(("link", link_matches)) => quix::commands::link::app(link_matches),
        _ => unreachable!(),
    }
}
