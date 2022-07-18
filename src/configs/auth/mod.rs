//! # Retrieves the user session from the VTEX CLI.
//! Here we set the `Session` struct, which is used for the authentication all over the app.
//!
//! ## Examples
//! Session:
//! ```rust
//! let session = Session::new();
//! ```
//!
//! ## Panics
//! This function will panic if the VTEX CLI is not properly installed and if your login and authentication token are not set.
//! This is because the CLI will not be able to authenticate with the VTEX API.
//! With that further requests will not be able to be sent to the builder.

use serde::Deserialize;

use std::{fs::File, path::PathBuf};

use crate::json;

/// # Session struct.
/// Here we set the `Session` struct, which is used for the authentication all over the app.
/// This struct is used to store the login and authentication token.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    pub account: String, // VTEX Account
    pub login: String,   // User email
    pub token: String,   // Session token
}

/// # Implements the `Session` and define the new method.
/// This method will return the session struct.
///
/// # Examples
/// ```rust
/// let session = Session::new();
/// ```
impl Session {
    pub fn new() -> Session {
        let mut session: Session = Session {
            account: String::new(),
            login: String::new(),
            token: String::new(),
        };

        // ? Get the home directory
        match home::home_dir() {
            Some(path) => {
                session = get_session(path);
            }
            None => println!("No home directory found."),
        }

        return session;
    }
}

/// # Get the session data from the session file.
/// This function will get the session from the session file.
///
/// # Panics
/// This function will panic if the session file is not found.
/// This is because the CLI will not be able to authenticate with the VTEX API.
pub fn get_session(path: PathBuf) -> Session {
    // ? Join `home` path + `.vtex` path + `session.json` file
    let path = path.join(".vtex/session/session.json");

    // ? Tries to open the file
    match File::open(path) {
        // * File exists
        Ok(file) => json::read(file),
        // ! Wasn't able to open the file
        Err(_) => panic!("No session file found."),
    }
}
