//! GTS error type trait for RFC 9457 Problem Details serialization.
//!
//! This module provides:
//! - [`BaseErrorV1`] — canonical base error struct for the 2-segment GTS error chain
//! - [`GtsError`] trait — integrates with `#[struct_to_gts_schema]` for RFC 9457 output
//! - [`is_empty_metadata`] — helper for `#[serde(skip_serializing_if)]`
//!
//! # Example
//!
//! ```ignore
//! use modkit_errors::{BaseErrorV1, GtsError};
//! use gts_macros::struct_to_gts_schema;
//!
//! // Define a child error struct
//! #[struct_to_gts_schema(
//!     dir_path = "schemas",
//!     schema_id = "gts.cf.core.errors.err.v1~cf.types_registry.errors.not_found.v1~",
//!     description = "Entity not found",
//!     properties = "entity_id",
//!     base = BaseErrorV1,
//! )]
//! pub struct EntityNotFoundErrorV1 {
//!     pub entity_id: String,
//!     #[serde(skip_serializing)]
//!     pub internal_details: String,
//! }
//!
//! impl GtsError for EntityNotFoundErrorV1 {
//!     const STATUS: u16 = 404;
//!     const TITLE: &'static str = "Entity Not Found";
//! }
//!
//! let problem = EntityNotFoundErrorV1 {
//!     entity_id: "abc-123".into(),
//!     internal_details: "sensitive".into(),
//! }
//! .into_problem()
//! .with_trace_id("4bf92f3577b34da6a3ce929d0e0e4736")
//! .expect("valid trace_id");
//! // Problem {
//! //   type_url: "gts://gts.cf.core.errors.err.v1~cf.types_registry.errors.not_found.v1~",
//! //   title: "Entity Not Found",
//! //   status: 404,
//! //   trace_id: Some("4bf92f3577b34da6a3ce929d0e0e4736"),
//! //   metadata: Some({"entity_id": "abc-123"})
//! // }
//! ```

use std::collections::HashMap;

use crate::problem::Problem;

/// Returns `true` if `val` serializes to an empty JSON object `{}`.
///
/// Intended for use with `#[serde(skip_serializing_if = "is_empty_metadata")]`
/// on the `metadata` field of error structs, so that the `"metadata"` key is
/// omitted when there are no serializable fields.
pub fn is_empty_metadata<T: serde::Serialize>(val: &T) -> bool {
    serde_json::to_value(val)
        .ok()
        .and_then(|v| v.as_object().map(serde_json::Map::is_empty))
        .unwrap_or(false)
}

/// Canonical base error struct for the 2-segment GTS error chain.
///
/// This struct is annotated with `#[struct_to_gts_schema]` and serves as the
/// root of all GTS error types. The generic parameter `M` carries the error
/// metadata (the child error struct fields).
///
/// - `gts_type` is skipped during serialization (internal GTS plumbing)
/// - `metadata` is serialized as a nested `"metadata": { ... }` object;
///   omitted entirely when the metadata struct has no serializable fields
///
/// # Usage
///
/// Define a child struct with `struct_to_gts_schema`
/// and use `BaseErrorV1<YourChildStruct>`.
#[gts_macros::struct_to_gts_schema(
    dir_path = "schemas",
    schema_id = "gts.cf.core.errors.err.v1~",
    description = "Base error",
    properties = "",
    base = true
)]
#[derive(Debug)]
pub struct BaseErrorV1<M = ()> {
    #[serde(skip_serializing)]
    pub gts_type: gts::GtsSchemaId,
    #[serde(skip_serializing_if = "crate::is_empty_metadata")]
    pub metadata: M,
}

