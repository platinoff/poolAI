//! UI i18n subsets moved from `i18n_core.js` (PH-S154 admin jobs/grid; PH-S162 auth/dash shell; PH-S197 updates-compat; PH-S207 monitoring; PH-S211 jobs-only patch; PH-S214 raid-only patch; PH-S217 grid-pricing-only patch).
//!
//! Admin jobs/grid: `window.__poolaiAdminI18nRust` on admin layout (PH-S154); jobs page uses slim `admin_jobs_patch` (PH-S211); raid — `admin_raid_patch` (PH-S214); monitoring — `admin_monitoring_patch` (PH-S220); updates-compat — `admin_updates_compat_patch` (PH-S221).
//! Auth + dashboard shell: `window.__poolaiAuthDashI18nRust` on login, dashboard layout, admin layout (PH-S162).

use std::collections::BTreeMap;

/// Single locale row: `(i18n key, translated value)`.
pub type I18nRow<'a> = (&'a str, &'a str);

/// English admin jobs keys (subset moved from `i18n_core.js`; PH-S211).
pub const ADMIN_JOBS_EN: &[I18nRow<'_>] = &[
    ("admin.page.jobs", "Jobs"),
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
];

/// English grid-pricing keys (subset moved from `i18n_core.js`; PH-S217 slim patch).
pub const ADMIN_GRID_PRICING_EN: &[I18nRow<'_>] = &[
    ("admin.page.gridPricing", "Grid pricing"),
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

/// Ukrainian admin jobs keys (PH-S211).
pub const ADMIN_JOBS_UK: &[I18nRow<'_>] = &[
    ("admin.page.jobs", "Задачі"),
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
];

/// English RAID admin keys (subset moved from `i18n_core.js`; PH-S214).
pub const ADMIN_RAID_EN: &[I18nRow<'_>] = &[
    ("admin.page.raid", "RAID Management"),
    ("admin.raidadm.loading", "Loading RAID admin…"),
    ("admin.raidadm.loadingArt", "Loading artifacts…"),
    ("admin.raidadm.errLoad", "Error loading RAID data: "),
    ("admin.raidadm.unknown", "Unknown"),
    ("admin.raidadm.strategyTitle", "RAID Strategy Status"),
    ("admin.raidadm.clusterTitle", "Cluster status"),
    ("admin.raidadm.raftTitle", "Raft consensus"),
    ("admin.raidadm.label.cluster", "Cluster:"),
    ("admin.raidadm.label.nodes", "Nodes:"),
    ("admin.raidadm.label.artCount", "Artifacts:"),
    ("admin.raidadm.label.storageUse", "Storage used:"),
    ("admin.raidadm.label.replStatus", "Replication:"),
    ("admin.raidadm.label.raftRole", "Role:"),
    ("admin.raidadm.label.raftTerm", "Term:"),
    ("admin.raidadm.label.raftLeader", "Leader ID:"),
    (
        "admin.raidadm.raftAbsent",
        "Raft not attached (build without consensus or node not initialized).",
    ),
    ("admin.raidadm.label.mode", "Mode:"),
    ("admin.raidadm.label.init", "Initialized:"),
    ("admin.raidadm.label.active", "Active:"),
    ("admin.raidadm.label.rebal", "Rebalancing Enabled:"),
    ("admin.raidadm.burstTitle", "BurstRAID Metrics"),
    ("admin.raidadm.label.totalArt", "Total Artifacts:"),
    ("admin.raidadm.label.artBurst", "Artifacts in Burst:"),
    ("admin.raidadm.label.repl", "Replication (base / max):"),
    ("admin.raidadm.label.totalReq", "Total Requests:"),
    ("admin.raidadm.label.burstRps", "Burst threshold (RPS):"),
    ("admin.raidadm.label.swRepl", "Base replication:"),
    (
        "admin.raidadm.burstInactive",
        "BurstRAID strategy not active on this node.",
    ),
    (
        "admin.raidadm.swInactive",
        "SmallWorld strategy not active on this node.",
    ),
    ("admin.raidadm.sparkBurstPct", "Burst load %"),
    ("admin.raidadm.sparkClust", "Avg clustering ×100"),
    ("admin.raidadm.btn.rebalance", "Rebalance"),
    ("admin.raidadm.swTitle", "SmallWorld Network Metrics"),
    ("admin.raidadm.label.totalNodes", "Total Nodes:"),
    (
        "admin.raidadm.label.avgClust",
        "Avg Clustering Coefficient:",
    ),
    ("admin.raidadm.label.tgtClust", "Target Clustering:"),
    (
        "admin.raidadm.confirmRebal",
        "Are you sure you want to trigger RAID rebalancing? This may impact system performance.",
    ),
    ("admin.raidadm.rebalOk", "Rebalancing triggered successfully"),
    ("admin.raidadm.emptyArt", "No artifacts found"),
    ("admin.raidadm.col.id", "ID"),
    ("admin.raidadm.col.name", "Name"),
    ("admin.raidadm.col.size", "Size"),
    ("admin.raidadm.snapshotTitle", "Current Snapshot"),
    ("admin.raidadm.seq", "Sequence:"),
    ("admin.raidadm.created", "Created:"),
    ("admin.raidadm.restoreBtn", "Restore from Snapshot"),
    ("admin.raidadm.noSnap", "No snapshot available"),
    ("admin.raidadm.artTitle", "Artifacts"),
    ("admin.raidadm.section", "RAID Artifacts"),
    ("admin.raidadm.btn.upload", "Upload Artifact"),
    ("admin.raidadm.btn.snapshot", "Create Snapshot"),
    ("admin.raidadm.btn.sync", "Sync Artifacts"),
    ("admin.raidadm.btn.gc", "Run GC"),
    ("admin.raidadm.uploadTitle", "Upload Artifact"),
    ("admin.raidadm.label.artName", "Artifact Name"),
    ("admin.raidadm.ph.artName", "my-artifact"),
    ("admin.raidadm.label.artData", "Artifact Data (Base64)"),
    ("admin.raidadm.ph.b64", "Paste base64-encoded data here"),
    (
        "admin.raidadm.hint.b64",
        "Paste base64-encoded artifact data",
    ),
    ("admin.raidadm.uploading", "Uploading…"),
    ("admin.raidadm.reqNameData", "Name and data are required"),
    ("admin.raidadm.uploadOk", "Artifact uploaded successfully"),
    (
        "admin.raidadm.confirmDelArt",
        "Delete artifact \"{id}\"? This action cannot be undone.",
    ),
    ("admin.raidadm.delOk", "Artifact deleted successfully"),
    (
        "admin.raidadm.confirmSnap",
        "Create a new snapshot? This will capture the current state of all artifacts.",
    ),
    ("admin.raidadm.snapOk", "Snapshot created successfully"),
    ("admin.raidadm.errSnap", "Error creating snapshot: "),
    (
        "admin.raidadm.confirmRestore",
        "Restore from snapshot? This will restore the RAID state from the latest snapshot. This action cannot be undone.",
    ),
    ("admin.raidadm.restoring", "Restoring from snapshot…"),
    ("admin.raidadm.restoreOk", "Restored from snapshot successfully"),
    ("admin.raidadm.errRestore", "Error restoring from snapshot: "),
    ("admin.raidadm.syncOk", "Artifacts sync started"),
    ("admin.raidadm.errSync", "Error syncing artifacts: "),
    (
        "admin.raidadm.confirmGc",
        "Run garbage collection? This will remove old artifacts that are no longer referenced.",
    ),
    (
        "admin.raidadm.gcOk",
        "Garbage collection completed. Removed {n} artifacts.",
    ),
    ("admin.raidadm.errGc", "Error running GC: "),
];

/// Ukrainian RAID admin keys (PH-S214).
pub const ADMIN_RAID_UK: &[I18nRow<'_>] = &[
    ("admin.page.raid", "Керування RAID"),
    ("admin.raidadm.loading", "Завантаження RAID…"),
    ("admin.raidadm.loadingArt", "Завантаження артефактів…"),
    ("admin.raidadm.errLoad", "Помилка завантаження RAID: "),
    ("admin.raidadm.unknown", "Невідомо"),
    ("admin.raidadm.strategyTitle", "Статус стратегії RAID"),
    ("admin.raidadm.clusterTitle", "Статус кластера"),
    ("admin.raidadm.raftTitle", "Raft консенсус"),
    ("admin.raidadm.label.cluster", "Кластер:"),
    ("admin.raidadm.label.nodes", "Вузли:"),
    ("admin.raidadm.label.artCount", "Артефакти:"),
    ("admin.raidadm.label.storageUse", "Сховище:"),
    ("admin.raidadm.label.replStatus", "Реплікація:"),
    ("admin.raidadm.label.raftRole", "Роль:"),
    ("admin.raidadm.label.raftTerm", "Термін:"),
    ("admin.raidadm.label.raftLeader", "ID лідера:"),
    (
        "admin.raidadm.raftAbsent",
        "Raft не підключено (збірка без консенсусу або вузол не ініціалізовано).",
    ),
    ("admin.raidadm.label.mode", "Режим:"),
    ("admin.raidadm.label.init", "Ініціалізовано:"),
    ("admin.raidadm.label.active", "Активно:"),
    ("admin.raidadm.label.rebal", "Ребалансування:"),
    ("admin.raidadm.burstTitle", "Метрики BurstRAID"),
    ("admin.raidadm.label.totalArt", "Усього артефактів:"),
    ("admin.raidadm.label.artBurst", "У burst:"),
    ("admin.raidadm.label.repl", "Реплікація (база / макс):"),
    ("admin.raidadm.label.totalReq", "Усього запитів:"),
    ("admin.raidadm.label.burstRps", "Поріг burst (RPS):"),
    ("admin.raidadm.label.swRepl", "Базова реплікація:"),
    (
        "admin.raidadm.burstInactive",
        "Стратегія BurstRAID не активна на цьому вузлі.",
    ),
    (
        "admin.raidadm.swInactive",
        "Стратегія SmallWorld не активна на цьому вузлі.",
    ),
    ("admin.raidadm.sparkBurstPct", "Навантаження burst %"),
    ("admin.raidadm.sparkClust", "Середній clustering ×100"),
    ("admin.raidadm.btn.rebalance", "Ребаланс"),
    ("admin.raidadm.swTitle", "Мережа SmallWorld"),
    ("admin.raidadm.label.totalNodes", "Усього вузлів:"),
    ("admin.raidadm.label.avgClust", "Сер. коеф. кластеризації:"),
    ("admin.raidadm.label.tgtClust", "Цільовий коеф.:"),
    (
        "admin.raidadm.confirmRebal",
        "Запустити ребалансування RAID? Можливе навантаження на систему.",
    ),
    ("admin.raidadm.rebalOk", "Ребалансування запущено"),
    ("admin.raidadm.emptyArt", "Артефактів не знайдено"),
    ("admin.raidadm.col.id", "ID"),
    ("admin.raidadm.col.name", "Назва"),
    ("admin.raidadm.col.size", "Розмір"),
    ("admin.raidadm.snapshotTitle", "Поточний знімок"),
    ("admin.raidadm.seq", "Послідовність:"),
    ("admin.raidadm.created", "Створено:"),
    ("admin.raidadm.restoreBtn", "Відновити зі знімка"),
    ("admin.raidadm.noSnap", "Знімка недоступна"),
    ("admin.raidadm.artTitle", "Артефакти"),
    ("admin.raidadm.section", "Артефакти RAID"),
    ("admin.raidadm.btn.upload", "Завантажити артефакт"),
    ("admin.raidadm.btn.snapshot", "Створити знімок"),
    ("admin.raidadm.btn.sync", "Синхронізувати"),
    ("admin.raidadm.btn.gc", "Збір сміття"),
    ("admin.raidadm.uploadTitle", "Завантажити артефакт"),
    ("admin.raidadm.label.artName", "Назва артефакту"),
    ("admin.raidadm.ph.artName", "my-artifact"),
    ("admin.raidadm.label.artData", "Дані (Base64)"),
    ("admin.raidadm.ph.b64", "Вставте дані в Base64"),
    (
        "admin.raidadm.hint.b64",
        "Дані артефакту в кодуванні Base64",
    ),
    ("admin.raidadm.uploading", "Завантаження…"),
    ("admin.raidadm.reqNameData", "Потрібні назва і дані"),
    ("admin.raidadm.uploadOk", "Артефакт завантажено"),
    (
        "admin.raidadm.confirmDelArt",
        "Видалити артефакт «{id}»? Дію не скасувати.",
    ),
    ("admin.raidadm.delOk", "Артефакт видалено"),
    (
        "admin.raidadm.confirmSnap",
        "Створити знімок поточного стану всіх артефактів?",
    ),
    ("admin.raidadm.snapOk", "Знімок створено"),
    ("admin.raidadm.errSnap", "Помилка створення знімка: "),
    (
        "admin.raidadm.confirmRestore",
        "Відновити зі знімка? Стан RAID буде замінено. Дію не скасувати.",
    ),
    ("admin.raidadm.restoring", "Відновлення зі знімка…"),
    ("admin.raidadm.restoreOk", "Відновлено зі знімка"),
    ("admin.raidadm.errRestore", "Помилка відновлення: "),
    ("admin.raidadm.syncOk", "Синхронізацію розпочато"),
    ("admin.raidadm.errSync", "Помилка синхронізації: "),
    (
        "admin.raidadm.confirmGc",
        "Запустити збір сміття? Старі непотрібні артефакти будуть видалені.",
    ),
    (
        "admin.raidadm.gcOk",
        "Збір сміття завершено. Видалено артефактів: {n}.",
    ),
    ("admin.raidadm.errGc", "Помилка GC: "),
];

/// Ukrainian grid-pricing keys (PH-S217 slim patch).
pub const ADMIN_GRID_PRICING_UK: &[I18nRow<'_>] = &[
    ("admin.page.gridPricing", "Ціни Grid"),
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

/// English updates & compatibility admin keys (PH-S197; moved from `i18n_core.js`).
pub const ADMIN_UPDATES_COMPAT_EN: &[I18nRow<'_>] = &[
    ("admin.page.updatesCompat", "Updates & compatibility"),
    ("admin.updatesCompat.section", "Updates & compatibility"),
    (
        "admin.updatesCompat.hint",
        "Read-only Galaxy governance pointers. Policy prose lives in docs — not duplicated here.",
    ),
    ("admin.updatesCompat.protocolTitle", "Protocol version"),
    (
        "admin.updatesCompat.protocolHint",
        "Coordinator wire baseline (PH-S65 protocol_compat). Workers send protocol_version on POST /api/v1/discovery/register-remote.",
    ),
    ("admin.updatesCompat.col.coordinator", "Coordinator protocol"),
    ("admin.updatesCompat.col.build", "Coordinator build"),
    ("admin.updatesCompat.col.env", "Env override"),
    (
        "admin.updatesCompat.col.negotiation",
        "Default negotiation (no worker version)",
    ),
    (
        "admin.updatesCompat.compatStatusHint",
        "Registration may return compat_status with HTTP 403/426 when the worker is outside the matrix window.",
    ),
    ("admin.updatesCompat.verifyTitle", "Verify signed release"),
    (
        "admin.updatesCompat.verifyHint",
        "Operator quickstart — poolai-verify-release (PH-S66/S85). See SECURITY_HARDENING for the full checklist.",
    ),
    ("admin.updatesCompat.link.security", "SECURITY_HARDENING.md"),
    (
        "admin.updatesCompat.link.verifyQuickstart",
        "Verify-release quickstart (PH-S71)",
    ),
    ("admin.updatesCompat.link.manifest", "RELEASE_MANIFEST_SAMPLE.md"),
    ("admin.updatesCompat.link.fixtures", "Dev fixtures README"),
    ("admin.updatesCompat.matrixTitle", "Protocol compatibility matrix"),
    (
        "admin.updatesCompat.matrixHint",
        "Canonical compat matrix and negotiation rules — Galaxy §9.3. Implementation: src/grid/protocol_compat.rs.",
    ),
    (
        "admin.updatesCompat.link.matrix",
        "Galaxy §9.3 compat matrix (docs)",
    ),
];

/// Ukrainian updates & compatibility admin keys (PH-S197).
pub const ADMIN_UPDATES_COMPAT_UK: &[I18nRow<'_>] = &[
    ("admin.page.updatesCompat", "Оновлення та сумісність"),
    ("admin.updatesCompat.section", "Оновлення та сумісність"),
    (
        "admin.updatesCompat.hint",
        "Read-only вказівники Galaxy governance. Повна політика — у docs, без дублювання тут.",
    ),
    ("admin.updatesCompat.protocolTitle", "Версія протоколу"),
    (
        "admin.updatesCompat.protocolHint",
        "Базовий wire coordinator (PH-S65 protocol_compat). Воркери надсилають protocol_version на POST /api/v1/discovery/register-remote.",
    ),
    ("admin.updatesCompat.col.coordinator", "Протокол coordinator"),
    ("admin.updatesCompat.col.build", "Збірка coordinator"),
    ("admin.updatesCompat.col.env", "Env override"),
    (
        "admin.updatesCompat.col.negotiation",
        "Negotiation за замовчуванням (без версії воркера)",
    ),
    (
        "admin.updatesCompat.compatStatusHint",
        "Реєстрація може повернути compat_status з HTTP 403/426, якщо воркер поза вікном матриці.",
    ),
    ("admin.updatesCompat.verifyTitle", "Перевірка підписаного релізу"),
    (
        "admin.updatesCompat.verifyHint",
        "Operator quickstart — poolai-verify-release (PH-S66/S85). Повний чекліст — SECURITY_HARDENING.",
    ),
    ("admin.updatesCompat.link.security", "SECURITY_HARDENING.md"),
    (
        "admin.updatesCompat.link.verifyQuickstart",
        "Verify-release quickstart (PH-S71)",
    ),
    ("admin.updatesCompat.link.manifest", "RELEASE_MANIFEST_SAMPLE.md"),
    ("admin.updatesCompat.link.fixtures", "Dev fixtures README"),
    ("admin.updatesCompat.matrixTitle", "Матриця сумісності протоколу"),
    (
        "admin.updatesCompat.matrixHint",
        "Канонічна матриця та правила negotiation — Galaxy §9.3. Код: src/grid/protocol_compat.rs.",
    ),
    (
        "admin.updatesCompat.link.matrix",
        "Galaxy §9.3 compat matrix (docs)",
    ),
];

/// English monitoring admin keys (PH-S207; moved from `i18n_core.js`).
pub const ADMIN_MONITORING_EN: &[I18nRow<'_>] = &[
    ("admin.page.monitoring", "Monitoring Dashboard"),
    ("admin.mon.loading", "Loading monitoring…"),
    ("admin.mon.errLoad", "Error loading monitoring: "),
    ("admin.mon.noData", "No data available"),
    ("admin.mon.noAlerts", "No active alerts"),
    ("admin.mon.section", "Monitoring"),
    ("admin.mon.createDashBtn", "Create Dashboard"),
    ("admin.mon.createRuleBtn", "Create Alert Rule"),
    (
        "admin.mon.vizTitle",
        "Metrics Visualization (Last 24 Hours)",
    ),
    ("admin.mon.activeAlertsTitle", "Active Alerts"),
    ("admin.mon.dashboardsTitle", "Dashboards"),
    ("admin.mon.alertRulesTitle", "Alert Rules"),
    ("admin.mon.chartPoints", "{n} points"),
    ("admin.mon.statMin", "Min:"),
    ("admin.mon.statMax", "Max:"),
    ("admin.mon.statAvg", "Avg:"),
    ("admin.mon.col.severity", "Severity"),
    ("admin.mon.col.metric", "Metric"),
    ("admin.mon.col.currentVal", "Current Value"),
    ("admin.mon.col.threshold", "Threshold"),
    ("admin.mon.col.triggered", "Triggered"),
    ("admin.mon.col.statusCol", "Status"),
    ("admin.mon.col.actions", "Actions"),
    ("admin.mon.ackBtn", "Acknowledge"),
    ("admin.mon.statusAck", "Acknowledged"),
    ("admin.mon.statusActiveLbl", "Active"),
    ("admin.mon.noDashboards", "No dashboards created"),
    ("admin.mon.metricsN", "{n} metrics"),
    ("admin.mon.public", "Public"),
    ("admin.mon.private", "Private"),
    ("admin.mon.col.name", "Name"),
    ("admin.mon.col.description", "Description"),
    ("admin.mon.col.metrics", "Metrics"),
    ("admin.mon.col.public", "Public"),
    ("admin.mon.col.created", "Created"),
    ("admin.mon.col.operator", "Operator"),
    ("admin.mon.col.ruleStatus", "Status"),
    ("admin.mon.enabled", "Enabled"),
    ("admin.mon.disabled", "Disabled"),
    ("admin.mon.dashCreatedOk", "Dashboard created successfully"),
    ("admin.mon.ruleCreatedOk", "Alert rule created successfully"),
    ("admin.mon.alertAckOk", "Alert acknowledged"),
    ("admin.mon.creatingDash", "Creating…"),
    ("admin.mon.creatingRule", "Creating…"),
    ("admin.mon.modalCreateDash", "Create Dashboard"),
    ("admin.mon.modalCreateRule", "Create Alert Rule"),
    ("admin.mon.lbl.dashName", "Dashboard Name"),
    ("admin.mon.lbl.dashDesc", "Description"),
    ("admin.mon.lbl.dashMetrics", "Metrics (comma-separated)"),
    (
        "admin.mon.hint.dashMetrics",
        "Enter metric names separated by commas",
    ),
    ("admin.mon.lbl.dashLayout", "Layout (JSON, optional)"),
    ("admin.mon.lbl.dashPublic", "Public Dashboard"),
    ("admin.mon.lbl.ruleName", "Rule Name"),
    ("admin.mon.lbl.metricName", "Metric Name"),
    ("admin.mon.lbl.operator", "Operator"),
    ("admin.mon.lbl.threshold", "Threshold"),
    ("admin.mon.lbl.severity", "Severity"),
    ("admin.mon.op.gt", "Greater than (>)"),
    ("admin.mon.op.lt", "Less than (<)"),
    ("admin.mon.op.ge", "Greater or equal (>=)"),
    ("admin.mon.op.le", "Less or equal (<=)"),
    ("admin.mon.op.eq", "Equal (==)"),
    ("admin.mon.lbl.ruleEnabled", "Enabled"),
    ("admin.mon.mlTitle", "ML Pipeline Step Metrics"),
    ("admin.mon.mlEmpty", "No ML pipeline step metrics yet"),
    (
        "admin.mon.mlEmptyHint",
        "Run the demo pipeline or execute a pipeline via the AI/ML API.",
    ),
    ("admin.mon.mlUnavailable", "ML pipeline API unavailable"),
    (
        "admin.mon.mlUnavailableHint",
        "Build with enterprise + ml features to enable AI/ML pipelines.",
    ),
    ("admin.mon.mlDemoBtn", "Run ML Demo"),
    ("admin.mon.mlDemoRunning", "Running demo…"),
    ("admin.mon.mlDemoOk", "ML demo pipeline completed"),
    ("admin.mon.mlCol.pipeline", "Pipeline"),
    ("admin.mon.mlCol.step", "Step"),
    ("admin.mon.mlCol.kind", "Kind"),
    ("admin.mon.mlCol.status", "Status"),
    ("admin.mon.mlCol.metrics", "Metrics"),
    ("admin.mon.ph.dashboard", "My Dashboard"),
    ("admin.mon.ph.dashDesc", "Dashboard description"),
    (
        "admin.mon.ph.metricsCsv",
        "cpu_usage, memory_usage, request_rate",
    ),
    ("admin.mon.ph.layoutJson", r#"{"widgets": []}"#),
    ("admin.mon.ph.ruleName", "high-cpu-alert"),
    ("admin.mon.ph.metric", "cpu_usage"),
];

/// Ukrainian monitoring admin keys (PH-S207).
pub const ADMIN_MONITORING_UK: &[I18nRow<'_>] = &[
    ("admin.page.monitoring", "Моніторинг"),
    ("admin.mon.loading", "Завантаження моніторингу…"),
    ("admin.mon.errLoad", "Помилка завантаження моніторингу: "),
    ("admin.mon.noData", "Немає даних"),
    ("admin.mon.noAlerts", "Немає активних сповіщень"),
    ("admin.mon.section", "Моніторинг"),
    ("admin.mon.createDashBtn", "Створити дашборд"),
    ("admin.mon.createRuleBtn", "Створити правило сповіщень"),
    ("admin.mon.vizTitle", "Візуалізація метрик (останні 24 год)"),
    ("admin.mon.activeAlertsTitle", "Активні сповіщення"),
    ("admin.mon.dashboardsTitle", "Дашборди"),
    ("admin.mon.alertRulesTitle", "Правила сповіщень"),
    ("admin.mon.chartPoints", "точок: {n}"),
    ("admin.mon.statMin", "Мін:"),
    ("admin.mon.statMax", "Макс:"),
    ("admin.mon.statAvg", "Сер:"),
    ("admin.mon.col.severity", "Важливість"),
    ("admin.mon.col.metric", "Метрика"),
    ("admin.mon.col.currentVal", "Поточне значення"),
    ("admin.mon.col.threshold", "Поріг"),
    ("admin.mon.col.triggered", "Час спрацювання"),
    ("admin.mon.col.statusCol", "Статус"),
    ("admin.mon.col.actions", "Дії"),
    ("admin.mon.ackBtn", "Підтвердити"),
    ("admin.mon.statusAck", "Підтверджено"),
    ("admin.mon.statusActiveLbl", "Активне"),
    ("admin.mon.noDashboards", "Дашбордів не створено"),
    ("admin.mon.metricsN", "Метрик: {n}"),
    ("admin.mon.public", "Публічний"),
    ("admin.mon.private", "Приватний"),
    ("admin.mon.col.name", "Назва"),
    ("admin.mon.col.description", "Опис"),
    ("admin.mon.col.metrics", "Метрики"),
    ("admin.mon.col.public", "Публічний"),
    ("admin.mon.col.created", "Створено"),
    ("admin.mon.col.operator", "Оператор"),
    ("admin.mon.col.ruleStatus", "Статус"),
    ("admin.mon.enabled", "Увімкнено"),
    ("admin.mon.disabled", "Вимкнено"),
    ("admin.mon.dashCreatedOk", "Дашборд створено"),
    ("admin.mon.ruleCreatedOk", "Правило сповіщень створено"),
    ("admin.mon.alertAckOk", "Сповіщення підтверджено"),
    ("admin.mon.creatingDash", "Створення…"),
    ("admin.mon.creatingRule", "Створення…"),
    ("admin.mon.modalCreateDash", "Створити дашборд"),
    ("admin.mon.modalCreateRule", "Створити правило сповіщень"),
    ("admin.mon.lbl.dashName", "Назва дашборду"),
    ("admin.mon.lbl.dashDesc", "Опис"),
    ("admin.mon.lbl.dashMetrics", "Метрики (через кому)"),
    ("admin.mon.hint.dashMetrics", "Назви метрик через кому"),
    ("admin.mon.lbl.dashLayout", "Макет (JSON, необов’язково)"),
    ("admin.mon.lbl.dashPublic", "Публічний дашборд"),
    ("admin.mon.lbl.ruleName", "Назва правила"),
    ("admin.mon.lbl.metricName", "Назва метрики"),
    ("admin.mon.lbl.operator", "Оператор"),
    ("admin.mon.lbl.threshold", "Поріг"),
    ("admin.mon.lbl.severity", "Важливість"),
    ("admin.mon.op.gt", "Більше ніж (>)"),
    ("admin.mon.op.lt", "Менше ніж (<)"),
    ("admin.mon.op.ge", "Більше або дорівнює (>=)"),
    ("admin.mon.op.le", "Менше або дорівнює (<=)"),
    ("admin.mon.op.eq", "Дорівнює (==)"),
    ("admin.mon.lbl.ruleEnabled", "Увімкнено"),
    ("admin.mon.mlTitle", "Метрики кроків ML pipeline"),
    ("admin.mon.mlEmpty", "Метрик кроків ML pipeline ще немає"),
    (
        "admin.mon.mlEmptyHint",
        "Запустіть demo pipeline або виконайте pipeline через AI/ML API.",
    ),
    ("admin.mon.mlUnavailable", "ML pipeline API недоступний"),
    (
        "admin.mon.mlUnavailableHint",
        "Зберіть з features enterprise + ml для AI/ML pipelines.",
    ),
    ("admin.mon.mlDemoBtn", "Запустити ML Demo"),
    ("admin.mon.mlDemoRunning", "Demo виконується…"),
    ("admin.mon.mlDemoOk", "ML demo pipeline завершено"),
    ("admin.mon.mlCol.pipeline", "Pipeline"),
    ("admin.mon.mlCol.step", "Крок"),
    ("admin.mon.mlCol.kind", "Тип"),
    ("admin.mon.mlCol.status", "Статус"),
    ("admin.mon.mlCol.metrics", "Метрики"),
    ("admin.mon.ph.dashboard", "Мій дашборд"),
    ("admin.mon.ph.dashDesc", "Опис дашборду"),
    (
        "admin.mon.ph.metricsCsv",
        "cpu_usage, memory_usage, request_rate",
    ),
    ("admin.mon.ph.layoutJson", r#"{"widgets": []}"#),
    ("admin.mon.ph.ruleName", "high-cpu-alert"),
    ("admin.mon.ph.metric", "cpu_usage"),
];

/// English auth keys (login, OAuth, bootstrap banner, lang toggle).
pub const AUTH_SHELL_EN: &[I18nRow<'_>] = &[
    ("auth.pageTitle", "Login - PoolAI"),
    ("auth.cardTitle", "Login"),
    ("auth.username", "Username"),
    ("auth.password", "Password"),
    ("auth.submit", "Login"),
    ("auth.loggingIn", "Logging in…"),
    ("auth.loginFailed", "Login failed"),
    ("auth.oauthStartFail", "Failed to start OAuth2 login: "),
    ("auth.oauthFail", "OAuth2 authentication failed: "),
    ("auth.oauthTokenFail", "Failed to process OAuth2 token: "),
    ("auth.signInWithAria", "Sign in with {provider}"),
    ("auth.oauthOr", "Or sign in with:"),
    ("auth.providerTelegram", "Telegram"),
    ("auth.telegramPageTitle", "Telegram sign-in"),
    ("auth.telegramSignIn", "Sign in with Telegram"),
    ("auth.telegramCloseHint", "You can close this page after signing in."),
    ("auth.testAccounts", "Test accounts:"),
    ("auth.testAdmin", "Admin: admin / admin123"),
    ("auth.testOperator", "Operator: operator / op123"),
    ("auth.testViewer", "Viewer: viewer / view123"),
    (
        "auth.bootstrapLine1",
        "First launch: you are signed in as the built-in administrator. Login: admin — password: admin123.",
    ),
    (
        "auth.bootstrapLine2",
        "Change the password in Admin → Users, or continue with the default and update it anytime in settings.",
    ),
    ("auth.bootstrapUsersLink", "Admin → Users"),
    ("auth.bootstrapDismiss", "Got it"),
    ("auth.lang.en", "EN"),
    ("auth.lang.uk", "UA"),
];

/// Ukrainian auth keys.
pub const AUTH_SHELL_UK: &[I18nRow<'_>] = &[
    ("auth.pageTitle", "Вхід - PoolAI"),
    ("auth.cardTitle", "Вхід"),
    ("auth.username", "Користувач"),
    ("auth.password", "Пароль"),
    ("auth.submit", "Увійти"),
    ("auth.loggingIn", "Вхід…"),
    ("auth.loginFailed", "Не вдалося увійти"),
    ("auth.oauthStartFail", "Не вдалося розпочати OAuth2: "),
    ("auth.oauthFail", "Помилка OAuth2: "),
    ("auth.oauthTokenFail", "Не вдалося обробити токен OAuth2: "),
    ("auth.signInWithAria", "Увійти через {provider}"),
    ("auth.oauthOr", "Або увійдіть через:"),
    ("auth.providerTelegram", "Telegram"),
    ("auth.telegramPageTitle", "Вхід через Telegram"),
    ("auth.telegramSignIn", "Увійдіть через Telegram"),
    ("auth.telegramCloseHint", "Після входу цю сторінку можна закрити."),
    ("auth.testAccounts", "Тестові обліковки:"),
    ("auth.testAdmin", "Адмін: admin / admin123"),
    ("auth.testOperator", "Оператор: operator / op123"),
    ("auth.testViewer", "Глядач: viewer / view123"),
    (
        "auth.bootstrapLine1",
        "Перший запуск: ви увійшли як вбудований адміністратор. Логін: admin — пароль: admin123.",
    ),
    (
        "auth.bootstrapLine2",
        "Змініть пароль у Адмінка → Користувачі або продовжуйте зі штатним паролем і оновіть його будь-коли в налаштуваннях.",
    ),
    ("auth.bootstrapUsersLink", "Адмінка → Користувачі"),
    ("auth.bootstrapDismiss", "Зрозуміло"),
    ("auth.lang.en", "EN"),
    ("auth.lang.uk", "UA"),
];

/// English dashboard shell keys (nav, theme, search, titles).
pub const DASH_SHELL_EN: &[I18nRow<'_>] = &[
    ("dash.brand", "PoolAI UI"),
    ("dash.subtitle", "Dashboard with write operations (Stage 3)"),
    ("dash.skipMain", "Skip to main content"),
    ("dash.skipNav", "Skip to navigation"),
    ("dash.nav.home", "Home"),
    ("dash.nav.status", "Status"),
    ("dash.nav.health", "Health"),
    ("dash.nav.metrics", "Metrics"),
    ("dash.nav.workers", "Workers"),
    ("dash.nav.libs", "Libs"),
    ("dash.nav.vm", "VM"),
    ("dash.nav.raid", "RAID"),
    ("dash.nav.admin", "Admin"),
    ("dash.aria.admin", "Enterprise admin panel"),
    ("dash.aria.mainNav", "Main navigation"),
    ("dash.aria.home", "Home page"),
    ("dash.aria.status", "System status"),
    ("dash.aria.health", "Health check"),
    ("dash.aria.metrics", "System metrics"),
    ("dash.aria.workers", "Worker management"),
    ("dash.aria.libs", "Library management"),
    ("dash.aria.vm", "VM instance management"),
    ("dash.aria.raid", "RAID artifact management"),
    ("dash.aria.mobileNav", "Mobile navigation"),
    ("dash.aria.openMenu", "Open navigation menu"),
    ("dash.aria.closeMenu", "Close navigation menu"),
    ("dash.menuTitle", "Menu"),
    ("dash.themeLabel", "Theme:"),
    ("dash.aria.theme", "Select theme"),
    ("dash.themeOptDark", "🌙 Dark"),
    ("dash.themeOptLight", "☀️ Light"),
    ("dash.themeOptHC", "🔆 High Contrast"),
    ("dash.login", "Login"),
    ("dash.logout", "Logout"),
    (
        "dash.pageAutoRefresh",
        "Auto-refresh is enabled (5s). Write operations are available for authenticated users with appropriate permissions.",
    ),
    ("dash.title.home", "Home"),
    ("dash.title.status", "Status"),
    ("dash.title.health", "Health"),
    ("dash.title.metrics", "Metrics"),
    ("dash.title.workers", "Workers"),
    ("dash.title.libraries", "Libraries"),
    ("dash.title.vm", "VM Instances"),
    ("dash.title.raid", "RAID"),
    ("dash.updatedPrefix", "Updated:"),
    ("dash.search.title", "Search"),
    ("dash.search.closeAria", "Close search dialog"),
    ("dash.search.placeholder", "Search pages, workers, artifacts..."),
    ("dash.search.inputAria", "Search input"),
    ("dash.search.category.page", "Page"),
    ("dash.search.item.home", "Home"),
    ("dash.search.item.status", "Status"),
    ("dash.search.item.health", "Health"),
    ("dash.search.item.metrics", "Metrics"),
    ("dash.search.item.workers", "Workers"),
    ("dash.search.item.libs", "Libraries"),
    ("dash.search.item.vm", "VM Instances"),
    ("dash.search.item.raid", "RAID"),
    ("dash.search.typeToSearch", "Type to search..."),
    ("dash.search.noResults", "No results found"),
    ("dash.search.resultsAria", "Search results"),
    ("dash.search.oneResult", "1 result found"),
    ("dash.search.manyResults", "{count} results found"),
    ("dash.search.resultsWithCount", "Search results: {status}"),
];

/// Ukrainian dashboard shell keys.
pub const DASH_SHELL_UK: &[I18nRow<'_>] = &[
    ("dash.brand", "PoolAI UI"),
    ("dash.subtitle", "Панель з операціями запису (етап 3)"),
    ("dash.skipMain", "Перейти до основного вмісту"),
    ("dash.skipNav", "Перейти до навігації"),
    ("dash.nav.home", "Головна"),
    ("dash.nav.status", "Статус"),
    ("dash.nav.health", "Здоров\u{2019}я"),
    ("dash.nav.metrics", "Метрики"),
    ("dash.nav.workers", "Воркери"),
    ("dash.nav.libs", "Бібліотеки"),
    ("dash.nav.vm", "VM"),
    ("dash.nav.raid", "RAID"),
    ("dash.nav.admin", "Адмінка"),
    ("dash.aria.admin", "Панель адміністратора (enterprise)"),
    ("dash.aria.mainNav", "Головна навігація"),
    ("dash.aria.home", "Головна сторінка"),
    ("dash.aria.status", "Статус системи"),
    ("dash.aria.health", "Перевірка здоров\u{2019}я"),
    ("dash.aria.metrics", "Метрики системи"),
    ("dash.aria.workers", "Керування воркерами"),
    ("dash.aria.libs", "Керування бібліотеками"),
    ("dash.aria.vm", "Керування інстансами VM"),
    ("dash.aria.raid", "Керування артефактами RAID"),
    ("dash.aria.mobileNav", "Мобільна навігація"),
    ("dash.aria.openMenu", "Відкрити меню навігації"),
    ("dash.aria.closeMenu", "Закрити меню навігації"),
    ("dash.menuTitle", "Меню"),
    ("dash.themeLabel", "Тема:"),
    ("dash.aria.theme", "Обрати тему"),
    ("dash.themeOptDark", "🌙 Темна"),
    ("dash.themeOptLight", "☀️ Світла"),
    ("dash.themeOptHC", "🔆 Високий контраст"),
    ("dash.login", "Увійти"),
    ("dash.logout", "Вийти"),
    (
        "dash.pageAutoRefresh",
        "Автооновлення кожні 5 с. Операції запису доступні автентифікованим користувачам з відповідними правами.",
    ),
    ("dash.title.home", "Головна"),
    ("dash.title.status", "Статус"),
    ("dash.title.health", "Здоров\u{2019}я"),
    ("dash.title.metrics", "Метрики"),
    ("dash.title.workers", "Воркери"),
    ("dash.title.libraries", "Бібліотеки"),
    ("dash.title.vm", "Інстанси VM"),
    ("dash.title.raid", "RAID"),
    ("dash.updatedPrefix", "Оновлено:"),
    ("dash.search.title", "Пошук"),
    ("dash.search.closeAria", "Закрити діалог пошуку"),
    ("dash.search.placeholder", "Пошук сторінок, воркерів, артефактів..."),
    ("dash.search.inputAria", "Поле пошуку"),
    ("dash.search.category.page", "Сторінка"),
    ("dash.search.item.home", "Головна"),
    ("dash.search.item.status", "Статус"),
    ("dash.search.item.health", "Здоров\u{02BC}я"),
    ("dash.search.item.metrics", "Метрики"),
    ("dash.search.item.workers", "Воркери"),
    ("dash.search.item.libs", "Бібліотеки"),
    ("dash.search.item.vm", "Інстанси VM"),
    ("dash.search.item.raid", "RAID"),
    ("dash.search.typeToSearch", "Введіть запит для пошуку..."),
    ("dash.search.noResults", "Нічого не знайдено"),
    ("dash.search.resultsAria", "Результати пошуку"),
    ("dash.search.oneResult", "Знайдено 1 результат"),
    ("dash.search.manyResults", "Знайдено результатів: {count}"),
    ("dash.search.resultsWithCount", "Результати пошуку: {status}"),
];

fn rows_to_map(rows: &[I18nRow<'_>]) -> BTreeMap<String, String> {
    rows.iter()
        .map(|(k, v)| ((*k).to_string(), (*v).to_string()))
        .collect()
}

/// `{"en":{...},"uk":{...}}` patch — jobs page only (PH-S211).
pub fn admin_jobs_patch() -> BTreeMap<String, BTreeMap<String, String>> {
    let mut root = BTreeMap::new();
    root.insert("en".into(), rows_to_map(ADMIN_JOBS_EN));
    root.insert("uk".into(), rows_to_map(ADMIN_JOBS_UK));
    root
}

/// JSON literal for jobs-only admin patch.
pub fn admin_jobs_patch_json() -> String {
    serde_json::to_string(&admin_jobs_patch()).expect("admin jobs i18n patch serializes")
}

/// Inline script for jobs admin layout (PH-S211).
pub fn admin_jobs_patch_script() -> String {
    format!("window.__poolaiAdminI18nRust={};", admin_jobs_patch_json())
}

/// `{"en":{...},"uk":{...}}` patch — raid page only (PH-S214).
pub fn admin_raid_patch() -> BTreeMap<String, BTreeMap<String, String>> {
    let mut root = BTreeMap::new();
    root.insert("en".into(), rows_to_map(ADMIN_RAID_EN));
    root.insert("uk".into(), rows_to_map(ADMIN_RAID_UK));
    root
}

/// JSON literal for raid-only admin patch.
pub fn admin_raid_patch_json() -> String {
    serde_json::to_string(&admin_raid_patch()).expect("admin raid i18n patch serializes")
}

/// Inline script for raid admin layout (PH-S214).
pub fn admin_raid_patch_script() -> String {
    format!("window.__poolaiAdminI18nRust={};", admin_raid_patch_json())
}

/// `{"en":{...},"uk":{...}}` patch — grid-pricing page only (PH-S217).
pub fn admin_grid_pricing_patch() -> BTreeMap<String, BTreeMap<String, String>> {
    let mut root = BTreeMap::new();
    root.insert("en".into(), rows_to_map(ADMIN_GRID_PRICING_EN));
    root.insert("uk".into(), rows_to_map(ADMIN_GRID_PRICING_UK));
    root
}

/// JSON literal for grid-pricing-only admin patch.
pub fn admin_grid_pricing_patch_json() -> String {
    serde_json::to_string(&admin_grid_pricing_patch())
        .expect("admin grid-pricing i18n patch serializes")
}

/// Inline script for grid-pricing admin layout (PH-S217).
pub fn admin_grid_pricing_patch_script() -> String {
    format!(
        "window.__poolaiAdminI18nRust={};",
        admin_grid_pricing_patch_json()
    )
}

/// Monitoring admin page — slim `admin.mon.*` patch only (PH-S220).
pub fn admin_monitoring_patch() -> BTreeMap<String, BTreeMap<String, String>> {
    let mut root = BTreeMap::new();
    root.insert("en".into(), rows_to_map(ADMIN_MONITORING_EN));
    root.insert("uk".into(), rows_to_map(ADMIN_MONITORING_UK));
    root
}

pub fn admin_monitoring_patch_json() -> String {
    serde_json::to_string(&admin_monitoring_patch())
        .expect("admin monitoring i18n patch serializes")
}

pub fn admin_monitoring_patch_script() -> String {
    format!(
        "window.__poolaiAdminI18nRust={};",
        admin_monitoring_patch_json()
    )
}

/// Updates-compat admin page — slim `admin.updatesCompat.*` patch only (PH-S221).
pub fn admin_updates_compat_patch() -> BTreeMap<String, BTreeMap<String, String>> {
    let mut root = BTreeMap::new();
    root.insert("en".into(), rows_to_map(ADMIN_UPDATES_COMPAT_EN));
    root.insert("uk".into(), rows_to_map(ADMIN_UPDATES_COMPAT_UK));
    root
}

pub fn admin_updates_compat_patch_json() -> String {
    serde_json::to_string(&admin_updates_compat_patch())
        .expect("admin updates-compat i18n patch serializes")
}

pub fn admin_updates_compat_patch_script() -> String {
    format!(
        "window.__poolaiAdminI18nRust={};",
        admin_updates_compat_patch_json()
    )
}

/// Default admin layout — slim `admin.jobs.*` patch only (PH-S221; updates-compat on dedicated page).
pub fn admin_jobs_grid_patch() -> BTreeMap<String, BTreeMap<String, String>> {
    let mut root = BTreeMap::new();
    root.insert("en".into(), rows_to_map(ADMIN_JOBS_EN));
    root.insert("uk".into(), rows_to_map(ADMIN_JOBS_UK));
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

fn merge_rows(map: &mut BTreeMap<String, String>, rows: &[I18nRow<'_>]) {
    for (k, v) in rows {
        map.insert((*k).to_string(), (*v).to_string());
    }
}

/// `{"en":{...},"uk":{...}}` patch for auth + dashboard shell keys (PH-S162).
pub fn auth_dash_shell_patch() -> BTreeMap<String, BTreeMap<String, String>> {
    let mut en = BTreeMap::new();
    merge_rows(&mut en, AUTH_SHELL_EN);
    merge_rows(&mut en, DASH_SHELL_EN);
    let mut uk = BTreeMap::new();
    merge_rows(&mut uk, AUTH_SHELL_UK);
    merge_rows(&mut uk, DASH_SHELL_UK);
    let mut root = BTreeMap::new();
    root.insert("en".into(), en);
    root.insert("uk".into(), uk);
    root
}

/// JSON literal assigned to `window.__poolaiAuthDashI18nRust`.
pub fn auth_dash_shell_patch_json() -> String {
    serde_json::to_string(&auth_dash_shell_patch()).expect("auth/dash i18n patch serializes")
}

/// Inline script for login, dashboard layout, and admin layout (auth bootstrap + lang toggle).
pub fn auth_dash_shell_patch_script() -> String {
    format!(
        "window.__poolaiAuthDashI18nRust={};",
        auth_dash_shell_patch_json()
    )
}

/// Lookup EN string by key (tests / server-side parity).
pub fn t_en(key: &str) -> Option<&'static str> {
    for rows in [
        ADMIN_JOBS_EN,
        ADMIN_GRID_PRICING_EN,
        ADMIN_UPDATES_COMPAT_EN,
        ADMIN_MONITORING_EN,
    ] {
        if let Some((_, v)) = rows.iter().find(|(k, _)| *k == key) {
            return Some(*v);
        }
    }
    None
}

/// Lookup UK string by key (tests / server-side parity).
pub fn t_uk(key: &str) -> Option<&'static str> {
    for rows in [
        ADMIN_JOBS_UK,
        ADMIN_GRID_PRICING_UK,
        ADMIN_UPDATES_COMPAT_UK,
        ADMIN_MONITORING_UK,
    ] {
        if let Some((_, v)) = rows.iter().find(|(k, _)| *k == key) {
            return Some(*v);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn patch_has_matching_en_uk_key_counts() {
        assert_eq!(ADMIN_JOBS_EN.len(), ADMIN_JOBS_UK.len());
        assert_eq!(ADMIN_GRID_PRICING_EN.len(), ADMIN_GRID_PRICING_UK.len());
        assert_eq!(ADMIN_UPDATES_COMPAT_EN.len(), ADMIN_UPDATES_COMPAT_UK.len());
        assert_eq!(ADMIN_MONITORING_EN.len(), ADMIN_MONITORING_UK.len());
        let patch = admin_jobs_grid_patch();
        assert_eq!(patch["en"].len(), ADMIN_JOBS_EN.len());
        assert_eq!(patch["uk"].len(), ADMIN_JOBS_UK.len());
    }

    #[test]
    fn jobs_patch_json_jobs_only_ph_s211() {
        let json = admin_jobs_patch_json();
        assert!(json.contains(r#""admin.jobs.leaseState.active""#));
        assert!(json.contains(r#""admin.page.jobs""#));
        assert!(!json.contains(r#""admin.gridPricing.col.price""#));
        assert!(!json.contains(r#""admin.mon.mlTitle""#));
    }

    #[test]
    fn raid_patch_has_matching_en_uk_key_counts_ph_s214() {
        assert_eq!(ADMIN_RAID_EN.len(), ADMIN_RAID_UK.len());
    }

    #[test]
    fn raid_patch_json_raid_only_ph_s214() {
        let json = admin_raid_patch_json();
        assert!(json.contains(r#""admin.page.raid""#));
        assert!(json.contains(r#""admin.raidadm.section""#));
        assert!(!json.contains(r#""admin.jobs.leaseState.active""#));
        assert!(!json.contains(r#""admin.gridPricing.col.price""#));
    }

    #[test]
    fn raid_patch_script_assigns_window_patch_ph_s214() {
        let script = admin_raid_patch_script();
        assert!(script.starts_with("window.__poolaiAdminI18nRust="));
        assert!(script.ends_with(';'));
    }

    #[test]
    fn grid_pricing_patch_has_matching_en_uk_key_counts_ph_s217() {
        assert_eq!(ADMIN_GRID_PRICING_EN.len(), ADMIN_GRID_PRICING_UK.len());
    }

    #[test]
    fn grid_pricing_patch_json_grid_only_ph_s217() {
        let json = admin_grid_pricing_patch_json();
        assert!(json.contains(r#""admin.page.gridPricing""#));
        assert!(json.contains(r#""admin.gridPricing.col.price""#));
        assert!(!json.contains(r#""admin.jobs.leaseState.active""#));
        assert!(!json.contains(r#""admin.mon.mlTitle""#));
    }

    #[test]
    fn patch_json_contains_jobs_only_not_grid_or_updates_ph_s221() {
        let json = admin_jobs_grid_patch_json();
        assert!(json.contains(r#""admin.jobs.leaseState.active""#));
        assert!(!json.contains(r#""admin.gridPricing.col.price""#));
        assert!(json.contains(r#""admin.jobs.status.migrating""#));
        assert!(!json.contains(r#""admin.updatesCompat.section""#));
        assert!(!json.contains(r#""admin.mon.mlTitle""#));
        assert!(!json.contains(r#""admin.page.monitoring""#));
    }

    #[test]
    fn monitoring_patch_has_matching_en_uk_key_counts_ph_s220() {
        assert_eq!(ADMIN_MONITORING_EN.len(), ADMIN_MONITORING_UK.len());
    }

    #[test]
    fn monitoring_patch_json_monitoring_only_ph_s220() {
        let json = admin_monitoring_patch_json();
        assert!(json.contains(r#""admin.page.monitoring""#));
        assert!(json.contains(r#""admin.mon.mlTitle""#));
        assert!(!json.contains(r#""admin.jobs.leaseState.active""#));
        assert!(!json.contains(r#""admin.updatesCompat.section""#));
    }

    #[test]
    fn updates_compat_patch_has_matching_en_uk_key_counts_ph_s221() {
        assert_eq!(ADMIN_UPDATES_COMPAT_EN.len(), ADMIN_UPDATES_COMPAT_UK.len());
    }

    #[test]
    fn updates_compat_patch_json_updates_only_ph_s221() {
        let json = admin_updates_compat_patch_json();
        assert!(json.contains(r#""admin.page.updatesCompat""#));
        assert!(json.contains(r#""admin.updatesCompat.section""#));
        assert!(!json.contains(r#""admin.jobs.leaseState.active""#));
        assert!(!json.contains(r#""admin.mon.mlTitle""#));
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

    #[test]
    fn auth_dash_patch_has_matching_en_uk_key_counts() {
        assert_eq!(AUTH_SHELL_EN.len(), AUTH_SHELL_UK.len());
        assert_eq!(DASH_SHELL_EN.len(), DASH_SHELL_UK.len());
        let patch = auth_dash_shell_patch();
        let expected = AUTH_SHELL_EN.len() + DASH_SHELL_EN.len();
        assert_eq!(patch["en"].len(), expected);
        assert_eq!(patch["uk"].len(), expected);
    }

    #[test]
    fn auth_dash_patch_json_contains_auth_and_dash_keys() {
        let json = auth_dash_shell_patch_json();
        assert!(json.contains(r#""auth.pageTitle""#));
        assert!(json.contains(r#""auth.bootstrapDismiss""#));
        assert!(json.contains(r#""dash.nav.workers""#));
        assert!(json.contains(r#""dash.search.manyResults""#));
    }

    #[test]
    fn auth_dash_script_assigns_window_patch() {
        let script = auth_dash_shell_patch_script();
        assert!(script.starts_with("window.__poolaiAuthDashI18nRust="));
        assert!(script.ends_with(';'));
    }
}
