//! Framework error catalog — common HTTP error structs for the entire platform.
//!
//! Each error is a metadata struct annotated with `#[struct_to_gts_schema]`
//! and an [`GtsError`] implementation. Use `into_problem()` to convert
//! instances into RFC 9457 Problem responses with structured metadata.
//!
//! Module-specific errors should be defined in their own crates, not here.

use gts_macros::struct_to_gts_schema;

use crate::{BaseErrorV1, GtsError};

// ---------------------------------------------------------------------------
// Bad Request — 400
// ---------------------------------------------------------------------------

#[struct_to_gts_schema(
    dir_path = "schemas",
    schema_id = "gts.cf.core.errors.err.v1~cf.core.errors.bad_request.v1~",
    description = "Bad request",
    properties = "message",
    base = BaseErrorV1,
)]
#[derive(Debug)]
pub struct BadRequestV1 {
    pub message: String,
}

impl GtsError for BadRequestV1 {
    const STATUS: u16 = 400;
    const TITLE: &'static str = "Bad Request";
}

// ---------------------------------------------------------------------------
// Forbidden — 403
// ---------------------------------------------------------------------------

#[struct_to_gts_schema(
    dir_path = "schemas",
    schema_id = "gts.cf.core.errors.err.v1~cf.core.errors.forbidden.v1~",
    description = "Access forbidden",
    properties = "",
    base = BaseErrorV1,
)]
#[derive(Debug)]
pub struct ForbiddenV1;

impl GtsError for ForbiddenV1 {
    const STATUS: u16 = 403;
    const TITLE: &'static str = "Forbidden";
}

// ---------------------------------------------------------------------------
// Not Found — 404
// ---------------------------------------------------------------------------

#[struct_to_gts_schema(
    dir_path = "schemas",
    schema_id = "gts.cf.core.errors.err.v1~cf.core.errors.not_found.v1~",
    description = "Resource not found",
    properties = "message",
    base = BaseErrorV1,
)]
#[derive(Debug)]
pub struct NotFoundV1 {
    pub message: String,
}

impl GtsError for NotFoundV1 {
    const STATUS: u16 = 404;
    const TITLE: &'static str = "Not Found";
}

// ---------------------------------------------------------------------------
// Conflict — 409
// ---------------------------------------------------------------------------

#[struct_to_gts_schema(
    dir_path = "schemas",
    schema_id = "gts.cf.core.errors.err.v1~cf.core.errors.conflict.v1~",
    description = "Resource conflict",
    properties = "message",
    base = BaseErrorV1,
)]
#[derive(Debug)]
pub struct ConflictV1 {
    pub message: String,
}

impl GtsError for ConflictV1 {
    const STATUS: u16 = 409;
    const TITLE: &'static str = "Conflict";
}

// ---------------------------------------------------------------------------
// Unsupported Media Type — 415
// ---------------------------------------------------------------------------

#[struct_to_gts_schema(
    dir_path = "schemas",
    schema_id = "gts.cf.core.errors.err.v1~cf.core.errors.unsupported_media_type.v1~",
    description = "Unsupported media type",
    properties = "message",
    base = BaseErrorV1,
)]
#[derive(Debug)]
pub struct UnsupportedMediaTypeV1 {
    pub message: String,
}

impl GtsError for UnsupportedMediaTypeV1 {
    const STATUS: u16 = 415;
    const TITLE: &'static str = "Unsupported Media Type";
}

// ---------------------------------------------------------------------------
// Internal Error — 500
// ---------------------------------------------------------------------------

#[struct_to_gts_schema(
    dir_path = "schemas",
    schema_id = "gts.cf.core.errors.err.v1~cf.core.errors.internal.v1~",
    description = "Internal server error",
    properties = "",
    base = BaseErrorV1,
)]
#[derive(Debug)]
pub struct InternalErrorV1;

impl GtsError for InternalErrorV1 {
    const STATUS: u16 = 500;
    const TITLE: &'static str = "Internal Server Error";
}

// ---------------------------------------------------------------------------
// Configuration Error — 500
// ---------------------------------------------------------------------------

#[struct_to_gts_schema(
    dir_path = "schemas",
    schema_id = "gts.cf.core.errors.err.v1~cf.core.errors.config.v1~",
    description = "Configuration error",
    properties = "message",
    base = BaseErrorV1,
)]
#[derive(Debug)]
pub struct ConfigErrorV1 {
    pub message: String,
}

impl GtsError for ConfigErrorV1 {
    const STATUS: u16 = 500;
    const TITLE: &'static str = "Configuration Error";
}

// ---------------------------------------------------------------------------
// Unknown Error — 500 (fallback for unrecognized error types)
// ---------------------------------------------------------------------------

#[struct_to_gts_schema(
    dir_path = "schemas",
    schema_id = "gts.cf.core.errors.err.v1~cf.core.errors.unknown.v1~",
    description = "Unknown error",
    properties = "",
    base = BaseErrorV1,
)]
#[derive(Debug)]
pub struct UnknownErrorV1;

impl GtsError for UnknownErrorV1 {
    const STATUS: u16 = 500;
    const TITLE: &'static str = "Unknown Error";
}

// ---------------------------------------------------------------------------
// Validation Failed — 422
// ---------------------------------------------------------------------------

#[struct_to_gts_schema(
    dir_path = "schemas",
    schema_id = "gts.cf.core.errors.err.v1~cf.core.errors.validation_failed.v1~",
    description = "Validation failed with field-level details",
    properties = "message",
    base = BaseErrorV1,
)]
#[derive(Debug)]
pub struct ValidationFailedV1 {
    pub message: String,
}

