use std::io::{Read, Write};

use zip::write::FileOptions;

pub fn zip() {
    match compress("test.zip") {
        Ok(_) => println!("File written to {}", "test.zip"),
        Err(e) => println!("Error: {:?}", e),
    }
}

fn compress(filename: &str) -> zip::result::ZipResult<()> {
    let path = std::path::Path::new(filename);
    let file = std::fs::File::create(&path).unwrap();
    let mut zip = zip::ZipWriter::new(file);

    // ? Options for the file to be added to the zip archive.
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .unix_permissions(0o755);

    // ? Add the file to the zip archive.
    let manifest_path = std::path::Path::new("example/manifest.json"); // File path
    let manifest_file = std::fs::File::open(&manifest_path).unwrap(); // File object
    let mut manifest_reader = std::io::BufReader::new(manifest_file); // Buffer
    let mut manifest_buffer = Vec::new(); // Buffer vec for the file
    manifest_reader.read_to_end(&mut manifest_buffer).unwrap(); // Read the file to the buffer
    zip.start_file("manifest.json", options)?; // Create de file in the zip archive
    zip.write_all(&manifest_buffer)?; // Write the buffer to the file
    zip.finish()?; // Finish the zip archive

    Ok(())
}
