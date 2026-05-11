use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq)]
pub enum CylinderWeightError {
    #[error("outer_diameter_mm must be greater than 0")]
    InvalidOuterDiameter,

    #[error("inner_diameter_mm must be greater than or equal to 0")]
    InvalidInnerDiameter,

    #[error("inner_diameter_mm must be smaller than outer_diameter_mm")]
    InnerDiameterNotSmallerThanOuter,

    #[error("length_mm must be greater than 0")]
    InvalidLength,

    #[error("density_kg_m3 must be greater than 0")]
    InvalidDensity,

    #[error("material_name must not be empty")]
    InvalidMaterialName,

    #[error("numerical instability detected")]
    NumericalInstability,
}
