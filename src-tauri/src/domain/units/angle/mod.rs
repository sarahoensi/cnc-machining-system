// domain/units/angle/mod.rs
#![allow(clippy::module_inception)]

mod angle;
mod error;

pub use angle::Angle;
pub use error::AngleError;