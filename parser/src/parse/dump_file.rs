use std::fs::{create_dir, exists, read_dir, read_to_string, remove_dir, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use serde_json::to_string_pretty;

use crate::parse::html_element::HTMLElement;
use crate::parse::paths::{JSON, MARKDOWN};
use crate::parse::to_html::parse_markdown;

pub fn dump_blogs() -> Result<(), std::io::Error> {
    for try_lang in read_dir(MARKDOWN)? {
        let lang = try_lang?.path();
        if !lang.is_dir() {
            continue;
        }
        let mut read_path = PathBuf::from(MARKDOWN);
        let language = lang.file_name().ok_or(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!(
                "basename could not be extracted from absolute path {}",
                read_path.display()
            ),
        ))?;
        read_path.push(language);
        // delete dump folder if it exists
        if exists(&read_path).is_ok() {
            remove_dir(&read_path)?;
        }
        create_dir(&read_path)?;

        for blog in read_dir(&read_path)? {
            let entry = blog?.path();
            let contents = dump_to_str(&read_path)?;
            let mut dump_path = PathBuf::from(JSON);
            dump_path.push(language);
            let json_name = lang
                .file_name()
                .ok_or(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "basename could not be extracted from absolute path {}",
                        read_path.display()
                    ),
                ))?
                .to_str()
                .ok_or(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "could not convert file name string into UTF-8 for path {}",
                ))?
                .trim_end_matches(".json")
                .to_string()
                + ".md";
            dump_path.push(json_name);
            let mut file = File::open(&dump_path)?;
            file.write(contents.as_bytes())?;
        }
    }
    Ok(())
}

fn dump_to_str(path: &PathBuf) -> Result<String, std::io::Error> {
    let markdown = read_to_string(path)?
        .lines()
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let json = parse_markdown(&markdown);
    to_string_pretty::<Vec<HTMLElement>>(&json).map_err(|_| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!(
                "Serde error occurred when serialising parsed data in {}",
                path.display()
            ),
        )
    })
}
