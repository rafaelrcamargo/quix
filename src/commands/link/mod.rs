//! # Link subcommand
//! The `link` subcommand is used to link a project to a VTEX account.
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
use serde::Deserialize;
use std::{env, path::PathBuf, sync::mpsc::channel, thread, time::Duration};

// * Eventsource for the CLI.
use eventsource::reqwest::Client as EventSourceClient;
use reqwest::blocking::Client;

// Project modules.
use crate::{
    clients,
    configs::VTEX,
    connections::{
        self,
        builder::{self, RelinkBody},
    },
    utils::{b64, gzip},
};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct VTEXError {
    code: String,
    message: String,
}

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
    let path = env::current_dir().unwrap();

    let client = match connections::builder::check_availability() {
        Ok(client) => client,
        Err(_) => {
            help!("Error finding a available builder, try again later.");
            panic!("Error finding a available builder, try again later.")
        }
    };

    // ? Args parsing.
    if args.contains_id("clean") {
        warn!("This feature can cause the CLI to run slower ⌛️, only use when really necessary.");
        trace!("🧹 Cleaning project cache...\n");
    } else if args.contains_id("quicker") {
        warn!(
            "This feature still under development, and can cause some issues 💣. Use it carefully."
        );
        trace!("⚗️  Linking your project quicker...\n");
    }

    // ? Initialize the link from the builder.
    first_link(&path, &client);

    // ? Clones the client and the path, for the Eventsource.
    let it_path = path.clone();
    let it_client = client.clone();
    thread::spawn(move || {
        // ? Instantiate a user session.
        let session = VTEX::info();

        // ? Create a new VTEX Client.
        let t_client = clients::vtex::new(&session.token);

        let t_client =
        EventSourceClient::new_with_client("https://infra.io.vtex.com/colossus/v0/avantivtexio/cli/events?onUnsubscribe=link_interrupted&sender=vtex.builder-hub&keys=build.status".parse().unwrap(), t_client);

        for event in t_client {
            match event {
                Ok(event) => {
                    if event.data == "link_interrupted" {
                        error!("Link interrupted.");
                    } else if event.data != "ping\n" {
                        if event.data.contains("initial_link_required") {
                            first_link(&it_path, &it_client);
                        } else {
                            // stringify!(&event.data);
                        }
                    }
                }
                Err(e) => {
                    error!("Error: {}", e);
                }
            }
        }
    });

    // * * * Starts the watcher, in the current project folder. * * *

    // ? Create a channel to receive the events.
    let (sender, receiver) = channel();
    // ? Create a watcher object, delivering debounced events.
    let mut watcher = watcher(sender, Duration::from_secs(1)).unwrap();
    // ? All files and directories at that path and below will be monitored for changes.
    watcher.watch(path, RecursiveMode::Recursive).unwrap();

    debug!("⏱  Waiting for events...");
    help!("You can use Ctrl+C to stop the watcher.\n");

    loop {
        // ? The watcher loop will run until the CLI receives a user interruption.
        match receiver.recv() {
            // ? Notify the user of the event.
            Ok(DebouncedEvent::NoticeWrite(path)) => trace!("Notice write: {:?}", path),

            // ? Common handling for all events.
            Ok(DebouncedEvent::Write(path)) => event(&client, &path),
            Ok(DebouncedEvent::Create(path)) => event(&client, &path),

            // TODO: Remove & Rename events.
            Ok(DebouncedEvent::Remove(path)) => todo!("Removed: {:?}", path),
            Ok(DebouncedEvent::Rename(o_path, n_path)) => {
                todo!("Renamed: {:?} to {:?}", o_path, n_path)
            }

            // !!! Unimplemented events.
            Err(e) => unimplemented!("Error: {:?}", e),
            _ => (), // !!! Everything else is ignored.
        }
    }
}

fn event(client: &Client, path: &PathBuf) {
    debug!("Preparing to link: {:?}", path);

    // ? Zip the file, using the zip utils.
    let file = b64::encode(path);
    let size = file.len();

    let final_path = path.to_str().unwrap().to_string();

    let relative_path = final_path
        .split(env::current_dir().unwrap().to_str().unwrap())
        .collect::<Vec<&str>>()[1];

    // * Thats definitely not the beauty way to do this, but it works.
    // ? The zip writer in windows devices, uses \\ to separate directories, but when handling it on linux, it uses /, this creates a problem, here we replace it.
    let mut p = str::replace(&relative_path, "\\", "/"); // Replace the backslashes with slashes.
    if p.starts_with('/') {
        p.remove(0);
    } // Remove the first '/' if exists.

    let body = RelinkBody {
        content: file,
        byte_size: size,
        path: p,
    };

    // ? Send the file to the builder.
    match builder::relink(client, body) {
        Ok(resp) => {
            if resp.status().is_success() {
                // stringify!(resp.text().unwrap().as_str());
                success!("Successfully sent the 💫 file to the builder.");
            } else if resp.status().is_server_error() {
                let error: VTEXError = resp.json().unwrap();
                // !!! Panic if the resp is not a success.
                help!("This looks like a Server Error (500ish). Please try again later.");
                error!("{:?}: {}", error.code, error.message);
            } else {
                let error: VTEXError = resp.json().unwrap();
                // !!! Panic if the resp is not a success.
                help!(
                    "This looks like a error. Please check your internet connection and try again."
                );
                error!("{:?}: {}", error.code, error.message);
            }
        }
        Err(e) => {
            error!("{:?}", e);
        }
    }
}

fn first_link(path: &PathBuf, client: &Client) {
    // For the first link command, we need to create a new zip file, with all the files in the folder.
    // ? Create a new zip bundle.
    let bundle = gzip::zip(&path).unwrap();

    // ? Send the bundle to the builder.
    match builder::link(&client, bundle) {
        Ok(resp) => {
            if resp.status().is_success() {
                // => The link was sent to the builder.
                success!("Successfully sent the ✨ bundle to the builder.");
            } else {
                let error: VTEXError = resp.json().unwrap();

                // !!! Panic if the resp is not a success.
                match error.code.as_str() {
                    "link_on_production" => {
                        help!("Action not allowed on production, change to a development environment to link your project.");
                    }
                    _ => (),
                }

                help!("Check your internet connection and VTEX credentials, try logging in again.");
                error!("Error: {:?}: {}", error.code, error.message);
            }
        }
        Err(e) => {
            help!("Error while sending the bundle to the builder.");
            error!("{:?}", e);
        }
    }
}
