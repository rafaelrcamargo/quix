// ? Zip
#[path = "../../utils/to/mod.rs"]
mod to;
use to::zip; // Zip utils */
             // ? CLI
use clap::ArgMatches; // CLI Argument parser

use notify::DebouncedEvent::{Create, NoticeWrite, Remove, Rename, Write}; // Notify crate
use notify::{watcher, RecursiveMode, Watcher}; // Notify crate
use std::sync::mpsc::channel; // Message channel queue
use std::time::Duration; // STD Duration

use reqwest::header::{ACCEPT, ACCEPT_ENCODING, AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE};
use std::path::Path; // STD Path

pub fn link(args: &ArgMatches) {
    // ? Get the path to the actual folder where the app is located, to watch.
    let path = Path::new("example\\"); // TODO: Get the path to the actual folder where the app is located, to watch.

    if args.is_present("clean") {
        println!("Cleaning project cache...");
    } else if args.is_present("quicker") {
        println!("Linking app quicker...");
    }

    // ? Create a channel to receive the events.
    let (sender, receiver) = channel();

    // ? Create a watcher object, delivering debounced events.
    // * The notification back-end is selected based on the platform.
    let mut watcher = watcher(sender, Duration::from_secs(1)).unwrap();

    // ? Add a path to be watched.
    // * All files and directories at that path and below will be monitored for changes.
    watcher.watch(path, RecursiveMode::Recursive).unwrap();

    println!("Watching {} for changes...", path.display());

    loop {
        match receiver.recv() {
            Ok(NoticeWrite(path)) => println!("Notice Write: {:?}", path),
            Ok(Write(path)) => {
                let file = zip(path.file_name().unwrap(), &path);

                println!("Write: {:?}", file);

                let client = reqwest::blocking::Client::new();
                let res = client
                    .post("https://app.io.vtex.com/vtex.builder-hub/v0/avantivtexio/cli/_v/builder/0/link/avantivtexio.store-theme@0.0.0?tsErrorsAsWarnings=false")
                    .header(ACCEPT, "application/json, text/plain, */*")
                    .header(ACCEPT_ENCODING, "gzip")
                    .header(CONTENT_LENGTH, file.len())
                    .header(CONTENT_TYPE, "application/octet-stream")
                    .header("x-vtex-bh-link-id", "")
                    .header("x-vtex-sticky-host", "")
                    .header(AUTHORIZATION, "Bearer ")
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

                return println!("Write: {:?}", path);
            }
            Ok(Create(path)) => println!("Create: {:?}", path),
            Ok(Remove(path)) => println!("Remove: {:?}", path),
            Ok(Rename(o_path, n_path)) => println!("Rename: {:?} to {:?}", o_path, n_path),
            Err(e) => println!("Error: {:?}", e),
            _ => (),
        }
    }
}
