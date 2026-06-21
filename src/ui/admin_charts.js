// PoolAI admin metrics charts (PH-S10) — SVG canvas glue; data parse via poolai-ui-wasm (PH-S155)

function poolaiChartT(key, enFallback) {
  return typeof poolaiT === 'function' ? poolaiT(key, enFallback) : enFallback;
}

function poolaiChartsWasm() {
  var w = window.poolaiUiWasm;
  return w && w.ready ? w : null;
}

function poolaiSanitizeChartId(name) {
  var wasm = poolaiChartsWasm();
  if (wasm && typeof wasm.sanitizeChartId === 'function') {
    return wasm.sanitizeChartId(name == null ? '' : String(name));
  }
  return String(name || 'metric').replace(/[^a-zA-Z0-9_-]/g, '_');
}

/** @param {Array<{value?: number}>} data */
function poolaiMetricPointValues(data) {
  var wasm = poolaiChartsWasm();
  if (wasm && typeof wasm.metricPointValues === 'function') {
    return wasm.metricPointValues(JSON.stringify(data || []));
  }
  return [];
}

function poolaiChartScale(values, width, height, padding) {
  var wasm = poolaiChartsWasm();
  if (wasm && typeof wasm.chartScale === 'function') {
    return wasm.chartScale(JSON.stringify(values || []), width, height, padding);
  }
  return { points: [], polyline: '', min: 0, max: 0, range: 1, chartWidth: width, chartHeight: height, padding: padding };
}

/**
 * @param {string} metricName
 * @param {{ hours?: number, limit?: number }} [opts]
 */
async function poolaiFetchMetricHistory(metricName, opts) {
  opts = opts || {};
  var hours = opts.hours != null ? opts.hours : 24;
  var limit = opts.limit != null ? opts.limit : 200;
  try {
    var endTime = new Date().toISOString();
    var startTime = new Date(Date.now() - hours * 60 * 60 * 1000).toISOString();
    var wasm = poolaiChartsWasm();
    var url =
      wasm && typeof wasm.buildMetricHistoryUrlWithHours === 'function'
        ? wasm.buildMetricHistoryUrlWithHours(metricName, hours, limit, endTime)
        : wasm && typeof wasm.buildMetricHistoryQuery === 'function'
          ? '/api/enterprise/monitoring/metrics?' +
            wasm.buildMetricHistoryQuery(metricName, startTime, endTime, limit)
          : wasm && typeof wasm.buildMetricHistoryUrl === 'function'
            ? wasm.buildMetricHistoryUrl(metricName, startTime, endTime, limit)
            : '/api/enterprise/monitoring/metrics?metric=' +
              encodeURIComponent(metricName) +
              '&start_time=' +
              encodeURIComponent(startTime) +
              '&end_time=' +
              encodeURIComponent(endTime) +
              '&limit=' +
              limit;
    var data = await fetchJson(url);
    return data || [];
  } catch (e) {
    console.error('poolaiFetchMetricHistory:', metricName, e);
    return [];
  }
}

/** Bulk metrics for a time window (all metric names in response). */
async function poolaiFetchMetricsWindow(opts) {
  opts = opts || {};
  var hours = opts.hours != null ? opts.hours : 1;
  var limit = opts.limit != null ? opts.limit : 60;
  try {
    var endTime = new Date().toISOString();
    var wasm = poolaiChartsWasm();
    var url =
      wasm && typeof wasm.buildMetricsWindowUrlWithHours === 'function'
        ? wasm.buildMetricsWindowUrlWithHours(hours, limit, endTime)
        : wasm && typeof wasm.buildMetricsWindowUrl === 'function'
          ? wasm.buildMetricsWindowUrl(
              new Date(Date.now() - hours * 60 * 60 * 1000).toISOString(),
              endTime,
              limit,
            )
          : '/api/enterprise/monitoring/metrics?start_time=' +
            encodeURIComponent(new Date(Date.now() - hours * 60 * 60 * 1000).toISOString()) +
            '&end_time=' +
            encodeURIComponent(endTime) +
            '&limit=' +
            limit;
    var data = await fetchJson(url);
    return data || [];
  } catch (e) {
    console.error('poolaiFetchMetricsWindow:', e);
    return [];
  }
}

