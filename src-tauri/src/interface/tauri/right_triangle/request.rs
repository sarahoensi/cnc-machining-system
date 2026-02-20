use serde::Deserialize;

/// UI payload for `solve_right_triangle`.
///
/// Tagged enum serialized/deserialized with `type`.
#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum SolveRightTriangleRequest {

    // ---------------------------------------------------------
    // SIDE + SIDE
    // ---------------------------------------------------------

    /// Solve from two known legs.
    Legs {
        a_mm: f64,
        b_mm: f64,
    },

    /// Solve from leg `a` and hypotenuse.
    LegAAndHypotenuse {
        a_mm: f64,
        c_mm: f64,
    },

    /// Solve from leg `b` and hypotenuse.
    LegBAndHypotenuse {
        b_mm: f64,
        c_mm: f64,
    },

    // ---------------------------------------------------------
    // SIDE + ANGLE
    // ---------------------------------------------------------

    /// Solve from leg `a` and alpha angle.
    LegAAndAlpha {
        a_mm: f64,
        alpha_deg: f64,
    },

    /// Solve from leg `a` and beta angle.
    LegAAndBeta {
        a_mm: f64,
        beta_deg: f64,
    },

    /// Solve from leg `b` and alpha angle.
    LegBAndAlpha {
        b_mm: f64,
        alpha_deg: f64,
    },

    /// Solve from leg `b` and beta angle.
    LegBAndBeta {
        b_mm: f64,
        beta_deg: f64,
    },

    /// Solve from hypotenuse and alpha angle.
    HypotenuseAndAlpha {
        c_mm: f64,
        alpha_deg: f64,
    },

    /// Solve from hypotenuse and beta angle.
    HypotenuseAndBeta {
        c_mm: f64,
        beta_deg: f64,
    },
}