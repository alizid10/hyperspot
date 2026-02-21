use modkit::api::problem::{GtsError as _, Problem};

use crate::domain::error::DomainError;
use crate::errors::{
    NodeInternalV1, NodeNotFoundV1, NodeValidationErrorV1, SysCapCollectionFailedV1,
    SysInfoCollectionFailedV1,
};

impl From<DomainError> for Problem {
    fn from(e: DomainError) -> Self {
        match e {
            DomainError::NodeNotFound(id) => NodeNotFoundV1 {
                node_id: id.to_string(),
            }
            .into_problem(),
            DomainError::SysInfoCollectionFailed(msg) => map_sysinfo_failed(&msg),
            DomainError::SysCapCollectionFailed(msg) => map_syscap_failed(&msg),
            DomainError::InvalidInput(msg) => NodeValidationErrorV1 { message: msg }.into_problem(),
            DomainError::Internal(msg) => map_internal(&msg),
        }
    }
}

fn map_sysinfo_failed(msg: &str) -> Problem {
    tracing::error!(error = %msg, "System information collection failed");
    SysInfoCollectionFailedV1.into_problem()
}

fn map_syscap_failed(msg: &str) -> Problem {
    tracing::error!(error = %msg, "System capabilities collection failed");
    SysCapCollectionFailedV1.into_problem()
}

fn map_internal(msg: &str) -> Problem {
    tracing::error!(error = %msg, "Internal server error");
    NodeInternalV1.into_problem()
}
