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

// FSWatcher
use notify::{event, Config, Event, RecommendedWatcher, RecursiveMode, Watcher};

// Debouncer
use debounce::EventDebouncer;

// Watcher for the link.
use reqwest::blocking::Client;
use serde::Deserialize;
use std::{
    env,
    path::{Path, PathBuf},
    thread,
    time::Duration,
};

// Project modules.
use crate::{
    connections::{
        self,
        builder::{self, RelinkBody},
        colossus,
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

        match builder::clean(&client) {
            Ok(_) => {
                debug!("⛔ Project cache cleaned.");
            }
            Err(e) => {
                error!("Error cleaning project cache: {}", e);
            }
        }
    } else if args.contains_id("quicker") {
        warn!(
            "This feature still under development, and can cause some issues 💣. Use it carefully."
        );
        trace!("⚗️  Linking your project quicker...\n");
    }

    // ? Initialize the link from the builder.
    send_package(&path, &client);

    // ! Starts the EventSource client.
    let c_path = path.clone();
    let c_client = client.clone();
    let logs = thread::spawn(|| {
        colossus::stream(c_path, c_client);
    });

    // * * * Starts the watcher, in the current project folder. * * *
    let (tx, rx) = std::sync::mpsc::channel();

    // This example is a little bit misleading as you can just create one Config and use it for all watchers.
    // That way the pollwatcher specific stuff is still configured, if it should be used.
    let mut watcher = RecommendedWatcher::new(
        tx,
        Config::default()
            .with_poll_interval(Duration::from_secs(1))
            .with_compare_contents(true),
    )
    .unwrap();

    // watch some stuff
    watcher.watch(&path, RecursiveMode::Recursive).unwrap();

    let delay = Duration::from_millis(500);
    let debouncer = EventDebouncer::new(delay, move |event: Event| {
        handle_event(event, &client, &path)
    });

    thread::spawn(move || {
        for e in rx {
            match e {
                Ok(event) => {
                    debouncer.put(event);
                }
                Err(e) => error!("🛑 Watcher error: {:?}", e),
            }
        }
    })
    .join()
    .unwrap();

    // * Waits for the logs thread to finish.
    logs.join().unwrap();
}

fn choose_action(paths: Vec<PathBuf>, client: &Client, path: &Path) {
    if !paths.is_empty() {
        send_package(path, client)
    } else {
        send_file(&paths[0], client)
    }
}

fn handle_event(event: Event, client: &Client, path: &Path) {
    match event.kind {
        event::EventKind::Create(_) => {
            debug!("📂 File created: {:?}", event.paths);
            choose_action(event.paths, client, path)
        }
        event::EventKind::Modify(_) => {
            debug!("📝 File modified: {:?}", event.paths);
            choose_action(event.paths, client, path)
        }
        event::EventKind::Remove(_) => {
            debug!("🗑️ File removed: {:?}", event.paths);
            choose_action(event.paths, client, path)
        }
        _ => {}
    }
}

fn send_file(path: &PathBuf, client: &Client) {
    let final_path = path.to_str().unwrap().to_string();

    if final_path.contains(".git") || final_path.contains("node_modules") {
        return;
    }

    // ? Zip the file, using the zip utils.
    let file = b64::encode(path);
    let size = file.len();

    let relative_path = final_path
        .split(env::current_dir().unwrap().to_str().unwrap())
        .collect::<Vec<&str>>()[1];

    // * Thats definitely not the beauty way to do this, but it works.
    // ? The zip writer in windows devices, uses \\ to separate directories, but when handling it on linux, it uses /, this creates a problem, here we replace it.
    let mut p = str::replace(relative_path, "\\", "/"); // Replace the backslashes with slashes.
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
                    "This looks like an Error. Please check your internet connection and try again."
                );
                error!("{:?}: {}", error.code, error.message);
            }
        }
        Err(e) => {
            error!("{:?}", e);
        }
    }
}

pub fn send_package(path: &Path, client: &Client) {
    // For the first link command, we need to create a new zip file, with all the files in the folder.
    // ? Create a new zip bundle.
    let bundle = gzip::zip(path).unwrap();

    // ? Send the bundle to the builder.
    match builder::link(client, bundle) {
        Ok(resp) => {
            if resp.status().is_success() {
                // => The link was sent to the builder.
                success!("Successfully sent the bundle to the builder.");
            } else {
                let error: VTEXError = resp.json().unwrap();

                // !!! Panic if the resp is not a success.
                if error.code.as_str() == "link_on_production" {
                    help!("Action not allowed on production, change to a development environment to link your project.");
                }

                help!("Check your internet connection and VTEX credentials, try logging in again.");
                error!("{:?}: {}", error.code, error.message);
            }
        }
        Err(e) => {
            help!("Error while sending the bundle to the builder.");
            error!("{:?}", e);
        }
    }
}
