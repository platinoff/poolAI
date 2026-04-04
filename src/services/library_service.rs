//! Library manager operations for the HTTP API.

use crate::core::error::AppError;
use crate::core::state::ApiContext;
use crate::libs::{LibraryInfo, LibraryType};

pub const LIBRARY_MANAGER_UNAVAILABLE_MESSAGE: &str = "Library manager not initialized. Context: Library manager is not available. Suggestion: Ensure library manager is initialized before managing libraries. Check system startup sequence and library manager initialization status.";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LibraryServiceError {
    ManagerUnavailable,
}

#[derive(Debug)]
pub enum LibraryMutationError {
    ManagerUnavailable,
    Operation(AppError),
}

pub struct LibraryService;

impl LibraryService {
    pub async fn list_libraries(ctx: &ApiContext) -> Result<Vec<LibraryInfo>, LibraryServiceError> {
        let arc = ctx
            .library_manager
            .get()
            .cloned()
            .ok_or(LibraryServiceError::ManagerUnavailable)?;
        let manager = arc.read().await;
        Ok(manager.list_libraries().await)
    }

    pub async fn get_library(
        ctx: &ApiContext,
        name: &str,
    ) -> Result<Option<LibraryInfo>, LibraryServiceError> {
        let arc = ctx
            .library_manager
            .get()
            .cloned()
            .ok_or(LibraryServiceError::ManagerUnavailable)?;
        let manager = arc.read().await;
        Ok(manager.get_library(name).await)
    }

    pub async fn install_library(
        ctx: &ApiContext,
        name: &str,
        version: &str,
        library_type: LibraryType,
    ) -> Result<LibraryInfo, LibraryMutationError> {
        let arc = ctx
            .library_manager
            .get()
            .cloned()
            .ok_or(LibraryMutationError::ManagerUnavailable)?;
        let manager = arc.read().await;
        manager
            .install_library(name, version, library_type)
            .await
            .map_err(LibraryMutationError::Operation)
    }

    pub async fn uninstall_library(
        ctx: &ApiContext,
        name: &str,
    ) -> Result<(), LibraryMutationError> {
        let arc = ctx
            .library_manager
            .get()
            .cloned()
            .ok_or(LibraryMutationError::ManagerUnavailable)?;
        let manager = arc.read().await;
        manager
            .uninstall_library(name)
            .await
            .map_err(LibraryMutationError::Operation)
    }

    pub async fn update_library(
        ctx: &ApiContext,
        name: &str,
    ) -> Result<LibraryInfo, LibraryMutationError> {
        let arc = ctx
            .library_manager
            .get()
            .cloned()
            .ok_or(LibraryMutationError::ManagerUnavailable)?;
        let manager = arc.read().await;
        manager
            .update_library(name)
            .await
            .map_err(LibraryMutationError::Operation)
    }

    pub async fn upload_library(
        ctx: &ApiContext,
        name: &str,
        version: &str,
        base64_data: &str,
        library_type: LibraryType,
    ) -> Result<LibraryInfo, LibraryMutationError> {
        let arc = ctx
            .library_manager
            .get()
            .cloned()
            .ok_or(LibraryMutationError::ManagerUnavailable)?;
        let manager = arc.read().await;
        manager
            .upload_library(name, version, base64_data, library_type)
            .await
            .map_err(LibraryMutationError::Operation)
    }
}