function poolaiGroupMetricsByName(metrics) {
  var wasm = poolaiChartsWasm();
  if (wasm && typeof wasm.groupMetricsByName === 'function') {
    return wasm.groupMetricsByName(JSON.stringify(metrics || []));
  }
  return {};
}

function poolaiAlertRulesUrl() {
  var wasm = poolaiChartsWasm();
  return wasm && typeof wasm.buildAlertRulesUrl === 'function'
    ? wasm.buildAlertRulesUrl()
    : '/api/enterprise/monitoring/alert-rules';
}

function poolaiMonitoringDashboardsUrl() {
  var wasm = poolaiChartsWasm();
  return wasm && typeof wasm.buildMonitoringDashboardsUrl === 'function'
    ? wasm.buildMonitoringDashboardsUrl()
    : '/api/enterprise/monitoring/dashboards';
}

function poolaiMonitoringAlertAcknowledgeUrl(alertId) {
  var wasm = poolaiChartsWasm();
  return wasm && typeof wasm.buildMonitoringAlertAcknowledgeUrl === 'function'
    ? wasm.buildMonitoringAlertAcknowledgeUrl(String(alertId || ''))
    : '/api/enterprise/monitoring/alerts/' + encodeURIComponent(String(alertId || '')) + '/acknowledge';
}

function poolaiMonitoringMetricLatestUrl(metricName, limit) {
  var wasm = poolaiChartsWasm();
  var lim = limit != null ? limit : 10;
  return wasm && typeof wasm.buildMonitoringMetricLatestUrl === 'function'
    ? wasm.buildMonitoringMetricLatestUrl(String(metricName || ''), lim)
    : '/api/enterprise/monitoring/metrics?metric=' +
        encodeURIComponent(String(metricName || '')) +
        '&limit=' +
        lim;
}

/**
 * Full line chart for monitoring dashboards.
 * @param {string} metricName
 * @param {Array<{value?: number}>} data
 * @param {{ width?: number, height?: number, padding?: number }} [opts]
 */
function poolaiRenderLineChart(metricName, data, opts) {
  opts = opts || {};
  var wasm = poolaiChartsWasm();
  if (!data || data.length === 0) {
    var noData = poolaiChartT('admin.mon.noData', 'No data available');
    if (wasm && typeof wasm.renderLineChartEmptyHtml === 'function') {
      return wasm.renderLineChartEmptyHtml(noData);
    }
    return '<div class="muted">' + escapeHtml(noData) + '</div>';
  }

  var values = poolaiMetricPointValues(data);
  var width = opts.width != null ? opts.width : 600;
  var height = opts.height != null ? opts.height : 200;
  var padding = opts.padding != null ? opts.padding : 40;
  var pointsLabel = poolaiChartT('admin.mon.chartPoints', '{n} points').replace(
    /\{n\}/g,
    String(data.length),
  );
  var statMin = poolaiChartT('admin.mon.statMin', 'Min:');
  var statMax = poolaiChartT('admin.mon.statMax', 'Max:');
  var statAvg = poolaiChartT('admin.mon.statAvg', 'Avg:');
  if (wasm && typeof wasm.renderLineChartHtml === 'function') {
    return wasm.renderLineChartHtml(
      metricName,
      JSON.stringify(values),
      width,
      height,
      padding,
      pointsLabel,
      statMin,
      statMax,
      statAvg,
    );
  }
  return '<div class="muted">' + escapeHtml(noData) + '</div>';
}

/**
 * Compact sparkline for admin home dashboard.
 * @param {string} label
 * @param {number[]} values
 * @param {{ width?: number, height?: number }} [opts]
 */
function poolaiRenderSparkline(label, values, opts) {
  opts = opts || {};
  if (!values || values.length === 0) return '';
  var width = opts.width != null ? opts.width : 200;
  var height = opts.height != null ? opts.height : 40;
  var avgLabel = poolaiChartT('admin.dash.avg', 'Avg: ');
  var wasm = poolaiChartsWasm();
  if (wasm && typeof wasm.renderSparklineHtml === 'function') {
    return wasm.renderSparklineHtml(label, JSON.stringify(values), width, height, avgLabel);
  }
  return '';
}

