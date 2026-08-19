use std::{fmt, str::FromStr};

use super::ThreadError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreadType {
    Metric,
    Unc,
    Unf,
    Bsp,
    Npt,
}

impl ThreadType {
    pub fn as_str(self) -> &'static str {
        match self {
            ThreadType::Metric => "metric",
            ThreadType::Unc => "unc",
            ThreadType::Unf => "unf",
            ThreadType::Bsp => "bsp",
            ThreadType::Npt => "npt",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            ThreadType::Metric => "Metric",
            ThreadType::Unc => "UNC",
            ThreadType::Unf => "UNF",
            ThreadType::Bsp => "G/BSP",
            ThreadType::Npt => "NPT",
        }
    }

    pub fn depth_factor(self) -> f64 {
        match self {
            ThreadType::Bsp => 0.640327,
            ThreadType::Npt => 0.800,
            ThreadType::Metric | ThreadType::Unc | ThreadType::Unf => 0.541266,
        }
    }
}

impl FromStr for ThreadType {
    type Err = ThreadError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim().to_ascii_lowercase().as_str() {
            "metric" => Ok(ThreadType::Metric),
            "unc" => Ok(ThreadType::Unc),
            "unf" => Ok(ThreadType::Unf),
            "bsp" => Ok(ThreadType::Bsp),
            "npt" => Ok(ThreadType::Npt),
            other => Err(ThreadError::InvalidThreadType(other.to_string())),
        }
    }
}

impl fmt::Display for ThreadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
