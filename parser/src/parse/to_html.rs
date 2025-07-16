use crate::parse::html_element::{
    is_code, is_header, is_ordered_list, is_unordered_list, HTMLElement,
};

// the title will be the name of the file
// 1. '-' -> spaces
// 2. capitalised off spaces

// line number, items
pub fn parse_markdown(text: &Vec<String>) -> Result<Vec<HTMLElement>, usize> {
    let mut answer: Vec<HTMLElement> = Vec::new();
    let mut element: Option<HTMLElement> = None;
    for (number, line) in text.iter().enumerate() {
        match element {
            // not parsing anything
            None => match parse_fresh(line) {
                None => return Err(number),
                Some(html) => match html {
                    HTMLElement::Header(level, contents) => {
                        answer.push(HTMLElement::Header(level, contents))
                    }
                    _ => element = Some(html),
                },
            },
            // we are in the middle of parsing something
            Some(curr) => match curr {
                HTMLElement::Header(..) => {
                    // we already parse headers when we hit them
                }
                HTMLElement::Code(lang, mut code) => {
                    // end of code block
                    if is_code(line) {
                        answer.push(HTMLElement::Code(lang, code));
                        element = None;
                    } else {
                        code.push(line.to_string());
                    }
                }
                HTMLElement::OrderedList(mut list) => match parse_ordered_list(line) {
                    None => {
                        answer.push(HTMLElement::OrderedList(list));
                        element = None;
                    }
                    Some(line) => list.push(line.to_string()),
                },
                HTMLElement::UnorderedList(mut list) => match parse_unordered_list(line) {
                    None => {
                        answer.push(HTMLElement::UnorderedList(list));
                        element = None;
                    }
                    Some(line) => list.push(line.to_string()),
                },
                HTMLElement::Paragraph(mut content) => match parse_fresh(line) {
                    // TODO: this is wrong
                    None => content.push(line),
                    Some(html) => match html {
                        HTMLElement::Header(level, contents) => {
                            answer.push(HTMLElement::Header(level, contents))
                        }
                        _ => element = Some(html),
                    }
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

pub fn parse_unordered_list(line: &String) -> Option<String> {
    match line.split_once("- ") {
        None => None,
        Some((_, rhs)) => Some(rhs.to_string()),
    }
}

pub fn parse_ordered_list(line: &String) -> Option<String> {
    // there should be a 1. 2. (i.e. '.' to end the number)
    match line.split_once('.') {
        None => None,
        Some((_, rhs)) => Some(rhs.to_string()),
    }
}

/// Try to parse the current line fresh as if there was no previous element.
fn parse_fresh(line: &String) -> Option<HTMLElement> {
    if is_header(line) {
        parse_header(line)
    } else if is_code(line) {
        Some(HTMLElement::Code(get_code_language(line), Vec::new()))
    } else if is_unordered_list(line) {
        match parse_unordered_list(line) {
            None => None,
            Some(unordered_item) => Some(HTMLElement::UnorderedList(vec![unordered_item])),
        }
    } else if is_ordered_list(line) {
        match parse_ordered_list(line) {
            None => None,
            Some(ordered_item) => Some(HTMLElement::UnorderedList(vec![ordered_item])),
        }
    } else {
        Some(HTMLElement::Paragraph(vec![line.to_string()]))
    }
}

#[cfg(test)]
mod tests {
    use crate::parse::html_element::HTMLElement;
    use crate::parse::to_html::parse_markdown;

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
