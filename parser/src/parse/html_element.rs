#[derive(Debug, PartialEq)]
pub enum HTMLElement {
    Header(usize, String),
    Code(String, Vec<String>),
    OrderedList(Vec<String>),
    UnorderedList(Vec<String>),
    Paragraph(Vec<String>),
}
