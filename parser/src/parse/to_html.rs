use crate::parse::html_element::HTMLElement;

// the title will be the name of the file
// 1. '-' -> spaces
// 2. capitalised off spaces

// line number, items
pub fn parse_markdown(text: &Vec<String>) -> Result<Vec<HTMLElement>, usize> {
    let mut answer: Vec<HTMLElement> = Vec::new();
    let mut mode = CurrentElementType::NotSet;
    for (number, line) in text.iter().enumerate() {
        match mode {
            CurrentElementType::Paragraph(lines) => todo!(),
            CurrentElementType::Code(language, mut code) => {
                if line.starts_with("```") {
                    answer.push(HTMLElement::Code(language, code));
                    mode = CurrentElementType::NotSet;
                } else {
                    code.push(line.to_string());
                    mode = CurrentElementType::Code(language, code);
                }
            }
            CurrentElementType::List(number, items) => todo!(),
            CurrentElementType::NotSet => {
                // Header
                if line.starts_with('#') {
                    match parse_header(line) {
                        Some(header) => answer.push(header),
                        None => return Err(number),
                    }
                }
                // Paragraph
                // Code
                if line.starts_with("```") {
                    match get_code_language(line) {
                        Some(lang) => mode = CurrentElementType::Code(lang, Vec::new()),
                        None => return Err(number),
                    }
                }
                // List
            }
        }
    }
    Ok(answer)
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

/// ```
/// # use parser::parse::html_element::HTMLElement;
/// # use parser::parse::to_html::get_code_language;
///
/// assert_eq!(get_code_language(&String::from("``` cpp")), Some(String::from("cpp")));
/// ```
pub fn get_code_language(line: &String) -> Option<String> {
    match line.split_once(' ') {
        None => None,
        Some((_, lang)) => Some(String::from(lang.trim())),
    }
}

enum CurrentElementType {
    Paragraph(Vec<String>),
    // language
    Code(String, Vec<String>),
    // current index if ordered
    List(Option<usize>, Vec<String>),
    NotSet,
}