/// Trait for GTS error metadata structs with RFC 9457 Problem Details serialization.
///
/// Implement this on your metadata struct (annotated with `#[struct_to_gts_schema]`).
/// This enables cross-crate error definitions: `BaseErrorV1` and `GtsError` live in
/// `modkit-errors`, while metadata structs are defined in downstream crates.
///
/// ## How it works
///
/// - **`type`** (GTS URI): Built from `GtsSchema::SCHEMA_ID` (set by `struct_to_gts_schema`)
/// - **`title`**: From `GtsError::TITLE`
/// - **`status`**: From `GtsError::STATUS`
/// - **`metadata`**: Extracted via `serde::Serialize` from struct fields.
///   Use `#[serde(skip_serializing)]` on fields you want to exclude from metadata.
/// - **`trace_id`**: W3C trace-id (32 hex chars) passed at call site to `Problem::with_trace_id()`
///
/// ## RFC 9457 Problem Details output
///
/// ```json
/// {
///   "type": "gts://gts.cf.core.errors.err.v1~cf.core.errors.not_found.v1~",
///   "title": "Not Found",
///   "status": 404,
///   "trace_id": "4bf92f3577b34da6a3ce929d0e0e4736",
///   "metadata": { "entity_id": "abc-123" }
/// }
/// ```
///
/// # Example (in an external crate)
///
/// ```ignore
/// #[struct_to_gts_schema(
///     dir_path = "schemas",
///     schema_id = "gts.hx.core.errors.err.v1~hx.mymod.errors.not_found.v1~",
///     description = "Entity not found",
///     properties = "entity_id",
///     base = BaseErrorV1,
/// )]
/// pub struct EntityNotFoundV1 {
///     pub entity_id: String,
/// }
///
/// impl GtsError for EntityNotFoundV1 {
///     const STATUS: u16 = 404;
///     const TITLE: &'static str = "Entity Not Found";
/// }
/// ```
pub trait GtsError: gts::GtsSchema + serde::Serialize + schemars::JsonSchema {
    /// HTTP status code for this error type (e.g., 404, 500).
    const STATUS: u16;
    /// Human-readable error title (e.g., "Entity Not Found").
    const TITLE: &'static str;

    /// Full GTS type URI for this error.
    ///
    /// Format: `gts://{SCHEMA_ID}`
    ///
    /// Example: `gts://gts.cf.core.errors.err.v1~cf.core.errors.not_found.v1~`
    #[must_use]
    fn gts_type_uri() -> &'static str {
        static CACHE: std::sync::LazyLock<std::sync::Mutex<HashMap<&'static str, &'static str>>> =
            std::sync::LazyLock::new(|| std::sync::Mutex::new(HashMap::new()));

