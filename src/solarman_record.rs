use chrono::{DateTime, TimeZone, Utc};
use num_traits::Num;
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
    let i = match T::try_from(f.round() as i32) {
        Ok(i) => i,
        Err(_) => return Err(serde::de::Error::custom("")),
    };

    Ok(i)
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
