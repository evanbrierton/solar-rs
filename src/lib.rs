#![warn(clippy::pedantic, clippy::cargo, clippy::restriction)]
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::missing_inline_in_public_items,
    clippy::implicit_return,
    clippy::mod_module_files,
    clippy::float_arithmetic,
    clippy::exhaustive_enums,
    clippy::exhaustive_structs,
    clippy::separated_literal_suffix
)]
#![allow(
    clippy::blanket_clippy_restriction_lints,
    clippy::non_ascii_literal,
    clippy::as_conversions,
    clippy::arithmetic_side_effects
)]

pub mod aggregate_solar_record;
pub mod formatting;
pub mod parsers;
pub mod rate;
pub mod solar_data;
pub mod solar_record;
pub mod solarman_record;
