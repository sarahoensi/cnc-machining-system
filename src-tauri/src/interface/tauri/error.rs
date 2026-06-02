// interface/tauri/shared/error.rs

use crate::application::ApplicationError;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TauriFieldError {
    pub field: String,
    pub code: String,
    pub message: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TauriError {
    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_errors: Option<Vec<TauriFieldError>>,
}

pub fn map_application_error(err: ApplicationError) -> TauriError {
    match err {
        ApplicationError::Validation(validation) => {
            let field_errors: Vec<TauriFieldError> = validation
                .errors
                .into_iter()
                .map(|e| TauriFieldError {
                    field: e.field.to_string(),
                    code: e.code.to_string(),
                    message: e.message,
                })
                .collect();

            TauriError {
                message: validation.message.to_string(),
                field_errors: if field_errors.is_empty() {
                    None
                } else {
                    Some(field_errors)
                },
            }
        }

        other => TauriError {
            message: other.to_string(),
            field_errors: None,
        },
    }
}

impl TauriError {
    pub fn message(msg: impl Into<String>) -> Self {
        Self {
            message: msg.into(),
            field_errors: None,
        }
    }
}
