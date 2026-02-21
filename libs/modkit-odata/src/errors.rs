//! `OData` error catalog — explicit error definitions for all `OData` operations.
//!
//! Each error is defined as a metadata struct annotated with
//! `#[struct_to_gts_schema]` and a [`GtsError`] implementation.

use gts_macros::struct_to_gts_schema;
use modkit_errors::{BaseErrorV1, GtsError};

// ---------------------------------------------------------------------------
// Invalid Filter — 422
// ---------------------------------------------------------------------------

#[struct_to_gts_schema(
    dir_path = "schemas",
    schema_id = "gts.cf.core.errors.err.v1~cf.odata.errors.invalid_filter.v1~",
    description = "Invalid OData $filter expression",
    properties = "message",
    base = BaseErrorV1,
)]
#[derive(Debug)]
pub struct InvalidFilterV1 {
    pub message: String,
}

impl GtsError for InvalidFilterV1 {
    const STATUS: u16 = 422;
    const TITLE: &'static str = "Invalid Filter";
}

// ---------------------------------------------------------------------------
// Invalid OrderBy — 422
// ---------------------------------------------------------------------------

#[struct_to_gts_schema(
    dir_path = "schemas",
    schema_id = "gts.cf.core.errors.err.v1~cf.odata.errors.invalid_orderby.v1~",
    description = "Invalid OData $orderby expression",
    properties = "message",
    base = BaseErrorV1,
)]
#[derive(Debug)]
pub struct InvalidOrderByV1 {
    pub message: String,
}

impl GtsError for InvalidOrderByV1 {
    const STATUS: u16 = 422;
    const TITLE: &'static str = "Invalid OrderBy";
}

// ---------------------------------------------------------------------------
// Invalid Cursor — 422
// ---------------------------------------------------------------------------

#[struct_to_gts_schema(
    dir_path = "schemas",
    schema_id = "gts.cf.core.errors.err.v1~cf.odata.errors.invalid_cursor.v1~",
    description = "Invalid OData cursor token",
    properties = "message",
    base = BaseErrorV1,
)]
#[derive(Debug)]
pub struct InvalidCursorV1 {
    pub message: String,
}

impl GtsError for InvalidCursorV1 {
    const STATUS: u16 = 422;
    const TITLE: &'static str = "Invalid Cursor";
}

// ---------------------------------------------------------------------------
// Internal OData Error — 500
// ---------------------------------------------------------------------------

#[struct_to_gts_schema(
    dir_path = "schemas",
    schema_id = "gts.cf.core.errors.err.v1~cf.odata.errors.internal.v1~",
    description = "Internal OData processing error",
    properties = "",
    base = BaseErrorV1,
)]
#[derive(Debug)]
pub struct InternalODataErrorV1;

impl GtsError for InternalODataErrorV1 {
    const STATUS: u16 = 500;
    const TITLE: &'static str = "Internal OData Error";
}
