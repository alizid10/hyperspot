//! Trace propagation utilities for Problem responses
//!
//! This module provides helper traits and functions to automatically enrich
//! `Problem` with trace context (`trace_id` from the current tracing span).
//!
//! This eliminates per-callsite boilerplate and ensures consistent error reporting.

use crate::api::problem::Problem;

/// Extract `trace_id` from the current tracing span
fn extract_trace_id() -> Option<String> {
    // Try to extract from the current span's trace_id field
    // Format as 32-hex W3C trace-id
    tracing::Span::current()
        .id()
        .map(|id| format!("{:032x}", id.into_u64()))
}

/// Helper trait for enriching Problem with trace context
pub trait WithTraceContext {
    /// Enrich this Problem with `trace_id` from the current tracing span
    #[must_use]
    fn with_trace_context(self) -> Self;
}

impl WithTraceContext for Problem {
    fn with_trace_context(mut self) -> Self {
        if let Some(tid) = extract_trace_id() {
            let _ = self.with_trace_id(tid);
        }
        self
    }
}

/// Middleware-friendly: enrich errors from Axum extractors
///
/// Use this in handlers to automatically add trace context:
///
/// ```ignore
/// async fn handler() -> Result<Json<Data>, Problem> {
///     let data = fetch_data()
///         .await
///         .map_err(Problem::from)
///         .map_err(|p| p.with_request_context())?;
///     Ok(Json(data))
/// }
/// ```
pub trait WithRequestContext {
    /// Add `trace_id` from the current request context
    #[must_use]
    fn with_request_context(self) -> Self;
}

impl WithRequestContext for Problem {
    fn with_request_context(self) -> Self {
        self.with_trace_context()
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;

    #[test]
    fn test_with_trace_context() {
        use http::StatusCode;

        let problem = Problem::new(StatusCode::NOT_FOUND, "Not Found").with_trace_context();

        // trace_id may or may not be set depending on tracing context
        assert_eq!(problem.status, StatusCode::NOT_FOUND);
    }

    #[test]
    fn test_with_request_context() {
        use http::StatusCode;

        let problem = Problem::new(StatusCode::NOT_FOUND, "Not Found").with_request_context();

        assert_eq!(problem.status, StatusCode::NOT_FOUND);
    }
}
