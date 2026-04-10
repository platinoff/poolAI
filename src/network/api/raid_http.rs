//! RAID REST helpers: JSON error mapping for `RaidService` and shared HTTP errors (`HttpAppError`).

use axum::http::StatusCode;

use crate::core::error::{AppError, ErrorContext};
use crate::network::api::common::HttpAppError;
use crate::services::raid_service::{RaidServiceError, RAID_MANAGER_UNAVAILABLE_MESSAGE};

pub(crate) fn raid_api_err(
    code: &'static str,
    message: impl Into<String>,
    ctx: Option<ErrorContext>,
    status: StatusCode,
) -> HttpAppError {
    let mut h = HttpAppError::new(AppError::RestError {
        code,
        message: message.into(),
    })
    .with_status(status);
    if let Some(c) = ctx {
        h = h.with_context(c);
    }
    h
}

pub(crate) fn raid_event_store_unavailable(operation: impl Into<String>) -> HttpAppError {
    raid_api_err(
        "EVENT_STORE_UNAVAILABLE",
        "Event store is not initialized or accessible.",
        Some(ErrorContext::new(operation.into()).with_hint(
            "Enable event store in configuration and ensure initialization at startup.",
        )),
        StatusCode::SERVICE_UNAVAILABLE,
    )
}

pub(crate) fn raid_manager_unavailable() -> HttpAppError {
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

pub(crate) fn raid_service_http_err(e: RaidServiceError) -> HttpAppError {
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
        RaidServiceError::Operation(err) => HttpAppError::new(err)
            .with_context(ErrorContext::new("raid"))
            .with_status(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Worker update/delete historically return HTTP 404 with a RAID_* code (even for non-not-found errors).
pub(crate) fn raid_worker_mutation_err(
    code: &'static str,
    err: AppError,
    operation: &'static str,
    worker_id: &str,
) -> HttpAppError {
    HttpAppError::new(AppError::RestError {
        code,
        message: err.to_string(),
    })
    .with_context(ErrorContext::new(operation).with_resource("worker_id", worker_id.to_string()))
    .with_status(StatusCode::NOT_FOUND)
}

pub(crate) fn raid_invalid_worker_uuid(id: &str, operation: &'static str) -> HttpAppError {
    raid_api_err(
        "INVALID_UUID",
        format!("Invalid UUID format: {}", id),
        Some(ErrorContext::new(operation).with_resource("worker_id", id)),
        StatusCode::BAD_REQUEST,
    )
}

pub(crate) fn raid_events_load_failed(err: &AppError, ctx: ErrorContext) -> HttpAppError {
    raid_api_err(
        "RAID_EVENTS_LOAD_FAILED",
        format!("Failed to load events: {}", err),
        Some(ctx),
        StatusCode::INTERNAL_SERVER_ERROR,
    )
}

pub(crate) fn raid_snapshot_not_found() -> HttpAppError {
    raid_api_err(
        "RAID_SNAPSHOT_NOT_FOUND",
        "No snapshot available",
        Some(ErrorContext::new("raid_snapshot_get")),
        StatusCode::NOT_FOUND,
    )
}

pub(crate) fn raid_snapshot_load_failed(err: &AppError) -> HttpAppError {
    raid_api_err(
        "RAID_SNAPSHOT_LOAD_FAILED",
        format!("Failed to load snapshot: {}", err),
        Some(ErrorContext::new("raid_snapshot_get")),
        StatusCode::INTERNAL_SERVER_ERROR,
    )
}

pub(crate) fn raid_snapshot_create_failed(err: &AppError) -> HttpAppError {
    raid_api_err(
        "RAID_SNAPSHOT_CREATE_FAILED",
        format!("Failed to create snapshot: {}", err),
        Some(ErrorContext::new("raid_snapshot_create")),
        StatusCode::INTERNAL_SERVER_ERROR,
    )
}

pub(crate) fn raid_snapshot_restore_failed(err: &AppError) -> HttpAppError {
    raid_api_err(
        "RAID_SNAPSHOT_RESTORE_FAILED",
        format!("Failed to restore from snapshot: {}", err),
        Some(ErrorContext::new("raid_snapshot_restore")),
        StatusCode::INTERNAL_SERVER_ERROR,
    )
}

pub(crate) fn raid_gc_failed(err: &AppError) -> HttpAppError {
    raid_api_err(
        "RAID_GC_FAILED",
        format!("Garbage collection failed: {}", err),
        Some(ErrorContext::new("raid_gc")),
        StatusCode::INTERNAL_SERVER_ERROR,
    )
}

pub(crate) fn raid_strategy_status_failed(err: &AppError) -> HttpAppError {
    raid_api_err(
        "RAID_STRATEGY_STATUS_FAILED",
        format!("Failed to get strategy status: {}", err),
        Some(ErrorContext::new("raid_strategies")),
        StatusCode::INTERNAL_SERVER_ERROR,
    )
}

pub(crate) fn raid_rebalance_failed(err: &AppError) -> HttpAppError {
    raid_api_err(
        "RAID_REBALANCE_FAILED",
        format!("Failed to trigger rebalancing: {}", err),
        Some(ErrorContext::new("raid_rebalance").with_hint(
            "Local RAID mode does not support rebalancing; verify strategy configuration.",
        )),
        StatusCode::INTERNAL_SERVER_ERROR,
    )
}
