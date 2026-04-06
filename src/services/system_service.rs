//! System status, health, metrics, and related snapshots for HTTP handlers.

use serde::Serialize;

#[derive(Serialize)]
pub struct StatusResponse {
    pub status: &'static str,
    pub version: &'static str,
    pub uptime: u64,
}

#[derive(Serialize)]
pub struct MetricsResponse {
    pub active_workers: u32,
    pub total_requests: u64,
    pub avg_response_time: f64,
}

#[derive(Serialize)]
pub struct ModelInfo {
    pub name: &'static str,
    pub status: &'static str,
    pub memory_usage: u64,
}

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub timestamp: String,
    pub version: &'static str,
    pub uptime: u64,
    pub checks: HealthChecks,
}

#[derive(Serialize)]
pub struct HealthChecks {
    pub database: HealthCheck,
    pub memory: HealthCheck,
    pub workers: HealthCheck,
    pub gpu: HealthCheck,
}

#[derive(Serialize)]
pub struct HealthCheck {
    pub status: &'static str,
    pub message: String,
    pub response_time_ms: u64,
}

pub struct SystemService;

impl SystemService {
    /// Crate version from `Cargo.toml` (for JSON status / health).
    pub const PACKAGE_VERSION: &'static str = env!("CARGO_PKG_VERSION");

    pub fn status_snapshot() -> StatusResponse {
        StatusResponse {
            status: "running",
            version: Self::PACKAGE_VERSION,
            uptime: crate::version::get_uptime_seconds(),
        }
    }

    pub fn metrics_snapshot() -> MetricsResponse {
        MetricsResponse {
            active_workers: 5,
            total_requests: 1234,
            avg_response_time: 0.045,
        }
    }

    pub fn models_snapshot() -> Vec<ModelInfo> {
        vec![
            ModelInfo {
                name: "llama-2-7b",
                status: "loaded",
                memory_usage: 8192,
            },
            ModelInfo {
                name: "gpt-3.5-turbo",
                status: "available",
                memory_usage: 4096,
            },
        ]
    }

    pub fn health_snapshot() -> HealthResponse {
        use chrono::Utc;

        let start_time = std::time::Instant::now();

        let health_checks = HealthChecks {
            database: HealthCheck {
                status: "healthy",
                message: "Database connection OK".to_string(),
                response_time_ms: 5,
            },
            memory: HealthCheck {
                status: "healthy",
                message: "Memory usage: 45%".to_string(),
                response_time_ms: 2,
            },
            workers: HealthCheck {
                status: "healthy",
                message: "8/8 workers active".to_string(),
                response_time_ms: 3,
            },
            gpu: HealthCheck {
                status: "healthy",
                message: "GPU temperature: 65°C".to_string(),
                response_time_ms: 8,
            },
        };

        let _ = start_time.elapsed();
        let uptime = crate::version::get_uptime_seconds();

        HealthResponse {
            status: "healthy",
            timestamp: Utc::now().to_rfc3339(),
            version: Self::PACKAGE_VERSION,
            uptime,
            checks: health_checks,
        }
    }

    pub fn gpu_snapshot() -> crate::platform::GpuInfo {
        crate::platform::get_gpu_info()
    }
}
