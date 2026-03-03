// domain/units/mod.rs
//! Measurement unit domain models.

mod error;
mod length;
mod angle;
mod motion;
mod machining;
mod core;
// mod ratio; // optional later

// ---------- Public unit facade ----------

pub use error::UnitsError;

pub use length::{
    Length,
    PositiveLength,
    Diameter,
    Radius,
    Pitch,
    LengthUnitError,
};

pub use angle::{Angle, AcuteAngle,AngleError};

pub use motion::{
    FeedRate,
    Rpm,
    MotionUnitError,
};

pub use machining::{
    ChipLoad,
    CuttingSpeed,
    ToothCount,
    MachiningUnitError
};

pub use core::{
    PositiveScalar,
};
