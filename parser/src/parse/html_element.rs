pub enum HTMLElement {
    Header(usize, String),
    Code(String, Vec<String>),
    OrderedList(Vec<String>),
    UnorderedList(Vec<String>),
    Paragraph(Vec<String>),
}

pub fn is_header(line: &String) -> bool {
    line.starts_with('#')
}

pub fn is_code(line: &String) -> bool {
    line.starts_with("```")
}

pub fn is_unordered_list(line: &String) -> bool {
    line.starts_with("- ")
}

pub fn is_ordered_list(line: &String) -> bool {
    line.starts_with(|c| c >= '0' && c <= '9')
}
