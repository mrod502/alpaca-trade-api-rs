use std::str::FromStr;

#[derive(Debug)]
pub enum TimeFrame {
    Min1,
    Min5,
    Min15,
    Hour1,
    Day1,
}
impl ToString for TimeFrame {
    fn to_string(&self) -> String {
        match self {
            Self::Min1 => "1Min".into(),
            Self::Min5 => "5Min".into(),
            Self::Min15 => "15Min".into(),
            Self::Hour1 => "1H".into(),
            Self::Day1 => "1D".into(),
        }
    }
}

impl FromStr for TimeFrame {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1Min" => Ok(Self::Min1),
            "5Min" => Ok(Self::Min5),
            "15Min" => Ok(Self::Min15),
            "1H" => Ok(Self::Hour1),
            "1D" => Ok(Self::Day1),
            _ => Err(format!("invalid time frame: {}", s)),
        }
    }
}
