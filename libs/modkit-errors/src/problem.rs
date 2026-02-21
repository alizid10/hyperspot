//! RFC 9457 Problem Details for HTTP APIs (pure data model, no HTTP framework dependencies)
//!
//! Per the Unified Error System DESIGN, the Problem struct contains:
//! - `type` (GTS URI) — machine-readable error classification
//! - `title` — static human-readable error name
//! - `status` — HTTP status code
//! - `trace_id` — W3C trace-id (32 hex chars) for request correlation
//! - `metadata` — structured extension data as key-value pairs
//!
//! Intentionally omitted from RFC 9457 (security-first, ADR-0004):
//! - `detail` — free-text field prone to sensitive data leakage
//! - `instance` — replaced by `trace_id` for correlation

use std::collections::HashMap;

use http::StatusCode;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[cfg(feature = "utoipa")]
use utoipa::ToSchema;

// @cpt-constraint:cpt-cf-ues-constraint-rfc9457:p1
/// Content type for Problem Details as per RFC 9457.
pub const APPLICATION_PROBLEM_JSON: &str = "application/problem+json";

/// Custom serializer for `StatusCode` to u16
#[allow(clippy::trivially_copy_pass_by_ref)] // serde requires &T signature
fn serialize_status_code<S>(status: &StatusCode, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_u16(status.as_u16())
}

/// Custom deserializer for `StatusCode` from u16
fn deserialize_status_code<'de, D>(deserializer: D) -> Result<StatusCode, D::Error>
where
    D: Deserializer<'de>,
{
    let code = u16::deserialize(deserializer)?;
    StatusCode::from_u16(code).map_err(serde::de::Error::custom)
}

// @cpt-interface:cpt-cf-ues-interface-problem:p1
// @cpt-constraint:cpt-cf-ues-constraint-no-detail:p1
/// RFC 9457 Problem Details for HTTP APIs.
///
/// Fields match the Unified Error System DESIGN §3.3 response schema:
/// - `type`: GTS chained type URI (`gts://...~`)
/// - `title`: static human-readable error name
/// - `status`: HTTP status code
/// - `trace_id`: 32 hex chars W3C trace-id (omitted when unavailable)
/// - `metadata`: segment-specific extension data as key-value pairs
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
#[cfg_attr(
    feature = "utoipa",
    schema(
        title = "Problem",
        description = "RFC 9457 Problem Details for HTTP APIs"
    )
)]
#[must_use]
pub struct Problem {
    /// A URI reference that identifies the problem type.
    /// For GTS errors: `gts://<schema_id>`
    #[serde(rename = "type")]
    pub type_url: String,
    /// A short, human-readable summary of the problem type.
    pub title: String,
    /// The HTTP status code for this occurrence of the problem.
    /// Serializes as u16 for RFC 9457 compatibility.
    #[serde(
        serialize_with = "serialize_status_code",
        deserialize_with = "deserialize_status_code"
    )]
    #[cfg_attr(feature = "utoipa", schema(value_type = u16))]
    pub status: StatusCode,
    /// W3C trace-id (32 hex chars) for request correlation.
    /// `None` when no trace context is available — empty string `""` MUST NOT be emitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
    /// Segment-specific extension data as key-value pairs.
    /// Populated from GTS error struct fields via `GtsError::into_problem()`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

impl Problem {
    /// Create a new Problem with the given status and title.
    ///
    /// Note: This function accepts `http::StatusCode` for type safety.
    /// The status is serialized as `u16` for RFC 9457 compatibility.
    pub fn new(status: StatusCode, title: impl Into<String>) -> Self {
        Self {
            type_url: "about:blank".to_owned(),
            title: title.into(),
            status,
            trace_id: None,
            metadata: None,
        }
    }

    /// Set the W3C trace-id for request correlation.
    ///
    /// # Errors
    /// Returns an error if:
    /// - `id` is not empty but not exactly 32 hexadecimal characters
    ///
    /// An empty string is treated as "no trace context" and leaves `trace_id` as `None`.
    pub fn with_trace_id(&mut self, id: impl AsRef<str>) -> Result<(), String> {
        let tid = id.as_ref();
        if tid.is_empty() {
            return Ok(());
        }
        if tid.len() != 32 || !tid.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(format!(
                "trace_id must be exactly 32 hex characters (W3C trace-id), got: {tid:?}"
            ));
        }
        self.trace_id = Some(tid.to_owned());
        Ok(())
    }
}

