// domain/units/mod.rs
//! Measurement unit domain models.

mod angle;
mod core;
mod error;
mod length;
mod machining;
mod motion;
// mod ratio; // optional later

// ---------- Public unit facade ----------

pub use error::UnitsError;

pub use length::{Diameter, Length, LengthUnitError, Pitch, PositiveLength, Radius};

pub use angle::{AcuteAngle, Angle, AngleError};

pub use motion::{FeedRate, MotionUnitError, Rpm};

pub use machining::{ChipLoad, CuttingSpeed, MachiningUnitError, ToothCount};

pub use core::PositiveScalar;
