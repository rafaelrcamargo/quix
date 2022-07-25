//! # Retrieves the project data from `manifest.json`.
//! This function will return a `Project` struct.
//!
//! # Examples
//! ```rust
//! let project = Project::info();
//! ```
//!
//! # Panics
//! This function will panic if the JSON file is not properly formatted.
//! If the JSON file is not found, this function will panic.

use crate::json;

use serde::Deserialize;

use std::{env, fs::File, path::PathBuf, process::exit};

/// # Project struct.
/// This struct will contain the project data.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub vendor: String,  // Vendor name
    pub name: String,    // Project name
    pub version: String, // Project version
}

/// # Implements the `Project` and define the **info** method.
/// This method will return the project struct.
///
/// # Examples
/// ```rust
/// let project = Project::info();
/// ```
impl Project {
    pub fn info() -> Project {
        let project: Project = Project {
            vendor: String::new(),
            name: String::new(),
            version: String::new(),
        };

        // ? Get the project directory
        match env::current_dir() {
            Ok(path) => match get_project(path) {
                Ok(project) => {
                    return project;
                }
                Err(e) => {
                    error!("Project info failed: {:?}", e);
                    exit(exitcode::CONFIG);
                }
            },
            Err(e) => error!("{}", e),
        }

        return project;
    }
}

/// # Get the manifest data from the manifest file.
/// This function will get the manifest from the manifest file.
///
/// # Panics
/// This function will panic if the manifest file is not found.
/// If the file cannot be parsed correctly, this function will panic.
pub fn get_project(path: PathBuf) -> Result<Project, ()> {
    // ? Join current path + `manifest.json` file
    let path = path.join("manifest.json");

    // ? Tries to open the file
    match File::open(path) {
        // * File exists
        Ok(file) => {
            match json::read(file) {
                // * File is properly formatted
                Ok(manifest) => {
                    return Ok(manifest);
                }
                // * File is not properly formatted
                Err(e) => {
                    error!("JSON Error during parsing: {:?}", e);
                    exit(exitcode::UNAVAILABLE)
                }
            }
        }
        // ! Wasn't able to open the file
        Err(_) => panic!("No manifest file found."),
    }
}
