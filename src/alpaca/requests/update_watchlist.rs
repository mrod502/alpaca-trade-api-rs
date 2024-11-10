use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UpdateWatchlist {
    pub name: String,
    pub symbols: Vec<String>,
}
