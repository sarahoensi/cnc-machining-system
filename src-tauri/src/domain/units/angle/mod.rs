// domain/units/angle/mod.rs
#![allow(clippy::module_inception)]

mod angle;
mod error;

pub use angle::{AcuteAngle, Angle};
pub use error::AngleError;