/**
 * @param {string[]} metricNames
 * @param {{ hours?: number, limit?: number, title?: string, chart?: object }} [opts]
 */
async function poolaiRenderMetricsChartGrid(metricNames, opts) {
  opts = opts || {};
  var parts = [];
  for (var i = 0; i < metricNames.length; i++) {
    var name = metricNames[i];
    var history = await poolaiFetchMetricHistory(name, {
      hours: opts.hours != null ? opts.hours : 24,
      limit: opts.limit != null ? opts.limit : 200,
    });
    if (history.length > 0) {
      parts.push(poolaiRenderLineChart(name, history, opts.chart || {}));
    }
  }
  if (parts.length === 0) return '';
  var title =
    opts.title ||
    poolaiChartT('admin.mon.vizTitle', 'Metrics Visualization (Last 24 Hours)');
  var wasm = poolaiChartsWasm();
  if (wasm && typeof wasm.renderMetricsChartGridHtml === 'function') {
    return wasm.renderMetricsChartGridHtml(title, JSON.stringify(parts));
  }
  return '';
}

/** @returns {function} stop */
function poolaiStartMetricsPolling(fn, intervalMs) {
  var id = setInterval(fn, intervalMs);
  return function poolaiStopMetricsPolling() {
    clearInterval(id);
  };
}

function poolaiParseMlNumeric(val) {
  var wasm = poolaiChartsWasm();
  if (wasm && typeof wasm.parseMlNumeric === 'function') {
    var n = wasm.parseMlNumeric(val == null ? '' : String(val));
    return n == null ? null : Number(n);
  }
  return null;
}

/** Flatten `step_results` from pipeline list API into table rows. */
function poolaiFlattenMlStepRows(pipelines) {
  var wasm = poolaiChartsWasm();
  if (wasm && typeof wasm.flattenMlStepRows === 'function') {
    return wasm.flattenMlStepRows(JSON.stringify(pipelines || []));
  }
  return [];
}

function poolaiFormatMlMetricSummary(output) {
  var wasm = poolaiChartsWasm();
  if (wasm && typeof wasm.formatMlMetricSummary === 'function') {
    return wasm.formatMlMetricSummary(JSON.stringify(output || {}));
  }
  return '—';
}

function poolaiCollectMlSparklineSeries(rows) {
  var wasm = poolaiChartsWasm();
  if (wasm && typeof wasm.collectMlSparklineSeries === 'function') {
    return wasm.collectMlSparklineSeries(JSON.stringify(rows || []));
  }
  return {};
}

/**
 * PH-S43: ML pipeline step metrics panel (table + sparklines).
 * @param {Array<object>} pipelines
 * @param {{ title?: string, emptyMessage?: string }} [opts]
 */
function poolaiRenderMlPipelineMetricsPanel(pipelines, opts) {
  opts = opts || {};
  var wasm = poolaiChartsWasm();
  if (wasm && typeof wasm.renderMlPipelineMetricsPanel === 'function') {
    return wasm.renderMlPipelineMetricsPanel(
      JSON.stringify(pipelines || []),
      opts.title || poolaiChartT('admin.mon.mlTitle', 'ML Pipeline Step Metrics'),
      opts.emptyMessage ||
        poolaiChartT('admin.mon.mlEmpty', 'No ML pipeline step metrics yet'),
      poolaiChartT(
        'admin.mon.mlEmptyHint',
        'Run the demo pipeline or execute a pipeline via the AI/ML API.',
      ),
      JSON.stringify([
        poolaiChartT('admin.mon.mlCol.pipeline', 'Pipeline'),
        poolaiChartT('admin.mon.mlCol.step', 'Step'),
        poolaiChartT('admin.mon.mlCol.kind', 'Kind'),
        poolaiChartT('admin.mon.mlCol.status', 'Status'),
        poolaiChartT('admin.mon.mlCol.metrics', 'Metrics'),
      ]),
      poolaiChartT('admin.charts.avg', 'Avg: '),
    );
  }
  return '';
}

