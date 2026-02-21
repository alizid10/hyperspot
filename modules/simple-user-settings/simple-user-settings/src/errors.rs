//! Error catalog for `simple_user_settings` — explicit error definitions.
//!
//! Each error is defined as a metadata struct annotated with
//! `#[struct_to_gts_schema]` and a [`GtsError`] implementation.

use gts_macros::struct_to_gts_schema;
use modkit_errors::{BaseErrorV1, GtsError};

// ---------------------------------------------------------------------------
// Settings Not Found — 404
// ---------------------------------------------------------------------------

#[struct_to_gts_schema(
    dir_path = "schemas",
    schema_id = "gts.cf.core.errors.err.v1~hx.settings.simple_user_settings.not_found.v1~",
    description = "Settings not found",
    properties = "message",
    base = BaseErrorV1,
)]
#[derive(Debug)]
pub struct SettingsNotFoundV1 {
    pub message: String,
}

impl GtsError for SettingsNotFoundV1 {
    const STATUS: u16 = 404;
    const TITLE: &'static str = "Settings Not Found";
}

// ---------------------------------------------------------------------------
// Validation Error — 422
// ---------------------------------------------------------------------------

#[struct_to_gts_schema(
    dir_path = "schemas",
    schema_id = "gts.cf.core.errors.err.v1~hx.settings.simple_user_settings.validation.v1~",
    description = "Validation error",
    properties = "message",
    base = BaseErrorV1,
)]
#[derive(Debug)]
pub struct SettingsValidationV1 {
    pub message: String,
}

impl GtsError for SettingsValidationV1 {
    const STATUS: u16 = 422;
    const TITLE: &'static str = "Validation Error";
}

// ---------------------------------------------------------------------------
// Internal Database Error — 500
// ---------------------------------------------------------------------------

#[struct_to_gts_schema(
    dir_path = "schemas",
    schema_id = "gts.cf.core.errors.err.v1~hx.settings.simple_user_settings.internal_database.v1~",
    description = "Internal database error",
    properties = "",
    base = BaseErrorV1,
)]
#[derive(Debug)]
pub struct InternalDatabaseV1;

impl GtsError for InternalDatabaseV1 {
    const STATUS: u16 = 500;
    const TITLE: &'static str = "Internal Database Error";
}
