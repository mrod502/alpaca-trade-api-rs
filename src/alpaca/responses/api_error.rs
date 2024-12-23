use std::fmt::Display;

use hyper::StatusCode;

#[derive(Debug, Clone)]
pub struct APIError {
    status_code: StatusCode,

    pub message: String,
    body: String,
}

pub enum ClientError {
    APIError(APIError),
    SerdeError(String),
    NetworkError(String),
}

impl Default for APIError {
    fn default() -> Self {
        Self {
            status_code: StatusCode::BAD_REQUEST,
            message: "unknown error".into(),
            body: "".into(),
        }
    }
}

impl Display for APIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}):{}", self.status_code, self.message)
    }
}

impl APIError {
    pub fn with_status(self, code: StatusCode) -> Self {
        Self {
            status_code: code,
            body: self.body,
            message: self.message,
        }
    }

    pub fn with_message(self, message: String) -> Self {
        Self {
            status_code: self.status_code,
            message,
            body: self.body,
        }
    }
    pub fn with_body(self, body: String) -> Self {
        Self {
            status_code: self.status_code,
            message: self.message,
            body: body,
        }
    }
}
