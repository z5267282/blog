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
                    mode = CurrentElementType::Code(get_code_language(line), Vec::new());
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
/// assert_eq!(get_code_language(&String::from("```cpp")), "cpp".to_string());
/// ```
pub fn get_code_language(line: &String) -> String {
    line.chars().skip(3).collect::<String>()
}

enum CurrentElementType {
    Paragraph(Vec<String>),
    // language
    Code(String, Vec<String>),
    // current index if ordered
    List(Option<usize>, Vec<String>),
    NotSet,
}

#[cfg(test)]
mod tests {
    use crate::parse::{html_element::HTMLElement, to_html::parse_markdown};

    #[test]
    fn test_code() {
        let code: Vec<String> = vec!["```py", "print('hello mate')", "print('cya')", "```"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let result = parse_markdown(&code);
        match result {
            Err(_) => assert!(false),
            Ok(elements) => {
                assert!(elements.len() == 1);
                match elements.first() {
                    None => assert!(false),
                    Some(element) => match element {
                        HTMLElement::Code(lang, code_lines) => {
                            assert!(lang == "py");
                            let exp: Vec<String> = vec!["print('hello mate')", "print('cya')"]
                                .iter()
                                .map(|s| s.to_string())
                                .collect();
                            assert!(code_lines == &exp);
                        }
                        _ => assert!(false),
                    },
                }
            }
        }
    }
}
