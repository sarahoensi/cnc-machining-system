//domain/geometry/righnt_triangle/mod.rs
//! Right-triangle geometry primitives and solvers for machining.
//!
//! This module exposes `RightTriangle` as the canonical representation of a
//! right triangle (two legs) and `RightTriangleSolver` which provides validated
//! construction from alternative input pairs (legs, hypotenuse, angles).
//!
//! Domain invariants
//! - Leg and hypotenuse lengths are positive and finite.
//! - Angles used are acute for right-triangle construction where applicable.
#![allow(clippy::module_inception)]

mod right_triangle;
mod right_triangle_solver;
mod error;

pub use right_triangle::RightTriangle;
pub use right_triangle_solver::RightTriangleSolver;
pub use error::RightTriangleError;
