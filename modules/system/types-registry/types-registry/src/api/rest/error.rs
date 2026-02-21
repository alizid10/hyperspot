//! REST error mapping for the Types Registry module.

use modkit::api::problem::{GtsError as _, Problem};

use crate::domain::error::DomainError;
use crate::errors::{
    InvalidGtsIdV1, TypeActivationFailedV1, TypeEntityAlreadyExistsV1, TypeEntityNotFoundV1,
    TypeInternalV1, TypeNotReadyV1, TypeValidationFailedV1,
};

impl From<DomainError> for Problem {
    fn from(e: DomainError) -> Self {
        match e {
            DomainError::InvalidGtsId(msg) => InvalidGtsIdV1 { message: msg }.into_problem(),
            DomainError::NotFound(id) => TypeEntityNotFoundV1 { gts_id: id }.into_problem(),
            DomainError::AlreadyExists(id) => {
                TypeEntityAlreadyExistsV1 { gts_id: id }.into_problem()
            }
            DomainError::ValidationFailed(msg) => {
                TypeValidationFailedV1 { message: msg }.into_problem()
            }
            DomainError::NotInReadyMode => TypeNotReadyV1 {}.into_problem(),
            DomainError::ReadyCommitFailed(errors) => {
                let error_strings: Vec<String> = errors.iter().map(ToString::to_string).collect();
                tracing::error!(
                    error_count = errors.len(),
                    "Registry activation failed: {}",
                    error_strings.join("; ")
                );
                TypeActivationFailedV1 {
                    error_count: errors.len(),
                }
                .into_problem()
            }
            DomainError::Internal(e) => {
                tracing::error!(error = ?e, "Internal error in types_registry");
                TypeInternalV1 {}.into_problem()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use modkit::api::prelude::StatusCode;

    #[test]
    fn test_domain_error_to_problem_not_found() {
        let err = DomainError::not_found("gts.x.core.events.test.v1~");
        let problem: Problem = err.into();
        assert_eq!(problem.status, StatusCode::NOT_FOUND);
    }

    #[test]
    fn test_domain_error_to_problem_already_exists() {
        let err = DomainError::already_exists("gts.x.core.events.test.v1~");
        let problem: Problem = err.into();
        assert_eq!(problem.status, StatusCode::CONFLICT);
    }

    #[test]
    fn test_domain_error_to_problem_invalid_gts_id() {
        let err = DomainError::invalid_gts_id("bad format");
        let problem: Problem = err.into();
        assert_eq!(problem.status, StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_domain_error_to_problem_validation_failed() {
        let err = DomainError::validation_failed("schema invalid");
        let problem: Problem = err.into();
        assert_eq!(problem.status, StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[test]
    fn test_domain_error_to_problem_not_in_ready_mode() {
        let err = DomainError::NotInReadyMode;
        let problem: Problem = err.into();
        assert_eq!(problem.status, StatusCode::SERVICE_UNAVAILABLE);
    }

    #[test]
    fn test_domain_error_to_problem_ready_commit_failed() {
        use crate::domain::error::ValidationError;
        let err = DomainError::ReadyCommitFailed(vec![
            ValidationError::new("gts.test1~", "error1"),
            ValidationError::new("gts.test2~", "error2"),
            ValidationError::new("gts.test3~", "error3"),
        ]);
        let problem: Problem = err.into();
        assert_eq!(problem.status, StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_domain_error_to_problem_internal() {
        let err = DomainError::Internal(anyhow::anyhow!("test error"));
        let problem: Problem = err.into();
        assert_eq!(problem.status, StatusCode::INTERNAL_SERVER_ERROR);
    }
}
