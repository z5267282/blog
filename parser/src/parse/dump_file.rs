use std::fs::File;
use std::io::{Read, Result};

use serde_json::to_string_pretty;

use crate::parse::html_element::HTMLElement;
use crate::parse::to_html::parse_markdown;

pub fn dump_to_json(path: &str) -> () {
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
