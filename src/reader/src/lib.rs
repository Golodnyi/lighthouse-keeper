use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::PathBuf;

pub fn read_file(path: String) -> Result<String, io::Error> {
    let mut file = PathBuf::from(path);
    file.set_extension("json");

    if !check_extension(&file, "json") {
        let err = io::Error::new(io::ErrorKind::Other, "Is not json file");
        return Err(err);
    } if !check_extension(&file, "json") {
        let err = io::Error::new(io::ErrorKind::Other, "Is not json file");
        return Err(err);
    }

    let mut data = "".to_owned();
    let mut file = File::open(file)?;
    file.read_to_string(&mut data)?;
    Ok(data)
}

pub fn write_file(path: String, data: String) -> Result<(), io::Error> {
    let mut path = PathBuf::from(path);
    path.set_extension("json");

    if path.is_file() {
        fs::remove_file(&path)?;
    }

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)?;

    file.write_all(data.as_bytes()).unwrap();

    Ok(())
}

fn check_extension(file: &PathBuf, ext: &str) -> bool {
    let extension = match file.extension() {
        Some(ext) => ext,
        None => OsStr::new("None"),
    };
    extension == ext
}

pub fn read_dir(dir: PathBuf) -> Result<Vec<PathBuf>, io::Error> {
    let mut files: Vec<PathBuf> = vec![];

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() || !check_extension(&path, "json") {
            continue;
        }

        files.push(entry.path());
    }

    Ok(files)
}
