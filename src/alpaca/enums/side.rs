use std::str::FromStr;

#[derive(Debug)]
pub enum Side {
    Buy,
    Sell,
}

impl ToString for Side {
    fn to_string(&self) -> String {
        match self {
            Self::Buy => "buy".into(),
            Self::Sell => "sell".into(),
        }
    }
}

impl FromStr for Side {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "buy" => Ok(Self::Buy),
            "sell" => Ok(Self::Sell),
            _ => Err(format!("invalid side: {}", s)),
        }
    }
}
