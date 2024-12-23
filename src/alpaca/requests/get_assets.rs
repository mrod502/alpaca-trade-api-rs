use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct GetAssets {
    pub status: Option<String>,
    pub asset_class: Option<String>,
    pub exchange: Option<String>,
}
