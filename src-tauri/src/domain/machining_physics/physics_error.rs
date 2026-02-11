// domain/machining_physics/physics_error.rs

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum PhysicsError {
    InvalidTool,
    InvalidInputs(&'static str),
}

impl fmt::Display for PhysicsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PhysicsError::InvalidTool => write!(f, "Invalid tool"),
            PhysicsError::InvalidInputs(msg) => write!(f, "Invalid inputs: {msg}"),
        }
    }
}

impl std::error::Error for PhysicsError {}
