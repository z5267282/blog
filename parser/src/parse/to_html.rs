use crate::parse::html_element::HTMLElement;

enum Region {
    NotSet,
    Code(String, Vec<String>),
    OrderedList(Vec<String>),
    UnorderedList(Vec<String>),
    Paragraph(Vec<String>),
}

// the title will be the name of the file
// 1. '-' -> spaces
// 2. capitalised off spaces

/// Return an error with the line number and a diagnostic message if one occurred
pub fn parse_markdown(text: &Vec<String>) -> Vec<HTMLElement> {
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
                else if line.starts_with("```") {
                    let lang = line.trim_start_matches("```").trim().to_string();
                    region = Region::Code(lang, Vec::new());
                }
                // ordered list
                else if line.starts_with(char::is_numeric) {
                    match line.split_once('.') {
                        // list item
                        Some((_, rhs)) => {
                            region = Region::OrderedList(vec![rhs.trim_start().to_string()])
                        }
                        // normal text
                        None => region = Region::Paragraph(vec![line.to_string()]),
                    }
                }
                // unordered list
                else if line.starts_with("- ") {
                    region = Region::UnorderedList(vec![line.trim_start_matches("- ").to_string()]);
                }
                // paragraph
                else {
                    region = Region::Paragraph(vec![line.to_string()])
                }
            }
            Region::Code(lang, ref lines) => {
                if line.starts_with("```") {
                    elements.push(HTMLElement::Code(lang, lines.clone()));
                    region = Region::NotSet;
                } else {
                    let mut updated_lines = lines.clone();
                    updated_lines.push(line.to_string());
                    region = Region::Code(lang, updated_lines);
                }
            }
            Region::OrderedList(ref list) => {
                if line.starts_with(char::is_numeric) {
                    match line.split_once('.') {
                        // list item
                        Some((_, rhs)) => {
                            let mut updated_list = list.clone();
                            updated_list.push(rhs.trim_start().to_string());
                            region = Region::OrderedList(updated_list);
                        }
                        // end of list
                        None => {
                            elements.push(HTMLElement::OrderedList(list.clone()));
                            region = Region::NotSet;
                        }
                    }
                }
                // assume that there is a blank line to separate the end of the list
                else {
                    elements.push(HTMLElement::OrderedList(list.clone()));
                    region = Region::NotSet;
                }
            }
            Region::UnorderedList(ref list) => {
                let no_leading_dash = line.trim_start_matches("- ");
                // end of list
                if no_leading_dash.len() == line.len() {
                    elements.push(HTMLElement::UnorderedList(list.clone()));
                    region = Region::NotSet;
                } else {
                    let mut updated_list = list.clone();
                    updated_list.push(no_leading_dash.to_string());
                    region = Region::UnorderedList(updated_list);
                }
            }
            Region::Paragraph(lines) => {
                // end of paragraph and beginning of a new element
                if line.is_empty() {
                    elements.push(HTMLElement::Paragraph(lines));
                    region = Region::NotSet;
                } else {
                    let mut updated_lines = lines.clone();
                    // remove trailing "  " for forced line breaks
                    updated_lines.push(line.trim().to_string());
                    region = Region::Paragraph(updated_lines);
                }
            }
        }
    }

    match region {
        Region::Code(lang, code) => elements.push(HTMLElement::Code(lang, code)),
        Region::OrderedList(list) => elements.push(HTMLElement::OrderedList(list)),
        Region::UnorderedList(list) => elements.push(HTMLElement::UnorderedList(list)),
        Region::Paragraph(lines) => elements.push(HTMLElement::Paragraph(lines)),
        _ => {}
    }
    elements
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
        let exp_code = vec!["print('hello mate')", "print('cya')"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(
            parse_markdown(&code),
            vec![HTMLElement::Code("py".to_string(), exp_code)]
        );
    }
}
