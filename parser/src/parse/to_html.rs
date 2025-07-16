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
            Region::Code(lang, mut lines) => {
                if line.starts_with("```") {
                    elements.push(HTMLElement::Code(lang, lines));
                    region = Region::NotSet;
                } else {
                    lines.push(line.to_string());
                    region = Region::Code(lang, lines);
                }
            }
            // Region::OrderedList(mut list) => {
            //     if line.starts_with(char::is_numeric) {
            //         match line.split_once('.') {
            //             // list item
            //             Some((_, rhs)) => {
            //                 list.push(rhs.trim_start().to_string());
            //                 region = Region::OrderedList(list);
            //             }
            //             // end of list
            //             None => region = Region::NotSet,
            //         }
            //     }
            // }
            _ => todo!("other code elements"),
        }
    }

    todo!("handle last element");
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