async function poolaiFetchMlPipelines() {
  try {
    var wasm = poolaiChartsWasm();
    var url =
      wasm && typeof wasm.buildMlPipelinesUrl === 'function'
        ? wasm.buildMlPipelinesUrl()
        : '/api/enterprise/ai-ml/pipeline';
    var data = await fetchJson(url);
    return data || [];
  } catch (e) {
    console.warn('poolaiFetchMlPipelines:', e);
    return null;
  }
}

/** @param {{ limit?: number, acknowledged?: boolean }} [opts] */
async function poolaiFetchMonitoringAlerts(opts) {
  opts = opts || {};
  var limit = opts.limit != null ? opts.limit : 20;
  try {
    var wasm = poolaiChartsWasm();
    var url;
    if (opts.acknowledged === false) {
      url =
        wasm && typeof wasm.buildMonitoringActiveAlertsUrl === 'function'
          ? wasm.buildMonitoringActiveAlertsUrl(limit)
          : '/api/enterprise/monitoring/alerts?limit=' +
            limit +
            '&acknowledged=false';
    } else {
      url =
        wasm && typeof wasm.buildMonitoringAlertsUrl === 'function'
          ? wasm.buildMonitoringAlertsUrl(
              limit,
              opts.acknowledged != null ? opts.acknowledged : null,
            )
          : '/api/enterprise/monitoring/alerts?limit=' + limit;
    }
    var data = await fetchJson(url);
    return data || [];
  } catch (e) {
    console.warn('poolaiFetchMonitoringAlerts:', e);
    return [];
  }
}

async function poolaiFetchAlertRules() {
  try {
    var wasm = poolaiChartsWasm();
    var url =
      wasm && typeof wasm.buildAlertRulesUrl === 'function'
        ? wasm.buildAlertRulesUrl()
        : '/api/enterprise/monitoring/alert-rules';
    return (await fetchJson(url)) || [];
  } catch (e) {
    console.warn('poolaiFetchAlertRules:', e);
    return [];
  }
}

async function poolaiRunMlPipelineDemo() {
  var wasm = poolaiChartsWasm();
  var url =
    wasm && typeof wasm.buildMlPipelineDemoUrl === 'function'
      ? wasm.buildMlPipelineDemoUrl()
      : '/api/enterprise/ai-ml/pipeline/demo';
  return fetchJson(url);
}

/**
 * PH-S461: monitoring active alerts table (wasm-first).
 * @param {Array<object>} alerts
 * @param {object} labels i18n label map
 */
function poolaiRenderMonitoringAlertsPanel(alerts, labels) {
  labels = labels || {};
  var wasm = poolaiChartsWasm();
  if (wasm && typeof wasm.renderMonitoringAlertsPanel === 'function') {
    return wasm.renderMonitoringAlertsPanel(
      JSON.stringify(alerts || []),
      labels.na || poolaiChartT('admin.na', 'N/A'),
      labels.ack || poolaiChartT('admin.mon.statusAck', 'Acknowledged'),
      labels.active || poolaiChartT('admin.mon.statusActiveLbl', 'Active'),
      labels.ackBtn || poolaiChartT('admin.mon.ackBtn', 'Acknowledge'),
      labels.severity || poolaiChartT('admin.mon.col.severity', 'Severity'),
      labels.metric || poolaiChartT('admin.mon.col.metric', 'Metric'),
      labels.current || poolaiChartT('admin.mon.col.currentVal', 'Current Value'),
      labels.threshold || poolaiChartT('admin.mon.col.threshold', 'Threshold'),
      labels.triggered || poolaiChartT('admin.mon.col.triggered', 'Triggered'),
      labels.status || poolaiChartT('admin.mon.col.statusCol', 'Status'),
      labels.actions || poolaiChartT('admin.mon.col.actions', 'Actions'),
      labels.tableAria || poolaiChartT('admin.mon.activeAlertsTitle', 'Active Alerts'),
      labels.empty || poolaiChartT('admin.mon.noAlerts', 'No active alerts'),
    );
  }
  return adminEmptyStateHtml(
    labels.empty || poolaiChartT('admin.mon.noAlerts', 'No active alerts'),
    { icon: '✅' },
  );
}

