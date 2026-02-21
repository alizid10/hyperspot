//! Error catalog for `types_registry` — explicit GTS error definitions.
//!
//! Each error is defined as a metadata struct annotated with
//! `#[struct_to_gts_schema]` and an [`GtsError`] implementation.

use gts_macros::struct_to_gts_schema;
use modkit_errors::{BaseErrorV1, GtsError};

// ---------------------------------------------------------------------------
// Invalid GTS ID — 400
// ---------------------------------------------------------------------------

#[struct_to_gts_schema(
    dir_path = "schemas",
    schema_id = "gts.cf.core.errors.err.v1~cf.types_registry.errors.invalid_gts_id.v1~",
    description = "Invalid GTS ID format",
    properties = "message",
    base = BaseErrorV1,
)]
#[derive(Debug)]
pub struct InvalidGtsIdV1 {
    pub message: String,
}

impl GtsError for InvalidGtsIdV1 {
    const STATUS: u16 = 400;
    const TITLE: &'static str = "Invalid GTS ID";
}

// ---------------------------------------------------------------------------
// Entity Not Found — 404
// ---------------------------------------------------------------------------

#[struct_to_gts_schema(
    dir_path = "schemas",
    schema_id = "gts.cf.core.errors.err.v1~cf.types_registry.errors.not_found.v1~",
    description = "Entity not found in types registry",
    properties = "gts_id",
    base = BaseErrorV1,
)]
#[derive(Debug)]
pub struct TypeEntityNotFoundV1 {
    pub gts_id: String,
}

impl GtsError for TypeEntityNotFoundV1 {
    const STATUS: u16 = 404;
    const TITLE: &'static str = "Entity Not Found";
}

// ---------------------------------------------------------------------------
// Entity Already Exists — 409
// ---------------------------------------------------------------------------

#[struct_to_gts_schema(
    dir_path = "schemas",
    schema_id = "gts.cf.core.errors.err.v1~cf.types_registry.errors.already_exists.v1~",
    description = "Entity already exists in types registry",
    properties = "gts_id",
    base = BaseErrorV1,
)]
#[derive(Debug)]
pub struct TypeEntityAlreadyExistsV1 {
    pub gts_id: String,
}

impl GtsError for TypeEntityAlreadyExistsV1 {
    const STATUS: u16 = 409;
    const TITLE: &'static str = "Entity Already Exists";
}

// ---------------------------------------------------------------------------
// Validation Failed — 422
// ---------------------------------------------------------------------------

#[struct_to_gts_schema(
    dir_path = "schemas",
    schema_id = "gts.cf.core.errors.err.v1~cf.types_registry.errors.validation_failed.v1~",
    description = "Validation failed",
    properties = "message",
    base = BaseErrorV1,
)]
#[derive(Debug)]
pub struct TypeValidationFailedV1 {
    pub message: String,
}

impl GtsError for TypeValidationFailedV1 {
    const STATUS: u16 = 422;
    const TITLE: &'static str = "Validation Failed";
}

// ---------------------------------------------------------------------------
// Service Not Ready — 503
// ---------------------------------------------------------------------------

#[struct_to_gts_schema(
    dir_path = "schemas",
    schema_id = "gts.cf.core.errors.err.v1~cf.types_registry.errors.not_ready.v1~",
    description = "Types registry is not yet ready",
    properties = "",
    base = BaseErrorV1,
)]
#[derive(Debug)]
pub struct TypeNotReadyV1;

impl GtsError for TypeNotReadyV1 {
    const STATUS: u16 = 503;
    const TITLE: &'static str = "Service Not Ready";
}

// ---------------------------------------------------------------------------
// Activation Failed — 500
// ---------------------------------------------------------------------------

#[struct_to_gts_schema(
    dir_path = "schemas",
    schema_id = "gts.cf.core.errors.err.v1~cf.types_registry.errors.activation_failed.v1~",
    description = "Registry activation failed",
    properties = "error_count",
    base = BaseErrorV1,
)]
#[derive(Debug)]
pub struct TypeActivationFailedV1 {
    pub error_count: usize,
}

impl GtsError for TypeActivationFailedV1 {
    const STATUS: u16 = 500;
    const TITLE: &'static str = "Registry Activation Failed";
}

// ---------------------------------------------------------------------------
// Internal Error — 500
// ---------------------------------------------------------------------------

#[struct_to_gts_schema(
    dir_path = "schemas",
    schema_id = "gts.cf.core.errors.err.v1~cf.types_registry.errors.internal.v1~",
    description = "Internal types registry error",
    properties = "",
    base = BaseErrorV1,
)]
#[derive(Debug)]
pub struct TypeInternalV1;

impl GtsError for TypeInternalV1 {
    const STATUS: u16 = 500;
    const TITLE: &'static str = "Internal Server Error";
}
