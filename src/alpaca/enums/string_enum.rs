use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct StringEnum<T>(T)
where
    T: ToString + FromStr;

impl<T> Serialize for StringEnum<T>
where
    T: ToString + FromStr,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de, T> Deserialize<'de> for StringEnum<T>
where
    T: ToString + FromStr,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let res = String::deserialize(deserializer)?;
        let des = T::from_str(&res);
        println!("got string value {}", res);
        match des {
            Ok(s) => Ok(StringEnum(s)),
            Err(_) => todo!(),
        }
    }
}

impl<T> ToString for StringEnum<T>
where
    T: ToString + FromStr,
{
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
