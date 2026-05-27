//! PH-S58: Galaxy Grid fee split — Criterion micro-bench (`split_gross_payment`).
//!
//! Run: `cargo bench -j 1 --bench galaxy_fee_split_benchmarks -- --noplot`

use criterion::{criterion_group, criterion_main, Criterion};
use poolai::grid::galaxy_fee_split::{split_gross_payment, SECONDARY_ADMIN_FEE_MIN_BPS};
use std::hint::black_box;

fn bench_split_gross(c: &mut Criterion) {
    let gross = 1_000_000_000u64;
    let mut group = c.benchmark_group("galaxy_fee_split");
    group.bench_function("split_1sol_secondary_min", |b| {
        b.iter(|| {
            black_box(split_gross_payment(
                black_box(gross),
                black_box(SECONDARY_ADMIN_FEE_MIN_BPS),
            ))
        });
    });
    group.bench_function("split_1sol_secondary_max", |b| {
        b.iter(|| black_box(split_gross_payment(black_box(gross), black_box(500u16))));
    });
    group.finish();
}

criterion_group!(benches, bench_split_gross);
criterion_main!(benches);
