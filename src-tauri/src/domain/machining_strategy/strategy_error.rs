// domain/machining_strategy/strategy_errors.rs

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum StrategyError {
    InvalidInputs(&'static str),
    ImpossiblePlan(&'static str),
}

impl fmt::Display for StrategyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StrategyError::InvalidInputs(msg) => write!(f, "Invalid inputs: {msg}"),
            StrategyError::ImpossiblePlan(msg) => write!(f, "Impossible plan: {msg}"),
        }
    }
}

impl std::error::Error for StrategyError {}
