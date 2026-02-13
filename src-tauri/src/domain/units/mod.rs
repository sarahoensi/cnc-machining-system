// domain/units/mod.rs
//! Measurement unit domain models.

mod errors;
mod length;
mod angle;
mod motion;
mod machining;
// mod ratio; // optional later

// ---------- Public unit facade ----------

pub use errors::UnitError;

pub use length::{
    Length,
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
