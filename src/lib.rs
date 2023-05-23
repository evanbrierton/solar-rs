#![warn(clippy::pedantic, clippy::cargo)]
#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_lossless
)]

pub mod aggregate_solar_record;
pub mod formatting;
pub mod parsers;
pub mod rate;
pub mod solar_data;
pub mod solar_record;
pub mod solarman_record;
