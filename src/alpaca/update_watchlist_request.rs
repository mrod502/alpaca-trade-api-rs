use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UpdateWatchlistRequest {
    pub name: String,
    pub symbols: Vec<String>,
}
