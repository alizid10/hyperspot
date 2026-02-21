//! Error catalog for `tenant_resolver_example_gw` — explicit GTS error definitions.

use gts_macros::struct_to_gts_schema;
use modkit_errors::{BaseErrorV1, GtsError};

// ---------------------------------------------------------------------------
// Plugin Not Found — 404
// ---------------------------------------------------------------------------

#[struct_to_gts_schema(
    dir_path = "schemas",
    schema_id = "gts.cf.core.errors.err.v1~cf.tenant_resolver.errors.plugin_not_found.v1~",
    description = "Tenant resolver plugin not found",
    properties = "vendor",
    base = BaseErrorV1,
)]
#[derive(Debug)]
pub struct PluginNotFoundV1 {
    pub vendor: String,
}

impl GtsError for PluginNotFoundV1 {
    const STATUS: u16 = 404;
    const TITLE: &'static str = "Plugin Not Found";
}

// ---------------------------------------------------------------------------
// Plugin Unavailable — 503
// ---------------------------------------------------------------------------

#[struct_to_gts_schema(
    dir_path = "schemas",
    schema_id = "gts.cf.core.errors.err.v1~cf.tenant_resolver.errors.plugin_unavailable.v1~",
    description = "Tenant resolver plugin is unavailable",
    properties = "gts_id",
    base = BaseErrorV1,
)]
#[derive(Debug)]
pub struct PluginUnavailableV1 {
    pub gts_id: String,
}

impl GtsError for PluginUnavailableV1 {
    const STATUS: u16 = 503;
    const TITLE: &'static str = "Plugin Unavailable";
}

// ---------------------------------------------------------------------------
// Invalid Plugin Instance — 400
// ---------------------------------------------------------------------------

#[struct_to_gts_schema(
    dir_path = "schemas",
    schema_id = "gts.cf.core.errors.err.v1~cf.tenant_resolver.errors.invalid_plugin_instance.v1~",
    description = "Invalid plugin instance",
    properties = "gts_id,message",
    base = BaseErrorV1,
)]
#[derive(Debug)]
pub struct InvalidPluginInstanceV1 {
    pub gts_id: String,
    pub message: String,
}

impl GtsError for InvalidPluginInstanceV1 {
    const STATUS: u16 = 400;
    const TITLE: &'static str = "Invalid Plugin Instance";
}

// ---------------------------------------------------------------------------
// Types Registry Unavailable — 500
// ---------------------------------------------------------------------------

#[struct_to_gts_schema(
    dir_path = "schemas",
    schema_id = "gts.cf.core.errors.err.v1~cf.tenant_resolver.errors.types_registry_unavailable.v1~",
    description = "Types registry is unavailable",
    properties = "",
    base = BaseErrorV1,
)]
#[derive(Debug)]
pub struct TypesRegistryUnavailableV1;

impl GtsError for TypesRegistryUnavailableV1 {
    const STATUS: u16 = 500;
    const TITLE: &'static str = "Types Registry Unavailable";
}

// ---------------------------------------------------------------------------
// Tenant Not Found — 404
// ---------------------------------------------------------------------------

#[struct_to_gts_schema(
    dir_path = "schemas",
    schema_id = "gts.cf.core.errors.err.v1~cf.tenant_resolver.errors.tenant_not_found.v1~",
    description = "Tenant not found",
    properties = "",
    base = BaseErrorV1,
)]
#[derive(Debug)]
pub struct TenantNotFoundV1;

impl GtsError for TenantNotFoundV1 {
    const STATUS: u16 = 404;
    const TITLE: &'static str = "Tenant Not Found";
}

// ---------------------------------------------------------------------------
// Permission Denied — 403
// ---------------------------------------------------------------------------

#[struct_to_gts_schema(
    dir_path = "schemas",
    schema_id = "gts.cf.core.errors.err.v1~cf.tenant_resolver.errors.permission_denied.v1~",
    description = "Permission denied",
    properties = "",
    base = BaseErrorV1,
)]
#[derive(Debug)]
pub struct PermissionDeniedV1;

impl GtsError for PermissionDeniedV1 {
    const STATUS: u16 = 403;
    const TITLE: &'static str = "Permission Denied";
}

// ---------------------------------------------------------------------------
// Internal Error — 500
// ---------------------------------------------------------------------------

#[struct_to_gts_schema(
    dir_path = "schemas",
    schema_id = "gts.cf.core.errors.err.v1~cf.tenant_resolver.errors.internal.v1~",
    description = "Internal tenant resolver error",
    properties = "",
    base = BaseErrorV1,
)]
#[derive(Debug)]
pub struct TenantInternalV1;

impl GtsError for TenantInternalV1 {
    const STATUS: u16 = 500;
    const TITLE: &'static str = "Internal Error";
}
