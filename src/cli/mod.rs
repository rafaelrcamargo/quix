//! # Mod for handling and parsing the arguments & subcommands.
//! Here we handle the arguments and subcommands, and also the help and version commands.
//!
//! ## Modules
//! - Args: Handles and parses the arguments.
//! - Commands: Handles the subcommands.
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

pub mod args;
