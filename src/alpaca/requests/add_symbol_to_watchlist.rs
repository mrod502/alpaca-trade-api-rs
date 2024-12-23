use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct AddSymbolToWatchlist {
    pub symbol: String,
}
