//! This module provides a parser for converting Markdown text into HTML elements.

use crate::parse::html_element::HTMLElement;

/// Represents the current parsing region.
enum Region {
    /// Flag indicating that no region is currently being parsed.
    NotSet,
    /// Represents a code block with an optional language.
    Code(String, Vec<String>),
    /// Represents an ordered list.
    OrderedList(Vec<String>),
    /// Represents an unordered list.
    UnorderedList(Vec<String>),
    /// Represents a HTML table.
    Table(Vec<String>, Vec<Vec<String>>, bool),
    /// Represents a paragraph block.
    Paragraph(Vec<String>),
}

/// Returns a list of HTML elements parsed from the provided Markdown text.
/// Parsed paragraph lines have their leading and trailing whitespace stripped.
///
/// # Arguments
/// * `text` - A vector of strings containing Markdown text.
///
/// # Examples
/// ```
/// use parser::parse::to_html::parse_markdown;
///
/// let markdown = vec![
///     "# Header".to_string(),
///     "This is a paragraph.".to_string(),
/// ];
/// let elements = parse_markdown(&markdown);
/// assert_eq!(elements.len(), 2);
/// assert!(matches!(elements[0], HTMLElement::Header { level: 1, content: _ }));
/// assert!(matches!(elements[1], HTMLElement::Paragraph { lines: _ }));
/// ```
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

/// Unit tests for the Markdown parser.
/// These tests cover various Markdown elements such as headers, paragraphs, code blocks, lists,
/// and tables.
#[cfg(test)]
mod tests {
    use crate::parse::html_element::HTMLElement;
    use crate::parse::to_html::parse_markdown;

    /// Test for a simple paragraph of the py language.
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

    /// Test for a simple header and paragraph.
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

    /// Test for a simple table.
    #[test]
    fn test_table_unit() {
        let table = vec![
            "| Letter | Description           |".to_string(),
            "| ------ | --------------------- |".to_string(),
            "| c      | make new window       |".to_string(),
            "| &      | kill current window   |".to_string(),
            "| 1..9   | go to window 1..9     |".to_string(),
            "| ,      | rename window         |".to_string(),
            "| p      | go to previous window |".to_string(),
            "| n      | go to next window     |".to_string(),
        ];
        let exp_headers = vec!["Letter".to_string(), "Description".to_string()];
        let exp_rows = vec![
            vec!["c".to_string(), "make new window".to_string()],
            vec!["&".to_string(), "kill current window".to_string()],
            vec!["1..9".to_string(), "go to window 1..9".to_string()],
            vec![",".to_string(), "rename window".to_string()],
            vec!["p".to_string(), "go to previous window".to_string()],
            vec!["n".to_string(), "go to next window".to_string()],
        ];
        assert_eq!(
            parse_markdown(&table),
            vec![HTMLElement::Table {
                headers: exp_headers,
                rows: exp_rows
            }]
        );
    }
}
