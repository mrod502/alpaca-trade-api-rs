use std::str::FromStr;

#[derive(Debug)]
pub enum AssetStatus {
    Active,
    Inactive,
}

impl ToString for AssetStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Active => "active".into(),
            Self::Inactive => "inactive".into(),
        }
    }
}

impl FromStr for AssetStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "active" => Ok(Self::Active),
            "inactive" => Ok(Self::Inactive),
            _ => Err(format!("invalid asset status: {}", s)),
        }
    }
}
