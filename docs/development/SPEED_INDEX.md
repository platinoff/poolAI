# Speed index — test-ci & Criterion wall times

**Оновлено:** 2026-07-27  
**Артефакт:** [`speed_index.json`](./speed_index.json) · mirror [`../../docs/vision/speed_index.json`](../../docs/vision/speed_index.json)  
**Bin:** `poolai-speed-index` · wrapper: `bin/record-test-ci-speed.sh`  
**UI:** GSV → panel **Speeds** (`http://127.0.0.1:8891/`) · legacy `docs/vision/index.html` deactivated (band 117)

---

## Навіщо

Індексувати **реальний wall-clock** `cargo test-ci` і вибіркові Criterion medians між сесіями **`абракадабра`**, без ручних «~16 хв» у FM.

---

## Запис (MSYS2)

```bash
export PATH="$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export RUSTUP_TOOLCHAIN=stable-x86_64-pc-windows-gnu
export K8S_OPENAPI_ENABLED_VERSION=1.28
export CARGO_TARGET_DIR=/s/rust/poolAI/target
cd /s/rust/poolAI

# Повний прогін + запис у JSON (+ vision mirror)
bash bin/record-test-ci-speed.sh

# Лише показати індекс
cargo run --bin poolai-speed-index -- --print

# Criterion median (після short bench)
cargo run --bin poolai-speed-index -- --record-bench \
  --bench runtime_benchmarks \
  --group memory_pool/acquire_release \
  --median-ns 1280 \
  --profile short
```

Short Criterion (trends only):

```bash
cargo bench -j 1 --bench runtime_benchmarks -- \
  --sample-size 20 --warm-up-time 0.3 --measurement-time 0.5
```

Канон таблиць — [`../performance/BENCHMARKS.md`](../performance/BENCHMARKS.md).

---

## Vision

1. `.\bin\open-docs-vision.ps1`
2. Панель **Speeds** — latest `test-ci` wall time + Criterion history.
3. Auto-reload слідкує за `docs/vision/speed_index.json` і `docs/development/speed_index.json`.

Після drain **`абракадабра`:** record → `poolai-vision-sync` → Speeds panel оновлюється.

---

## Changelog

| Дата | Зміна |
|------|--------|
| 2026-07-27 | Перший index + Speeds panel + `record-test-ci-speed.sh` |
