//! DTOs for right-triangle solving workflows.
//!
//! These types define application-facing input variants and normalized output
//! for geometry use cases in machining setup.

use crate::domain::RightTriangle;

/// Input DTO for right-triangle solving.
///
/// This application input contract accepts one of the supported known-value
/// combinations needed to solve a right triangle.
///
/// Validation expectations:
/// - Length values must be positive and valid for a right-triangle context.
/// - Angle values must satisfy domain angle constraints.
///
/// Unit expectations:
/// - Lengths in millimeters (`mm`).
/// - Angles in degrees (`deg`).
pub enum SolveRightTriangleInput {

    // ---------------------------------------------------------
    // SIDE + SIDE
    // ---------------------------------------------------------

    /// Solve from both legs.
    Legs {
        /// First leg (`mm`).
        a_mm: f64,
        /// Second leg (`mm`).
        b_mm: f64,
    },

    /// Solve from leg `a` and hypotenuse.
    LegAAndHypotenuse {
        /// Known leg `a` (`mm`).
        a_mm: f64,
        /// Hypotenuse (`mm`).
        c_mm: f64,
    },

    /// Solve from leg `b` and hypotenuse.
    LegBAndHypotenuse {
        /// Known leg `b` (`mm`).
        b_mm: f64,
        /// Hypotenuse (`mm`).
        c_mm: f64,
    },

    // ---------------------------------------------------------
    // SIDE + ANGLE
    // ---------------------------------------------------------

    /// Solve from leg `a` and angle alpha.
    LegAAndAlpha {
        /// Known leg `a` (`mm`).
        a_mm: f64,
        /// Angle alpha (`deg`).
        alpha_deg: f64,
    },

    /// Solve from leg `a` and angle beta.
    LegAAndBeta {
        /// Known leg `a` (`mm`).
        a_mm: f64,
        /// Angle beta (`deg`).
        beta_deg: f64,
    },

    /// Solve from leg `b` and angle alpha.
    LegBAndAlpha {
        /// Known leg `b` (`mm`).
        b_mm: f64,
        /// Angle alpha (`deg`).
        alpha_deg: f64,
    },

    /// Solve from leg `b` and angle beta.
    LegBAndBeta {
        /// Known leg `b` (`mm`).
        b_mm: f64,
        /// Angle beta (`deg`).
        beta_deg: f64,
    },

    /// Solve from hypotenuse and angle alpha.
    HypotenuseAndAlpha {
        /// Hypotenuse (`mm`).
        c_mm: f64,
        /// Angle alpha (`deg`).
        alpha_deg: f64,
    },

    /// Solve from hypotenuse and angle beta.
    HypotenuseAndBeta {
        /// Hypotenuse (`mm`).
        c_mm: f64,
        /// Angle beta (`deg`).
        beta_deg: f64,
    },
}

/// Output DTO for a solved right triangle.
///
/// This application output contract returns all triangle sides and acute
/// angles after domain solving.
pub struct SolveRightTriangleOutput {
    /// Solved side `a` (`mm`).
    pub a_mm: f64,
    /// Solved side `b` (`mm`).
    pub b_mm: f64,
    /// Solved hypotenuse `c` (`mm`).
    pub c_mm: f64,
    /// Solved acute angle alpha (`deg`).
    pub alpha_deg: f64,
    /// Solved acute angle beta (`deg`).
    pub beta_deg: f64,
}

// ---------------------------------------------------------
// Domain → Application DTO mapping
// ---------------------------------------------------------

impl From<RightTriangle> for SolveRightTriangleOutput {
    fn from(triangle: RightTriangle) -> Self {
        Self {
            a_mm: triangle.a().mm_value(),
            b_mm: triangle.b().mm_value(),
            c_mm: triangle.c().mm_value(),
            alpha_deg: triangle.alpha().degrees_value(),
            beta_deg: triangle.beta().degrees_value(),
        }
    }
}