/**
 * PH-S470: monitoring dashboards table (wasm-first).
 * @param {Array<object>} dashboards
 * @param {object} labels i18n label map
 */
function poolaiRenderMonitoringDashboardsPanel(dashboards, labels) {
  labels = labels || {};
  var wasm = poolaiChartsWasm();
  if (wasm && typeof wasm.renderMonitoringDashboardsPanel === 'function') {
    return wasm.renderMonitoringDashboardsPanel(
      JSON.stringify(dashboards || []),
      labels.name || poolaiChartT('admin.mon.col.name', 'Name'),
      labels.description || poolaiChartT('admin.mon.col.description', 'Description'),
      labels.metrics || poolaiChartT('admin.mon.col.metrics', 'Metrics'),
      labels.publicCol || poolaiChartT('admin.mon.col.public', 'Public'),
      labels.created || poolaiChartT('admin.mon.col.created', 'Created'),
      labels.tableAria || poolaiChartT('admin.mon.dashboardsTitle', 'Dashboards'),
      labels.emDash || poolaiChartT('admin.sec.emDash', '—'),
      labels.na || poolaiChartT('admin.na', 'N/A'),
      labels.public || poolaiChartT('admin.mon.public', 'Public'),
      labels.private || poolaiChartT('admin.mon.private', 'Private'),
      labels.metricsN || poolaiChartT('admin.mon.metricsN', '{n} metrics'),
      labels.empty || poolaiChartT('admin.mon.noDashboards', 'No dashboards created'),
    );
  }
  return adminEmptyStateHtml(
    labels.empty || poolaiChartT('admin.mon.noDashboards', 'No dashboards created'),
    { icon: '📊' },
  );
}

/**
 * PH-S480: workers table (wasm-only, PH-S821).
 * @param {Array<object>} workers
 * @param {object} labels i18n label map
 */
function poolaiRenderWorkersPanel(workers, labels) {
  labels = labels || {};
  var wasm = poolaiChartsWasm();
  if (wasm && typeof wasm.renderWorkersPanel === 'function') {
    return wasm.renderWorkersPanel(
      JSON.stringify(workers || []),
      labels.id || poolaiChartT('admin.wrk.col.id', 'ID'),
      labels.status || poolaiChartT('admin.wrk.col.status', 'Status'),
      labels.metrics || poolaiChartT('admin.wrk.col.metrics', 'Metrics'),
      labels.actions || poolaiChartT('admin.wrk.col.actions', 'Actions'),
      labels.tableAria || poolaiChartT('admin.nav.workers', 'Workers'),
      labels.healthy || poolaiChartT('workers.healthy', 'Healthy'),
      labels.unhealthy || poolaiChartT('workers.unhealthy', 'Unhealthy'),
      labels.reqLabel || poolaiChartT('admin.wrk.reqLabel', 'Requests:'),
      labels.delete || poolaiChartT('ui.delete', 'Delete'),
      labels.empty || poolaiChartT('admin.wrk.empty', 'No workers found'),
    );
  }
  return '';
}

/**
 * PH-S490: instances table (wasm-first).
 * @param {Array<object>} instances
 * @param {object} labels i18n label map
 */
function poolaiRenderInstancesPanel(instances, labels) {
  labels = labels || {};
  var wasm = poolaiChartsWasm();
  if (wasm && typeof wasm.renderInstancesPanel === 'function') {
    return wasm.renderInstancesPanel(
      JSON.stringify(instances || []),
      labels.instanceId || poolaiChartT('admin.inst.col.instanceId', 'Instance ID'),
      labels.modelId || poolaiChartT('admin.inst.col.modelId', 'Model ID'),
      labels.status || poolaiChartT('admin.inst.col.status', 'Status'),
      labels.strategy || poolaiChartT('admin.inst.col.strategy', 'Strategy'),
      labels.nodes || poolaiChartT('admin.inst.col.nodes', 'Nodes'),
      labels.created || poolaiChartT('admin.inst.col.created', 'Created'),
      labels.actions || poolaiChartT('admin.inst.col.actions', 'Actions'),
      labels.tableAria || poolaiChartT('admin.inst.title', 'Model Instances'),
      labels.view || poolaiChartT('admin.inst.viewBtn', 'View'),
      labels.delete || poolaiChartT('ui.delete', 'Delete'),
      labels.empty || poolaiChartT('admin.inst.empty', 'No instances found'),
    );
  }
  return adminEmptyStateHtml(
    labels.empty || poolaiChartT('admin.inst.empty', 'No instances found'),
    { icon: '🧠' },
  );
}

