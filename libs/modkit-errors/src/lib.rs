// @cpt-component:cpt-cf-ues-component-modkit-errors:p1
//! Core error types for the modkit framework
//!
//! This crate provides pure data types for error handling, with no dependencies
//! on HTTP frameworks. It includes:
//! - RFC 9457 Problem Details (`Problem`)
//! - GTS error definitions (`GtsError`, `BaseErrorV1`)
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

pub mod catalog;
pub mod gts_error;
pub mod problem;

// Re-export commonly used types
pub use catalog::{
    BadRequestV1, ConfigErrorV1, ConflictV1, ForbiddenV1, InternalErrorV1, NotFoundV1,
    UnknownErrorV1, UnsupportedMediaTypeV1, ValidationFailedV1,
};
pub use gts_error::{BaseErrorV1, GtsError, is_empty_metadata};
pub use problem::{APPLICATION_PROBLEM_JSON, Problem};

/// Helper to attach `trace_id` to a Problem.
///
/// This is a convenience function for enriching Problem instances with
/// request-specific context before returning them as HTTP responses.
pub fn finalize(mut p: Problem, trace_id: Option<String>) -> Problem {
    if let Some(tid) = trace_id {
        let _ = p.with_trace_id(tid);
    }
    p
}
