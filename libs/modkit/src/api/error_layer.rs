//! Centralized error mapping for Axum
//!
//! This module provides utilities for automatically converting all framework
//! and module errors into consistent RFC 9457 Problem+JSON responses, eliminating
//! per-route boilerplate.

use axum::{extract::Request, http::HeaderMap, middleware::Next, response::Response};
use std::any::Any;

use crate::api::problem::Problem;
use crate::config::ConfigError;
use modkit_errors::GtsError as _;
use modkit_odata::Error as ODataError;

/// Middleware function that provides centralized error mapping
///
/// This middleware can be applied to routes to automatically extract request context
/// and provide it to error handlers. The actual error conversion happens in the
/// `IntoProblem` trait implementations and `map_error_to_problem` function.
pub async fn error_mapping_middleware(request: Request, next: Next) -> Response {
    let _uri = request.uri().clone();
    let _headers = request.headers().clone();

    let response = next.run(request).await;

    // If the response is already successful or is already a Problem response, pass it through
    if response.status().is_success() || is_problem_response(&response) {
        return response;
    }

    // For error responses, the actual error conversion should happen in the handlers
    // using the IntoProblem trait or map_error_to_problem function
    // This middleware provides the infrastructure for extracting request context
    response
}

/// Check if a response is already a Problem+JSON response
fn is_problem_response(response: &Response) -> bool {
    response
        .headers()
        .get(axum::http::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .is_some_and(|ct| ct.contains("application/problem+json"))
}

/// Extract trace ID from headers or generate one
pub fn extract_trace_id(headers: &HeaderMap) -> Option<String> {
    // Try to get trace ID from various common headers
    headers
        .get("x-trace-id")
        .or_else(|| headers.get("x-request-id"))
        .or_else(|| headers.get("traceparent"))
        .and_then(|v| v.to_str().ok())
        .map(ToString::to_string)
        .or_else(|| {
            // Try to get from current tracing span (format as 32-hex W3C trace-id)
            tracing::Span::current()
                .id()
                .map(|id| format!("{:032x}", id.into_u64()))
        })
}

/// Centralized error mapping function
///
/// This function provides a single place to convert all framework and module errors
/// into consistent Problem responses with proper trace IDs.
pub fn map_error_to_problem(error: &dyn Any, trace_id: Option<String>) -> Problem {
    // Try to downcast to known error types
    let problem = if let Some(odata_err) = error.downcast_ref::<ODataError>() {
        crate::api::odata::error::odata_error_to_problem(odata_err)
    } else if let Some(config_err) = error.downcast_ref::<ConfigError>() {
        match config_err {
            ConfigError::ModuleNotFound { module } => {
                tracing::error!(module = %module, "Module configuration not found");
            }
            ConfigError::InvalidModuleStructure { module } => {
                tracing::error!(module = %module, "Invalid module configuration structure");
            }
            ConfigError::MissingConfigSection { module } => {
                tracing::error!(module = %module, "Missing required config section");
            }
            ConfigError::InvalidConfig { module, .. } => {
                tracing::error!(module = %module, "Invalid configuration");
            }
        }

        modkit_errors::ConfigErrorV1 {
            message: config_err.to_string(),
        }
        .into_problem()
    } else if let Some(anyhow_err) = error.downcast_ref::<anyhow::Error>() {
        tracing::error!(error = %anyhow_err, "Internal server error");
        modkit_errors::InternalErrorV1.into_problem()
    } else {
        tracing::error!("Unknown error type in error mapping layer");
        modkit_errors::UnknownErrorV1.into_problem()
    };

    modkit_errors::finalize(problem, trace_id)
}

/// Helper trait for converting errors to Problem responses with context
pub trait IntoProblem {
    fn into_problem(self, trace_id: Option<String>) -> Problem;
}

impl IntoProblem for ODataError {
    fn into_problem(self, trace_id: Option<String>) -> Problem {
        let problem = crate::api::odata::error::odata_error_to_problem(&self);
        modkit_errors::finalize(problem, trace_id)
    }
}

impl IntoProblem for ConfigError {
    fn into_problem(self, trace_id: Option<String>) -> Problem {
        map_error_to_problem(&self as &dyn Any, trace_id)
    }
}

impl IntoProblem for anyhow::Error {
    fn into_problem(self, trace_id: Option<String>) -> Problem {
        map_error_to_problem(&self as &dyn Any, trace_id)
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use http::StatusCode;

    #[test]
    fn test_odata_error_mapping() {
        let error = ODataError::InvalidFilter("malformed".to_owned());
        let tid = "4bf92f3577b34da6a3ce929d0e0e4736".to_owned();
        let problem = error.into_problem(Some(tid.clone()));

        assert_eq!(problem.status, StatusCode::UNPROCESSABLE_ENTITY);
        assert!(problem.type_url.contains("invalid_filter"));
        assert_eq!(problem.trace_id, Some(tid));
    }

    #[test]
    fn test_config_error_mapping() {
        let error = ConfigError::ModuleNotFound {
            module: "test_module".to_owned(),
        };
        let problem = error.into_problem(None);

        assert_eq!(problem.status, StatusCode::INTERNAL_SERVER_ERROR);
        assert!(problem.type_url.contains("config"));
    }

    #[test]
    fn test_anyhow_error_mapping() {
        let error = anyhow::anyhow!("Something went wrong");
        let tid = "abcdef01234567890abcdef012345678".to_owned();
        let problem = error.into_problem(Some(tid.clone()));

        assert_eq!(problem.status, StatusCode::INTERNAL_SERVER_ERROR);
        assert!(problem.type_url.contains("internal"));
        assert_eq!(problem.trace_id, Some(tid));
    }

    #[test]
    fn test_extract_trace_id_from_headers() {
        let mut headers = HeaderMap::new();
        headers.insert("x-trace-id", "test-trace-123".parse().unwrap());

        let trace_id = extract_trace_id(&headers);
        assert_eq!(trace_id, Some("test-trace-123".to_owned()));
    }
}
