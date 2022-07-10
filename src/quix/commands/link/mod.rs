// ? CLI
use clap::ArgMatches; // CLI Argument parser

// ? File watcher
use notify::DebouncedEvent::{Create, NoticeWrite, Remove, Rename, Write}; // Notify crate
use notify::{watcher, RecursiveMode, Watcher}; // Notify crate
use std::sync::mpsc::channel; // Message channel queue
use std::time::Duration; // STD Duration

pub fn app(link_matches: &ArgMatches) {
    if link_matches.contains_id("clean") {
        println!("Cleaning project cache...");
    }

    if link_matches.contains_id("quicker") {
        println!("Linking app quicker...");
    }

    // ? Create a channel to receive the events.
    let (sender, receiver) = channel();

    // ? Create a watcher object, delivering debounced events.
    // * The notification back-end is selected based on the platform.
    let mut watcher = watcher(sender, Duration::from_secs(1)).unwrap();

    // ? Add a path to be watched.
    // * All files and directories at that path and below will be monitored for changes.
    watcher
        .watch("example\\", RecursiveMode::Recursive)
        .unwrap();

    println!("Linking app...");

    loop {
        match receiver.recv() {
            Ok(NoticeWrite(path)) => println!("Notice Write: {:?}", path),
            Ok(Write(path)) => println!("Write: {:?}", path),
            Ok(Create(path)) => println!("Create: {:?}", path),
            Ok(Remove(path)) => println!("Remove: {:?}", path),
            Ok(Rename(o_path, n_path)) => println!("Rename: {:?} to {:?}", o_path, n_path),
            Err(e) => println!("Error: {:?}", e),
            _ => (),
        }
    }
}
