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
};

pub use angle::Angle;

pub use motion::{
    FeedRate,
    Rpm,
};

pub use machining::{
    ChipLoad,
    CuttingSpeed,
};

pub use core::{
    PositiveScalar,
};
