// cutting_data/model/values.rs

use super::super::errors::DomainError;
use serde::{Deserialize, Serialize};

/// Shared validator for all floating numeric value objects
fn validate_positive_finite(
    value: f64,
    message: &'static str,
) -> Result<f64, DomainError> {
    if value.is_finite() && value > 0.0 {
        Ok(value)
    } else {
        Err(DomainError::InvalidValue(message))
    }
}

// ======================================================
// DIAMETER
// ======================================================

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct DiameterMm(f64);

impl DiameterMm {
    pub fn new(value: f64) -> Result<Self, DomainError> {
        Ok(Self(validate_positive_finite(
            value,
            "Diameter must be finite and > 0",
        )?))
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

// ======================================================
// CUTTING SPEED
// ======================================================

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CuttingSpeedMMin(f64);

impl CuttingSpeedMMin {
    pub fn new(value: f64) -> Result<Self, DomainError> {
        Ok(Self(validate_positive_finite(
            value,
            "Cutting speed must be finite and > 0",
        )?))
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

// ======================================================
// SPINDLE SPEED
// ======================================================

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SpindleSpeedRpm(f64);

impl SpindleSpeedRpm {
    pub fn new(value: f64) -> Result<Self, DomainError> {
        Ok(Self(validate_positive_finite(
            value,
            "Spindle speed must be finite and > 0",
        )?))
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

// ======================================================
// FEED RATE
// ======================================================

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct FeedRateMmMin(f64);

impl FeedRateMmMin {
    pub fn new(value: f64) -> Result<Self, DomainError> {
        Ok(Self(validate_positive_finite(
            value,
            "Feed rate must be finite and > 0",
        )?))
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

// ======================================================
// FEED PER TOOTH
// ======================================================

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct FeedPerToothMm(f64);

impl FeedPerToothMm {
    pub fn new(value: f64) -> Result<Self, DomainError> {
        Ok(Self(validate_positive_finite(
            value,
            "Feed per tooth must be finite and > 0",
        )?))
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

// ======================================================
// TOOTH COUNT
// ======================================================

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ToothCount(u32);

impl ToothCount {
    pub fn new(value: u32) -> Result<Self, DomainError> {
        if value >= 1 {
            Ok(Self(value))
        } else {
            Err(DomainError::InvalidValue("Tooth count must be ≥ 1"))
        }
    }

    pub fn value(&self) -> u32 {
        self.0
    }
}
