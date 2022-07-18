//! # Send file module.
//! Here we handle the process for zipping the file, usually the changed one from the link command.

use std::io::{Cursor, Read, Write};
use std::path::PathBuf;

use zip::write::{FileOptions, ZipWriter};

/// # Zip file, and prepare it to be sent to the builder.
/// This function will zip the file, and prepare it to be sent to the builder.
/// - It will return the zipped file as a `Vec<u8>`.
///
/// # Examples
/// ```
/// let file = PathBuf::from("test.txt");
/// let zip = zip(file);
/// ```
///
/// # Panics
/// This function will panic if the file does not exist.
/// Thats because the CLI will not be able to send the file to the builder.
#[allow(dead_code)] // While this is not implemented, dead_code suppresses the warning.
pub fn zip(path: &PathBuf) -> Vec<u8> {
    // ? First start by creating the foundation for the zip file.
    let mut buf = Vec::new(); // Responsible for handling the buffer and the bytes in it.
    let mut cursor = Cursor::new(&mut buf); // Responsible for the cursor.
    let mut zip = ZipWriter::new(&mut cursor); // Our zip writer.

    // ? Options for the file to be added to the zip archive.
    let options = FileOptions::default() // Default options.
        .compression_method(zip::CompressionMethod::Stored) // Compression method.
        .unix_permissions(0o755); // Permissions (UNIX).

    // ? Add the file to the zip archive.
    let file = std::fs::File::open(path).unwrap(); // File object
    let mut file_reader = std::io::BufReader::new(file); // Buffer
    let mut file_buffer = Vec::new(); // Buffer vec for the file
    file_reader.read_to_end(&mut file_buffer).unwrap(); // Read the file to the buffer

    // ? Create de file in the zip archive
    let file_name = path.file_name().unwrap().to_str().unwrap(); // File name
    zip.start_file(file_name, options).unwrap(); // Create the file in the zip archive
    zip.write_all(&file_buffer).unwrap(); // Write the buffer to the file

    // ? Finish the zip archive.
    zip.finish().unwrap(); // Close the file in the zip archive
    drop(zip); // ! We drop zip here to deallocate `buf`, so we can return the buffer without returning function lifeline values.

    return buf; // Return the buffer, witch has the bytes of the zip file.
}
