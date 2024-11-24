use bigdecimal::BigDecimal;

pub struct ClosePosition {
    pub qty: BigDecimal,
    pub percentage: BigDecimal,
    pub symbol: String,
}
