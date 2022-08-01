//! # Read the JSON file.
//! Read the JSON file and deserialize it.
//!
//! # Panics
//! This function will panic if the JSON file is not properly formatted.

use serde::de;
use std::{fs::File, io::BufReader};

/// # Read the JSON file.
/// Read the JSON file and deserialize it.
///
/// # Panics
/// This function will panic if the JSON file is not properly formatted.
#[allow(dead_code)] // While this is not implemented, dead_code suppresses the warning.
pub fn read<T>(file: File) -> Result<T, ()>
where
    T: de::DeserializeOwned,
{
    // ? Create a buffer reader
    let reader = BufReader::new(file);

    // ? Deserializes the JSON
    match serde_json::from_reader(reader) {
        // * Deserialization successful
        Ok(data) => Ok(data),
        // !!! Wasn't able to deserialize the JSON file
        Err(e) => Err(error!("JSON Parsing failed during read: {:?}", e)),
    }
}
