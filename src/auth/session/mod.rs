// * Std
use serde::Deserialize;
use std::{fs::File, io::BufReader, path::PathBuf};

/// Session struct.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    pub account: String, // VTEX Account
    pub login: String,   // User email
    pub token: String,   // Session token
}

/// Implements the `Session` and define the new method.
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

/// Get the session from the session file.
pub fn get_session(path: PathBuf) -> Session {
    // ? Join `home` path + `.vtex` path + `session.json` file
    let path = path.join(".vtex/session/session.json");

    // ? Tries to open the file
    match File::open(path) {
        // * File exists
        Ok(file) => read_json(file),
        // ! Wasn't able to open the file
        Err(_) => panic!("No session file found."),
    }
}

/// Read the JSON file.
fn read_json(file: File) -> Session {
    // ? Create a buffer reader
    let reader = BufReader::new(file);

    // ? Deserializes the JSON
    match serde_json::from_reader(reader) {
        // * Deserialization successful
        Ok(session) => session,
        // ! Wasn't able to deserialize the JSON file
        Err(e) => panic!("Failed to parse session file: {}", e),
    }
}
