// domain/geometry/helix/mod.rs
//! Helix geometry primitives used in machining calculations.
//!
//! This module provides the validated `Helix` primitive and `HelixAngle` helper
//! for modelling cylindrical helices (diameter, pitch). The types express
//! quantities in domain units and expose derived measures like helix angle,
//! circumference, and axial travel for machining use.
#![allow(clippy::module_inception)]

mod helix;
//mod helix_angle;
//mod effective_diameters;
mod error;
//mod helix_solver;

pub use helix::{Helix, HelixMode};
//pub use helix_angle::HelixAngle;
//pub use effective_diameters::{EffectiveDiameter, HelixMode};
pub use error::HelixError;
//pub use helix_solver::HelixSolver;
