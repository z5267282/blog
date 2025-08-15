//! The `dump_blogs` function contains the main code used for the binary crate.
//! It will parse the Markdown text for all blogs in `../blog` and create a combined JSON file in `../website/src`.

use log::info;
use std::fs::{read_dir, read_to_string, File};
use std::io::Write;
use std::path::PathBuf;

use serde_json::{to_string, to_string_pretty};

use super::html_element::HTMLElement;
use super::paths::{JSON, MARKDOWN};
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
/// use parser::parse::dump_file::dump_blogs;
///
/// dump_blogs(false).expect("Failed to dump blogs");
/// ```
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

/// Parses a blog from Markdown into HTML representation.
///
/// # Arguments
/// * `path` - The path to the blog file.
///
/// # Errors
/// If there was an error reading the file from path.
///
/// # Examples
/// ```
/// use std::path::PathBuf;
/// use parser::parse::dump_file::parse_blog;
///
/// let path = PathBuf::from("my-blog-post.md");
/// let html_elements = parse_blog(&path).expect("Failed to parse blog");
/// ```
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
///
/// # Examples
/// ```
/// use std::path::PathBuf;
/// use parser::parse::dump_file::basename;
///
/// let path = PathBuf::from("my-blog-post.md");
/// let base = basename(&path).expect("Failed to extract basename");
/// assert_eq!(base, "file.md");
/// ```
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
///
/// # Examples
/// ```
/// use std::path::PathBuf;
/// use parser::parse::dump_file::get_lang_name;
///
/// let lang = PathBuf::from("/path/to/lang");
/// let name = get_lang_name(&lang).expect("Failed to get language name");
/// assert_eq!(name, "lang");
/// ```
fn get_lang_name(lang: &PathBuf) -> Result<String, std::io::Error> {
    basename(&lang)
}

/// Helper function to generate an error when the basename cannot be extracted.
///
/// # Arguments
/// * `path` - The path from which the basename could not be extracted.
///
/// # Examples
/// ```
/// use std::path::PathBuf;
/// use parser::parse::dump_file::gen_cannot_extract_basename;
///
/// let path = PathBuf::from("my-blog-post.md");
/// let error = gen_cannot_extract_basename(&path);
/// assert!(error.to_string().contains("basename could not be extracted from absolute path /path/to/file.md"));
/// ```
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
///
/// # Examples
/// ```
/// use parser::parse::dump_file::{dump_to_str, LanguageDump};
/// use serde_json::to_string_pretty;
///
/// let parsed = vec![LanguageDump {
///     title: "My Blog Post".into(),
///     content: "This is the content of my blog post.".into(),
/// }];
/// let json = dump_to_str(&parsed, true).expect("Failed to dump to JSON");
/// assert!(json.contains("My Blog Post"));
/// assert!(json.contains("This is the content of my blog post."));
/// ```
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
///
/// # Examples
/// ```
/// use std::path::PathBuf;
/// use parser::parse::dump_file::prepare_title;
///
/// let blog = PathBuf::from("my-blog-post.md");
/// let title = prepare_title(&blog).expect("Failed to prepare title");
/// assert_eq!(title, "my blog post");
/// ```
fn prepare_title(blog: &PathBuf) -> Result<String, std::io::Error> {
    info!("preparing blog title for {}", blog.display());
    let base = basename(blog)?;
    let title = base.replace('-', " ").trim_end_matches(".md").to_string();
    info!("prepared title {}", title.as_str());
    Ok(title)
}
