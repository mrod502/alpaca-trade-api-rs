use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RemoveSymbolFromWatchlistRequest {
    pub symbol: String,
}
