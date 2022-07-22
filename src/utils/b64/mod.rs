use std::{path::PathBuf, fs::File, io::{BufReader, Read}};

use base64;

pub fn encode(path: &PathBuf) -> String{
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).unwrap();
    let encoded = base64::encode(&contents);

    encoded
}
