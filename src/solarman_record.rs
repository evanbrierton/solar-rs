use chrono::{DateTime, TimeZone, Utc};
use num_traits::{Num, NumCast};
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SolarManRecord {
    #[serde(rename = "Updated Time", deserialize_with = "deserialize_date")]
    pub time: DateTime<Utc>,
    #[serde(
        rename = "Production Power(W)",
        deserialize_with = "deserialize_decimal"
    )]
    pub production: u16,
    #[serde(
        rename = "Consumption Power(W)",
        deserialize_with = "deserialize_decimal"
    )]
    pub consumption: u32,
    #[serde(rename = "Grid Power(W)", deserialize_with = "deserialize_decimal")]
    pub grid: i32,
    #[serde(rename = "Battery Power(W)", deserialize_with = "deserialize_decimal")]
    pub battery: i32,
    #[serde(rename = "SoC(%)", deserialize_with = "deserialize_decimal")]
    pub soc: u8,
}

fn deserialize_decimal<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Num + TryFrom<i32>,
    D: Deserializer<'de>,
{
    let f: f32 = Deserialize::deserialize(deserializer).map_err(serde::de::Error::custom)?;

    let i: i32 = match NumCast::from(f) {
        Some(x) => x,
        None => return Err(serde::de::Error::custom("")),
    };

    let d = match T::try_from(i) {
        Ok(d) => d,
        Err(_) => return Err(serde::de::Error::custom("")),
    };

    Ok(d)
}

fn deserialize_date<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    const FORMAT: &str = "%Y/%m/%d %H:%M";
    let s = String::deserialize(deserializer)?;

    Utc.datetime_from_str(&s, FORMAT)
        .map_err(serde::de::Error::custom)
}
