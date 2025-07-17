use std::fs::File;
use std::io::{Read, Result};

use parser::parse::html_element::HTMLElement;
use parser::parse::to_html::parse_markdown;
use serde_json::to_string_pretty;

fn main() -> Result<()> {
    let mut file = File::open("../blog/lang/shell/background-fg.md")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let markdown = contents
        .lines()
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let json = parse_markdown(&markdown);
    let jsonified = to_string_pretty::<Vec<HTMLElement>>(&json)?;
    println!("{}", jsonified);
    Result::Ok(())
}
