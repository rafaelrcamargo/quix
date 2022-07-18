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

// CLI Argument parser
use clap::ArgMatches;

// Watcher for the link.
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::{env, path::PathBuf, sync::mpsc::channel, time::Duration};

// Project modules.
use crate::{clients::vtex::builder, configs::auth::Session, utils::gzip};

/// # Link command.
/// This command will send the bundle to the builder, and watch the directory for changes.
/// - It will return the link to the builder.
///
/// # Examples
/// ```
/// quix link
/// ```
///
/// # Panics
/// If the session is not valid or the token is not set, this function will panic.
/// This is because the CLI will not be able to authenticate with the VTEX API.
pub fn link(args: &ArgMatches) {
    // ? Get the path to the actual folder where the app is located, to watch.
    let path = env::current_dir().unwrap(); // TODO: Get the path to the actual folder where the app is located, to watch.

    // ? Instantiate a user session.
    let session = Session::new();

    // For the first link command, we need to create a new zip file, with all the files in the folder.
    // ? Create a new zip file.
    let file = gzip::dir::zip(&path).unwrap();

    // ? Send the file to the builder.
    match builder::link(file, &session.token) {
        Ok(res) => {
            println!("{:?}", res.text());
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }

    // ? Args parsing.
    if args.is_present("clean") {
        println!("Cleaning project cache...");
    } else if args.is_present("quicker") {
        println!("Linking app quicker...");
    }

    // * * * Starts the watcher, in the current project folder. * * *

    // ? Create a channel to receive the events.
    let (sender, receiver) = channel();
    // ? Create a watcher object, delivering debounced events.
    let mut watcher = watcher(sender, Duration::from_secs(1)).unwrap();
    // ? All files and directories at that path and below will be monitored for changes.
    watcher.watch(path, RecursiveMode::Recursive).unwrap();

    loop {
        // ? The watcher loop will run until the CLI receives a user interruption.
        match receiver.recv() {
            // ? Notify the user of the event.
            Ok(DebouncedEvent::NoticeWrite(path)) => println!("Notice write: {:?}", path),

            // ? Common handling for all events.
            Ok(DebouncedEvent::Write(path)) => event(&path, session.token.as_str()),
            Ok(DebouncedEvent::Create(path)) => event(&path, session.token.as_str()),

            // TODO: Remove & Rename events.
            Ok(DebouncedEvent::Remove(path)) => todo!("Removed: {:?}", path),
            Ok(DebouncedEvent::Rename(o_path, n_path)) => {
                todo!("Renamed: {:?} to {:?}", o_path, n_path)
            }

            // ! Unimplemented events.
            Err(e) => unimplemented!("Error: {:?}", e),
            _ => (), // ! Everything else is ignored.
        }
    }
}

fn event(path: &PathBuf, token: &str) {
    // ? Zip the file, using the zip utils.
    let file = gzip::file::zip(path);

    // ? Send the file to the builder.
    match builder::link(file, token) {
        Ok(res) => {
            println!("{:?}", res.text());
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
}