/**
 * PH-S499: VM instances table (wasm-only, PH-S820).
 */
function poolaiRenderVmPanel(instances, labels) {
  labels = labels || {};
  var wasm = poolaiChartsWasm();
  if (wasm && typeof wasm.renderVmPanel === 'function') {
    return wasm.renderVmPanel(
      JSON.stringify(instances || []),
      labels.name || poolaiChartT('admin.vmadm.col.name', 'Name'),
      labels.status || poolaiChartT('admin.vmadm.col.status', 'Status'),
      labels.resources || poolaiChartT('admin.vmadm.col.resources', 'Resources'),
      labels.actions || poolaiChartT('admin.vmadm.col.actions', 'Actions'),
      labels.tableAria || poolaiChartT('admin.vmadm.section', 'VM Instances'),
      labels.resCpu || poolaiChartT('admin.vmadm.resCpu', 'CPU:'),
      labels.resMem || poolaiChartT('admin.vmadm.resMem', 'Memory:'),
      labels.start || poolaiChartT('vm.start', 'Start'),
      labels.stop || poolaiChartT('vm.stop', 'Stop'),
      labels.delete || poolaiChartT('ui.delete', 'Delete'),
      labels.empty || poolaiChartT('admin.vmadm.empty', 'No VM instances found'),
    );
  }
  return '';
}

/** PH-S821: libraries table (wasm-only). */
function poolaiRenderLibsPanel(libs, labels) {
  labels = labels || {};
  var wasm = poolaiChartsWasm();
  if (wasm && typeof wasm.renderLibsPanel === 'function') {
    return wasm.renderLibsPanel(
      JSON.stringify(libs || []),
      labels.name || poolaiChartT('admin.lib.label.name', 'Library Name'),
      labels.version || poolaiChartT('admin.lib.label.version', 'Version'),
      labels.status || poolaiChartT('admin.wrk.col.status', 'Status'),
      labels.actions || poolaiChartT('admin.wrk.col.actions', 'Actions'),
      labels.tableAria || poolaiChartT('admin.page.libs', 'Libraries'),
      labels.installed || poolaiChartT('admin.lib.installed', 'Installed'),
      labels.notInstalled || poolaiChartT('admin.lib.notInstalled', 'Not Installed'),
      labels.uninstall || poolaiChartT('ui.uninstall', 'Uninstall'),
      labels.update || poolaiChartT('ui.update', 'Update'),
      labels.install || poolaiChartT('ui.install', 'Install'),
      labels.empty || poolaiChartT('admin.lib.empty', 'No libraries found'),
    );
  }
  return '';
}

/** PH-S508: Galaxy virtual nodes table (wasm-first). */
function poolaiRenderGalaxyVirtualNodesPanel(nodes, labels) {
  labels = labels || {};
  var wasm = poolaiChartsWasm();
  if (wasm && typeof wasm.renderGalaxyVirtualNodesPanel === 'function') {
    return wasm.renderGalaxyVirtualNodesPanel(
      JSON.stringify(nodes || []),
      labels.peer || 'Peer',
      labels.origin || 'Origin',
      labels.region || 'Region',
      labels.latency || 'Latency',
      labels.stale || 'Liveness',
      labels.tableAria || 'Virtual nodes',
      labels.empty || 'No virtual nodes',
    );
  }
  return adminEmptyStateHtml(labels.empty || 'No virtual nodes', { icon: '🌐' });
}

