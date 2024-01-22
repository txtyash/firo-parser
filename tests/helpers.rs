use anyhow::Result;
use std::fs;
pub fn file_to_string(path: &str) -> Result<String> {
    println!("Path: {:?}", path);
    Ok(fs::read_to_string(path)?)
}
