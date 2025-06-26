use std::fmt;
use thiserror::Error;
use std::io;
use serde_json;
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use log::{info, warn, error};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use std::time::Duration;
use uuid;

#[derive(Error, Debug)]
pub enum PoolAIError {
    #[error("IO error: {0}")]
    IoError(io::Error),
    #[error("JSON error: {0}")]
    JsonError(serde_json::Error),
    #[error("Serialization error: {0}")]
    SerializationError(serde::ser::Error),
    #[error("Deserialization error: {0}")]
    DeserializationError(serde::de::Error),
    #[error("Sync error: {0}")]
    SyncError(tokio::sync::MutexError),
    #[error("Log error: {0}")]
    LogError(log::SetLoggerError),
    #[error("Collection error: {0}")]
    CollectionError(std::collections::CollectionError),
    #[error("Chrono error: {0}")]
    ChronoError(chrono::Error),
    #[error("Time error: {0}")]
    TimeError(std::time::SystemTimeError),
    #[error("UUID error: {0}")]
    UuidError(uuid::Error),
}

impl From<io::Error> for PoolAIError {
    fn from(e: io::Error) -> Self {
        Self::IoError(e)
    }
}

impl From<serde_json::Error> for PoolAIError {
    fn from(e: serde_json::Error) -> Self {
        Self::JsonError(e)
    }
}

impl From<serde::ser::Error> for PoolAIError {
    fn from(e: serde::ser::Error) -> Self {
        Self::SerializationError(e)
    }
}

impl From<serde::de::Error> for PoolAIError {
    fn from(e: serde::de::Error) -> Self {
        Self::DeserializationError(e)
    }
}

impl From<tokio::sync::MutexError> for PoolAIError {
    fn from(e: tokio::sync::MutexError) -> Self {
        Self::SyncError(e)
    }
}

impl From<log::SetLoggerError> for PoolAIError {
    fn from(e: log::SetLoggerError) -> Self {
        Self::LogError(e)
    }
}

impl From<std::collections::CollectionError> for PoolAIError {
    fn from(e: std::collections::CollectionError) -> Self {
        Self::CollectionError(e)
    }
}

impl From<chrono::Error> for PoolAIError {
    fn from(e: chrono::Error) -> Self {
        Self::ChronoError(e)
    }
}

impl From<std::time::SystemTimeError> for PoolAIError {
    fn from(e: std::time::SystemTimeError) -> Self {
        Self::TimeError(e)
    }
}

impl From<uuid::Error> for PoolAIError {
    fn from(e: uuid::Error) -> Self {
        Self::UuidError(e)
    }
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Model error: {0}")]
    ModelError(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Unknown error")]
    Unknown,
}

impl AppError {
    pub fn log(&self) {
        match self {
            AppError::ModelError(msg) => log::error!("Model error: {}", msg),
            AppError::IoError(e) => log::error!("IO error: {}", e),
            AppError::Unknown => log::error!("Unknown error"),
        }
    }
    // Восстановление после сбоя (заглушка)
    pub fn recover(&self) {
        log::warn!("Attempting recovery from error: {:?}", self);
    }
} 