#![allow(clippy::unwrap_used, clippy::expect_used)]

//! Tests for error handling and conversions
//!
//! These tests verify domain error conversions including HTTP Problem mapping and anyhow integration.

use axum::http::StatusCode;
use modkit::api::problem::Problem;
use nodes_registry::domain::error::DomainError;

#[test]
fn test_error_conversion_mapping() {
    let test_id = uuid::Uuid::new_v4();

    // Test all error types with their expected mappings
    let test_cases: Vec<(DomainError, StatusCode, &str)> = vec![
        (
            DomainError::NodeNotFound(test_id),
            StatusCode::NOT_FOUND,
            "not_found",
        ),
        (
            DomainError::SysInfoCollectionFailed("Failed to read CPU info".to_owned()),
            StatusCode::INTERNAL_SERVER_ERROR,
            "sysinfo_failed",
        ),
        (
            DomainError::SysCapCollectionFailed("GPU detection failed".to_owned()),
            StatusCode::INTERNAL_SERVER_ERROR,
            "syscap_failed",
        ),
        (
            DomainError::InvalidInput("Invalid capability key format".to_owned()),
            StatusCode::BAD_REQUEST,
            "validation",
        ),
        (
            DomainError::Internal("Database connection lost".to_owned()),
            StatusCode::INTERNAL_SERVER_ERROR,
            "internal",
        ),
    ];

    for (error, expected_status, expected_code) in test_cases {
        let problem: Problem = error.into();

        assert_eq!(problem.status, expected_status, "Status code should match");
        assert!(!problem.type_url.is_empty(), "Type URL should not be empty");
        assert!(
            problem.type_url.contains(expected_code),
            "Type URL should contain error code, got: {}",
            problem.type_url
        );
        assert!(
            problem.type_url.starts_with("gts://"),
            "Type URL should have gts:// prefix, got: {}",
            problem.type_url
        );
    }
}

#[test]
fn test_error_into_problem_trait() {
    let node_id = uuid::Uuid::new_v4();
    let error = DomainError::NodeNotFound(node_id);

    // Test From<DomainError> for Problem
    let problem: Problem = error.into();

    assert_eq!(problem.status, StatusCode::NOT_FOUND);
    // node_id is in metadata
    let meta = problem.metadata.expect("should have metadata");
    assert_eq!(meta["node_id"].as_str().unwrap(), node_id.to_string());
}

#[test]
fn test_domain_error_from_anyhow() {
    let anyhow_err = anyhow::anyhow!("something went wrong");
    let domain_err: DomainError = anyhow_err.into();

    match domain_err {
        DomainError::Internal(msg) => {
            assert!(msg.contains("something went wrong"));
        }
        _ => panic!("Expected Internal error"),
    }
}
