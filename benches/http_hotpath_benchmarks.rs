//! FM-042: HTTP / JSON hot-path micro-benchmarks (Criterion).
//!
//! Complements `runtime_benchmarks` (health JSON) and FM-028 ops snapshots.
//! Run: `cargo bench -j 1 --bench http_hotpath_benchmarks -- --noplot`

use axum::body::Body;
use axum::http::Request;
use criterion::{criterion_group, criterion_main, Criterion};
use poolai::core::error::AppError;
use poolai::network::json_errors::{api_error_response, http_status_for_app_error};
use poolai::observability::make_http_span;
use std::hint::black_box;
use tracing_subscriber::util::SubscriberInitExt;

fn bench_json_error_hotpaths(c: &mut Criterion) {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::new("off"))
        .try_init();

    let not_found = AppError::ApiNotFound("worker missing".into());
    let validation = AppError::ValidationError("invalid field".into());

    let mut group = c.benchmark_group("http_json_errors");
    group.bench_function("http_status_for_app_error_not_found", |b| {
        b.iter(|| black_box(http_status_for_app_error(black_box(&not_found))));
    });
    group.bench_function("api_error_response_not_found", |b| {
        b.iter(|| {
            let (_status, json) = api_error_response(black_box(&not_found), None, None);
            black_box(serde_json::to_vec(&json.0).unwrap());
        });
    });
    group.bench_function("api_error_response_validation", |b| {
        b.iter(|| {
            let (_status, json) = api_error_response(black_box(&validation), None, None);
            black_box(serde_json::to_vec(&json.0).unwrap());
        });
    });
    group.finish();
}

fn bench_http_trace_span(c: &mut Criterion) {
    let request = Request::builder()
        .uri("/api/v1/health")
        .method("GET")
        .body(Body::empty())
        .expect("request");

    let mut group = c.benchmark_group("http_trace");
    group.bench_function("make_http_span_health", |b| {
        b.iter(|| {
            let span = make_http_span(black_box(&request));
            black_box(span);
        });
    });
    group.finish();
}

criterion_group!(
    http_hotpath_benches,
    bench_json_error_hotpaths,
    bench_http_trace_span
);
criterion_main!(http_hotpath_benches);
