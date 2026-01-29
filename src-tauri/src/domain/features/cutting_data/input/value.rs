// value_objects.rs

use super::errors::DomainError;

/// Tool diameter in millimeters (mm)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DiameterMm(f64);

impl DiameterMm {
    pub fn new(value: f64) -> Result<Self, DomainError> {
        if value > 0.0 {
            Ok(Self(value))
        } else {
            Err(DomainError::InvalidValue("Diameter must be > 0"))
        }
    }

    pub fn value(self) -> f64 {
        self.0
    }
}

/// Cutting speed in meters per minute (m/min)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CuttingSpeedMMin(f64);

impl CuttingSpeedMMin {
    pub fn new(value: f64) -> Result<Self, DomainError> {
        if value > 0.0 {
            Ok(Self(value))
        } else {
            Err(DomainError::InvalidValue("Cutting speed must be > 0"))
        }
    }

    pub fn value(self) -> f64 {
        self.0
    }
}

/// Spindle speed in revolutions per minute (RPM)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpindleSpeedRpm(f64);

impl SpindleSpeedRpm {
    pub fn new(value: f64) -> Result<Self, DomainError> {
        if value > 0.0 {
            Ok(Self(value))
        } else {
            Err(DomainError::InvalidValue("Spindle speed must be > 0"))
        }
    }

    pub fn value(self) -> f64 {
        self.0
    }
}

/// Feed rate in millimeters per minute (mm/min)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FeedRateMmMin(f64);

impl FeedRateMmMin {
    pub fn new(value: f64) -> Result<Self, DomainError> {
        if value > 0.0 {
            Ok(Self(value))
        } else {
            Err(DomainError::InvalidValue("Feed rate must be > 0"))
        }
    }

    pub fn value(self) -> f64 {
        self.0
    }
}

/// Feed per tooth in millimeters per tooth (mm/tooth)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FeedPerToothMm(f64);

impl FeedPerToothMm {
    pub fn new(value: f64) -> Result<Self, DomainError> {
        if value > 0.0 {
            Ok(Self(value))
        } else {
            Err(DomainError::InvalidValue("Feed per tooth must be > 0"))
        }
    }

    pub fn value(self) -> f64 {
        self.0
    }
}

/// Number of teeth (integer ≥ 1)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ToothCount(u32);

impl ToothCount {
    pub fn new(value: u32) -> Result<Self, DomainError> {
        if value >= 1 {
            Ok(Self(value))
        } else {
            Err(DomainError::InvalidValue("Tooth count must be ≥ 1"))
        }
    }

    pub fn value(self) -> u32 {
        self.0
    }
}
