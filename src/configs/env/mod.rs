//! # Retrieves the user environment data from various places.
//! This function will return a different struct depending on the environment.
//!
//! # Examples
//! ```rust
//! let workspace = Environment::workspace();
//! ```
//!
//! # Panics
//! This function will panic if the JSON file is not properly formatted.
//! If the JSON file is not found, this function will panic.

use crate::json;
use serde::Deserialize;
use std::{fs::File, path::PathBuf};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Environment {
    pub current_workspace: String,
    pub last_workspace: String,
}

/// # Environment struct.
/// This struct will contain the environment data.
impl Environment {
    pub fn info() -> Environment {
        let mut env: Environment = Environment {
            current_workspace: String::new(),
            last_workspace: String::new(),
        };

        // ? Get the home directory
        match home::home_dir() {
            Some(path) => {
                env = get_workspace(path);
            }
            None => println!("No home directory found."),
        }

        return env;
    }
}

/// # Get the session data from the session file.
/// This function will get the session from the session file.
///
/// # Panics
/// This function will panic if the session file is not found.
/// This is because the CLI will not be able to authenticate with the VTEX API.
pub fn get_workspace(path: PathBuf) -> Environment {
    // ? Join `home` path + `.vtex` path + `session.json` file
    let path = path.join(".vtex/session/workspace.json");

    // ? Tries to open the file
    match File::open(path) {
        // * File exists
        Ok(file) => json::read(file),
        // ! Wasn't able to open the file
        Err(_) => panic!("No session file found."),
    }
}
