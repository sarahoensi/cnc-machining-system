use std::collections::HashMap;
use crate::application::ApplicationError;

// interface/tauri/shared/error.rs

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct TauriError {
    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_errors: Option<HashMap<String, String>>,
}


pub fn map_application_error(err: ApplicationError) -> TauriError {
    match err {
        ApplicationError::Validation(validation) => {
            // Konverter Vec<FieldError> → HashMap<String, String>
            let map: HashMap<String, String> = validation
                .errors
                .into_iter()
                .map(|field_error| {
                    (
                        field_error.field.to_string(),
                        field_error.message,
                    )
                })
                .collect();

            TauriError {
                message: validation.message.to_string(),
                field_errors: if map.is_empty() { None } else { Some(map) },
            }
        }

        // Alle andre feil → global feil
        other => TauriError {
            message: other.to_string(),
            field_errors: None,
        },
    }
}