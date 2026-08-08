# Repository layout (code & ops)

**Оновлено:** 2026-05-24 · Для людей і онбордингу. Канон стеку — [`STRUCTURE.md`](../STRUCTURE.md) §7, [`.cursor/rules/project-structure.mdc`](../../.cursor/rules/project-structure.mdc).

---

## Не плутати «три bin»

| Шлях | Що це | Приклад |
|------|--------|---------|
| **`src/`** | Rust **бібліотека** + модулі продукту (`poolai` crate) | `src/network/`, `src/ml/` |
| **`src/bin/`** | Rust **виконувані файли Cargo** (`cargo run --bin …`) | `poolai_health_load.rs`, `poolai-worker.rs` |
| **`bin/`** (корінь) | **Shell/PowerShell ops** — запуск, LAN, verify, e2e (не компілюється Cargo) | `run-poolai.sh`, `verify-dev-stand.sh` |
| **`scripts/`** | **Toolchain / MSYS / deploy** — PATH, gcc, git-push helpers | `setup_rust_path.sh`, `verify_build.sh` |
| **`tests/`** | **Інтеграційні тести** Rust (окремо від `src/`, як у ecosystem) | `admin_ui_api_contracts.rs` |
| **`crates/*/src/`** | **Окремий workspace member** (не дубль головного `src/`) | `poolai-solana-adapter` |

```text
poolAI/
├── src/              ← продукт (Rust lib + main)
│   ├── bin/          ← cargo binaries only
│   └── ui/           ← Admin UI (JS)
├── bin/              ← dev/launch ops (.sh / .ps1)
├── comitmsg/         ← локальні чернетки commit message (.txt, gitignored)
├── scripts/          ← toolchain & deploy helpers
├── tests/            ← integration tests
├── crates/           ← workspace members
│   └── poolai-solana-adapter/src/
└── e2e/              ← Playwright (TypeScript)
```

---

## Куди додавати нове

| Тип зміни | Каталог |
|-----------|---------|
| API, сервіси, ML, RAID | `src/` |
| Новий CLI/tool на Rust | `src/bin/` + `Cargo.toml` `[[bin]]` |
| Запуск стенду, verify, метрики ops | **`bin/`** |
| Чернетка subject для `commit-tree` / PH-S* push | **`comitmsg/`** (див. [`comitmsg/README.md`](../../comitmsg/README.md)) |
| MSYS, PATH, gcc, старий git-push shell | **`scripts/`** |
| HTTP/JSON контракт проти API | `tests/` (`admin_ui_api_contracts.rs`, `vm_api_contracts.rs`, `distributed_raid_wire_integration.rs`, …) |
| Raft wire + multi-node harness (`feature raft`) | `tests/raft_wire_integration.rs`, `tests/raft_multi_node_harness.rs` — **`cargo test-raft-ci`** |
| Sidecar без `solana-sdk` у main | `crates/poolai-solana-adapter/` |

**Не створювати** кореневий `bin/` для `.rs` — лише `src/bin/`.

---

## `crates/` vs `src/`

`crates/poolai-solana-adapter` — **свідомий** окремий crate (workspace у кореневому `Cargo.toml`):

- без важких Solana-залежностей у головному `poolai`;
- окремі тести: `cargo test -p poolai-solana-adapter`.

Об’єднувати в `src/solana/` — лише окремим архітектурним рішенням.

---

## Дублікати (політика)

- Один сценарій — **один канонічний файл** у `bin/` або `scripts/`.
- Застарілий шлях — thin forwarder з `DEPRECATED: use bin/...` (не повна копія).
- Приклад: `scripts/run-lan-nodes.ps1` → викликає `bin/run-lan-nodes.ps1`.

---

## Документація

| Тема | Файл |
|------|------|
| Запуск локально | [`RUN_LOCAL.md`](./RUN_LOCAL.md) |
| Скрипти `bin/` | [`../../bin/README.md`](../../bin/README.md) |
| Toolchain `scripts/` | [`../../scripts/README.md`](../../scripts/README.md) |
| LAN / метрики | [`../performance/LAN_BENCHMARK_RUNBOOK.md`](../performance/LAN_BENCHMARK_RUNBOOK.md) |
