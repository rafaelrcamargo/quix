//! # Encode and decode Base64 strings
//! This module is used to encode and decode Base64 strings, mainly for the Relink method.
//!
//! # Examples
//! ```rust
//! use b64::encode(path); // Encode a file into a Base64 string.
//! ```

use base64::{Engine as _, engine::{general_purpose}};

use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};

/// Lib for Base64 encoding/decoding.
use base64;

/// # Encode into a Base64.
/// This function will encode a file into a Base64 string.
pub fn encode(path: &PathBuf) -> String {
    // Check if the path is a file.
    if path.is_file() {
        // Open the file.
        let file = File::open(path).unwrap();
        let mut reader = BufReader::new(file);

        // Read the file.
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer).unwrap();

        // Encode the file.
        general_purpose::STANDARD.encode(&buffer)
    } else {
        // Return an empty string.
        String::new()
    }
}
