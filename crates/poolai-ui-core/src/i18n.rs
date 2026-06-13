//! Admin i18n subset for jobs + grid-pricing (PH-S154).
//!
//! EN/UK strings are injected on admin pages via `window.__poolaiAdminI18nRust`
//! before `i18n_core.js` loads. Parity: `src/ui/admin/jobs.rs`, `grid_pricing.rs`.

use std::collections::BTreeMap;

/// Single locale row: `(i18n key, translated value)`.
pub type I18nRow<'a> = (&'a str, &'a str);

/// English admin jobs + grid-pricing keys (subset moved from `i18n_core.js`).
pub const ADMIN_JOBS_GRID_EN: &[I18nRow<'_>] = &[
    ("admin.jobs.section", "Jobs"),
    ("admin.jobs.loading", "Loading jobs…"),
    ("admin.jobs.empty", "No jobs yet"),
    ("admin.jobs.errLoad", "Error loading jobs: "),
    ("admin.jobs.storeLoading", "Loading store…"),
    ("admin.jobs.storeLabel", "Store:"),
    (
        "admin.jobs.storeHint",
        "Job persistence backend (POOLAI_JOB_STORE)",
    ),
    (
        "admin.jobs.hint",
        "Job queue from the coordinator store. Backend is set at startup via POOLAI_JOB_STORE. Lease columns are read-only; lease state badge is derived from lease_expires_at (Galaxy §4.3.1).",
    ),
    ("admin.jobs.store.json", "JSON"),
    ("admin.jobs.store.sqlite", "SQLite"),
    ("admin.jobs.store.raid", "RAID"),
    ("admin.jobs.col.id", "ID"),
    ("admin.jobs.col.kind", "Kind"),
    ("admin.jobs.col.status", "Status"),
    ("admin.jobs.col.created", "Created"),
    ("admin.jobs.col.worker", "Worker"),
    ("admin.jobs.col.vm", "VM"),
    ("admin.jobs.col.leaseOwner", "Lease owner"),
    ("admin.jobs.col.leaseEpoch", "Epoch"),
    ("admin.jobs.col.leaseState", "Lease state"),
    ("admin.jobs.col.leaseExpires", "Lease expires"),
    (
        "admin.jobs.tooltip.leaseOwnerCol",
        "Galaxy §4.3.1: holder of the active job lease",
    ),
    (
        "admin.jobs.tooltip.leaseEpochCol",
        "Monotonic CAS counter for renew / PATCH / grid result",
    ),
    (
        "admin.jobs.tooltip.leaseStateCol",
        "Derived from lease_expires_at vs coordinator clock",
    ),
    (
        "admin.jobs.tooltip.leaseExpiresCol",
        "UTC expiry; worker renew extends this timestamp",
    ),
    (
        "admin.jobs.tooltip.leaseOwner",
        "Galaxy §4.3.1: worker or peer id holding the active lease (acquire/renew CAS owner).",
    ),
    (
        "admin.jobs.tooltip.leaseEpoch",
        "Monotonic CAS generation; PATCH, renew, and grid result must match this epoch.",
    ),
    ("admin.jobs.leaseState.active", "Active"),
    ("admin.jobs.leaseState.expired", "Expired"),
    ("admin.jobs.status.migrating", "Migrating"),
    (
        "admin.jobs.tooltip.statusMigrating",
        "Galaxy re-migrate: worker handoff in progress (PH-S104).",
    ),
    ("admin.gridPricing.section", "Grid pricing"),
    (
        "admin.gridPricing.hint",
        "Read-only Galaxy pricing oracle snapshot. Configure L2 fallback via POOLAI_GALAXY_PRICING_FALLBACK_JSON when providers are unavailable.",
    ),
    ("admin.gridPricing.fetch", "Fetch snapshot"),
    ("admin.gridPricing.taskProfile", "Task profile"),
    ("admin.gridPricing.modelProfile", "Model profile"),
    ("admin.gridPricing.unitKey", "Unit key"),
    ("admin.gridPricing.loading", "Loading pricing…"),
    ("admin.gridPricing.errLoad", "Error loading pricing: "),
    ("admin.gridPricing.result", "Pricing snapshot"),
    ("admin.gridPricing.col.task", "Task profile"),
    ("admin.gridPricing.col.model", "Model profile"),
    ("admin.gridPricing.col.unit", "Unit key"),
    ("admin.gridPricing.col.price", "Price (USD)"),
    ("admin.gridPricing.col.updated", "Updated at"),
    ("admin.gridPricing.col.source", "Source"),
    ("admin.gridPricing.col.freshness", "Freshness"),
];

