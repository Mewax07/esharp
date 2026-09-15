use std::fmt::{self, Write};

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

    pub fn text(value: impl Into<String>) -> Self {
        Self {
            tag: String::new(),
            attributes: Vec::new(),
            children: Vec::new(),
            text: Some(value.into()),
            self_closing: false,
        }
    }

    pub fn append(mut self, child: Html) -> Self {
        self.children.push(child);
        self
    }

    pub fn children<I>(mut self, children: I) -> Self
    where
        I: IntoIterator<Item = Html>,
    {
        self.children.extend(children);
        self
    }

    pub fn attr(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.attributes.push((name.into(), value.into()));
        self
    }

    pub fn id(self, value: impl Into<String>) -> Self {
        self.attr("id", value)
    }

    pub fn class(self, value: impl Into<String>) -> Self {
        self.attr("class", value)
    }

    pub fn style(self, value: impl Into<String>) -> Self {
        self.attr("style", value)
    }

    pub fn action(self, value: impl Into<String>) -> Self {
        self.attr("action", value)
    }

    pub fn method(self, value: impl Into<String>) -> Self {
        self.attr("method", value)
    }

    pub fn href(self, value: impl Into<String>) -> Self {
        self.attr("href", value)
    }

    pub fn src(self, value: impl Into<String>) -> Self {
        self.attr("src", value)
    }

    pub fn value(self, value: impl Into<String>) -> Self {
        self.attr("value", value)
    }

    pub fn name(self, value: impl Into<String>) -> Self {
        self.attr("name", value)
    }

    pub fn placeholder(self, value: impl Into<String>) -> Self {
        self.attr("placeholder", value)
    }

    pub fn r#type(self, value: impl Into<String>) -> Self {
        self.attr("type", value)
    }

    pub fn onclick(self, value: impl Into<String>) -> Self {
        self.attr("onclick", value)
    }

    pub fn text_content(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn self_closing(mut self) -> Self {
        self.self_closing = true;
        self
    }

    pub fn render(&self) -> String {
        let mut output = String::new();
        self.write_to(&mut output).unwrap();
        output
    }

    fn write_to(&self, output: &mut String) -> fmt::Result {
        if self.tag.is_empty() {
            if let Some(text) = &self.text {
                output.push_str(&escape_html(text));
            }
            return Ok(());
        }

        write!(output, "<{}", self.tag)?;

        for (name, value) in &self.attributes {
            write!(output, " {}=\"{}\"", name, escape_attribute(value))?;
        }

        if self.self_closing {
            output.push_str(" />");
            return Ok(());
        }

        output.push('>');

        if let Some(text) = &self.text {
            output.push_str(&escape_html(text));
        }

        for child in &self.children {
            child.write_to(output)?;
        }

        write!(output, "</{}>", self.tag)
    }
}

fn escape_html(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn escape_attribute(value: &str) -> String {
    escape_html(value)
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
