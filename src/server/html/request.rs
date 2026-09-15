use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Head,
    Options,
    Unknown,
}

impl Method {
    pub fn parse(value: &str) -> Self {
        match value {
            "GET" => Self::Get,
            "POST" => Self::Post,
            "PUT" => Self::Put,
            "DELETE" => Self::Delete,
            "PATCH" => Self::Patch,
            "HEAD" => Self::Head,
            "OPTIONS" => Self::Options,
            _ => Self::Unknown,
        }
    }
}

pub struct Request {
    pub method: Method,
    pub path: String,
    pub query: HashMap<String, String>,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Request {
    pub fn parse(raw: &str) -> Option<Self> {
        let (header, body) = match raw.split_once("\r\n\r\n") {
            Some(value) => value,
            None => (raw, ""),
        };

        let mut lines = header.lines();

        let request_line = lines.next()?;
        let mut parts = request_line.split_whitespace();

        let method = Method::parse(parts.next()?);
        let target = parts.next()?;

        let (path, query) = parse_target(target);

        let mut headers = HashMap::new();

        for line in lines {
            if let Some((name, value)) = line.split_once(':') {
                headers.insert(name.trim().to_ascii_lowercase(), value.trim().to_string());
            }
        }

        Some(Self {
            method,
            path,
            query,
            headers,
            body: body.to_string(),
        })
    }
}

fn parse_target(target: &str) -> (String, HashMap<String, String>) {
    let Some((path, query)) = target.split_once('?') else {
        return (target.to_string(), HashMap::new());
    };

    let mut params = HashMap::new();

    for pair in query.split('&') {
        if let Some((key, value)) = pair.split_once('=') {
            params.insert(url_decode(key), url_decode(value));
        }
    }

    (path.to_string(), params)
}

fn url_decode(value: &str) -> String {
    let mut output = String::new();
    let bytes = value.as_bytes();

    let mut i = 0;

    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let Ok(hex) = u8::from_str_radix(&value[i + 1..i + 3], 16) {
                output.push(hex as char);
                i += 3;
                continue;
            }
        }

        if bytes[i] == b'+' {
            output.push(' ');
        } else {
            output.push(bytes[i] as char);
        }

        i += 1;
    }

    output
}
