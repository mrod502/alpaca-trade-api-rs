use std::str::FromStr;

#[derive(Debug)]
pub enum TradeConfirmEmail {
    None,
    All,
}

impl ToString for TradeConfirmEmail {
    fn to_string(&self) -> String {
        match self {
            Self::None => "none".into(),
            Self::All => "all".into(),
        }
    }
}

impl FromStr for TradeConfirmEmail {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),
            "all" => Ok(Self::All),
            _ => Err(format!("invalid side: {}", s)),
        }
    }
}
