use crate::parse::html_element::HTMLElement;

enum Region {
    NotSet,
    Code(String, Vec<String>),
    OrderedList(Vec<String>),
    UnorderedList(Vec<String>),
    Table(Vec<String>, Vec<Vec<String>>, bool),
    Paragraph(Vec<String>),
}

/// Return an error with the line number and a diagnostic message if one occurred.
/// Parsed paragraph lines have their leading and trailing whitespace stripped.
pub fn parse_markdown(text: &Vec<String>) -> Vec<HTMLElement> {
    let mut region = Region::NotSet;
    let mut elements: Vec<HTMLElement> = Vec::new();

    for line in text {
        // end the current region
        if line.is_empty() {
            match region {
                // special elements code and table have their own syntax for detecting the end of a region
                Region::NotSet | Region::Code(..) | Region::Table(..) => continue,
                Region::OrderedList(list) => elements.push(HTMLElement::OrderedList { list }),
                Region::UnorderedList(list) => elements.push(HTMLElement::UnorderedList { list }),
                Region::Paragraph(lines) => elements.push(HTMLElement::Paragraph { lines }),
            };
            region = Region::NotSet;
            continue;
        }

        match region {
            Region::NotSet => {
                // header
                if line.starts_with('#') {
                    let content = line.trim_start_matches('#').trim_start().to_string();
                    let level = line.find(|c| c != '#').unwrap_or(0);
                    elements.push(HTMLElement::Header { level, content });
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
                // table
                else if line.starts_with("|") {
                    let mut headers = Vec::new();

                    for cell in line.trim_matches('|').split('|') {
                        headers.push(cell.trim().to_string());
                    }

                    region = Region::Table(headers, Vec::new(), true);
                }
                // paragraph
                else {
                    region = Region::Paragraph(vec![line.trim().to_string()])
                }
            }
            Region::Code(lang, mut lines) => {
                if line.starts_with("```") {
                    elements.push(HTMLElement::Code {
                        language: lang,
                        code: lines,
                    });
                    region = Region::NotSet;
                } else {
                    lines.push(line.to_string());
                    region = Region::Code(lang, lines);
                }
            }
            Region::OrderedList(mut list) => {
                if line.starts_with(char::is_numeric) {
                    match line.split_once('.') {
                        // list item
                        Some((_, rhs)) => {
                            list.push(rhs.trim_start().to_string());
                            region = Region::OrderedList(list);
                        }
                        // end of list
                        None => {
                            elements.push(HTMLElement::OrderedList { list });
                            region = Region::NotSet;
                        }
                    }
                }
                // assume that there is a blank line to separate the end of the list
                else {
                    elements.push(HTMLElement::OrderedList { list });
                    region = Region::NotSet;
                }
            }
            Region::UnorderedList(mut list) => {
                let no_leading_dash = line.trim_start_matches("- ");
                // end of list
                if no_leading_dash.len() == line.len() {
                    elements.push(HTMLElement::UnorderedList { list });
                    region = Region::NotSet;
                } else {
                    list.push(no_leading_dash.to_string());
                    region = Region::UnorderedList(list);
                }
            }
            Region::Paragraph(mut lines) => {
                // remove trailing "  " for forced line breaks
                lines.push(line.trim().to_string());
                region = Region::Paragraph(lines);
            }
            Region::Table(headers, mut rows, is_separator) => {
                if is_separator {
                    region = Region::Table(headers, rows, false);
                } else if line.starts_with("|") {
                    let mut next_row = Vec::new();
                    for cell in line.trim_matches('|').split('|') {
                        next_row.push(cell.trim().to_string());
                    }
                    rows.push(next_row);
                    region = Region::Table(headers, rows, false);
                } else {
                    elements.push(HTMLElement::Table { headers, rows });
                    region = Region::NotSet;
                }
            }
        }
    }

    match region {
        Region::NotSet => {}
        Region::Code(lang, code) => elements.push(HTMLElement::Code {
            language: lang,
            code,
        }),
        Region::OrderedList(list) => elements.push(HTMLElement::OrderedList { list }),
        Region::UnorderedList(list) => elements.push(HTMLElement::UnorderedList { list }),
        Region::Table(headers, rows, _) => elements.push(HTMLElement::Table { headers, rows }),
        Region::Paragraph(lines) => elements.push(HTMLElement::Paragraph { lines }),
    }
    elements
}

#[cfg(test)]
mod tests {
    use crate::parse::html_element::HTMLElement;
    use crate::parse::to_html::parse_markdown;

    #[test]
    fn test_code() {
        let code = vec![
            "```py".to_string(),
            "print('hello mate')".to_string(),
            "print('cya')".to_string(),
            "```".to_string(),
        ];
        assert_eq!(
            parse_markdown(&code),
            vec![HTMLElement::Code {
                language: "py".to_string(),
                code: vec![
                    "print('hello mate')".to_string(),
                    "print('cya')".to_string()
                ]
            }]
        );
    }

    #[test]
    fn test_header_and_text() {
        let text = vec![
            "# Overview".to_string(),
            "".to_string(),
            "This blog contains some information.  ".to_string(),
            "The information will be explained below.  ".to_string(),
            "".to_string(),
            "## Details".to_string(),
            "".to_string(),
            "There are seven countries in the G7.  ".to_string(),
            "Japan is a part of the G7.  ".to_string(),
            "".to_string(),
        ];
        let parsed = parse_markdown(&text);
        let exp = vec![
            HTMLElement::Header {
                level: 1,
                content: "Overview".to_string(),
            },
            HTMLElement::Paragraph {
                lines: vec![
                    "This blog contains some information.".to_string(),
                    "The information will be explained below.".to_string(),
                ],
            },
            HTMLElement::Header {
                level: 2,
                content: "Details".to_string(),
            },
            HTMLElement::Paragraph {
                lines: vec![
                    "There are seven countries in the G7.".to_string(),
                    "Japan is a part of the G7.".to_string(),
                ],
            },
        ];
        assert_eq!(parsed, exp);
    }
}
