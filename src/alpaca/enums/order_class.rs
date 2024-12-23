use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum OrderClass {
    Bracket,
    OTO,
    OCO,
    Simple,
}
impl ToString for OrderClass {
    fn to_string(&self) -> String {
        match self {
            Self::Bracket => "bracket".into(),
            Self::OTO => "oto".into(),
            Self::OCO => "oco".into(),
            Self::Simple => "simple".into(),
        }
    }
}

impl FromStr for OrderClass {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bracket" => Ok(Self::Bracket),
            "oto" => Ok(Self::OTO),
            "oco" => Ok(Self::OCO),
            "simple" => Ok(Self::Simple),
            _ => Err(format!("invalid order class: {}", s)),
        }
    }
}
