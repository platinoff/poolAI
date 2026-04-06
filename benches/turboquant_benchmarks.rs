//! Criterion benchmarks for TurboQuant pack/unpack and dot product (`feature = "ml"`).

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use poolai::ml::turboquant::{dot_f32, pack_uniform_rows, unpack_to_rows};

fn bench_turboquant(c: &mut Criterion) {
    let rows: Vec<Vec<f32>> = (0..64)
        .map(|i| (0..256).map(|j| ((i * 256 + j) as f32) * 0.001).collect())
        .collect();
    let packed = pack_uniform_rows(&rows).unwrap();
    let bytes = packed.bytes.clone();

    let n = 4096_usize;
    let a: Vec<f32> = (0..n).map(|i| (i as f32) * 0.01).collect();
    let b: Vec<f32> = (0..n).map(|i| ((n - i) as f32) * 0.01).collect();

    let mut g = c.benchmark_group("turboquant");
    g.bench_function("pack_uniform_rows_64x256", |be| {
        be.iter(|| black_box(pack_uniform_rows(black_box(&rows)).unwrap()));
    });
    g.bench_function("unpack_to_rows_64x256", |be| {
        be.iter(|| black_box(unpack_to_rows(black_box(bytes.as_slice())).unwrap()));
    });
    g.bench_function("dot_f32_4096", |be| {
        be.iter(|| black_box(dot_f32(black_box(&a), black_box(&b))));
    });
    g.finish();
}

criterion_group!(turbo_benches, bench_turboquant);
criterion_main!(turbo_benches);
