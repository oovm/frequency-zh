use crate::data::{CharMap, ErrorHere};
use serde_json::Value;
use std::fs::{read_to_string, File};
use std::io::Write;
use std::path::PathBuf;
use walkdir::WalkDir;

pub fn count_baike2018() -> std::io::Result<()> {
    let mut count = CharMap::default();
    for entry in WalkDir::new("baike2018qa") {
        let path = entry?.into_path();
        println!("Now parsing: {}", path.to_str().unwrap_or("Error!"));
        if let Ok(o) = tally_file(path) {
            count += o
        };
    }
    let mut file = File::create("baike2018qa.json")?;
    file.write_all(count.to_json().to_string().as_bytes())?;
    Ok(())
}

fn tally_file(path: PathBuf) -> Result<CharMap, ErrorHere> {
    let mut count = CharMap::default();
    for line in read_to_string(path)?.lines() {
        if let Ok(o) = tally_line(line) {
            count += o
        };
    }
    return Ok(count);
}

fn tally_line(data: &str) -> Result<CharMap, ErrorHere> {
    let mut count = CharMap::default();
    let json = serde_json::from_str::<Value>(data)?;
    for c in json.get("title")?.as_str()?.trim().chars() {
        *count.entry(c).or_insert(0) += 1;
    }
    for c in json.get("desc")?.as_str()?.trim().chars() {
        *count.entry(c).or_insert(0) += 1;
    }
    for c in json.get("answer")?.as_str()?.trim().chars() {
        *count.entry(c).or_insert(0) += 1;
    }
    return Ok(count);
}