/// Ukrainian admin jobs + grid-pricing keys.
pub const ADMIN_JOBS_GRID_UK: &[I18nRow<'_>] = &[
    ("admin.jobs.section", "Задачі"),
    ("admin.jobs.loading", "Завантаження задач…"),
    ("admin.jobs.empty", "Задач ще немає"),
    ("admin.jobs.errLoad", "Помилка завантаження задач: "),
    ("admin.jobs.storeLoading", "Завантаження сховища…"),
    ("admin.jobs.storeLabel", "Сховище:"),
    (
        "admin.jobs.storeHint",
        "Бекенд персистенції задач (POOLAI_JOB_STORE)",
    ),
    (
        "admin.jobs.hint",
        "Черга задач координатора. Бекенд — POOLAI_JOB_STORE. Колонки lease read-only; badge стану lease формується з lease_expires_at (Galaxy §4.3.1).",
    ),
    ("admin.jobs.store.json", "JSON"),
    ("admin.jobs.store.sqlite", "SQLite"),
    ("admin.jobs.store.raid", "RAID"),
    ("admin.jobs.col.id", "ID"),
    ("admin.jobs.col.kind", "Тип"),
    ("admin.jobs.col.status", "Статус"),
    ("admin.jobs.col.created", "Створено"),
    ("admin.jobs.col.worker", "Воркер"),
    ("admin.jobs.col.vm", "VM"),
    ("admin.jobs.col.leaseOwner", "Власник lease"),
    ("admin.jobs.col.leaseEpoch", "Epoch"),
    ("admin.jobs.col.leaseState", "Стан lease"),
    ("admin.jobs.col.leaseExpires", "Lease до"),
    (
        "admin.jobs.tooltip.leaseOwnerCol",
        "Galaxy §4.3.1: хто тримає активний lease задачі",
    ),
    (
        "admin.jobs.tooltip.leaseEpochCol",
        "Монотонний CAS для renew / PATCH / grid result",
    ),
    (
        "admin.jobs.tooltip.leaseStateCol",
        "З lease_expires_at відносно годинника координатора",
    ),
    (
        "admin.jobs.tooltip.leaseExpiresCol",
        "UTC закінчення; worker renew подовжує timestamp",
    ),
    (
        "admin.jobs.tooltip.leaseOwner",
        "Galaxy §4.3.1: id воркера або peer з активним lease (acquire/renew CAS).",
    ),
    (
        "admin.jobs.tooltip.leaseEpoch",
        "Монотонний CAS; PATCH, renew і grid result мають збігатися з цим epoch.",
    ),
    ("admin.jobs.leaseState.active", "Активний"),
    ("admin.jobs.leaseState.expired", "Протермінований"),
    ("admin.jobs.status.migrating", "Міграція"),
    (
        "admin.jobs.tooltip.statusMigrating",
        "Galaxy re-migrate: передача воркера в процесі (PH-S104).",
    ),
    ("admin.gridPricing.section", "Ціни Grid"),
    (
        "admin.gridPricing.hint",
        "Read-only знімок Galaxy pricing oracle. L2 fallback — POOLAI_GALAXY_PRICING_FALLBACK_JSON, якщо провайдери недоступні.",
    ),
    ("admin.gridPricing.fetch", "Отримати знімок"),
    ("admin.gridPricing.taskProfile", "Task profile"),
    ("admin.gridPricing.modelProfile", "Model profile"),
    ("admin.gridPricing.unitKey", "Unit key"),
    ("admin.gridPricing.loading", "Завантаження знімка цін…"),
    ("admin.gridPricing.errLoad", "Помилка завантаження цін: "),
    ("admin.gridPricing.result", "Знімок цін"),
    ("admin.gridPricing.col.task", "Task profile"),
    ("admin.gridPricing.col.model", "Model profile"),
    ("admin.gridPricing.col.unit", "Unit key"),
    ("admin.gridPricing.col.price", "Ціна (USD)"),
    ("admin.gridPricing.col.updated", "Оновлено"),
    ("admin.gridPricing.col.source", "Джерело"),
    ("admin.gridPricing.col.freshness", "Свіжість"),
];

