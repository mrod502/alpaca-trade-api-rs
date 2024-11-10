use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateWatchlist {
    pub name: String,
    pub symbols: Vec<String>,
}
