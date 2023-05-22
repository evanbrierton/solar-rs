use chrono::{DateTime, Duration, Utc};
use num_traits::Num;
use serde::{Deserialize, Deserializer, Serialize};

use crate::rate::Rate;

#[derive(Debug, Deserialize, Serialize)]
pub struct SolarRecord {
    #[serde(rename = "Updated Time", with = "date_format")]
    pub time: DateTime<Utc>,
    #[serde(
        rename = "Production Power(W)",
        deserialize_with = "deserialize_decimal"
    )]
    production: u32,
    #[serde(
        rename = "Consumption Power(W)",
        deserialize_with = "deserialize_decimal"
    )]
    consumption: u32,
    #[serde(rename = "Grid Power(W)", deserialize_with = "deserialize_decimal")]
    grid: i32,
    #[serde(rename = "Battery Power(W)", deserialize_with = "deserialize_decimal")]
    battery: i32,
    #[serde(rename = "SoC(%)", deserialize_with = "deserialize_decimal")]
    soc: u8,
}

impl SolarRecord {
    pub fn rate(&self) -> Rate {
        if self.grid > 0 {
            return Rate::Purchase;
        }

        self.time.into()
    }

    pub fn old_rate(&self) -> Rate {
        self.time.into()
    }

    pub fn cost(&self, duration: &Duration) -> f32 {
        self.rate().cost(-self.grid) * (duration.num_minutes() as f32 / 60.0)
    }

    pub fn old_cost(&self, duration: &Duration) -> f32 {
        self.old_rate().cost(self.consumption as i32) * (duration.num_minutes() as f32 / 60.0)
    }

    pub fn savings(&self, duration: &Duration) -> f32 {
        self.old_cost(duration) - self.cost(duration)
    }

    pub fn production(&self, duration: &Duration) -> f32 {
        self.production as f32 * (duration.num_minutes() as f32 / 60.0)
    }

    pub fn consumption(&self, duration: &Duration) -> f32 {
        self.consumption as f32 * (duration.num_minutes() as f32 / 60.0)
    }
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
