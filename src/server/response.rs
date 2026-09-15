use std::collections::HashMap;

use crate::server::StatusCode;

pub struct Response {
    pub status: StatusCode,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl Response {
    pub fn new(status: StatusCode) -> Self {
        Self {
            status,
            headers: HashMap::new(),
            body: Vec::new(),
        }
    }

    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(name.into(), value.into());
        self
    }

    pub fn body(mut self, body: impl Into<Vec<u8>>) -> Self {
        self.body = body.into();
        self
    }

    pub fn status(mut self, status: StatusCode) -> Self {
        self.status = status;
        self
    }

    pub fn html(html: impl Into<String>) -> Self {
        let html = html.into();

        Self::new(StatusCode::Ok)
            .header("Content-Type", "text/html; charset=utf-8")
            .body(html.into_bytes())
    }

    pub fn json(json: impl Into<String>) -> Self {
        let json = json.into();

        Self::new(StatusCode::Ok)
            .header("Content-Type", "application/json; charset=utf-8")
            .body(json.into_bytes())
    }

    pub fn allow(mut self, methods: impl Into<String>) -> Self {
        self.headers.insert("Allow".into(), methods.into());
        self
    }

    pub fn not_found() -> Self {
        let html = "<h1>404 Not Found</h1>";

        Self::new(StatusCode::NotFound)
            .header("Content-Type", "text/html; charset=utf-8")
            .body(html.as_bytes().to_vec())
    }

    pub fn method_not_allowed() -> Self {
        Self::new(StatusCode::MethodNotAllowed)
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        let message = message.into();

        Self::new(StatusCode::BadRequest)
            .header("Content-Type", "text/plain; charset=utf-8")
            .body(message.into_bytes())
    }

    pub fn to_bytes(self) -> Vec<u8> {
        let mut response = format!(
            "HTTP/1.1 {} {}\r\n",
            self.status.code(),
            self.status.reason()
        );

        response.push_str(&format!("Content-Length: {}\r\n", self.body.len()));

        response.push_str("Connection: close\r\n");

        for (name, value) in self.headers {
            response.push_str(&format!("{}: {}\r\n", name, value));
        }

        response.push_str("\r\n");

        let mut bytes = response.into_bytes();
        bytes.extend(self.body);

        bytes
    }
}
