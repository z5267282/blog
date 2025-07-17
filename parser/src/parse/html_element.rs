use serde::{ser::SerializeTuple, Serialize, Serializer};

#[derive(Debug, PartialEq)]
pub enum HTMLElement {
    Header(usize, String),
    Code(String, Vec<String>),
    OrderedList(Vec<String>),
    UnorderedList(Vec<String>),
    Paragraph(Vec<String>),
}

impl Serialize for HTMLElement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        todo!("implement for all variants")
    }
}
