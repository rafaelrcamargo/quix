//! # Retrieves the user vtex from the VTEX CLI.
//! Here we set the `VTEX` struct, which is used for the authentication all over the app.
//!
//! ## Examples
//! VTEX:
//! ```rust
//! let vtex = VTEX::new();
//! ```
//!
//! ## Panics
//! This function will panic if the VTEX CLI is not properly installed and if your login and authentication token are not set.
//! This is because the CLI will not be able to authenticate with the VTEX API.
//! With that further requests will not be able to be sent to the builder.

use crate::json;
use serde::Deserialize;
use serde_json::Value;
use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

use super::Project;

use serde_json::json;

/// # VTEX struct.
/// Here we set the `VTEX` struct, which is used for the authentication all over the app.
/// This struct is used to store the login and authentication token.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VTEX {
    // * Session Data
    pub account: String,
    pub login: String,
    pub token: String,

    // * Workspace Data
    pub workspace: String,
    #[serde(alias = "_lastUsedWorkspace")]
    pub last_used_workspace: String,
}

/// # Implements the `VTEX` and define the info method.
/// This method will return the vtex struct.
///
/// # Examples
/// ```rust
/// let vtex = VTEX::info();
/// ```
impl VTEX {
    pub fn info() -> VTEX {
        let mut vtex: VTEX = VTEX {
            account: String::new(),
            login: String::new(),
            token: String::new(),
            workspace: String::new(),
            last_used_workspace: String::new(),
        };

        // ? Get the home directory
        match home::home_dir() {
            Some(path) => match get_session(path) {
                Ok(session) => {
                    vtex = session;
                }
                Err(e) => {
                    help!("Check if the VTEX CLI is installed and if your login and authentication token are set.");
                    error!(
                        "We ran into an error searching for the VTEX session: {:?}",
                        e
                    );
                }
            },
            None => error!("No home directory found."),
        }

        vtex
    }

    pub fn raw_info() -> Result<Value, ()> {
        // ? Get the home directory
        match home::home_dir() {
            Some(path) => {
                // ? Join `home` path + `.vtex` path + `vtex.json` file
                let path = path.join(".config/configstore/vtex.json");
                // ? Read the file to a string
                let mut file = File::open(path).unwrap();
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                let input = contents.as_str();

                Ok(serde_json::from_str(input).unwrap())
            }
            None => Err(error!("No home directory found.")),
        }
    }

    pub fn set_sticky_host(host: &str) {
        // ? Create a new Project config.
        let project = Project::info().unwrap();

        // ? Get the home directory
        match home::home_dir() {
            Some(path) => {
                // ? Join `home` path + `.vtex` path + `vtex.json` file
                let path = path.join(".config/configstore/vtex.json");

                // ? Read the file to a string
                let mut file = File::open(&path).unwrap();
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                let input = contents.as_str();

                // ? Convert the string to a json object.
                let mut vtex: Value = serde_json::from_str(input).unwrap();
                let sticky_obj = vtex
                    .get("apps")
                    .and_then(|value| value.get(&project.vendor))
                    .and_then(|value| value.get(&project.name))
                    .and_then(|value| value.get("sticky-host"));

                // ? Change the sticky host, to the new one.
                if let Some(_sticky_host) = sticky_obj {
                    vtex.get_mut("apps")
                        .and_then(|value| value.get_mut(&project.vendor))
                        .and_then(|value| value.get_mut(&project.name))
                        .and_then(|value| value.get_mut("sticky-host"))
                        .and_then(|value| Some(*value = host.into()));
                    let vtex = serde_json::to_string(&vtex).unwrap();
                    let mut file = File::create(&path).unwrap();
                    file.write_all(vtex.as_bytes()).unwrap();
                } else {
                    vtex.get_mut("apps").and_then(|value| {
                        Some(
                            *value = json!({
                                project.vendor: {
                                    project.name: {
                                        "sticky-host": {
                                            "stickyHost": host,
                                            "lastUpdated": 0,
                                        }
                                    }
                                }
                            }),
                        )
                    });

                    // Write vtex to file.
                    let vtex = serde_json::to_string(&vtex).unwrap();
                    let mut file = File::create(&path).unwrap();
                    file.write_all(vtex.as_bytes()).unwrap();
                }
            }
            None => error!("No home directory found."),
        }
    }
}

/// # Get the vtex data from the vtex file.
/// This function will get the vtex from the vtex file.
///
/// # Panics
/// This function will panic if the vtex file is not found.
/// This is because the CLI will not be able to authenticate with the VTEX API.
pub fn get_session(path: PathBuf) -> Result<VTEX, ()> {
    // ? Join `home` path + `.vtex` path + `vtex.json` file
    let path = path.join(".config/configstore/vtex.json");

    // ? Tries to open the file
    match File::open(path) {
        // * File exists
        Ok(file) => json::read(file),
        // ! Wasn't able to open the file
        Err(_) => {
            help!("Login to your account using the VETX CLI, then try again.");
            Err(error!("No VTEX session found."))
        }
    }
}
