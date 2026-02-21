//! Mapping from `OData` errors to Problem (pure data)
//!
//! This provides a baseline conversion from `OData` errors to RFC 9457 Problem
//! without HTTP framework dependencies. The HTTP layer in `modkit` adds
//! instance paths and trace IDs before the Problem is converted to an HTTP response.

use crate::Error;
use crate::errors::{InternalODataErrorV1, InvalidCursorV1, InvalidFilterV1, InvalidOrderByV1};
use modkit_errors::GtsError as _;
use modkit_errors::problem::Problem;

impl From<Error> for Problem {
    fn from(err: Error) -> Self {
        use Error::{
            CursorInvalidBase64, CursorInvalidDirection, CursorInvalidFields, CursorInvalidJson,
            CursorInvalidKeys, CursorInvalidVersion, Db, FilterMismatch, InvalidCursor,
            InvalidFilter, InvalidLimit, InvalidOrderByField, OrderMismatch, OrderWithCursor,
            ParsingUnavailable,
        };

        match err {
            // Filter parsing errors → 422
            InvalidFilter(msg) => InvalidFilterV1 {
                message: format!("Invalid $filter: {msg}"),
            }
            .into_problem(),

            // OrderBy parsing and validation errors → 422
            InvalidOrderByField(field) => InvalidOrderByV1 {
                message: format!("Unsupported $orderby field: {field}"),
            }
            .into_problem(),

            // All cursor-related errors → 422
            InvalidCursor
            | CursorInvalidBase64
            | CursorInvalidJson
            | CursorInvalidVersion
            | CursorInvalidKeys
            | CursorInvalidFields
            | CursorInvalidDirection => InvalidCursorV1 {
                message: err.to_string(),
            }
            .into_problem(),

            // Pagination validation errors → 422
            OrderMismatch => InvalidOrderByV1 {
                message: "Order mismatch between cursor and query".into(),
            }
            .into_problem(),

            FilterMismatch => InvalidFilterV1 {
                message: "Filter mismatch between cursor and query".into(),
            }
            .into_problem(),

            InvalidLimit => InvalidFilterV1 {
                message: "Invalid limit parameter".into(),
            }
            .into_problem(),

            OrderWithCursor => InvalidCursorV1 {
                message: "Cannot specify both $orderby and cursor parameters".into(),
            }
            .into_problem(),

            // Database errors → 500 (should be caught earlier)
            Db(_msg) => InternalODataErrorV1 {}.into_problem(),

            // Configuration errors → 500 (feature not enabled)
            ParsingUnavailable(_msg) => InternalODataErrorV1 {}.into_problem(),
        }
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;

    #[test]
    fn test_filter_error_converts_to_problem() {
        use http::StatusCode;

        let err = Error::InvalidFilter("malformed".to_owned());
        let problem: Problem = err.into();

        assert_eq!(problem.status, StatusCode::UNPROCESSABLE_ENTITY);
        assert_eq!(problem.title, "Invalid Filter");
        assert!(problem.type_url.contains("odata"));
        assert!(problem.type_url.contains("invalid_filter"));
    }

    #[test]
    fn test_orderby_error_converts_to_problem() {
        use http::StatusCode;

        let err = Error::InvalidOrderByField("unknown".to_owned());
        let problem: Problem = err.into();

        assert_eq!(problem.status, StatusCode::UNPROCESSABLE_ENTITY);
        assert_eq!(problem.title, "Invalid OrderBy");
        assert!(problem.type_url.contains("odata"));
        assert!(problem.type_url.contains("invalid_orderby"));
    }

    #[test]
    fn test_cursor_error_converts_to_problem() {
        use http::StatusCode;

        let err = Error::CursorInvalidBase64;
        let problem: Problem = err.into();

        assert_eq!(problem.status, StatusCode::UNPROCESSABLE_ENTITY);
        assert_eq!(problem.title, "Invalid Cursor");
        assert!(problem.type_url.contains("odata"));
        assert!(problem.type_url.contains("invalid_cursor"));
    }
}
