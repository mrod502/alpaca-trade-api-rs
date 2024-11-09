use std::str::FromStr;

#[derive(Debug)]
pub enum DateType {
    DeclarationDate,
    RecordDate,
    ExDate,
    PayableDate,
}
impl ToString for DateType {
    fn to_string(&self) -> String {
        match self {
            Self::DeclarationDate => "declaration_date".into(),
            Self::RecordDate => "record_date".into(),
            Self::ExDate => "ex_date".into(),
            Self::PayableDate => "payable_date".into(),
        }
    }
}

impl FromStr for DateType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "declaration_date" => Ok(Self::DeclarationDate),
            "record_date" => Ok(Self::RecordDate),
            "ex_date" => Ok(Self::ExDate),
            "payable_date" => Ok(Self::PayableDate),
            _ => Err(format!("invalid date type: {}", s)),
        }
    }
}
