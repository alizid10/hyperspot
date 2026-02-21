//! Error catalog for `users_info` — explicit error definitions.
//!
//! Each error is defined as a metadata struct annotated with
//! `#[struct_to_gts_schema]` and a [`GtsError`] implementation.

use gts_macros::struct_to_gts_schema;
use modkit_errors::{BaseErrorV1, GtsError};

// ---------------------------------------------------------------------------
// User Not Found — 404
// ---------------------------------------------------------------------------

#[struct_to_gts_schema(
    dir_path = "schemas",
    schema_id = "gts.cf.core.errors.err.v1~hx.example1.user.not_found.v1~",
    description = "User not found",
    properties = "message",
    base = BaseErrorV1,
)]
#[derive(Debug)]
pub struct UserNotFoundV1 {
    pub message: String,
}

impl GtsError for UserNotFoundV1 {
    const STATUS: u16 = 404;
    const TITLE: &'static str = "User Not Found";
}

// ---------------------------------------------------------------------------
// Email Already Exists — 409
// ---------------------------------------------------------------------------

#[struct_to_gts_schema(
    dir_path = "schemas",
    schema_id = "gts.cf.core.errors.err.v1~hx.example1.user.email_conflict.v1~",
    description = "Email already exists",
    properties = "message",
    base = BaseErrorV1,
)]
#[derive(Debug)]
pub struct EmailConflictV1 {
    pub message: String,
}

impl GtsError for EmailConflictV1 {
    const STATUS: u16 = 409;
    const TITLE: &'static str = "Email Already Exists";
}

// ---------------------------------------------------------------------------
// Invalid Email — 400
// ---------------------------------------------------------------------------

#[struct_to_gts_schema(
    dir_path = "schemas",
    schema_id = "gts.cf.core.errors.err.v1~hx.example1.user.invalid_email.v1~",
    description = "Invalid email format",
    properties = "message",
    base = BaseErrorV1,
)]
#[derive(Debug)]
pub struct InvalidEmailV1 {
    pub message: String,
}

impl GtsError for InvalidEmailV1 {
    const STATUS: u16 = 400;
    const TITLE: &'static str = "Invalid Email";
}

// ---------------------------------------------------------------------------
// Validation Error — 422
// ---------------------------------------------------------------------------

#[struct_to_gts_schema(
    dir_path = "schemas",
    schema_id = "gts.cf.core.errors.err.v1~hx.example1.user.validation.v1~",
    description = "Validation error",
    properties = "message",
    base = BaseErrorV1,
)]
#[derive(Debug)]
pub struct UserValidationV1 {
    pub message: String,
}

impl GtsError for UserValidationV1 {
    const STATUS: u16 = 422;
    const TITLE: &'static str = "Validation Error";
}

// ---------------------------------------------------------------------------
// Internal Database Error — 500
// ---------------------------------------------------------------------------

#[struct_to_gts_schema(
    dir_path = "schemas",
    schema_id = "gts.cf.core.errors.err.v1~hx.example1.user.internal_database.v1~",
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