/** PH-S512: verification checker tasks table (wasm-first). */
function poolaiRenderGridVerificationPanel(tasks, pendingTotal, labels) {
  labels = labels || {};
  var wasm = poolaiChartsWasm();
  if (wasm && typeof wasm.renderGridVerificationPanel === 'function') {
    return wasm.renderGridVerificationPanel(
      JSON.stringify(tasks || []),
      pendingTotal || 0,
      labels.job || 'Job ID',
      labels.type || 'Task type',
      labels.pending || 'Pending total',
      labels.tableAria || 'Verification checker',
      labels.empty || 'No pending checker tasks',
    );
  }
  return adminEmptyStateHtml(labels.empty || 'No pending checker tasks', { icon: '🔍' });
}

/** PH-S517: Telegram seats snapshot table (wasm-first). */
function poolaiRenderTelegramSeatsPanel(snapshotJson, labels) {
  labels = labels || {};
  var wasm = poolaiChartsWasm();
  if (wasm && typeof wasm.renderTelegramSeatsPanel === 'function') {
    return wasm.renderTelegramSeatsPanel(
      snapshotJson || '{}',
      labels.policy || 'Policy',
      labels.limit || 'Seat limit',
      labels.active || 'Active workers',
      labels.bound || 'Bound wallets',
      labels.tableAria || 'Telegram seats',
    );
  }
  return adminEmptyStateHtml(labels.empty || 'Seat snapshot unavailable', { icon: '📊' });
}

/** PH-S801: payout batch panel (metrics strip + history + latest entry, wasm-only). */
function poolaiRenderPayoutBatchPanel(
  latest,
  history,
  settlementMetrics,
  trustMetrics,
  trustScoreGauge,
) {
  var wasm = poolaiChartsWasm();
  var metricsStrip = '';
  var historyStrip = '';
  var panelHtml = '';
  var i18nJson = JSON.stringify(window.__poolaiAdminI18nRust || {});
  if (wasm && typeof wasm.renderGridSettlementTrustMetricsStrip === 'function') {
    metricsStrip = wasm.renderGridSettlementTrustMetricsStrip(
      JSON.stringify(settlementMetrics || {}),
      JSON.stringify(trustMetrics || {}),
      trustScoreGauge || 0,
    );
  }
  if (wasm && typeof wasm.renderPayoutBatchHistoryStripHtml === 'function') {
    historyStrip = wasm.renderPayoutBatchHistoryStripHtml(
      JSON.stringify(history || {}),
      i18nJson,
    );
  }
  if (wasm && typeof wasm.renderPayoutBatchPanelHtml === 'function') {
    panelHtml = wasm.renderPayoutBatchPanelHtml(
      JSON.stringify(latest || {}),
      JSON.stringify(history || {}),
      i18nJson,
    );
  }
  return metricsStrip + historyStrip + panelHtml;
}

/** PH-S810: secret rotation panel (wasm-only). */
function poolaiRenderSecretRotationPanel(rows) {
  var wasm = poolaiChartsWasm();
  if (wasm && typeof wasm.renderSecretRotationPanelHtml === 'function') {
    return wasm.renderSecretRotationPanelHtml(
      JSON.stringify(rows || []),
      JSON.stringify(window.__poolaiAdminI18nRust || {}),
    );
  }
  return '';
}

/** PH-S811: topology stats strip with wasm timestamp (wasm-only). */
function poolaiRenderTopologyStatsStrip(summary) {
  var wasm = poolaiChartsWasm();
  if (wasm && typeof wasm.renderTopologyStatsStripHtml === 'function') {
    return wasm.renderTopologyStatsStripHtml(
      JSON.stringify(summary || {}),
      JSON.stringify(window.__poolaiAdminI18nRust || {}),
    );
  }
  return '';
}

/** PH-S700: replication/pricing metrics strip (wasm-first). */
function poolaiRenderGridReplicationPricingPanel(
  replicationMetrics,
  pricingMetrics,
  strictGauge,
  labels,
) {
  labels = labels || {};
  var wasm = poolaiChartsWasm();
  if (wasm && typeof wasm.renderGridReplicationPricingPanel === 'function') {
    return wasm.renderGridReplicationPricingPanel(
      JSON.stringify(replicationMetrics || {}),
      JSON.stringify(pricingMetrics || {}),
      strictGauge || 0,
      JSON.stringify(labels),
    );
  }
  return '';
}
