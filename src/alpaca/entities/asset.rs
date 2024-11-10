use serde::{Deserialize, Serialize};

use crate::alpaca::enums::{AssetClass, AssetStatus, StringEnum};

#[derive(Serialize, Deserialize)]
pub struct Asset {
    pub id: String,
    pub class: StringEnum<AssetClass>,
    pub exchange: String,
    pub symbol: String,
    pub name: String,
    pub status: StringEnum<AssetStatus>,
    pub tradable: bool,
    pub marginable: bool,
    pub maintenance_margin_requirement: usize,
    pub shortable: bool,
    pub easy_to_borrow: bool,
    pub fractionable: bool,
    pub attributes: Vec<String>,
}
