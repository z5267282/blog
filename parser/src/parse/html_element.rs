pub enum HTMLElement {
    // level, text
    Header(usize, String),
    // prose
    Paragraph(Vec<String>),
    // language, lines
    Code(String, Vec<String>),
    // ordered, items
    List(bool, Vec<String>),
}
