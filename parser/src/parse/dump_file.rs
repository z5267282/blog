//! The `dump_blogs` function contains the main code used for the binary crate.  
//! It will parse the Markdown text for all blogs in `../blog` and create a combined JSON file in `../website/src`.

use log::info;
use std::fs::{read_dir, read_to_string, File};
use std::io::Write;
use std::path::PathBuf;

use serde_json::{to_string, to_string_pretty};

use crate::parse::html_element::HTMLElement;
use crate::parse::paths::{JSON, MARKDOWN};
use crate::parse::to_html::parse_markdown;

pub fn dump_blogs(pretty: bool) -> Result<(), std::io::Error> {
    info!("commencing dump of markdown blogs to json");
    info!("iterating through all languages in {}", MARKDOWN);

    let mut parsed: Vec<LanguageDump> = vec![];
    for try_lang in read_dir(MARKDOWN)? {
        let lang = try_lang?.path();
        if !lang.is_dir() {
            continue;
        }

        let mut language = LanguageDump {
            language: get_lang_name(&lang)?,
            blogs: Vec::new(),
        };

        for entry in read_dir(&lang)? {
            let blog = entry?.path();
            let html = parse_blog(&blog)?;
            let title = prepare_title(&blog)?;
            language.blogs.push(Blog { title, html });
        }

        parsed.push(language);
    }

    let mut file = File::create(JSON)?;
    let dump = dump_to_str(&parsed, pretty)?;
    file.write(dump.as_bytes())?;
    info!("dumped file {}", JSON);
    Ok(())
}

#[derive(serde::Serialize)]
struct LanguageDump {
    language: String,
    blogs: Vec<Blog>,
}

#[derive(serde::Serialize)]
struct Blog {
    title: String,
    html: Vec<HTMLElement>,
}

fn parse_blog(path: &PathBuf) -> Result<Vec<HTMLElement>, std::io::Error> {
    info!("loading markdown from {}", path.display());
    let markdown = read_to_string(path)?
        .lines()
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    info!("markdown loaded, preparing to parse");
    let json = parse_markdown(&markdown);
    info!("parsed json successfully from {}", path.display());
    Ok(json)
}

fn basename(path: &PathBuf) -> Result<String, std::io::Error> {
    info!("trying to extract basename for {}", path.display());
    let basename = path
        .file_name()
        .ok_or(gen_cannot_extract_basename(path))?
        .to_str()
        .ok_or(gen_cannot_extract_basename(path))?
        .to_string();
    info!("extracted basename {}", basename.as_str());
    Ok(String::from(basename))
}

fn get_lang_name(lang: &PathBuf) -> Result<String, std::io::Error> {
    basename(&lang)
}

fn gen_cannot_extract_basename(path: &PathBuf) -> std::io::Error {
    std::io::Error::new(
        std::io::ErrorKind::Other,
        format!(
            "basename could not be extracted from absolute path {}",
            path.display()
        ),
    )
}

fn dump_to_str(parsed: &Vec<LanguageDump>, pretty: bool) -> Result<String, std::io::Error> {
    info!("preparing to parse markdown");
    let dumper = match pretty {
        true => to_string_pretty::<Vec<LanguageDump>>,
        false => to_string,
    };
    let dumped = dumper(&parsed).map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!(
                "Serde error occurred when serialising parsed data: {}",
                e.to_string()
            ),
        )
    })?;
    info!("markdown parsed");
    Ok(dumped)
}

fn prepare_title(blog: &PathBuf) -> Result<String, std::io::Error> {
    info!("preparing blog title for {}", blog.display());
    let base = basename(blog)?;
    let title = base.replace('-', " ").trim_end_matches(".md").to_string();
    info!("prepared title {}", title.as_str());
    Ok(title)
}
