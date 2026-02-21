//! Centralized `OData` error mapping
//!
//! This module adds HTTP-specific context (instance path, trace ID) to `OData` errors.
//! The core Error → Problem mapping is owned by modkit-odata.

use crate::api::problem::Problem;
use modkit_odata::Error as ODataError;

/// Convert an `OData` error into a Problem, adding server-side logging.
///
/// Trace ID attachment is handled by the caller (`map_error_to_problem`).
pub fn odata_error_to_problem(err: &ODataError) -> Problem {
    use modkit_odata::Error as OE;

    // Add logging for errors that need it before conversion
    match err {
        OE::Db(msg) => {
            tracing::error!(error = %msg, "Unexpected database error in OData layer");
        }
        OE::ParsingUnavailable(msg) => {
            tracing::error!(error = %msg, "OData parsing unavailable");
        }
        _ => {}
    }

    // Delegate to modkit-odata's base mapping (single source of truth)
    err.clone().into()
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;

    #[test]
    fn test_filter_error_mapping() {
        use http::StatusCode;

        let error = ODataError::InvalidFilter("malformed expression".to_owned());
        let problem = odata_error_to_problem(&error);

        assert_eq!(problem.status, StatusCode::UNPROCESSABLE_ENTITY);
        assert!(problem.type_url.contains("invalid_filter"));
    }

    #[test]
    fn test_orderby_error_mapping() {
        use http::StatusCode;

        let error = ODataError::InvalidOrderByField("unknown_field".to_owned());
        let problem = odata_error_to_problem(&error);

        assert_eq!(problem.status, StatusCode::UNPROCESSABLE_ENTITY);
        assert!(problem.type_url.contains("invalid_orderby"));
    }

    #[test]
    fn test_cursor_error_mapping() {
        use http::StatusCode;

        let error = ODataError::CursorInvalidBase64;
        let problem = odata_error_to_problem(&error);

        assert_eq!(problem.status, StatusCode::UNPROCESSABLE_ENTITY);
        assert!(problem.type_url.contains("invalid_cursor"));
    }

    #[test]
    fn test_gts_type_url_format() {
        let error = ODataError::InvalidFilter("test".to_owned());
        let problem = odata_error_to_problem(&error);

        // Verify the type_url follows GTS format
        assert!(
            problem
                .type_url
                .starts_with("gts://gts.cf.core.errors.err.v1~")
        );
        assert!(problem.type_url.contains("odata"));
    }
}
