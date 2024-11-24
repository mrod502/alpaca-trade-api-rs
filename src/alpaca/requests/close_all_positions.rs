use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CloseAllPositions {
    pub cancel_orders: bool,
}
