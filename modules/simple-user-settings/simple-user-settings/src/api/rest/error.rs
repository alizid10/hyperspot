use modkit::api::problem::Problem;
use modkit_errors::GtsError as _;

use crate::domain::error::DomainError;
use crate::errors::{InternalDatabaseV1, SettingsNotFoundV1, SettingsValidationV1};

impl From<DomainError> for Problem {
    fn from(e: DomainError) -> Self {
        match e {
            DomainError::NotFound => SettingsNotFoundV1 {
                message: "Settings not found".into(),
            }
            .into_problem(),
            DomainError::Validation { field, message } => SettingsValidationV1 {
                message: format!("Validation error on '{field}': {message}"),
            }
            .into_problem(),
            DomainError::Forbidden(msg) => map_forbidden(&msg),
            DomainError::Internal(msg) => map_internal(&msg),
            DomainError::Database(ref db_err) => map_database(db_err),
        }
    }
}

fn map_forbidden(msg: &str) -> Problem {
    tracing::warn!("Access forbidden: {}", msg);
    // Use not_found to avoid exposing sensitive scope information
    SettingsNotFoundV1 {
        message: "Settings not found or not accessible".into(),
    }
    .into_problem()
}

fn map_internal(msg: &str) -> Problem {
    tracing::error!("Internal error: {}", msg);
    InternalDatabaseV1.into_problem()
}

fn map_database(db_err: &impl std::fmt::Debug) -> Problem {
    tracing::error!(error = ?db_err, "Database error occurred");
    InternalDatabaseV1.into_problem()
}
