// domain/geometry/mod.rs
//! Domain geometry used for machining calculations.
//!
//! This module provides validated geometric primitives and solvers used across the
//! machining domain, such as right-triangle solving, circle geometry, and helix math.
//! The types here represent physical quantities (lengths, angles, pitches) and
//! encapsulate validation rules that reflect machining constraints (for example,
//! non-negative, finite measurements and triangle inequalities where applicable).
//!
//! Intended usage
//! - Use the public constructors and solvers to build validated geometry values.
//! - Handle `GeometryError` results to surface domain validation failures to calling code.
//!
//! Domain relationships
//! - Triangle solvers return geometric measures used by machining calculations.
//! - Circle and helix primitives provide derived measures (radii, diameters, pitches)
//!   consumed by higher-level machining logic.
//!
//! Note: documentation focuses on domain semantics and invariants rather than
//! implementation details.

mod right_triangle;
mod circle;
mod helix;

mod error;

pub use right_triangle::*;
pub use circle::*;
pub use helix::*;
pub use error::*;
