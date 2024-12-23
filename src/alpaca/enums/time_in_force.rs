use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum TimeInForce {
    Day,
    GTC,
    OPG,
    IOC,
    FOK,
    GTX,
    GTD,
    CLS,
}

impl ToString for TimeInForce {
    fn to_string(&self) -> String {
        match self {
            Self::Day => "day".into(),
            Self::GTC => "gtc".into(),
            Self::OPG => "opg".into(),
            Self::IOC => "ioc".into(),
            Self::FOK => "fok".into(),
            Self::GTX => "gtx".into(),
            Self::GTD => "gtd".into(),
            Self::CLS => "cls".into(),
        }
    }
}

impl FromStr for TimeInForce {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "day" => Ok(Self::Day),
            "gtc" => Ok(Self::GTC),
            "opg" => Ok(Self::OPG),
            "ioc" => Ok(Self::IOC),
            "fok" => Ok(Self::FOK),
            "gtx" => Ok(Self::GTX),
            "gtd" => Ok(Self::GTD),
            "cls" => Ok(Self::CLS),
            _ => Err(format!("invalid time in force: {}", s)),
        }
    }
}
