use crate::parse::html_element::HTMLElement;

enum Region {
    NotSet,
    Code(String, Vec<String>),
    OrderedList(Vec<String>),
    UnorderedList(Vec<String>),
    Paragraph(Vec<String>),
}

pub type Error = (usize, String);

// the title will be the name of the file
// 1. '-' -> spaces
// 2. capitalised off spaces

/// Return an error with the line number and a diagnostic message if one occurred
pub fn parse_markdown(text: &Vec<String>) -> Result<Vec<HTMLElement>, Error> {
    let mut region = Region::NotSet;
    let mut elements: Vec<HTMLElement> = Vec::new();

    for line in text {
        match region {
            Region::NotSet => {
                // header
                if line.starts_with('#') {
                    let content = line.trim_start_matches('#').trim_start().to_string();
                    let level = line.find(|c| c != '#').unwrap_or(0);
                    elements.push(HTMLElement::Header(level, content));
                }
                // code
                if line.starts_with("```") {
                    let lang = line.trim_start_matches("```").trim().to_string();
                    region = Region::Code(lang, Vec::new());
                }
                // ordered list
                if line.starts_with(char::is_numeric) {}
            }
            _ => todo!("other code elements"),
        }
    }

    todo!("handle last element");
    Ok(elements)
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
