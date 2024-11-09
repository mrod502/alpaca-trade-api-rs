use serde::{Deserialize, Serialize};

use super::asset::Asset;

#[derive(Serialize, Deserialize)]
pub struct Watchlist {
    pub account_id: String,
    pub id: String,
    pub created_at: String,
    pub updated_at: String,
    pub name: String,
    pub assets: Vec<Asset>,
}
