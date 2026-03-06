// domain/units/angle/mod.rs
#![allow(clippy::module_inception)]

mod angle;
mod error;

pub use angle::{Angle, AcuteAngle};
pub use error::AngleError;