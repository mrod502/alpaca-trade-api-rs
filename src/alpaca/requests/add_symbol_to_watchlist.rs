use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AddSymbolToWatchlist {
    pub symbol: String,
}
