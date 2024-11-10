use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct APIError {
    status_code: isize,
    pub code: isize,
    pub message: String,
    body: String,
}