        let schema_id = Self::innermost_schema_id();
        let mut cache = CACHE
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(&cached) = cache.get(schema_id) {
            return cached;
        }
        let uri = format!("gts://{schema_id}");
        let leaked: &'static str = Box::leak(uri.into_boxed_str());
        cache.insert(schema_id, leaked);
        leaked
    }

    /// Convert this error struct instance into a [`Problem`] with metadata
    /// populated from the struct's serializable fields.
    ///
    /// This is the **primary way** to create Problems from GTS error structs.
    /// Fields annotated with `#[serde(skip_serializing)]` are excluded from
    /// metadata (logged server-side only).
    ///
    /// # Example
    ///
    /// ```ignore
    /// SettingsNotFoundV1 { message: "Settings not found".into() }.into_problem()
    /// ```
    fn into_problem(self) -> Problem
    where
        Self: Sized,
    {
        let status = http::StatusCode::from_u16(Self::STATUS)
            .unwrap_or(http::StatusCode::INTERNAL_SERVER_ERROR);

        // Serialize just the metadata struct fields (not the BaseErrorV1 wrapper)
        let metadata_value = serde_json::to_value(&self)
            .unwrap_or_else(|_| serde_json::Value::Object(serde_json::Map::new()));
        let metadata = metadata_value
            .as_object()
            .filter(|o| !o.is_empty())
            .map(|o| o.iter().map(|(k, v)| (k.clone(), v.clone())).collect());

        Problem {
            type_url: Self::gts_type_uri().to_owned(),
            title: Self::TITLE.to_owned(),
            status,
            trace_id: None,
            metadata,
        }
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use gts_macros::struct_to_gts_schema;

    // -- Child error struct (entity not found) --

    #[struct_to_gts_schema(
        dir_path = "schemas",
        schema_id = "gts.cf.core.errors.err.v1~cf.test.entity.not_found.v1~",
        description = "Entity not found",
        properties = "entity_id",
        base = BaseErrorV1,
    )]
    #[derive(Debug)]
    pub struct TestEntityNotFoundV1 {
        pub entity_id: String,
        #[serde(skip_serializing)]
        pub internal_details: String,
    }

    impl GtsError for TestEntityNotFoundV1 {
        const STATUS: u16 = 404;
        const TITLE: &'static str = "Entity Not Found";
    }

    #[test]
    fn test_gts_type_uri_is_stable() {
        let uri = TestEntityNotFoundV1::gts_type_uri();
        assert_eq!(
            uri,
            "gts://gts.cf.core.errors.err.v1~cf.test.entity.not_found.v1~"
        );
        // Second call should return the same cached reference
        let uri2 = TestEntityNotFoundV1::gts_type_uri();
        assert!(std::ptr::eq(uri, uri2));
    }

    #[test]
    fn test_full_json_with_metadata_and_trace_id() {
        let error = TestEntityNotFoundV1 {
            entity_id: "abc-123".to_owned(),
            internal_details: "sensitive db info".to_owned(),
        };
        assert_eq!(error.internal_details, "sensitive db info");
        let mut problem = error.into_problem();
        problem
            .with_trace_id("4bf92f3577b34da6a3ce929d0e0e4736")
            .expect("valid trace_id");

        let json: serde_json::Value = serde_json::to_value(&problem).unwrap();
        assert_eq!(
            json,
            serde_json::json!({
                "type": "gts://gts.cf.core.errors.err.v1~cf.test.entity.not_found.v1~",
                "title": "Entity Not Found",
                "status": 404,
                "trace_id": "4bf92f3577b34da6a3ce929d0e0e4736",
                "metadata": { "entity_id": "abc-123" }
            })
        );
    }

    #[test]
    fn test_full_json_without_trace_id() {
        let problem = TestEntityNotFoundV1 {
            entity_id: "xyz".to_owned(),
            internal_details: "secret".to_owned(),
        }
        .into_problem();

        let json: serde_json::Value = serde_json::to_value(&problem).unwrap();
        assert_eq!(
            json,
            serde_json::json!({
                "type": "gts://gts.cf.core.errors.err.v1~cf.test.entity.not_found.v1~",
                "title": "Entity Not Found",
                "status": 404,
                "metadata": { "entity_id": "xyz" }
            })
        );
    }

    #[test]
    fn test_schema_child_error() {
        let schema: serde_json::Value =
            serde_json::from_str(&TestEntityNotFoundV1::gts_schema_with_refs_as_string())
                .expect("valid JSON schema");

        assert_eq!(
            schema,
            serde_json::json!({
                "$id": "gts://gts.cf.core.errors.err.v1~cf.test.entity.not_found.v1~",
                "$schema": "http://json-schema.org/draft-07/schema#",
                "type": "object",
                "allOf": [
                    {
                        "$ref": "gts://gts.cf.core.errors.err.v1~"
                    },
                    {
                        "type": "object",
                        "properties": {
                            "metadata": {
                                "type": "object",
                                "additionalProperties": false,
                                "properties": {
                                    "entity_id": {
                                        "type": "string"
                                    },
                                    "internal_details": {
                                        "type": "string",
                                        "writeOnly": true
                                    }
                                },
                                "required": ["entity_id", "internal_details"]
                            }
                        }
                    }
                ]
            })
        );
    }

    #[test]
    fn test_is_empty_metadata_true_for_empty() {
        #[derive(serde::Serialize)]
        struct Empty;
        assert!(is_empty_metadata(&Empty));
    }

    #[test]
    fn test_is_empty_metadata_false_for_non_empty() {
        #[derive(serde::Serialize)]
        struct HasField {
            x: i32,
        }
        assert!(!is_empty_metadata(&HasField { x: 1 }));
    }
}
