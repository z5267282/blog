//! A dataclass to store parsed Markdown, ready for conversion into a structured JSON format.

#[derive(Debug, PartialEq, serde::Serialize)]
#[serde(tag = "type")]
pub enum HTMLElement {
    Header { level: usize, content: String },
    Code { language: String, code: Vec<String> },
    OrderedList { list: Vec<String> },
    UnorderedList { list: Vec<String> },
    Paragraph { lines: Vec<String> },
}
