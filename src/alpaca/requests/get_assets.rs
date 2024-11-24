use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GetAssets {
    pub status: String,
    pub asset_class: String,
    pub exchange: String,
}
