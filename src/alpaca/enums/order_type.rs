use std::str::FromStr;
#[derive(Debug, Clone)]
pub enum OrderType {
    Market,       // "market"
    Limit,        // "limit"
    Stop,         // "stop"
    StopLimit,    // "stop_limit"
    TrailingStop, // "trailing_stop"
}

impl ToString for OrderType {
    fn to_string(&self) -> String {
        match self {
            Self::Market => "market".into(),
            Self::Limit => "limit".into(),
            Self::Stop => "stop".into(),
            Self::StopLimit => "stop_limit".into(),
            Self::TrailingStop => "trailing_stop".into(),
        }
    }
}

impl FromStr for OrderType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "market" => Ok(OrderType::Market),
            "limit" => Ok(OrderType::Limit),
            "stop" => Ok(OrderType::Stop),
            "stop_limit" => Ok(OrderType::StopLimit),
            "trailing_stop" => Ok(OrderType::TrailingStop),
            _ => Err(format!("invalid order type:{}", s)),
        }
    }
}
