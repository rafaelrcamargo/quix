//! # Mod to handle sending zipped files to the builder.
//! Here we handle the zipping process, for later sending this packages to the builder.

pub mod dir;
pub mod file;

// ? Debug zip file:
/* {
    println!("Write: {:?}", file);
    let mut zip_file = File::create("test.zip").unwrap();

    // Write a slice of bytes to the zip_file
    match zip_file.write_all(&file) {
        Ok(_) => println!("File written to the zip archive."),
        Err(e) => panic!("Error: {:?}", e),
    }
} */
