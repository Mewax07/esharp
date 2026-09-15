#[derive(Debug, Clone, Copy)]
pub enum StatusCode {
    Ok,
    BadRequest,
    NotFound,
    MethodNotAllowed,
    InternalServerError,
}

impl StatusCode {
    pub fn code(self) -> u16 {
        match self {
            Self::Ok => 200,
            Self::BadRequest => 400,
            Self::NotFound => 404,
            Self::MethodNotAllowed => 405,
            Self::InternalServerError => 500,
        }
    }

    pub fn reason(self) -> &'static str {
        match self {
            Self::Ok => "OK",
            Self::BadRequest => "Bad Request",
            Self::NotFound => "Not Found",
            Self::MethodNotAllowed => "Method Not Allowed",
            Self::InternalServerError => "Internal Server Error",
        }
    }
}
