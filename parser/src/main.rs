enum HTMLElement {
    Title(String),
    // level, text
    Header(usize, String),
    // prose
    Paragraph(Vec<String>),
    // language, lines
    Code(String, Vec<String>),
}

fn parse_title(line: &String) -> Option<(usize, String)> {
    for curr in line.char_indices() {
        match curr {
            None => break,
            Some(idx, letter) => {
                match letter {
                    '#' => continue,
                    _ => break,
                }
            }
        }
    }
    todo!()
}

fn parse_markdown(text: &Vec<String>) -> Vec<HTMLElement> {
    let mut answer: Vec<HTMLElement> = Vec::new();
    for line in text {
        if (line.fi)
    }
    answer
}

fn main() {
    println!("Hello, world!");
}
