use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct CreateWatchlist {
    pub name: String,
    pub symbols: Vec<String>,
}
