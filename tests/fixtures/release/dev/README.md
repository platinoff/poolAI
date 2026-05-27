# Dev fixtures: `poolai-verify-release` (PH-S85)

**Не для production.** Фіксований ed25519 ключ `poolai-dev` (той самий, що в `src/release/verify.rs` unit tests) — лише локальна перевірка CLI.

| Файл | Призначення |
|------|-------------|
| `maintainer_keys.json` | Trust root (`key_id` → `public_key_hex`) |
| `release-manifest.json` | Підписаний manifest (версія `0.2.2-dev`) |
| `release-manifest.json.sig` | Detached signature envelope |
| `poolai-sample.bin` | Sample artifact (`sha256` у manifest) |

**Регенерація** (після зміни формату manifest/sig):

```bash
cargo test --lib release::verify::tests::write_dev_release_fixtures -- --ignored --exact
```

**Перевірка** — див. [`RUN_LOCAL.md`](../../../docs/development/RUN_LOCAL.md) та [`SECURITY_HARDENING.md`](../../../docs/security/SECURITY_HARDENING.md) (operator quickstart).

Політика підписів — канон [`POOLAI_GALAXY_GRID.md`](../../../docs/concept/POOLAI_GALAXY_GRID.md) §9.2 (без дублювання тут).
