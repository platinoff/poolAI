# Rust diagnostics — Clippy warning/error index

**Оновлено:** 2026-07-28  
**Артефакт:** [`rust_diagnostics.json`](./rust_diagnostics.json) · mirror [`../../docs/vision/rust_diagnostics.json`](../../docs/vision/rust_diagnostics.json)  
**Bin:** `poolai-rust-diagnostics` · wrapper: `bin/record-rust-diagnostics.sh`  
**UI:** GSV → panel **Rust** (`http://127.0.0.1:8891/`) · legacy `docs/vision/index.html` deactivated (band 117)  
**CI:** job `rust-diagnostics` in [`.github/workflows/ci.yml`](../../.github/workflows/ci.yml) (artifact upload)

---

## Навіщо

Відслідковувати **кількість warnings / errors** Clippy між сесіями **`абракадабра`** і з GitHub Actions — поруч зі Speeds panel, без ручного «clippy був ок».

Сканування **без** `-D warnings`, щоб warnings лишались видимими (CI lint job з `-D` лишається окремим gate).

**Project scan (`абракадабра`):** перед формуванням топ-10 PH-S* читати `latest` / `top_codes` (або `cargo run --bin poolai-rust-diagnostics -- --print`). Виправні warnings/errors — **пріоритет 0** у смузі (див. [`poolai-session-iteration.mdc`](../../.cursor/rules/poolai-session-iteration.mdc) § Project scan). Повний `record-rust-diagnostics.sh` — на кінці drain (крок Test), не замість scan-time читання індексу.

---

## Запис (MSYS2)

```bash
export PATH="$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export RUSTUP_TOOLCHAIN=stable-x86_64-pc-windows-gnu
export K8S_OPENAPI_ENABLED_VERSION=1.28
export CARGO_TARGET_DIR=/s/rust/poolAI/target
cd /s/rust/poolAI

# Повний Clippy JSON scan + запис (+ vision mirror)
bash bin/record-rust-diagnostics.sh

# Лише показати індекс
cargo run --bin poolai-rust-diagnostics -- --print

# Ручний snapshot (CI helper / без повторного clippy)
cargo run --bin poolai-rust-diagnostics -- \
  --record --warnings 0 --errors 0 --ok --source local
```

Default scan command:

```text
cargo clippy --message-format=json --all-targets --features jwt,https
```

Override: `RUST_DIAGNOSTICS_CMD='…' bash bin/record-rust-diagnostics.sh`

Exit code ≠ 0 лише якщо **errors > 0** (warnings alone не валять record).

---

## CI workflow

Job **`rust-diagnostics`** (ubuntu):

1. `bash bin/record-rust-diagnostics.sh --ci`
2. Upload artifact `rust-diagnostics-json` (`docs/development/rust_diagnostics.json`)

Локальний drain комітить JSON у репо; CI артефакт — для порівняння / audit без auto-commit.

---

## Vision

1. `.\bin\open-docs-vision.ps1`
2. Панель **Rust** — latest warnings/errors + history + top lint codes.
3. Auto-reload слідкує за `docs/vision/rust_diagnostics.json` і `docs/development/rust_diagnostics.json`.

Після drain **`абракадабра`:** `record-rust-diagnostics.sh` → (опційно Speeds) → `poolai-vision-sync` → push.

---

## Changelog

| Дата | Зміна |
|------|--------|
| 2026-07-28 | Project scan: читати індекс **до** топ-10 PH-S*; виправні warnings = пріоритет 0 |
| 2026-07-27 | Перший index + Rust panel + CI job + `record-rust-diagnostics.sh` |
