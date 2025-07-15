use crate::parse::html_element::HTMLElement;

// the title will be the name of the file
// 1. '-' -> spaces
// 2. capitalised off spaces

pub fn parse_markdown(text: &Vec<String>) -> Vec<HTMLElement> {
    let answer: Vec<HTMLElement> = Vec::new();
    for line in text {
        // Header
        if line.starts_with('#') {
            todo!();
        }
        // Paragraph
        // Code
        // List
    }
    answer
}

/// ```
/// # use parser::parse::html_element::HTMLElement;
/// # use parser::parse::to_html::parse_header;
/// match parse_header(&String::from("## Rough Shell Translation")) {
///     None => assert!(false),
///     Some(html) => {
///         match html {
///             HTMLElement::Header(level, message) => assert!(level == 2 && message == String::from("Rough Shell Translation")),
///             _ => assert!(false)
///         }
///     }
/// }
/// ```
pub fn parse_header(line: &String) -> Option<HTMLElement> {
    let mut level = 0 as usize;
    let mut parsing_hash = true;
    let mut start: Option<usize> = None;
    for (idx, char) in line.chars().enumerate() {
        if parsing_hash {
            if char == '#' {
                level += 1;
            } else {
                parsing_hash = false;
            }
        } else {
            if char != ' ' {
                start = Some(idx);
                break;
            }
        }
    }
    match start {
        None => None,
        Some(idx) => Some(HTMLElement::Header(
            level,
            line.chars().skip(idx).collect::<String>(),
        )),
    }
}
