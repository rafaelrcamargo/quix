use std::{
    ffi::OsStr,
    io::{Read, Write},
    path::PathBuf,
};
use zip::read::ZipFile;
use zip::write::FileOptions;

pub fn zip(file_name: &OsStr, path: &PathBuf) -> Vec<u8> {
    let buf: &mut [u8] = &mut [0u8; 65536];
    let w = std::io::Cursor::new(buf);
    let mut zip = zip::ZipWriter::new(w);

    // ? Options for the file to be added to the zip archive.
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .unix_permissions(0o755);

    // ? Add the file to the zip archive.
    let file = std::fs::File::open(&path).unwrap(); // File object
    let mut file_reader = std::io::BufReader::new(file); // Buffer
    let mut file_buffer = Vec::new(); // Buffer vec for the file
    file_reader.read_to_end(&mut file_buffer).unwrap(); // Read the file to the buffer

    // Create de file in the zip archive
    match zip.start_file(file_name.to_str().unwrap(), options) {
        Ok(_) => (),
        Err(e) => panic!("Error: {:?}", e),
    }

    // Write the buffer to the file
    match zip.write_all(&file_buffer) {
        Ok(_) => (),
        Err(e) => panic!("Error: {:?}", e),
    };

    // Convert the buffer to bytes

    zip.finish().unwrap();
}
