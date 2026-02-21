//! Error catalog for `nodes_registry` — explicit GTS error definitions.

use gts_macros::struct_to_gts_schema;
use modkit_errors::{BaseErrorV1, GtsError};

// ---------------------------------------------------------------------------
// Node Not Found — 404
// ---------------------------------------------------------------------------

#[struct_to_gts_schema(
    dir_path = "schemas",
    schema_id = "gts.cf.core.errors.err.v1~cf.nodes_registry.errors.not_found.v1~",
    description = "Node not found",
    properties = "node_id",
    base = BaseErrorV1,
)]
#[derive(Debug)]
pub struct NodeNotFoundV1 {
    pub node_id: String,
}

impl GtsError for NodeNotFoundV1 {
    const STATUS: u16 = 404;
    const TITLE: &'static str = "Node Not Found";
}

// ---------------------------------------------------------------------------
// Validation Error — 400
// ---------------------------------------------------------------------------

#[struct_to_gts_schema(
    dir_path = "schemas",
    schema_id = "gts.cf.core.errors.err.v1~cf.nodes_registry.errors.validation.v1~",
    description = "Node validation error",
    properties = "message",
    base = BaseErrorV1,
)]
#[derive(Debug)]
pub struct NodeValidationErrorV1 {
    pub message: String,
}

impl GtsError for NodeValidationErrorV1 {
    const STATUS: u16 = 400;
    const TITLE: &'static str = "Validation Error";
}

// ---------------------------------------------------------------------------
// SysInfo Collection Failed — 500
// ---------------------------------------------------------------------------

#[struct_to_gts_schema(
    dir_path = "schemas",
    schema_id = "gts.cf.core.errors.err.v1~cf.nodes_registry.errors.sysinfo_failed.v1~",
    description = "System information collection failed",
    properties = "",
    base = BaseErrorV1,
)]
#[derive(Debug)]
pub struct SysInfoCollectionFailedV1;

impl GtsError for SysInfoCollectionFailedV1 {
    const STATUS: u16 = 500;
    const TITLE: &'static str = "System Information Collection Failed";
}

// ---------------------------------------------------------------------------
// SysCap Collection Failed — 500
// ---------------------------------------------------------------------------

#[struct_to_gts_schema(
    dir_path = "schemas",
    schema_id = "gts.cf.core.errors.err.v1~cf.nodes_registry.errors.syscap_failed.v1~",
    description = "System capabilities collection failed",
    properties = "",
    base = BaseErrorV1,
)]
#[derive(Debug)]
pub struct SysCapCollectionFailedV1;

impl GtsError for SysCapCollectionFailedV1 {
    const STATUS: u16 = 500;
    const TITLE: &'static str = "System Capabilities Collection Failed";
}

// ---------------------------------------------------------------------------
// Internal Error — 500
// ---------------------------------------------------------------------------

#[struct_to_gts_schema(
    dir_path = "schemas",
    schema_id = "gts.cf.core.errors.err.v1~cf.nodes_registry.errors.internal.v1~",
    description = "Internal nodes registry error",
    properties = "",
    base = BaseErrorV1,
)]
#[derive(Debug)]
pub struct NodeInternalV1;

impl GtsError for NodeInternalV1 {
    const STATUS: u16 = 500;
    const TITLE: &'static str = "Internal Server Error";
}
