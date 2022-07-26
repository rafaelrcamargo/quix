//! # Encode and decode Base64 strings
//! This module is used to encode and decode Base64 strings, mainly for the Relink method.
//!
//! # Examples
//! ```rust
//! use b64::encode(path); // Encode a file into a Base64 string.
//! ```

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
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).unwrap();
    let encoded = base64::encode(&contents);

    encoded
}
