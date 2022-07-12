// ? CLI
use clap::{Arg, ArgMatches, Command}; // CLI Argument parser

pub fn matches() -> ArgMatches {
    return Command::new("Quix")
    .version("0.0.1")
    .author("Rafael R. Camargo")
    .about("Quix is a CLI created aiming for the best developer experience, lets make VTEX IO quick again.")
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
  )
  .get_matches();
}
