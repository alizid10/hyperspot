use modkit::api::problem::Problem;
use modkit_errors::GtsError as _;

use crate::domain::error::DomainError;
use crate::errors::{
    EmailConflictV1, InternalDatabaseV1, InvalidEmailV1, UserNotFoundV1, UserValidationV1,
};

impl From<DomainError> for Problem {
    fn from(e: DomainError) -> Self {
        match e {
            DomainError::UserNotFound { id } => UserNotFoundV1 {
                message: format!("User with id {id} was not found"),
            }
            .into_problem(),
            DomainError::NotFound { entity_type, id } => UserNotFoundV1 {
                message: format!("{entity_type} with id {id} was not found"),
            }
            .into_problem(),
            DomainError::EmailAlreadyExists { email } => EmailConflictV1 {
                message: format!("Email '{email}' is already in use"),
            }
            .into_problem(),
            DomainError::InvalidEmail { email } => InvalidEmailV1 {
                message: format!("Email '{email}' is invalid"),
            }
            .into_problem(),
            DomainError::EmptyDisplayName => UserValidationV1 {
                message: "Display name cannot be empty".into(),
            }
            .into_problem(),
            DomainError::DisplayNameTooLong { .. } | DomainError::Validation { .. } => {
                UserValidationV1 {
                    message: e.to_string(),
                }
                .into_problem()
            }
            DomainError::Database { .. } => {
                tracing::error!(error = ?e, "Database error occurred");
                InternalDatabaseV1 {}.into_problem()
            }
            DomainError::InternalError => {
                tracing::error!("Internal error occurred");
                InternalDatabaseV1 {}.into_problem()
            }
        }
    }
}
