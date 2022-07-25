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
use std::{fs::File, path::PathBuf, process::exit};

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
                    error!(
                        "We ran into an error searching for the VTEX session: {:?}",
                        e
                    );
                    help!("Check if the VTEX CLI is installed and if your login and authentication token are set.");
                    exit(exitcode::UNAVAILABLE);
                }
            },
            None => error!("No home directory found."),
        }

        return vtex;
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
        Ok(file) => {
            return json::read(file);
        }
        // ! Wasn't able to open the file
        Err(_) => {
            error!("No VTEX session found.");
            help!("Login to your account using the VETX CLI, then try again.");
            exit(exitcode::UNAVAILABLE)
        }
    }
}
