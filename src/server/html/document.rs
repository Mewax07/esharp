use crate::server::Html;

pub struct Document {
    lang: String,
    head: Vec<Html>,
    body: Vec<Html>,
}

impl Document {
    pub fn new() -> Self {
        Self {
            lang: "en".to_string(),
            head: Vec::new(),
            body: Vec::new(),
        }
    }

    pub fn lang(mut self, lang: impl Into<String>) -> Self {
        self.lang = lang.into();
        self
    }

    pub fn head(mut self, element: Html) -> Self {
        self.head.push(element);
        self
    }

    pub fn body(mut self, element: Html) -> Self {
        self.body.push(element);
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.head.push(Html::new("title").text_content(title));
        self
    }

    pub fn render(self) -> String {
        Html::new("html")
            .attr("lang", self.lang)
            .append(Html::new("head").children(self.head))
            .append(Html::new("body").children(self.body))
            .render()
    }
}

impl Default for Document {
    fn default() -> Self {
        Self::new()
    }
}
