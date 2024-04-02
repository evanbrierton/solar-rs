#![warn(clippy::pedantic, clippy::cargo, clippy::restriction)]
#![allow(
    clippy::separated_literal_suffix,
    clippy::implicit_return,
    clippy::mod_module_files,
    clippy::std_instead_of_alloc
)]
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::question_mark_used,
    clippy::blanket_clippy_restriction_lints,
    clippy::float_arithmetic,
    clippy::arithmetic_side_effects,
    clippy::cast_precision_loss,
    clippy::as_conversions
)]

pub mod aggregate_solar_record;
pub mod formatting;
pub mod period;
pub mod rate;
pub mod solar_data;
pub mod solar_record;
pub mod solarman_record;
