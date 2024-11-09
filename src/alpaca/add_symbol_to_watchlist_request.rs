use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AddSymbolToWatchlistRequest {
    pub symbol: String,
}
