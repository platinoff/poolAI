//! Enterprise-facing operations for the HTTP API (multi-tenancy, audit, security, …).
//!
//! Handlers in `network::enterprise_api` stay thin: parse input, call `EnterpriseService`, map to HTTP.

use crate::core::error::AppError;
use crate::core::state::ApiContext;
use crate::enterprise::audit::{AuditEvent, AuditLevel, AuditQueryFilters};
use crate::enterprise::monitoring::{Alert, AlertRule, AlertSeverity, Dashboard, MetricDataPoint};
use crate::enterprise::multi_tenancy::{
    QuotaCheckResult, Tenant, TenantConfig, TenantResourceUsage,
};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// `create_tenant` runs `TenantManager::initialize` first; distinguish init vs create failures for HTTP mapping.
#[derive(Debug)]
pub enum TenantCreateError {
    Init(AppError),
    Create(AppError),
}

#[derive(Debug)]
pub enum EnterpriseAuditError {
    Init(AppError),
    Query(AppError),
}

/// HTTP query parameters for audit log search.
#[derive(Debug, Clone, Default)]
pub struct AuditEventsQuery {
    pub user_id: Option<String>,
    pub tenant_id: Option<String>,
    pub action: Option<String>,
    pub resource_type: Option<String>,
    pub result: Option<String>,
    pub min_level: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub limit: Option<usize>,
}

#[derive(Debug)]
pub enum EnterpriseMonitoringError {
    Init(AppError),
    Operation(AppError),
}

