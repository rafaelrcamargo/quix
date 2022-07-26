/// # Mod to handle JSON parsing
/// This module contains functions to parse JSON files.
pub mod json;

/// # Mod to handle Base64 encoding/decoding
/// This module contains functions to encode and decode Base64 strings.
pub mod b64;

/// # Mod to handle gzip compression to bytes
/// This module contains functions to compress files into bytes.
pub mod gzip;

// ? Debug zip file:
/* {
    debug!("Write: {:?}", file);
    let mut zip_file = File::create("test.zip").unwrap();

    // Write a slice of bytes to the zip_file
    match zip_file.write_all(&file) {
        Ok(_) => debug!("File written to the zip archive."),
        Err(e) => panic!("Error: {:?}", e),
    }
} */