fn rows_to_map(rows: &[I18nRow<'_>]) -> BTreeMap<String, String> {
    rows.iter()
        .map(|(k, v)| ((*k).to_string(), (*v).to_string()))
        .collect()
}

/// `{"en":{...},"uk":{...}}` patch object for `window.__poolaiAdminI18nRust`.
pub fn admin_jobs_grid_patch() -> BTreeMap<String, BTreeMap<String, String>> {
    let mut root = BTreeMap::new();
    root.insert("en".into(), rows_to_map(ADMIN_JOBS_GRID_EN));
    root.insert("uk".into(), rows_to_map(ADMIN_JOBS_GRID_UK));
    root
}

/// JSON literal assigned to `window.__poolaiAdminI18nRust`.
pub fn admin_jobs_grid_patch_json() -> String {
    serde_json::to_string(&admin_jobs_grid_patch()).expect("admin i18n patch serializes")
}

/// Inline script body (no `<script>` wrapper) for admin layout injection.
pub fn admin_jobs_grid_patch_script() -> String {
    format!(
        "window.__poolaiAdminI18nRust={};",
        admin_jobs_grid_patch_json()
    )
}

/// Lookup EN string by key (tests / server-side parity).
pub fn t_en(key: &str) -> Option<&'static str> {
    ADMIN_JOBS_GRID_EN
        .iter()
        .find(|(k, _)| *k == key)
        .map(|(_, v)| *v)
}

/// Lookup UK string by key (tests / server-side parity).
pub fn t_uk(key: &str) -> Option<&'static str> {
    ADMIN_JOBS_GRID_UK
        .iter()
        .find(|(k, _)| *k == key)
        .map(|(_, v)| *v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn patch_has_matching_en_uk_key_counts() {
        assert_eq!(ADMIN_JOBS_GRID_EN.len(), ADMIN_JOBS_GRID_UK.len());
        let patch = admin_jobs_grid_patch();
        assert_eq!(patch["en"].len(), ADMIN_JOBS_GRID_EN.len());
        assert_eq!(patch["uk"].len(), ADMIN_JOBS_GRID_UK.len());
    }

    #[test]
    fn patch_json_contains_jobs_and_grid_keys() {
        let json = admin_jobs_grid_patch_json();
        assert!(json.contains(r#""admin.jobs.leaseState.active""#));
        assert!(json.contains(r#""admin.gridPricing.col.price""#));
        assert!(json.contains(r#""admin.jobs.status.migrating""#));
    }

    #[test]
    fn script_assigns_window_patch() {
        let script = admin_jobs_grid_patch_script();
        assert!(script.starts_with("window.__poolaiAdminI18nRust="));
        assert!(script.ends_with(';'));
    }

    #[test]
    fn t_en_uk_lease_labels() {
        assert_eq!(t_en("admin.jobs.leaseState.active"), Some("Active"));
        assert_eq!(
            t_uk("admin.jobs.leaseState.expired"),
            Some("Протермінований")
        );
        assert_eq!(t_en("admin.gridPricing.result"), Some("Pricing snapshot"));
    }
}
