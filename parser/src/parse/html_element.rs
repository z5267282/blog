//! A dataclass to store parsed Markdown, ready for conversion into a structured JSON format.

/// A structured representation of HTML elements parsed from Markdown.
/// This enum captures various HTML elements that can be generated from Markdown content.
#[derive(Debug, PartialEq, serde::Serialize)]
#[serde(tag = "type")]
pub enum HTMLElement {
    /// Header blocks
    Header { level: usize, content: String },
    /// Code snippets
    Code { language: String, code: Vec<String> },
    /// Ordered lists
    OrderedList { list: Vec<String> },
    /// Unordered lists
    UnorderedList { list: Vec<String> },
    /// HTML tables - the length of headers and rows must match
    Table { headers: Vec<String>, rows: Vec<Vec<String>> },
    /// Paragraph text
    Paragraph { lines: Vec<String> },
}
