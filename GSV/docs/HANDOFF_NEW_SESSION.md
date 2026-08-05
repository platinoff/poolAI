# Передача контексту новій сесії (GSV)

**Оновлено:** 2026-08-05 (band 108 **PH-S1719…S1728** ✅ · ratio **95.52%** · tests **87** · clippy **0**)

**Наступна сесія:** **`абракадабра`** → S0 диск/git → project scan (warnings first) → drain ≤10 PH-S*
→ Speeds + Rust panel → vision-sync → **один commit** → **`git push` + самарі**. Канон:
[`GSV_ROLES.md`](GSV_ROLES.md).

## Стан зараз

- **GSV** — окремий Rust-first проєкт (`GSV/`), band 102 + band 108 **✅**.
- **Ratio:** `cargo run --bin gsv-loc-audit` → **95.52%** (gate ≥95% ✅) → `GSV/data/rust_ratio.json`.
- **Тести:** `cargo test` → **87** green · **clippy 0** · **fmt clean**.
- **Сервер живий:** `gsv-server` на порту **8891** (`--repo-root S:/rust/poolAI --data-dir S:/rust/poolAI/GSV/data`).
- **FM:** band 108 = §5.89 (PH-S1719…S1728 ✅). Master horizon poolAI: band 109 (PH-S1729…S1738).
- **Vision rev:** 458 (band 108 vision-sync).

## S0 (кожна сесія, disk/git first)

1. `df -h /s | tail -1` → `bash scripts/check_target_disk.sh` → `cargo clean` якщо <5G (12G дешево).
2. `git fetch` → `git status -sb` → `git log -1 --oneline`.
3. Прочитати цей HANDOFF + `NEXT_SESSION_PROMPT.md` + FM §5.12 §5.89.

## Project scan (якщо §5.12 < 10 відкритих)

- Warnings/diagnostics першими: `cargo run --bin poolai-rust-diagnostics -- --print` (poolAI),
  clippy warnings GSV (`cargo clippy --all-targets`).
- Роадмапи/архітектор-ряди: `docs/gsv/GSV_TECH_ROADMAP.md`, `GSV/docs/`, FM §5.1.
- Fallback-смуга: ratio contracts, UI compact, docs canon, vision sync, stand smoke.

## Build/test (MSYS2 bash)

```bash
export PATH="/c/Users/${USER}/.cargo/bin:$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export RUSTUP_TOOLCHAIN="stable-x86_64-pc-windows-gnu"
cd GSV
cargo fmt -- --check && cargo clippy --all-targets && cargo test && cargo run --bin gsv-loc-audit
```

**⚠️ Перед build/test зупинити `gsv-server`** (блокує `gsv-server.exe`, os error 5); після — перезапустити.

## Git (кінець сесії)

- Один commit (код + docs + FM/HANDOFF/NEXT). Не `git add -A` — тільки файли спринту.
- **`git push` + самарі** — обов'язково останній крок.
