//! # Mod to handle the CLI commands and subcommands.
//! Here are defied the CLI commands and subcommands.
//! - `link`: Handles the `link` subcommand.
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
//! This is because the CLI will not be able to authenticate with the VTEX API.

pub mod link;
pub use link::link;
