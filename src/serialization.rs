use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SolarRecord {
    #[serde(rename = "Updated Time", with = "date_format")]
    time: DateTime<Utc>,
    #[serde(
        rename = "Production Power(W)",
        deserialize_with = "deserialize_decimal_u32"
    )]
    production: u32,
    #[serde(
        rename = "Consumption Power(W)",
        deserialize_with = "deserialize_decimal_u32"
    )]
    consumption: u32,
    #[serde(rename = "Grid Power(W)", deserialize_with = "deserialize_decimal_i32")]
    grid: i32,
    #[serde(
        rename = "Battery Power(W)",
        deserialize_with = "deserialize_decimal_i32"
    )]
    battery: i32,
    #[serde(rename = "SoC(%)", deserialize_with = "deserialize_decimal_u8")]
    soc: u8,
}

fn deserialize_decimal_i32<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let f: f32 = s.parse().map_err(serde::de::Error::custom)?;
    let i: i32 = f.round() as i32;
    Ok(i)
}

fn deserialize_decimal_u32<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let f: f32 = s.parse().map_err(serde::de::Error::custom)?;
    let u: u32 = f.round() as u32;
    Ok(u)
}

fn deserialize_decimal_u8<'de, D>(deserializer: D) -> Result<u8, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let f: f32 = s.parse().map_err(serde::de::Error::custom)?;
    let u: u8 = f.round() as u8;
    Ok(u)
}

mod date_format {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y/%m/%d %H:%M";

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT)
            .map_err(serde::de::Error::custom)
    }
}
