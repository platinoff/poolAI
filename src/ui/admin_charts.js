// PoolAI admin metrics charts (PH-S10) — SVG canvas glue; data parse via poolai-ui-wasm (PH-S155)

function poolaiChartT(key, enFallback) {
  return typeof poolaiT === 'function' ? poolaiT(key, enFallback) : enFallback;
}

function poolaiChartsWasm() {
  var w = window.poolaiUiWasm;
  return w && w.ready ? w : null;
}

function poolaiSanitizeChartId(name) {
  return String(name || 'metric').replace(/[^a-zA-Z0-9_-]/g, '_');
}

/** @param {Array<{value?: number}>} data */
function poolaiMetricPointValues(data) {
  var wasm = poolaiChartsWasm();
  if (wasm && typeof wasm.metricPointValues === 'function') {
    return wasm.metricPointValues(JSON.stringify(data || []));
  }
  if (!Array.isArray(data)) return [];
  return data.map(function (d) {
    return d && d.value != null ? Number(d.value) : 0;
  });
}

function poolaiChartScaleFallback(values, width, height, padding) {
  var min = Math.min.apply(null, values);
  var max = Math.max.apply(null, values);
  var range = max - min || 1;
  var chartWidth = width - padding * 2;
  var chartHeight = height - padding * 2;
  var points = values.map(function (v, i) {
    var x = padding + (i / (values.length - 1 || 1)) * chartWidth;
    var y = padding + chartHeight - ((v - min) / range) * chartHeight;
    return { x: x, y: y };
  });
  return {
    min: min,
    max: max,
    range: range,
    chartWidth: chartWidth,
    chartHeight: chartHeight,
    padding: padding,
    points: points,
    polyline: points.map(function (p) { return p.x + ',' + p.y; }).join(' '),
  };
}

function poolaiChartScale(values, width, height, padding) {
  var wasm = poolaiChartsWasm();
  if (wasm && typeof wasm.chartScale === 'function') {
    return wasm.chartScale(JSON.stringify(values || []), width, height, padding);
  }
  return poolaiChartScaleFallback(values, width, height, padding);
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
    var url =
      '/api/enterprise/monitoring/metrics?metric=' +
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
    var startTime = new Date(Date.now() - hours * 60 * 60 * 1000).toISOString();
    var url =
      '/api/enterprise/monitoring/metrics?start_time=' +
      encodeURIComponent(startTime) +
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
  var by = {};
  if (!Array.isArray(metrics)) return by;
  metrics.forEach(function (m) {
    var key = m && m.metric ? m.metric : 'unknown';
    if (!by[key]) by[key] = [];
    by[key].push(m);
  });
  return by;
}

/**
 * Full line chart for monitoring dashboards.
 * @param {string} metricName
 * @param {Array<{value?: number}>} data
 * @param {{ width?: number, height?: number, padding?: number }} [opts]
 */
