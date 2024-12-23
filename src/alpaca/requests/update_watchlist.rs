use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct UpdateWatchlist {
    pub name: String,
    pub symbols: Vec<String>,
}
