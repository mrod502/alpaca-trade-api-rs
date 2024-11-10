use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CloseAllPositionsResponse {
    pub symbol: String,
    pub status: isize,
    pub body: String, //json.RawMessage
}
