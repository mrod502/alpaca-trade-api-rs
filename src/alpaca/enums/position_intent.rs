use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum PositionIntent {
    BuyToOpen,
    BuyToClose,
    SellToOpen,
    SellToClose,
}
impl ToString for PositionIntent {
    fn to_string(&self) -> String {
        match self {
            Self::BuyToOpen => "buy_to_open".into(),
            Self::BuyToClose => "buy_to_close".into(),
            Self::SellToOpen => "sell_to_open".into(),
            Self::SellToClose => "sell_to_close".into(),
        }
    }
}

impl FromStr for PositionIntent {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "buy_to_open" => Ok(Self::BuyToOpen),
            "buy_to_close" => Ok(Self::BuyToClose),
            "sell_to_open" => Ok(Self::SellToOpen),
            "sell_to_close" => Ok(Self::SellToClose),
            _ => Err(format!("invalid intent: {}", s)),
        }
    }
}
