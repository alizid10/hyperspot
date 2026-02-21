#[cfg(test)]
mod tests {
    use crate::domain::error::DomainError;
    use axum::http::StatusCode;
    use modkit::api::problem::Problem;

    #[test]
    fn test_not_found_error_to_problem() {
        let error = DomainError::NotFound;
        let problem: Problem = error.into();

        assert_eq!(problem.status, StatusCode::NOT_FOUND);
        // message is in metadata, populated from struct field
        let meta = problem.metadata.expect("should have metadata");
        let msg = meta["message"].as_str().unwrap();
        assert!(msg.contains("Settings not found"));
    }

    #[test]
    fn test_validation_error_to_problem() {
        let error = DomainError::Validation {
            field: "theme".to_owned(),
            message: "exceeds max length".to_owned(),
        };
        let problem: Problem = error.into();

        assert_eq!(problem.status, StatusCode::UNPROCESSABLE_ENTITY);
        let meta = problem.metadata.expect("should have metadata");
        let msg = meta["message"].as_str().unwrap();
        assert!(msg.contains("theme"));
        assert!(msg.contains("exceeds max length"));
    }

    #[test]
    fn test_database_error_to_problem() {
        let error = DomainError::Database(modkit_db::DbError::InvalidConfig(
            "connection failed".to_owned(),
        ));
        let problem: Problem = error.into();

        assert_eq!(problem.status, StatusCode::INTERNAL_SERVER_ERROR);
        // Internal errors should NOT expose details in metadata
        assert!(problem.metadata.is_none());
    }

    #[test]
    fn test_from_domain_error_for_problem_validation() {
        let error = DomainError::Validation {
            field: "language".to_owned(),
            message: "invalid format".to_owned(),
        };
        let problem: Problem = error.into();

        assert_eq!(problem.status, StatusCode::UNPROCESSABLE_ENTITY);
        let meta = problem.metadata.expect("should have metadata");
        assert!(meta["message"].as_str().unwrap().contains("language"));
    }
}
