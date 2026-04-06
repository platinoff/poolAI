//! RAID REST helpers: JSON error mapping for `RaidService` and shared HTTP error tuples.

use axum::{http::StatusCode, Json as AxumJson};

use crate::core::error::{AppError, ErrorContext};
use crate::network::api::common::{api_error_response, api_json_error};
use crate::services::raid_service::{RaidServiceError, RAID_MANAGER_UNAVAILABLE_MESSAGE};

pub(crate) type RaidHttpErr = (StatusCode, AxumJson<serde_json::Value>);

pub(crate) fn raid_api_err(
    code: impl AsRef<str>,
    message: impl Into<String>,
    ctx: Option<ErrorContext>,
    status: StatusCode,
) -> RaidHttpErr {
    let (s, j) = api_json_error(code, message, ctx, status);
    (s, AxumJson(j.0))
}

pub(crate) fn raid_event_store_unavailable(operation: impl Into<String>) -> RaidHttpErr {
    raid_api_err(
        "EVENT_STORE_UNAVAILABLE",
        "Event store is not initialized or accessible.",
        Some(ErrorContext::new(operation.into()).with_hint(
            "Enable event store in configuration and ensure initialization at startup.",
        )),
        StatusCode::SERVICE_UNAVAILABLE,
    )
}

pub(crate) fn raid_manager_unavailable() -> RaidHttpErr {
    raid_api_err(
        "RAID_MANAGER_UNAVAILABLE",
        RAID_MANAGER_UNAVAILABLE_MESSAGE,
        Some(
            ErrorContext::new("raid")
                .with_hint("Ensure RAID manager is initialized during application startup."),
        ),
        StatusCode::SERVICE_UNAVAILABLE,
    )
}

pub(crate) fn raid_service_http_err(e: RaidServiceError) -> RaidHttpErr {
    match e {
        RaidServiceError::ManagerUnavailable => raid_manager_unavailable(),
        RaidServiceError::ArtifactNotFound { id } => raid_api_err(
            "ARTIFACT_NOT_FOUND",
            format!("Artifact {} not found", id),
            Some(ErrorContext::new("raid_artifact").with_resource("artifact_id", id.to_string())),
            StatusCode::NOT_FOUND,
        ),
        RaidServiceError::WorkerNotFound { id } => raid_api_err(
            "RAID_WORKER_NOT_FOUND",
            format!("RAID worker not found: {}", id),
            Some(ErrorContext::new("raid_worker_get").with_resource("worker_id", id.to_string())),
            StatusCode::NOT_FOUND,
        ),
        RaidServiceError::EventStoreUnavailable { operation } => {
            raid_event_store_unavailable(operation)
        }
        RaidServiceError::Operation(ref err) => {
            let (s, j) = api_error_response(
                err,
                Some(ErrorContext::new("raid")),
                Some(StatusCode::INTERNAL_SERVER_ERROR),
            );
            (s, AxumJson(j.0))
        }
    }
}

/// Worker update/delete historically return HTTP 404 with a RAID_* code (even for non-not-found errors).
pub(crate) fn raid_worker_mutation_err(
    code: &'static str,
    err: AppError,
    operation: &'static str,
    worker_id: &str,
) -> RaidHttpErr {
    let (s, j) = api_json_error(
        code,
        err.to_string(),
        Some(ErrorContext::new(operation).with_resource("worker_id", worker_id.to_string())),
        StatusCode::NOT_FOUND,
    );
    (s, AxumJson(j.0))
}

pub(crate) fn raid_invalid_worker_uuid(id: &str, operation: &'static str) -> RaidHttpErr {
    raid_api_err(
        "INVALID_UUID",
        format!("Invalid UUID format: {}", id),
        Some(ErrorContext::new(operation).with_resource("worker_id", id)),
        StatusCode::BAD_REQUEST,
    )
}

pub(crate) fn raid_events_load_failed(err: &AppError, ctx: ErrorContext) -> RaidHttpErr {
    raid_api_err(
        "RAID_EVENTS_LOAD_FAILED",
        format!("Failed to load events: {}", err),
        Some(ctx),
        StatusCode::INTERNAL_SERVER_ERROR,
    )
}

pub(crate) fn raid_snapshot_not_found() -> RaidHttpErr {
    raid_api_err(
        "RAID_SNAPSHOT_NOT_FOUND",
        "No snapshot available",
        Some(ErrorContext::new("raid_snapshot_get")),
        StatusCode::NOT_FOUND,
    )
}

pub(crate) fn raid_snapshot_load_failed(err: &AppError) -> RaidHttpErr {
    raid_api_err(
        "RAID_SNAPSHOT_LOAD_FAILED",
        format!("Failed to load snapshot: {}", err),
        Some(ErrorContext::new("raid_snapshot_get")),
        StatusCode::INTERNAL_SERVER_ERROR,
    )
}

pub(crate) fn raid_snapshot_create_failed(err: &AppError) -> RaidHttpErr {
    raid_api_err(
        "RAID_SNAPSHOT_CREATE_FAILED",
        format!("Failed to create snapshot: {}", err),
        Some(ErrorContext::new("raid_snapshot_create")),
        StatusCode::INTERNAL_SERVER_ERROR,
    )
}

pub(crate) fn raid_snapshot_restore_failed(err: &AppError) -> RaidHttpErr {
    raid_api_err(
        "RAID_SNAPSHOT_RESTORE_FAILED",
        format!("Failed to restore from snapshot: {}", err),
        Some(ErrorContext::new("raid_snapshot_restore")),
        StatusCode::INTERNAL_SERVER_ERROR,
    )
}

pub(crate) fn raid_gc_failed(err: &AppError) -> RaidHttpErr {
    raid_api_err(
        "RAID_GC_FAILED",
        format!("Garbage collection failed: {}", err),
        Some(ErrorContext::new("raid_gc")),
        StatusCode::INTERNAL_SERVER_ERROR,
    )
}

pub(crate) fn raid_strategy_status_failed(err: &AppError) -> RaidHttpErr {
    raid_api_err(
        "RAID_STRATEGY_STATUS_FAILED",
        format!("Failed to get strategy status: {}", err),
        Some(ErrorContext::new("raid_strategies")),
        StatusCode::INTERNAL_SERVER_ERROR,
    )
}

pub(crate) fn raid_rebalance_failed(err: &AppError) -> RaidHttpErr {
    raid_api_err(
        "RAID_REBALANCE_FAILED",
        format!("Failed to trigger rebalancing: {}", err),
        Some(ErrorContext::new("raid_rebalance").with_hint(
            "Local RAID mode does not support rebalancing; verify strategy configuration.",
        )),
        StatusCode::INTERNAL_SERVER_ERROR,
    )
}
