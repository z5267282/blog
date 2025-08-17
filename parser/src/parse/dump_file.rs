//! The `dump_blogs` function contains the main code used for the binary crate.
//! It will parse the Markdown text for all blogs in `../blog` and create a combined JSON file in `../website/src`.

use log::info;
use std::fs::{read_dir, read_to_string, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use serde_json::{to_string, to_string_pretty};

use super::html_element::HTMLElement;
use super::to_html::parse_markdown;

/// A structured representation of the parsed blogs, grouped by language.
#[derive(serde::Serialize)]
struct LanguageDump {
    /// Language used, e.g. Rust, Python, etc.
    language: String,
    /// Blogs written in this language.
    blogs: Vec<Blog>,
}

/// A structured representation of a blog, containing its title and parsed HTML elements.
#[derive(serde::Serialize)]
struct Blog {
    /// Title of the blog, derived from the filename.
    title: String,
    /// Parsed HTML elements from the blog's Markdown content.
    html: Vec<HTMLElement>,
}

/// Dumps all blogs from storage into a JSON file. Blogs are stored as Markdown files.
///
/// # Arguments
/// * `pretty` - If true, the JSON output will be pretty-printed.
///
/// # Errors
/// If there was an error reading blog files or writing blogs to JSON.
///
/// # Examples
/// ```
/// use std::fs::{File, create_dir};
/// use std::io::{Read, Write};
/// use tempfile::{tempdir, NamedTempFile};
///
/// use parser::parse::dump_file::dump_blogs;
///
/// let dir = tempdir().expect("could not create temporary directory");
/// let blog_folder = dir.path().join("blogs");
/// let lang = create_dir(&blog_folder).expect("could not create blogs subfolder");
/// let blog_path = &blog_folder.join("example-blog.md");
/// let mut blog = File::create(blog_path).expect("could not create temporary blog file");
/// let contents = r#"# Overview
///
/// This is a sample blog.  
/// No further content.
///
/// "#;
/// blog.write(contents.as_bytes()).expect("could not write blog contents");
/// let mut dump_file = NamedTempFile::with_suffix(".json").expect("could not create temporary blog file");
/// let dump_path = dump_file.path();
/// dump_blogs(&dir.path(), &dump_path, false).expect("failed to dump blogs");
/// let mut dump_file = File::open(dump_path).expect("could not open dumped file");
/// let mut dump_contents = String::new();
/// dump_file.read_to_string(&mut dump_contents).expect("could not read dumped file");
///
/// dbg!(&dump_contents);
///
/// assert!(&dump_contents.contains("Overview"));
/// assert!(&dump_contents.contains("This is a sample blog."));
/// assert!(&dump_contents.contains("No further content."));
/// ```
pub fn dump_blogs(
    markdown_blog_folder: &Path,
    json_dump_path: &Path,
    pretty: bool,
) -> Result<(), std::io::Error> {
    info!("commencing dump of markdown blogs to json");
    info!(
        "iterating through all languages in {}",
        markdown_blog_folder.display()
    );

    let mut parsed: Vec<LanguageDump> = vec![];
    for try_lang in read_dir(markdown_blog_folder)? {
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

    let mut file = File::create(json_dump_path)?;
    let dump = dump_to_str(&parsed, pretty)?;
    file.write(dump.as_bytes())?;
    info!("dumped file {}", json_dump_path.display());
    Ok(())
}

/// Parses a blog from Markdown into HTML representation.
///
/// # Arguments
/// * `path` - The path to the blog file.
///
/// # Errors
/// If there was an error reading the file from path.
///
/// # Examples
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

/// Extracts the basename from a path and returns it as a `String`.
///
/// # Arguments
/// * `path` - The path from which to extract the basename.
///
/// # Errors
/// If the basename could not be extracted from the path.
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

/// Gets the language name from the path by extracting the basename.
///
/// # Arguments
/// * `lang` - The path to the language directory.
///
/// # Errors
/// If the basename could not be extracted from the path.
fn get_lang_name(lang: &PathBuf) -> Result<String, std::io::Error> {
    basename(&lang)
}

/// Helper function to generate an error when the basename cannot be extracted.
///
/// # Arguments
/// * `path` - The path from which the basename could not be extracted.
///
/// # Examples
fn gen_cannot_extract_basename(path: &PathBuf) -> std::io::Error {
    std::io::Error::new(
        std::io::ErrorKind::Other,
        format!(
            "basename could not be extracted from absolute path {}",
            path.display()
        ),
    )
}

/// Dumps a list of parsed blogs into a JSON string.
///
/// # Arguments
/// * `parsed` - The parsed blogs to be dumped.
/// * `pretty` - If true, the JSON output will be pretty-printed.
///
/// # Errors
/// If there was an error serializing the parsed data to a JSON string.
fn dump_to_str(parsed: &Vec<LanguageDump>, pretty: bool) -> Result<String, std::io::Error> {
    info!("preparing to parse markdown");
    let dumper = if pretty {
        to_string_pretty::<Vec<LanguageDump>>
    } else {
        to_string
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

/// Prepares the title for a blog by extracting the basename and formatting it.
///
/// # Arguments
/// * `blog` - The path to the blog file.
///
/// # Errors
/// If there was an error extracting the basename or formatting the title.
fn prepare_title(blog: &PathBuf) -> Result<String, std::io::Error> {
    info!("preparing blog title for {}", blog.display());
    let base = basename(blog)?;
    let title = base.replace('-', " ").trim_end_matches(".md").to_string();
    info!("prepared title {}", title.as_str());
    Ok(title)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_basename_good() {
        let mut path = PathBuf::new();
        path.push("root");
        path.push("parent");
        path.push("child.md");
        match basename(&path) {
            Err(_) => assert!(false),
            Ok(base) => assert_eq!(base, "child.md"),
        };
    }

    #[test]
    fn test_lang_name_good() {
        let mut path = PathBuf::new();
        path.push("root");
        path.push("parent");
        match get_lang_name(&path) {
            Err(_) => assert!(false),
            Ok(lang) => assert_eq!(lang, "parent"),
        };
    }

    #[test]
    fn test_dump_to_str_not_pretty() {
        let parsed = vec![LanguageDump {
            language: "cpp".to_string(),
            blogs: vec![Blog {
                title: "My Blog Post".to_string(),
                html: vec![HTMLElement::Paragraph {
                    lines: vec!["This is the content of my blog post.".to_string()],
                }],
            }],
        }];
        let json = dump_to_str(&parsed, false).expect("Failed to dump to JSON");
        assert!(json.contains("My Blog Post"));
        assert!(json.contains("This is the content of my blog post."));
    }

    #[test]
    fn test_cannot_extract_basename() {
        let path = PathBuf::from("my-blog-post.md");
        let error = gen_cannot_extract_basename(&path);
        assert!(error
            .to_string()
            .contains("basename could not be extracted from absolute path my-blog-post.md"));
    }

    #[test]
    fn test_prepare_title() {
        let blog = PathBuf::from("my-blog-post.md");
        let title = prepare_title(&blog).expect("Failed to prepare title");
        assert_eq!(title, "my blog post");
    }
}
