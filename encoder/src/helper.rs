use std::io::{BufRead, Read};
use std::error::Error;
use std::env::{var, temp_dir};
use std::fs::File;
use std::process::Command;

pub fn read_line() -> String {
    let stdin1 = std::io::stdin();
    let mut iter = stdin1.lock().lines();
    iter.next().unwrap().unwrap().trim().to_owned()
}

pub fn read_multiline() -> Result<String, Box<dyn Error>> {
    let editor = var("EDITOR").unwrap();
    let mut file_path = temp_dir();
    file_path.push(".enc-msg");
    File::create(&file_path)?;

    Command::new(editor)
        .arg(&file_path)
        .status()?;

    let mut editable = String::new();
    File::open(file_path)?
        .read_to_string(&mut editable)?;
    Ok(editable)
}