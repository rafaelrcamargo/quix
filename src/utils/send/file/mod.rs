// * Zip
use zip::write::{FileOptions, ZipWriter};

// * Standard library
use std::{
    io::{Cursor, Read, Write},
    path::PathBuf,
};

pub fn zip(path: &PathBuf) -> Vec<u8> {
    let mut buf = Vec::new(); // Responsible for handling the buffer and the bytes in it.
    let mut cursor = Cursor::new(&mut buf); // Responsible for the cursor.
    let mut zip = ZipWriter::new(&mut cursor); // * Our zip writer.

    // ? Options for the file to be added to the zip archive.
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .unix_permissions(0o755);

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

    return buf; // * Return the buffer, witch has the bytes of the zip file.
}
