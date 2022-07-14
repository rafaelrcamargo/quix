use std::io::prelude::*;
use std::io::{Seek, Write};
use std::iter::Iterator;
use zip::result::ZipError;
use zip::write::FileOptions;

use std::fs::File;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

pub fn zip(src_dir: &str) -> zip::result::ZipResult<()> {
    if !Path::new(src_dir).is_dir() {
        return Err(ZipError::FileNotFound);
    }

    let path = Path::new("bundle.zip");
    let file = File::create(&path).unwrap();

    let walkdir = WalkDir::new(src_dir.to_string());
    let it = walkdir.into_iter();

    deep_search(&mut it.filter_map(|e| e.ok()), src_dir, file)?;

    Ok(())
}

fn deep_search<T>(
    it: &mut dyn Iterator<Item = DirEntry>,
    prefix: &str,
    writer: T,
) -> zip::result::ZipResult<()>
where
    T: Write + Seek,
{
    let mut zip = zip::ZipWriter::new(writer);
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();
    for entry in it {
        let path = entry.path(); // Get the file path

        // ? Check if the path is different than a list of files to ignore
        let ignore = vec!["node_modules"]; // TODO: get this from the .vtexignore file

        if ignore.iter().any(|x| path.to_str().unwrap().contains(x)) {
            continue;
        }

        let name = path.strip_prefix(Path::new(prefix)).unwrap();

        // Write file or directory explicitly
        // Some unzip tools unzip files with directory paths correctly, some do not!
        if path.is_file() {
            zip.start_file(name.to_str().unwrap(), options)?;
            let mut f = File::open(path)?;

            /*

            ? Minification
            TODO: Think about minification, sixe x timing.

            let ext = path.extension().unwrap().to_str().unwrap();

            if ext == "json" || ext == "jsonc" {
                let mut json = String::new();
                f.read_to_string(&mut json)?;
                let min_json = minifier::json::minify(&json.as_str());

                min_json.write(&mut buffer).unwrap();
            } else if ext == "js" || ext == "jsx" || ext == "ts" || ext == "tsx" {
                let mut js = String::new();
                f.read_to_string(&mut js)?;
                let min_js = minifier::js::minify(&js.as_str());

                min_js.write(&mut buffer).unwrap();
            } else if ext == "css" || ext == "scss" {
                let mut css = String::new();
                f.read_to_string(&mut css)?;
                let min_css = minifier::css::minify(&css.as_str());

                match min_css {
                    Ok(min) => min.write(&mut buffer).unwrap(),
                    Err(e) => panic!("CSS Minifier - ERROR: {:?}", e),
                }
            } else {
                f.read_to_end(&mut buffer)?;
            } */

            f.read_to_end(&mut buffer)?;
            zip.write_all(&*buffer)?;
            buffer.clear();
        } else if name.as_os_str().len() != 0 {
            // Only if not root! Avoids path spec / warning
            // and mapname conversion failed error on unzip
            zip.add_directory(name.to_str().unwrap(), options)?;
        }
    }
    zip.finish()?;
    Result::Ok(())
}
