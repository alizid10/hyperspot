//! Re-exports and convenience constructors for Problem types

pub use modkit_errors::problem::{APPLICATION_PROBLEM_JSON, Problem};

// Re-export framework error catalog and trait for downstream modules
pub use modkit_errors::{
    BadRequestV1, ConfigErrorV1, ConflictV1, ForbiddenV1, GtsError, InternalErrorV1, NotFoundV1,
    UnknownErrorV1, UnsupportedMediaTypeV1, ValidationFailedV1,
};

/// Convenience: create a Bad Request problem with a message.
pub fn bad_request(message: impl Into<String>) -> Problem {
    BadRequestV1 {
        message: message.into(),
    }
    .into_problem()
}

/// Convenience: create a Not Found problem with a message.
pub fn not_found(message: impl Into<String>) -> Problem {
    NotFoundV1 {
        message: message.into(),
    }
    .into_problem()
}

/// Convenience: create a Conflict problem with a message.
pub fn conflict(message: impl Into<String>) -> Problem {
    ConflictV1 {
        message: message.into(),
    }
    .into_problem()
}

/// Convenience: create an Internal Server Error problem (no message exposed).
pub fn internal_error() -> Problem {
    InternalErrorV1.into_problem()
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use axum::response::IntoResponse;

    #[test]
    fn problem_into_response_sets_status_and_content_type() {
        use axum::http::StatusCode;

        let p = Problem::new(StatusCode::BAD_REQUEST, "Bad Request");
        let resp = p.into_response();
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        let ct = resp
            .headers()
            .get(axum::http::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");
        assert_eq!(ct, APPLICATION_PROBLEM_JSON);
    }

    #[test]
    fn convenience_constructors() {
        use http::StatusCode;

        let bad_req = bad_request("Invalid input");
        assert_eq!(bad_req.status, StatusCode::BAD_REQUEST);
        assert_eq!(bad_req.title, "Bad Request");
        assert!(bad_req.type_url.contains("bad_request"));
        let meta = bad_req.metadata.expect("should have metadata");
        assert_eq!(meta["message"], "Invalid input");

        let not_found_resp = not_found("User not found");
        assert_eq!(not_found_resp.status, StatusCode::NOT_FOUND);
        assert_eq!(not_found_resp.title, "Not Found");
        assert!(not_found_resp.type_url.contains("not_found"));

        let conflict_resp = conflict("Email already exists");
        assert_eq!(conflict_resp.status, StatusCode::CONFLICT);
        assert_eq!(conflict_resp.title, "Conflict");
        assert!(conflict_resp.type_url.contains("conflict"));

        let internal_resp = internal_error();
        assert_eq!(internal_resp.status, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(internal_resp.title, "Internal Server Error");
        assert!(internal_resp.metadata.is_none());
    }
}
