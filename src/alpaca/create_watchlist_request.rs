use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateWatchlistRequest {
    pub name: String,
    pub symbols: Vec<String>,
}
