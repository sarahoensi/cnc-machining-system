// domain/mod.rs
mod error;
mod geometry;
pub mod machining;
pub mod units;

pub use error::DomainError;

// --------------------------------
// Geometry
// --------------------------------
pub use geometry::{
    Circle, GeometryError, Helix, HelixError, HelixMode, RightTriangle, RightTriangleError,
};
