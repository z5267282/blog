use std::fs::{self, create_dir, exists, read_dir, remove_dir, File};
use std::io::{Read, Result};
use std::path::PathBuf;

use serde_json::to_string_pretty;

use crate::parse::html_element::HTMLElement;
use crate::parse::paths::{JSON, MARKDOWN};
use crate::parse::to_html::parse_markdown;

pub fn dump_blogs() -> () {
    for lang in fs::read_dir(MARKDOWN).unwrap() {
        let mut dump_path = PathBuf::from(JSON);
        let name = lang.unwrap().path().file_name().unwrap();
        dump_path.push(name);
        // clear out the old json
        if exists(dump_path).is_ok() {
            println!("deleting old folder: {}", dump_path.display());
            remove_dir(dump_path);
        }
        println!("reset folder: {}", dump_path.display());
        create_dir(dump_path);
        for blog in read_dir(lang.unwrap().path()) {}
    }
}

fn dump_to_json(path: &str) -> () {
    let mut file = File::open("../blog/lang/shell/background-fg.md").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let markdown = contents
        .lines()
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let json = parse_markdown(&markdown);
    let jsonified = to_string_pretty::<Vec<HTMLElement>>(&json).unwrap();
}
