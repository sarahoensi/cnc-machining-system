// src/application/shared/validation.rs

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct FieldError {
    pub field: &'static str,      // "a", "alpha", "diameter_mm", ...
    pub code: &'static str,       // "non_positive", "out_of_range", "impossible_triangle"
    pub message: String,          // UI-vennlig, men kan også lokaliseres senere
}

#[derive(Debug, Clone)]
pub struct ValidationErrors {
    pub message: &'static str,    // f.eks. "Ugyldige felt"
    pub errors: Vec<FieldError>,
}

impl ValidationErrors {
    pub fn new() -> Self {
        Self { message: "Ugyldige felt", errors: vec![] }
    }

    pub fn push(&mut self, field: &'static str, code: &'static str, message: impl Into<String>) {
        self.errors.push(FieldError { field, code, message: message.into() });
    }

    pub fn is_empty(&self) -> bool { self.errors.is_empty() }
}