use std::str::FromStr;

#[derive(Debug)]
pub enum AssetClass {
    USEquity, // "us_equity"
    Crypto,   // "crypto"
}

impl ToString for AssetClass {
    fn to_string(&self) -> String {
        match self {
            Self::USEquity => "us_equity".into(),
            Self::Crypto => "crypto".into(),
        }
    }
}

impl FromStr for AssetClass {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "us_equity" => Ok(Self::USEquity),
            "crypto" => Ok(Self::Crypto),
            _ => Err(format!("unknown asset class: {}", s)),
        }
    }
}
