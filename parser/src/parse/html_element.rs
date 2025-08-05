#[derive(Debug, PartialEq, serde::Serialize)]
#[serde(tag = "type")]
pub enum HTMLElement {
    Header { level: usize, content: String },
    Code { language: String, code: Vec<String> },
    OrderedList { list: Vec<String> },
    UnorderedList { list: Vec<String> },
    Table { headers: Vec<String>, rows: Vec<Vec<String>> },
    Paragraph { lines: Vec<String> },
}
