use std::str::FromStr;

#[derive(Debug)]
pub enum DTBPCheck {
    Entry,
    Exit,
    Both,
}
impl ToString for DTBPCheck {
    fn to_string(&self) -> String {
        match self {
            Self::Entry => "entry".into(),
            Self::Exit => "exit".into(),
            Self::Both => "both".into(),
        }
    }
}

impl FromStr for DTBPCheck {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "entry" => Ok(Self::Entry),
            "exit" => Ok(Self::Exit),
            "both" => Ok(Self::Both),
            _ => Err(format!("invalid dtbp check: {}", s)),
        }
    }
}
