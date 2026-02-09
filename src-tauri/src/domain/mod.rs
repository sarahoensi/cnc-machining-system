// domain/mod.rs
mod units;
mod geometry;
mod machining_physics;
mod machining_strategy;

// Public API
pub use units::{Length, Diameter, Radius, CuttingSpeed, FeedRate, Rpm, ChipLoad, Angle };
pub use geometry::*;
pub use machining_physics::*;
pub use machining_strategy::*;