use log::info;
use std::fs::{create_dir, exists, read_dir, read_to_string, remove_dir_all, File};
use std::io::Write;
use std::path::PathBuf;

use serde_json::to_string_pretty;

use crate::parse::html_element::HTMLElement;
use crate::parse::paths::{JSON, MARKDOWN};
use crate::parse::to_html::parse_markdown;

pub fn dump_blogs() -> Result<(), std::io::Error> {
    info!("commencing dump of markdown blogs to json");
    info!("iterating through all languages in {}", MARKDOWN);
    for try_lang in read_dir(MARKDOWN)? {
        let lang = try_lang?.path();
        if !lang.is_dir() {
            continue;
        }

        let dump_folder = join_dump_folder_path(&lang)?;
        reset_dump_folder(&dump_folder)?;

        for blog in read_dir(&lang)? {
            let entry = blog?.path();
            let contents = dump_to_str(&entry)?;

            let json_name = prepare_json_filename_from_markdown_path(&entry)?;
            let mut dump_path = PathBuf::from(&dump_folder);
            dump_path.push(json_name);

            let mut file = File::create(&dump_path)?;
            file.write(contents.as_bytes())?;
            info!("dumped file {}", dump_path.display());
        }
    }
    Ok(())
}

fn join_dump_folder_path(md_path: &PathBuf) -> Result<PathBuf, std::io::Error> {
    info!("handling language {}", md_path.display());
    let language = md_path.file_name().ok_or(std::io::Error::new(
        std::io::ErrorKind::Other,
        format!(
            "basename could not be extracted from absolute path {}",
            md_path.display()
        ),
    ))?;

    // first get the dump path (i.e. where the JSON is written to)
    let mut result = PathBuf::from(JSON);
    result.push(language);
    Ok(result)
}

/// Clear out the dump folder by deleting if necessary and then re-creating it.
fn reset_dump_folder(path: &PathBuf) -> Result<(), std::io::Error> {
    info!("resetting folder {}", path.display());
    if let Ok(dir_exists) = exists(&path) {
        if dir_exists {
            remove_dir_all(&path)?;
            info!("folder was deleted {}", path.display());
        }
    }
    create_dir(&path)?;
    info!("folder was re-created {}", path.display());
    Ok(())
}

fn prepare_json_filename_from_markdown_path(
    markdown_path: &PathBuf,
) -> Result<String, std::io::Error> {
    let json_name = markdown_path
        // get the base filename from the full absolute path
        .file_name()
        .ok_or(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!(
                "basename could not be extracted from absolute path {}",
                markdown_path.display()
            ),
        ))?
        .to_str()
        .ok_or(std::io::Error::new(
            std::io::ErrorKind::Other,
            "could not convert file name string into UTF-8 for path {}",
        ))?
        .trim_end_matches(".md")
        .to_string()
        + ".json";
    Ok(json_name)
}

/// Read the markdown from a file and then return the dumped JSON string of its parsed contents
fn dump_to_str(path: &PathBuf) -> Result<String, std::io::Error> {
    info!("loading markdown from {}", path.display());
    let markdown = read_to_string(path)?
        .lines()
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    info!("markdown loaded, preparing to parse");
    let json = parse_markdown(&markdown);

    info!("markdown parsed");
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