function poolaiRenderLineChart(metricName, data, opts) {
  opts = opts || {};
  if (!data || data.length === 0) {
    return (
      '<div class="muted">' +
      escapeHtml(poolaiChartT('admin.mon.noData', 'No data available')) +
      '</div>'
    );
  }

  var values = poolaiMetricPointValues(data);
  var width = opts.width != null ? opts.width : 600;
  var height = opts.height != null ? opts.height : 200;
  var padding = opts.padding != null ? opts.padding : 40;
  var scale = poolaiChartScale(values, width, height, padding);
  var gradId = 'grad-' + poolaiSanitizeChartId(metricName);
  var pointsLabel = poolaiChartT('admin.mon.chartPoints', '{n} points').replace(
    /\{n\}/g,
    String(data.length),
  );
  var avg = values.reduce(function (a, b) {
    return a + b;
  }, 0) / values.length;

  var circles = scale.points
    .map(function (p) {
      return (
        '<circle cx="' +
        p.x +
        '" cy="' +
        p.y +
        '" r="3" fill="var(--primary, #67e480)" />'
      );
    })
    .join('');

  return (
    '<div class="metric-chart-container">' +
    '<h4>' +
    escapeHtml(metricName) +
    '</h4>' +
    '<svg width="' +
    width +
    '" height="' +
    height +
    '" class="metric-chart-svg" role="img" aria-label="' +
    escapeHtml(metricName) +
    '">' +
    '<defs><linearGradient id="' +
    gradId +
    '" x1="0%" y1="0%" x2="0%" y2="100%">' +
    '<stop offset="0%" style="stop-color:var(--primary, #67e480);stop-opacity:0.3" />' +
    '<stop offset="100%" style="stop-color:var(--primary, #67e480);stop-opacity:0.05" />' +
    '</linearGradient></defs>' +
    '<rect x="' +
    scale.padding +
    '" y="' +
    scale.padding +
    '" width="' +
    scale.chartWidth +
    '" height="' +
    scale.chartHeight +
    '" fill="url(#' +
    gradId +
    ')" />' +
    '<polyline points="' +
    scale.polyline +
    '" fill="none" stroke="var(--primary, #67e480)" stroke-width="2" />' +
    circles +
    '<text x="' +
    scale.padding +
    '" y="' +
    (scale.padding - 10) +
    '" fill="var(--text, #f8f8f2)" font-size="12">' +
    scale.max.toFixed(1) +
    '</text>' +
    '<text x="' +
    scale.padding +
    '" y="' +
    (height - scale.padding + 20) +
    '" fill="var(--text, #f8f8f2)" font-size="12">' +
    scale.min.toFixed(1) +
    '</text>' +
    '<text x="' +
    (width - scale.padding) +
    '" y="' +
    (height - scale.padding + 20) +
    '" fill="var(--text-muted, #a8b0bf)" font-size="10" text-anchor="end">' +
    escapeHtml(pointsLabel) +
    '</text>' +
    '</svg>' +
    '<div class="metric-stats">' +
    '<span>' +
    escapeHtml(poolaiChartT('admin.mon.statMin', 'Min:')) +
    ' <strong>' +
    scale.min.toFixed(2) +
    '</strong></span>' +
    '<span>' +
    escapeHtml(poolaiChartT('admin.mon.statMax', 'Max:')) +
    ' <strong>' +
    scale.max.toFixed(2) +
    '</strong></span>' +
    '<span>' +
    escapeHtml(poolaiChartT('admin.mon.statAvg', 'Avg:')) +
    ' <strong>' +
    avg.toFixed(2) +
    '</strong></span>' +
    '</div></div>'
  );
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
  var padding = 4;
  var scale = poolaiChartScale(values, width, height, padding);
  var avg =
    values.reduce(function (a, b) {
      return a + b;
    }, 0) / values.length;

  return (
    '<div class="metric-sparkline-card">' +
    '<div class="metric-sparkline-label">' +
    escapeHtml(label) +
    '</div>' +
    '<svg width="' +
    width +
    '" height="' +
    height +
    '" class="metric-sparkline-svg" role="img" aria-label="' +
    escapeHtml(label) +
    '">' +
    '<polyline points="' +
    scale.polyline +
    '" fill="none" stroke="var(--primary, #67e480)" stroke-width="1.5" />' +
    '</svg>' +
    '<div class="metric-sparkline-avg">' +
    '<span class="metric-sparkline-avg-label">' +
    escapeHtml(poolaiChartT('admin.dash.avg', 'Avg: ')) +
    '</span>' +
    '<strong>' +
    avg.toFixed(1) +
    '</strong></div></div>'
  );
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
  return (
    '<div class="admin-card">' +
    '<h3>' +
    escapeHtml(title) +
    '</h3>' +
    '<div class="metrics-charts-grid">' +
    parts.join('') +
    '</div></div>'
  );
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
  if (val == null || val === '') return null;
  var parsed = parseFloat(String(val));
  return isNaN(parsed) ? null : parsed;
}

/** Flatten `step_results` from pipeline list API into table rows. */
function poolaiFlattenMlStepRows(pipelines) {
  var wasm = poolaiChartsWasm();
  if (wasm && typeof wasm.flattenMlStepRows === 'function') {
    return wasm.flattenMlStepRows(JSON.stringify(pipelines || []));
  }
  var rows = [];
  (pipelines || []).forEach(function (p) {
    var results = p && p.step_results ? p.step_results : {};
    Object.keys(results).forEach(function (stepId) {
      var sr = results[stepId] || {};
      var out = sr.output || {};
      rows.push({
        pipelineId: p.id || '',
        pipelineName: p.name || p.id || 'pipeline',
        pipelineStatus: p.status || '',
        stepId: stepId,
        stepStatus: sr.status || '',
        stepKind: out.step_kind || out.step_id || stepId,
        output: out,
      });
    });
  });
  return rows;
}

