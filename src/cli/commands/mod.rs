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

#[path = "../../utils/send/mod.rs"]
mod send;
#[path = "../../auth/session/mod.rs"]
mod session;
use session::Session;

// CLI Argument parser
use clap::ArgMatches;

// Watcher for the link.
use notify::DebouncedEvent; // Notify crate
use notify::{watcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel; // Message channel queue
use std::time::Duration; // Standard Duration // Standard path

// HTTP Client
use reqwest::header::{ACCEPT, ACCEPT_ENCODING, AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE};

// Misc
use std::path::Path;

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
    let path = Path::new("example\\"); // TODO: Get the path to the actual folder where the app is located, to watch.

    // ? Instantiate a user session.
    let session = Session::new();

    // ? For the first link command, we need to create a new zip file, with all the files in the folder.

    ////

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

    println!("Watching {} for changes...", path.display());

    loop {
        // ? The watcher loop will run until the CLI receives a user interruption.
        match receiver.recv() {
            Ok(DebouncedEvent::NoticeWrite(path)) => println!("Notice Write: {:?}", path),
            Ok(DebouncedEvent::Write(path)) => {
                // ? Zip the file, using the zip utils.
                let file = send::file::zip(&path);

                let client = reqwest::blocking::Client::new();
                let res = client
                    .post("https://app.io.vtex.com/vtex.builder-hub/v0/avantivtexio/cli/_v/builder/0/link/avantivtexio.store-starter@0.0.0?tsErrorsAsWarnings=false")
                    .header(ACCEPT, "application/json, text/plain, */*")
                    .header(ACCEPT_ENCODING, "gzip")
                    .header(CONTENT_LENGTH, file.len())
                    .header(CONTENT_TYPE, "application/octet-stream")
                    .header(AUTHORIZATION, format!("Bearer {}", session.token))
                    .body(file)
                    .send();

                match res {
                    Ok(res) => {
                        // print the response body text
                        println!("{:?}", res.text());
                    }
                    Err(e) => {
                        println!("{:?}", e);
                    }
                }
            }
            Ok(DebouncedEvent::Create(path)) => println!("Create: {:?}", path),
            Ok(DebouncedEvent::Remove(path)) => println!("Remove: {:?}", path),
            Ok(DebouncedEvent::Rename(o_path, n_path)) => {
                println!("Rename: {:?} to {:?}", o_path, n_path)
            }
            Err(e) => println!("Error: {:?}", e),
            _ => (),
        }
    }
}
