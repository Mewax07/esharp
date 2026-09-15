pub mod page;

pub use page::*;

#[derive(Debug, Clone)]
pub struct Html {
    tag: String,
    attributes: Vec<(String, String)>,
    children: Vec<Html>,
    text: Option<String>,
    self_closing: bool,
}

impl Html {
    pub fn new(tag: impl Into<String>) -> Self {
        Self {
            tag: tag.into(),
            attributes: Vec::new(),
            children: Vec::new(),
            text: None,
            self_closing: false,
        }
    }
}
