//domain/geometry/cirlcle/mod.rs
//! Circle geometry primitives used in machining calculations.
//!
//! This module exposes the validated `Circle` type and its derived geometric
//! measures such as circumference, area, arc length, and sector area. Values
//! are expressed with domain `Length`, `Radius`, and `Diameter` types to maintain
//! unit correctness.
#![allow(clippy::module_inception)]

mod circle;

pub use circle::Circle;
