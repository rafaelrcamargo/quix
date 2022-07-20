//! # Send dir module.
//! Here we handle the process for zipping the project directory, thats useful in the first link command.
//!
//! _That can also be useful for later implementing the `deploy` workflow._

use std::fs::File;
use std::io::{prelude::*, Cursor, Write};
use std::iter::Iterator;
use std::path::Path;

use walkdir::{DirEntry, WalkDir};
use zip::{result::ZipError, write::FileOptions, ZipWriter};

/// # Zip dir, and prepare it to be sent to the builder.
/// This function will zip the directory, and prepare it to be sent to the builder.
/// - It will return the zipped directory as a `Vec<u8>`.
///
/// # Examples
/// ```
/// let dir = Path::new("test//");
/// let zip = zip(dir);
/// ```
///
/// # Panics
/// This function will panic if the directory does not exist.
/// Thats because the CLI will not be able to send the directory to the builder.
/// __That can also be useful for later implementing the `deploy` workflow.__
#[allow(dead_code)] // While this is not implemented, dead_code suppresses the warning.
pub fn zip(path: &Path) -> Result<Vec<u8>, ZipError> {
    if !path.is_dir() {
        return Err(ZipError::UnsupportedArchive("The path is not a directory."));
    }

    // ? Create a new zip writer.
    let walkdir = WalkDir::new(path);
    let it = walkdir.into_iter();

    // ? Iterate through the files in the directory.
    return Ok(deep_search(&mut it.filter_map(|e| e.ok()), path));
}

/// # Deep search.
/// This function will iterate through the files in the directory, and zip them.
/// - It will return the zipped directory as a `Vec<u8>`.
/// - It will return an empty vector if the directory is empty.
///
/// # Examples
/// ```
/// let dir = Path::new("test//");
/// let zip = deep_search(&mut it.filter_map(|e| e.ok()), path);
/// ```
fn deep_search(it: &mut dyn Iterator<Item = DirEntry>, prefix: &Path) -> Vec<u8> {
    // ? Create a new zip writer.
    let mut buf = Vec::new(); // Responsible for handling the buffer and the bytes in it.
    let mut cursor = Cursor::new(&mut buf); // Responsible for the cursor.
    let mut zip = ZipWriter::new(&mut cursor); // Responsible for the zip writer.

    // ? Options for the file to be added to the zip archive.
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .unix_permissions(0o755);

    // ? Buffer for files inside the directories.
    let mut buffer = Vec::new();

    // ? Iterate through the files in the directory.
    for entry in it {
        let path = entry.path(); // Get the file path

        // ? Check if the path is different than a list of files to ignore
        let ignore = vec!["node_modules"]; // TODO: get this from the .vtexignore file

        if ignore.iter().any(|x| path.to_str().unwrap().contains(x)) {
            continue;
        }

        // ? Check if the path is a directory.
        let name = path.strip_prefix(prefix).unwrap();

        // Write file or directory explicitly
        // Some unzip tools unzip files with directory paths correctly, some do not!
        if path.is_file() {
            // ? Thats definitely not the beauty way to do this, but it works.
            // !!! The zip writer in windows devices, uses \\ to separate directories, but when handling it on linux, it uses /, this creates a problem, here we replace it.
            let mut p = str::replace(&name.to_str().unwrap(), "\\", "/"); // Replace the backslashes with slashes.
            if p.starts_with('/') {
                p.remove(0);
            } // Remove the first '/'

            // ? Write the file to the buffer.
            zip.start_file(p, options).unwrap();
            let mut f = File::open(path).unwrap();

            /* // ? Minify the file.
            // TODO: Think about minification, size reduction x timing.
            minify(path, &mut f, &mut buffer); // ! Apparently its not working, for JSX & TSX files.
            */

            f.read_to_end(&mut buffer).unwrap();
            zip.write_all(&*buffer).unwrap();
            buffer.clear();
        } else if name.as_os_str().len() != 0 {
            // Only if not root! Avoids path spec / warning
            // and mapname conversion failed error on unzip

            // * Thats definitely not the beauty way to do this, but it works.
            // ? The zip writer in windows devices, uses \\ to separate directories, but when handling it on linux, it uses /, this creates a problem, here we replace it.
            let mut p = str::replace(&name.to_str().unwrap(), "\\", "/"); // Replace the backslashes with slashes.
            if p.starts_with('/') {
                p.remove(0);
            } // Remove the first '/' if exists.

            // ? Write the file to the buffer.
            zip.add_directory(p, options).unwrap();
        }
    }

    // ? Finish the zip archive.
    zip.finish().unwrap(); // Close the file in the zip archive
    drop(zip); // ! We drop zip here to deallocate `buf`, so we can return the buffer without returning function lifeline values.

    return buf; // * Return the buffer, witch has the bytes of the zip file.
}

/// # Minify the file.
/// This function will minify the file, and write it to the buffer.
/// - It will return the buffer.
///
/// # Examples
/// ```
/// let min = minify(path, &mut f, &mut buffer);
/// ```
#[allow(dead_code)] // While this is not implemented, dead_code suppresses the warning.
fn minify(path: &Path, f: &mut File, buffer: &mut Vec<u8>) {
    let ext = path.extension().unwrap().to_str().unwrap();

    if ext == "json" || ext == "jsonc" {
        let mut json = String::new();
        match f.read_to_string(&mut json) {
            Ok(_) => {
                let min_json = minifier::json::minify(&json.as_str());
                min_json.write(buffer).unwrap();
            }
            Err(e) => panic!("JSON Minifier - ERROR: {:?}", e),
        }
    } else if ext == "js" || ext == "jsx" || ext == "ts" || ext == "tsx" {
        let mut js = String::new();
        match f.read_to_string(&mut js) {
            Ok(_) => {
                let min_js = minifier::js::minify(&js.as_str());
                min_js.write(buffer).unwrap();
            }
            Err(e) => panic!("JS Minifier - ERROR: {:?}", e),
        }
    } else if ext == "css" || ext == "scss" {
        let mut css = String::new();
        match f.read_to_string(&mut css) {
            Ok(_) => {
                let min_css = minifier::css::minify(&css.as_str());
                match min_css {
                    Ok(min) => min.write(buffer).unwrap(),
                    Err(e) => panic!("CSS Minifier - ERROR: {:?}", e),
                }
            }
            Err(e) => panic!("CSS Minifier - ERROR: {:?}", e),
        }
    } else {
        match f.read_to_end(buffer) {
            Ok(_) => println!("File skipped: {}", path.to_str().unwrap()),
            Err(e) => panic!("Minifier - ERROR: {:?}", e),
        }
    }
}