/// Axum integration: make Problem directly usable as a response.
///
/// Automatically enriches the Problem with `trace_id` from the current
/// tracing span if not already set. Sets response headers per DESIGN §3.3:
/// - `Content-Type: application/problem+json`
/// - `X-Trace-Id` (when available)
/// - `X-Error-Code` (the GTS type URI)
#[cfg(feature = "axum")]
impl axum::response::IntoResponse for Problem {
    fn into_response(self) -> axum::response::Response {
        use axum::http::HeaderValue;

        // Enrich with trace_id from current span if not already set.
        // with_trace_id validates W3C format; silently skip if invalid.
        let mut problem = self;
        if problem.trace_id.is_none()
            && let Some(span_id) = tracing::Span::current().id()
        {
            // format!("{:032x}", u64) always yields exactly 32 hex chars.
            let _ = problem.with_trace_id(format!("{:032x}", span_id.into_u64()));
        }

        let status = problem.status;

        // Prepare header values before moving problem into Json
        let trace_id_header = problem
            .trace_id
            .as_deref()
            .and_then(|tid| HeaderValue::from_str(tid).ok());
        let error_code_header = HeaderValue::from_str(&problem.type_url).ok();

        let mut resp = axum::Json(problem).into_response();
        *resp.status_mut() = status;
        resp.headers_mut().insert(
            axum::http::header::CONTENT_TYPE,
            HeaderValue::from_static(APPLICATION_PROBLEM_JSON),
        );
        if let Some(tid) = trace_id_header {
            resp.headers_mut().insert("x-trace-id", tid);
        }
        if let Some(code) = error_code_header {
            resp.headers_mut().insert("x-error-code", code);
        }
        resp
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;

    #[test]
    fn problem_new_defaults() {
        let p = Problem::new(StatusCode::NOT_FOUND, "Not Found");
        assert_eq!(p.status, StatusCode::NOT_FOUND);
        assert_eq!(p.title, "Not Found");
        assert_eq!(p.type_url, "about:blank");
        assert!(p.trace_id.is_none());
        assert!(p.metadata.is_none());
    }

    #[test]
    fn problem_with_trace_id_valid() {
        let mut p = Problem::new(StatusCode::UNPROCESSABLE_ENTITY, "Validation Failed");
        p.with_trace_id("4bf92f3577b34da6a3ce929d0e0e4736")
            .expect("valid trace_id");

        assert_eq!(p.status, StatusCode::UNPROCESSABLE_ENTITY);
        assert_eq!(
            p.trace_id,
            Some("4bf92f3577b34da6a3ce929d0e0e4736".to_owned())
        );
    }

    #[test]
    fn problem_with_trace_id_empty_is_none() {
        let mut p = Problem::new(StatusCode::NOT_FOUND, "Not Found");
        p.with_trace_id("").expect("empty is ok");
        assert!(p.trace_id.is_none());
    }

    #[test]
    fn problem_with_trace_id_rejects_invalid() {
        let mut p = Problem::new(StatusCode::NOT_FOUND, "Not Found");
        let result = p.with_trace_id("too-short");
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("trace_id must be exactly 32 hex characters")
        );
        assert!(p.trace_id.is_none());
    }

    #[test]
    fn problem_serializes_status_as_u16() {
        let p = Problem::new(StatusCode::NOT_FOUND, "Not Found");
        let json = serde_json::to_string(&p).unwrap();
        assert!(json.contains("\"status\":404"));
    }

    #[test]
    fn problem_omits_none_fields() {
        let p = Problem::new(StatusCode::NOT_FOUND, "Not Found");
        let json = serde_json::to_string(&p).unwrap();
        assert!(!json.contains("trace_id"));
        assert!(!json.contains("metadata"));
    }

    #[test]
    fn problem_deserializes_status_from_u16() {
        let json = r#"{"type":"about:blank","title":"Not Found","status":404}"#;
        let p: Problem = serde_json::from_str(json).unwrap();
        assert_eq!(p.status, StatusCode::NOT_FOUND);
    }
}