#[derive(Debug, Clone, Default)]
pub struct MonitoringAlertsQueryInput {
    pub severity: Option<String>,
    pub tenant_id: Option<String>,
    pub acknowledged: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct DashboardCreateInput {
    pub name: String,
    pub description: Option<String>,
    pub metrics: Vec<String>,
    pub layout: Option<String>,
    pub is_public: Option<bool>,
    pub tenant_id: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct MetricHistoryQueryInput {
    pub metric: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub tenant_id: Option<String>,
    pub limit: Option<usize>,
}

fn audit_filters_from_query(q: AuditEventsQuery) -> AuditQueryFilters {
    AuditQueryFilters {
        user_id: q.user_id,
        tenant_id: q.tenant_id,
        action: q.action,
        resource_type: q.resource_type,
        result: q.result,
        min_level: q
            .min_level
            .as_ref()
            .and_then(|level| match level.to_uppercase().as_str() {
                "INFO" => Some(AuditLevel::Info),
                "WARNING" => Some(AuditLevel::Warning),
                "ERROR" => Some(AuditLevel::Error),
                "CRITICAL" => Some(AuditLevel::Critical),
                _ => None,
            }),
        start_time: q.start_time.and_then(|s| {
            chrono::DateTime::parse_from_rfc3339(&s)
                .ok()
                .map(|dt| dt.with_timezone(&Utc))
        }),
        end_time: q.end_time.and_then(|s| {
            chrono::DateTime::parse_from_rfc3339(&s)
                .ok()
                .map(|dt| dt.with_timezone(&Utc))
        }),
        limit: q.limit,
    }
}

fn parse_alert_severity(severity: &Option<String>) -> Option<AlertSeverity> {
    severity
        .as_ref()
        .and_then(|s| match s.to_uppercase().as_str() {
            "INFO" => Some(AlertSeverity::Info),
            "WARNING" => Some(AlertSeverity::Warning),
            "ERROR" => Some(AlertSeverity::Error),
            "CRITICAL" => Some(AlertSeverity::Critical),
            _ => None,
        })
}

fn parse_rfc3339_utc(s: Option<String>) -> Option<DateTime<Utc>> {
    s.and_then(|raw| {
        chrono::DateTime::parse_from_rfc3339(&raw)
            .ok()
            .map(|dt| dt.with_timezone(&Utc))
    })
}

pub struct EnterpriseService;

impl EnterpriseService {
    pub async fn list_tenants(ctx: &ApiContext) -> Result<Vec<Tenant>, AppError> {
        ctx.tenant_manager.list_tenants().await
    }

    pub async fn create_tenant(
        ctx: &ApiContext,
        name: String,
        config: TenantConfig,
    ) -> Result<Tenant, TenantCreateError> {
        if let Err(e) = ctx.tenant_manager.initialize().await {
            return Err(TenantCreateError::Init(e));
        }
        ctx.tenant_manager
            .create_tenant(name, config)
            .await
            .map_err(TenantCreateError::Create)
    }

    pub async fn get_tenant(ctx: &ApiContext, id: Uuid) -> Result<Option<Tenant>, AppError> {
        ctx.tenant_manager.get_tenant(id).await
    }

    pub async fn update_tenant(
        ctx: &ApiContext,
        id: Uuid,
        config: Option<TenantConfig>,
        active: Option<bool>,
    ) -> Result<Tenant, AppError> {
        ctx.tenant_manager.update_tenant(id, config, active).await
    }

    pub async fn delete_tenant(ctx: &ApiContext, id: Uuid) -> Result<(), AppError> {
        ctx.tenant_manager.delete_tenant(id).await
    }

    pub async fn get_tenant_usage(
        ctx: &ApiContext,
        tenant_id: Uuid,
    ) -> Result<TenantResourceUsage, AppError> {
        ctx.tenant_manager.get_usage(tenant_id).await
    }

    pub async fn check_tenant_quota(
        ctx: &ApiContext,
        tenant_id: Uuid,
        workers: usize,
        memory_mb: u64,
        cpu_cores: usize,
        storage_mb: Option<u64>,
        vm_instances: Option<usize>,
    ) -> Result<QuotaCheckResult, AppError> {
        ctx.tenant_manager
            .check_quota(
                tenant_id,
                workers,
                memory_mb,
                cpu_cores,
                storage_mb,
                vm_instances,
            )
            .await
    }

    pub async fn query_audit_events(
        ctx: &ApiContext,
        q: AuditEventsQuery,
    ) -> Result<Vec<AuditEvent>, EnterpriseAuditError> {
        if let Err(e) = ctx.audit_logger.initialize().await {
            return Err(EnterpriseAuditError::Init(e));
        }
        let filters = audit_filters_from_query(q);
        ctx.audit_logger
            .query_events(&filters)
            .await
            .map_err(EnterpriseAuditError::Query)
    }

    async fn ensure_monitoring(ctx: &ApiContext) -> Result<(), EnterpriseMonitoringError> {
        ctx.enterprise_monitoring_manager
            .initialize()
            .await
            .map_err(EnterpriseMonitoringError::Init)
    }

    pub async fn list_monitoring_alerts(
        ctx: &ApiContext,
        q: MonitoringAlertsQueryInput,
    ) -> Result<Vec<Alert>, EnterpriseMonitoringError> {
        Self::ensure_monitoring(ctx).await?;
        let severity = parse_alert_severity(&q.severity);
        let tenant_id = q.tenant_id.and_then(|id| Uuid::parse_str(&id).ok());
        ctx.enterprise_monitoring_manager
            .get_active_alerts(severity, tenant_id, q.acknowledged)
            .await
            .map_err(EnterpriseMonitoringError::Operation)
    }

    pub async fn acknowledge_monitoring_alert(
        ctx: &ApiContext,
        alert_id: Uuid,
    ) -> Result<(), EnterpriseMonitoringError> {
        Self::ensure_monitoring(ctx).await?;
        ctx.enterprise_monitoring_manager
            .acknowledge_alert(alert_id)
            .await
            .map_err(EnterpriseMonitoringError::Operation)
    }

    pub async fn list_monitoring_dashboards(
        ctx: &ApiContext,
        tenant_id_str: Option<String>,
    ) -> Result<Vec<Dashboard>, EnterpriseMonitoringError> {
        Self::ensure_monitoring(ctx).await?;
        let tenant_id = tenant_id_str.and_then(|id| Uuid::parse_str(&id).ok());
        ctx.enterprise_monitoring_manager
            .list_dashboards(tenant_id)
            .await
            .map_err(EnterpriseMonitoringError::Operation)
    }

    pub async fn create_monitoring_dashboard(
        ctx: &ApiContext,
        input: DashboardCreateInput,
    ) -> Result<Dashboard, EnterpriseMonitoringError> {
        Self::ensure_monitoring(ctx).await?;
        let tenant_id = input.tenant_id.and_then(|id| Uuid::parse_str(&id).ok());
        let dashboard = Dashboard {
            id: Uuid::new_v4(),
            name: input.name,
            description: input.description.unwrap_or_default(),
            metrics: input.metrics,
            layout: input.layout.unwrap_or_else(|| "{}".to_string()),
            is_public: input.is_public.unwrap_or(false),
            tenant_id,
            created_at: Utc::now(),
        };
        ctx.enterprise_monitoring_manager
            .create_dashboard(dashboard.clone())
            .await
            .map_err(EnterpriseMonitoringError::Operation)?;
        Ok(dashboard)
    }

    pub async fn query_monitoring_metric_history(
        ctx: &ApiContext,
        q: MetricHistoryQueryInput,
    ) -> Result<Vec<MetricDataPoint>, EnterpriseMonitoringError> {
        Self::ensure_monitoring(ctx).await?;
        let start_time = parse_rfc3339_utc(q.start_time);
        let end_time = parse_rfc3339_utc(q.end_time);
        let tenant_id = q.tenant_id.and_then(|id| Uuid::parse_str(&id).ok());
        ctx.enterprise_monitoring_manager
            .get_metric_history(
                q.metric.as_deref(),
                start_time,
                end_time,
                tenant_id,
                q.limit,
            )
            .await
            .map_err(EnterpriseMonitoringError::Operation)
    }

    pub async fn list_monitoring_alert_rules(
        ctx: &ApiContext,
    ) -> Result<Vec<AlertRule>, EnterpriseMonitoringError> {
        Self::ensure_monitoring(ctx).await?;
        ctx.enterprise_monitoring_manager
            .list_alert_rules()
            .await
            .map_err(EnterpriseMonitoringError::Operation)
    }

    pub async fn create_monitoring_alert_rule(
        ctx: &ApiContext,
        rule: AlertRule,
    ) -> Result<(), EnterpriseMonitoringError> {
        Self::ensure_monitoring(ctx).await?;
        ctx.enterprise_monitoring_manager
            .create_alert_rule(rule)
            .await
            .map_err(EnterpriseMonitoringError::Operation)
    }
}
