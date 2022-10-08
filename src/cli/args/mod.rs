//! # Clap arguments
//! This module contains the clap arguments for the CLI.
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
use clap::{Arg, ArgAction, ArgMatches, ColorChoice, Command}; // CLI Argument parser

pub fn matches() -> ArgMatches {
    // ? CLI Setup
    Command::new("Quix")
      .version("0.0.1") // Version
      .author("Rafael R. Camargo") // Author

      .about(
          "Quix is a CLI created aiming for the best developer experience, lets make VTEX IO quick again.",
      ) // About
      .after_help(
          "“Asking for help is never a sign of weakness.\nIt's one of the bravest things you can do.\nAnd it can save your life.”\n- Lily Collins\n",
      ) // Motivational quote, because why not.

      .alias("qx") // CLI Alias
      .propagate_version(true)
      .subcommand_required(true)
      .arg_required_else_help(true)
      .color(ColorChoice::Always)

      .arg(
          Arg::new("verbose")
              .short('v')
              .long("verbose")
              .action(ArgAction::Count),
      )
      .subcommand(
          Command::new("link")
              .about("Link a app or store to a project.")
              .arg(
                  Arg::new("clean")
                      .conflicts_with("quicker")
                      .short('c')
                      .long("clean")
                      .required(false)
                      .action(ArgAction::SetTrue)
                      .help("Clean the project before linking."),
              )
              .arg(
                  Arg::new("quicker")
                      .conflicts_with("clean")
                      .short('q')
                      .long("quicker")
                      .required(false)
                      .action(ArgAction::SetTrue)
                      .help("Ignores some validations and links the app faster."),
              ),
      )
      .get_matches()
}