impl GtsError for ValidationFailedV1 {
    const STATUS: u16 = 422;
    const TITLE: &'static str = "Validation Failed";
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;

    const TRACE_ID: &str = "4bf92f3577b34da6a3ce929d0e0e4736";

    #[test]
    fn bad_request_json() {
        let mut problem = BadRequestV1 {
            message: "invalid input".into(),
        }
        .into_problem();
        problem.with_trace_id(TRACE_ID).unwrap();

        let json: serde_json::Value = serde_json::to_value(&problem).unwrap();
        assert_eq!(
            json,
            serde_json::json!({
                "type": "gts://gts.cf.core.errors.err.v1~cf.core.errors.bad_request.v1~",
                "title": "Bad Request",
                "status": 400,
                "trace_id": TRACE_ID,
                "metadata": { "message": "invalid input" }
            })
        );
    }

    #[test]
    fn forbidden_json() {
        let mut problem = ForbiddenV1.into_problem();
        problem.with_trace_id(TRACE_ID).unwrap();

        let json: serde_json::Value = serde_json::to_value(&problem).unwrap();
        assert_eq!(
            json,
            serde_json::json!({
                "type": "gts://gts.cf.core.errors.err.v1~cf.core.errors.forbidden.v1~",
                "title": "Forbidden",
                "status": 403,
                "trace_id": TRACE_ID
            })
        );
    }

    #[test]
    fn not_found_json() {
        let mut problem = NotFoundV1 {
            message: "user 42 not found".into(),
        }
        .into_problem();
        problem.with_trace_id(TRACE_ID).unwrap();

        let json: serde_json::Value = serde_json::to_value(&problem).unwrap();
        assert_eq!(
            json,
            serde_json::json!({
                "type": "gts://gts.cf.core.errors.err.v1~cf.core.errors.not_found.v1~",
                "title": "Not Found",
                "status": 404,
                "trace_id": TRACE_ID,
                "metadata": { "message": "user 42 not found" }
            })
        );
    }

    #[test]
    fn conflict_json() {
        let mut problem = ConflictV1 {
            message: "duplicate key".into(),
        }
        .into_problem();
        problem.with_trace_id(TRACE_ID).unwrap();

        let json: serde_json::Value = serde_json::to_value(&problem).unwrap();
        assert_eq!(
            json,
            serde_json::json!({
                "type": "gts://gts.cf.core.errors.err.v1~cf.core.errors.conflict.v1~",
                "title": "Conflict",
                "status": 409,
                "trace_id": TRACE_ID,
                "metadata": { "message": "duplicate key" }
            })
        );
    }

    #[test]
    fn unsupported_media_type_json() {
        let mut problem = UnsupportedMediaTypeV1 {
            message: "expected application/json".into(),
        }
        .into_problem();
        problem.with_trace_id(TRACE_ID).unwrap();

        let json: serde_json::Value = serde_json::to_value(&problem).unwrap();
        assert_eq!(
            json,
            serde_json::json!({
                "type": "gts://gts.cf.core.errors.err.v1~cf.core.errors.unsupported_media_type.v1~",
                "title": "Unsupported Media Type",
                "status": 415,
                "trace_id": TRACE_ID,
                "metadata": { "message": "expected application/json" }
            })
        );
    }

    #[test]
    fn internal_error_json() {
        let mut problem = InternalErrorV1.into_problem();
        problem.with_trace_id(TRACE_ID).unwrap();

        let json: serde_json::Value = serde_json::to_value(&problem).unwrap();
        assert_eq!(
            json,
            serde_json::json!({
                "type": "gts://gts.cf.core.errors.err.v1~cf.core.errors.internal.v1~",
                "title": "Internal Server Error",
                "status": 500,
                "trace_id": TRACE_ID
            })
        );
    }

    #[test]
    fn config_error_json() {
        let mut problem = ConfigErrorV1 {
            message: "missing DATABASE_URL".into(),
        }
        .into_problem();
        problem.with_trace_id(TRACE_ID).unwrap();

        let json: serde_json::Value = serde_json::to_value(&problem).unwrap();
        assert_eq!(
            json,
            serde_json::json!({
                "type": "gts://gts.cf.core.errors.err.v1~cf.core.errors.config.v1~",
                "title": "Configuration Error",
                "status": 500,
                "trace_id": TRACE_ID,
                "metadata": { "message": "missing DATABASE_URL" }
            })
        );
    }

    #[test]
    fn unknown_error_json() {
        let mut problem = UnknownErrorV1.into_problem();
        problem.with_trace_id(TRACE_ID).unwrap();

        let json: serde_json::Value = serde_json::to_value(&problem).unwrap();
        assert_eq!(
            json,
            serde_json::json!({
                "type": "gts://gts.cf.core.errors.err.v1~cf.core.errors.unknown.v1~",
                "title": "Unknown Error",
                "status": 500,
                "trace_id": TRACE_ID
            })
        );
    }

    #[test]
    fn validation_failed_json() {
        let mut problem = ValidationFailedV1 {
            message: "field 'email' is required".into(),
        }
        .into_problem();
        problem.with_trace_id(TRACE_ID).unwrap();

        let json: serde_json::Value = serde_json::to_value(&problem).unwrap();
        assert_eq!(
            json,
            serde_json::json!({
                "type": "gts://gts.cf.core.errors.err.v1~cf.core.errors.validation_failed.v1~",
                "title": "Validation Failed",
                "status": 422,
                "trace_id": TRACE_ID,
                "metadata": { "message": "field 'email' is required" }
            })
        );
    }
}
