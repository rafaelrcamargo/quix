// Zip
#[path = "../../utils/send/mod.rs"]
mod send;

// CLI
use clap::ArgMatches; // CLI Argument parser

// Watch and notify
use notify::DebouncedEvent; // Notify crate
use notify::{watcher, RecursiveMode, Watcher};

// Notify crate
use std::sync::mpsc::channel; // Message channel queue
use std::time::Duration; // Standard Duration

// HTTP Client
use reqwest::header::{ACCEPT, ACCEPT_ENCODING, AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE};

// Misc
use std::path::Path; // Standard path

pub fn link(args: &ArgMatches) {
    // ? Get the path to the actual folder where the app is located, to watch.
    let path = Path::new("example\\"); // TODO: Get the path to the actual folder where the app is located, to watch.

    match send::dir::zip("example\\") {
        Ok(_) => println!("done writting zip file"),
        Err(e) => println!("Error: {:?}", e),
    }

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
            // Ok(DebouncedEvent::NoticeWrite(path)) => println!("Notice Write: {:?}", path),
            Ok(DebouncedEvent::Write(path)) => {
                // ? Zip the file, using the zip utils.
                let file = send::file::zip(&path);

                /*
                ? Debug zip file:
                    {
                        println!("Write: {:?}", file);
                        let mut zip_file = File::create("test.zip").unwrap();
                        // Write a slice of bytes to the zip_file
                        match zip_file.write_all(&file) {
                            Ok(_) => println!("File written to the zip archive."),
                            Err(e) => panic!("Error: {:?}", e),
                        }
                    }
                */

                let client = reqwest::blocking::Client::new();
                let res = client
                    .post("https://app.io.vtex.com/vtex.builder-hub/v0/avantivtexio/cli/_v/builder/0/link/avantivtexio.store-starter@0.0.0?tsErrorsAsWarnings=false")
                    .header(ACCEPT, "application/json, text/plain, */*")
                    .header(ACCEPT_ENCODING, "gzip")
                    .header(CONTENT_LENGTH, file.len())
                    .header(CONTENT_TYPE, "application/octet-stream")
                    .header(AUTHORIZATION, "Bearer eyJhbGciOiJFUzI1NiIsImtpZCI6IjM3ODFGRUY5NENBMkQ5MTI5QzI4NjRBNzdGMzI2OEYxMUY4M0Y4MzEiLCJ0eXAiOiJqd3QifQ.eyJzdWIiOiJyYWZhZWwuY2FtYXJnb0BwZW5zZWF2YW50aS5jb20uYnIiLCJhY2NvdW50IjoiYXZhbnRpdnRleGlvIiwiYXVkaWVuY2UiOiJhZG1pbiIsInNlc3MiOiJkMmQxM2MxYy00YTJiLTQ1N2ItODhmZC1kMzgzZWRmMTUyZDciLCJleHAiOjE2NTc4OTcwNTMsInVzZXJJZCI6IjkxM2I2NWFlLTNhZTEtNGY5YS1iOTg2LWI3ODE0OTRjNWJlNiIsImlhdCI6MTY1NzgxMDY1MywiaXNzIjoidG9rZW4tZW1pdHRlciIsImp0aSI6Ijc4YzUwODhiLTRiZDktNDdiZS1hZWYyLTA5MWE5ZWE3OGJiOSJ9.d2a9ejPptDULgBBv2Kg9xvvzfRfGEOyksoUzdT_uKhDE0-45H-4848UQCsk9JC8Ck0FERM5gVJjMnUP8R9MIRA")
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
