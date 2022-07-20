//! # Mod to handle sending zipped files to the builder.
//! Here we handle the zipping process, for later sending this packages to the builder.
//!
//! ## Modules
//! - Clap: Handles the arg parsing.
//!
//! ## Examples
//! ```bash
//! quix link
//! ```
//! ```bash
//! quix link --clean
//! ```
//!
//! ## Panics
//! This function will panic if the entered command does not follow any of the available.

// ? CLI
use clap::{Arg, ArgMatches, Command}; // CLI Argument parser

pub fn matches() -> ArgMatches {
    // ? CLI Setup
    return Command::new("Quix")
      .version("0.0.1") // Version
      .author("Rafael R. Camargo") // Author
      .about("Quix is a CLI created aiming for the best developer experience, lets make VTEX IO quick again.") // About
      .before_help(
        "“Asking for help is never a sign of weakness.\nIt's one of the bravest things you can do.\nAnd it can save your life.”\n- Lily Collins"
      ) // Motivational quote, because why not.

      .alias("qx") // CLI Alias
      .subcommand_required(true)
      .arg_required_else_help(true)
      .subcommand(
        Command::new("link")
        .about("Link a app or store to a project.")
        .arg(
          Arg::new("clean")
          .conflicts_with("quicker")
          .short('c')
          .long("clean")
          .takes_value(false)
          .help("Clean the project before linking.")
        )
        .arg(
          Arg::new("quicker")
          .conflicts_with("clean")
            .short('q')
            .long("quicker")
            .takes_value(false)
            .help("Ignores some validations and links the app faster.")
        )
  ).get_matches();
}
