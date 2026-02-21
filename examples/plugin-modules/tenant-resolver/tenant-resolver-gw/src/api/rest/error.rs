//! HTTP error mapping for the tenant resolver gateway example.
//!
//! This module converts domain errors to RFC 9457 Problem responses.
//!
//! - **Domain stays transport-agnostic**: `DomainError` should not know about HTTP
//! - **Correct dependency direction**: API depends on Domain, not vice versa
//! - **Different APIs, different mappings**: REST → 404, gRPC → `NOT_FOUND`, etc.
//!
//! See `guidelines/NEW_MODULE.md` for full explanation.

use modkit::api::problem::{GtsError as _, Problem};

use crate::domain::error::DomainError;
use crate::errors::{
    InvalidPluginInstanceV1, PermissionDeniedV1, PluginNotFoundV1, PluginUnavailableV1,
    TenantInternalV1, TenantNotFoundV1, TypesRegistryUnavailableV1,
};

impl From<DomainError> for Problem {
    fn from(e: DomainError) -> Self {
        match e {
            DomainError::PluginNotFound { vendor } => PluginNotFoundV1 { vendor }.into_problem(),
            DomainError::PluginUnavailable { gts_id, reason } => {
                map_plugin_unavailable(gts_id, &reason)
            }
            DomainError::InvalidPluginInstance { gts_id, reason } => InvalidPluginInstanceV1 {
                gts_id,
                message: reason,
            }
            .into_problem(),
            DomainError::TypesRegistryUnavailable(reason) => {
                map_types_registry_unavailable(&reason)
            }
            DomainError::TenantNotFound(_msg) => TenantNotFoundV1.into_problem(),
            DomainError::PermissionDenied(_msg) => PermissionDeniedV1.into_problem(),
            DomainError::Internal(reason) => map_internal(&reason),
        }
    }
}

fn map_plugin_unavailable(gts_id: String, reason: &str) -> Problem {
    tracing::warn!(gts_id = %gts_id, reason = %reason, "Plugin unavailable");
    PluginUnavailableV1 { gts_id }.into_problem()
}

fn map_types_registry_unavailable(reason: &str) -> Problem {
    tracing::error!(reason = %reason, "Types registry unavailable");
    TypesRegistryUnavailableV1.into_problem()
}

fn map_internal(reason: &str) -> Problem {
    tracing::error!(reason = %reason, "Internal error in tenant resolver");
    TenantInternalV1.into_problem()
}