function poolaiFormatMlMetricSummary(output) {
  var wasm = poolaiChartsWasm();
  if (wasm && typeof wasm.formatMlMetricSummary === 'function') {
    return wasm.formatMlMetricSummary(JSON.stringify(output || {}));
  }
  if (!output || typeof output !== 'object') return '—';
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
  var rows = poolaiFlattenMlStepRows(pipelines);
  var title =
    opts.title ||
    poolaiChartT('admin.mon.mlTitle', 'ML Pipeline Step Metrics');
  if (!rows.length) {
    return (
      '<div class="admin-card ml-pipeline-metrics-panel">' +
      '<h3>' +
      escapeHtml(title) +
      '</h3>' +
      (typeof adminEmptyStateHtml === 'function'
        ? adminEmptyStateHtml(
            opts.emptyMessage ||
              poolaiChartT('admin.mon.mlEmpty', 'No ML pipeline step metrics yet'),
            {
              hint: poolaiChartT(
                'admin.mon.mlEmptyHint',
                'Run the demo pipeline or execute a pipeline via the AI/ML API.',
              ),
              icon: '🧠',
            },
          )
        : '<div class="muted">' +
          escapeHtml(
            opts.emptyMessage ||
              poolaiChartT('admin.mon.mlEmpty', 'No ML pipeline step metrics yet'),
          ) +
          '</div>') +
      '</div>'
    );
  }

  var series = poolaiCollectMlSparklineSeries(rows);
  var sparkParts = [];
  Object.keys(series)
    .slice(0, 6)
    .forEach(function (label) {
      var values = series[label];
      if (values.length) {
        sparkParts.push(poolaiRenderSparkline(label, values, { width: 220, height: 44 }));
      }
    });

  var tableRows = rows.map(function (r) {
    var statusCls =
      String(r.stepStatus).toLowerCase() === 'completed'
        ? 'active'
        : String(r.stepStatus).toLowerCase() === 'failed'
          ? 'error'
          : 'warning';
    return [
      escapeHtml(r.pipelineName),
      escapeHtml(r.stepId),
      '<code>' + escapeHtml(String(r.stepKind)) + '</code>',
      '<span class="status-badge ' +
        statusCls +
        '">' +
        escapeHtml(String(r.stepStatus || '—')) +
        '</span>',
      escapeHtml(poolaiFormatMlMetricSummary(r.output)),
    ];
  });

  var tableHtml =
    typeof adminRenderTable === 'function'
      ? adminRenderTable(
          [
            poolaiChartT('admin.mon.mlCol.pipeline', 'Pipeline'),
            poolaiChartT('admin.mon.mlCol.step', 'Step'),
            poolaiChartT('admin.mon.mlCol.kind', 'Kind'),
            poolaiChartT('admin.mon.mlCol.status', 'Status'),
            poolaiChartT('admin.mon.mlCol.metrics', 'Metrics'),
          ],
          tableRows,
        )
      : '';

  return (
    '<div class="admin-card ml-pipeline-metrics-panel">' +
    '<h3>' +
    escapeHtml(title) +
    ' (' +
    rows.length +
    ')</h3>' +
    (sparkParts.length
      ? '<div class="metrics-sparklines-grid ml-step-sparklines">' +
        sparkParts.join('') +
        '</div>'
      : '') +
    tableHtml +
    '</div>'
  );
}

async function poolaiFetchMlPipelines() {
  try {
    var data = await fetchJson('/api/enterprise/ai-ml/pipeline');
    return data || [];
  } catch (e) {
    console.warn('poolaiFetchMlPipelines:', e);
    return null;
  }
}

async function poolaiRunMlPipelineDemo() {
  return fetchJson('/api/enterprise/ai-ml/pipeline/demo');
}
