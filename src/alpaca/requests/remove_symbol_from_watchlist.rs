use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RemoveSymbolFromWatchlist {
    pub symbol: String,
}
