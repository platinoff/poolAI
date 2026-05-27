# Release manifest sample (`poolai-verify-release`, PH-S88)

**Operator reference** for the JSON manifest verified by **`poolai-verify-release`** (Galaxy §9.2, PH-S66). Policy and governance — [`SECURITY_HARDENING.md`](../security/SECURITY_HARDENING.md#galaxy-governance-canonical-pointers-ph-s69-ph-s77) (не дублювати тут).

**Runnable dev fixtures (signed, dev key `poolai-dev`):** [`tests/fixtures/release/dev/`](../../tests/fixtures/release/dev/README.md) (PH-S85).

---

## Minimal manifest schema (implemented subset)

| Field | Required | Опис |
|-------|----------|------|
| `version` | так | Release version string (напр. `0.2.2`) |
| `git_tag` | ні | Git tag (напр. `v0.2.2`) |
| `protocol_min` | ні | Мінімальний Galaxy wire protocol (compat matrix §9.3) |
| `protocol_max` | ні | Максимальний Galaxy wire protocol |
| `artifacts[]` | так | Список артефактів для SHA-256 перевірки |
| `artifacts[].name` | так | Логічне ім’я (`poolai`, `poolai-worker`, …) |
| `artifacts[].path` | ні | Шлях у tarball/OCI (для документації) |
| `artifacts[].sha256` | так | Hex SHA-256 файлу (lowercase) |
| `artifacts[].sig_ref` | ні | Посилання на окремий підпис артефакта (roadmap) |

Код: [`src/release/manifest.rs`](../../src/release/manifest.rs).

---

## Example: production-style manifest

Плейсхолдер `REPLACE_SHA256` — замінити після `sha256sum` / `Get-FileHash` на реальному бінарнику.

```json
{
  "version": "0.2.2",
  "git_tag": "v0.2.2",
  "protocol_min": "1.0",
  "protocol_max": "1.2",
  "artifacts": [
    {
      "name": "poolai",
      "path": "poolai",
      "sha256": "REPLACE_SHA256"
    }
  ]
}
```

Підпис: ed25519 над **raw bytes** цього JSON (як збережено у файлі manifest, без зміни пробілів після підпису).

---

## Detached signature envelope (`.sig`)

Окремий JSON-файл поруч із manifest:

```json
{
  "algorithm": "ed25519",
  "key_id": "poolai-dev",
  "signature_hex": "64_BYTE_HEX"
}
```

| Поле | Опис |
|------|------|
| `algorithm` | Зараз лише `"ed25519"` |
| `key_id` | Ключ у `maintainer_keys.json` (trust root) |
| `signature_hex` | 64 байти підпису в hex |

Trust root (приклад):

```json
{
  "maintainer_keys": [
    { "key_id": "poolai-dev", "public_key_hex": "32_BYTE_HEX" }
  ]
}
```

---

## Verify (copy-paste)

**Dev fixtures** (готові файли в репо):

```bash
export PATH="$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
cd /s/rust/poolAI
FIX=tests/fixtures/release/dev

cargo run --bin poolai-verify-release -- \
  --manifest "$FIX/release-manifest.json" \
  --signature "$FIX/release-manifest.json.sig" \
  --trust-root "$FIX/maintainer_keys.json" \
  --artifact "$FIX/poolai-sample.bin" \
  --artifact-name poolai
```

**Власний manifest:** підставити шляхи до вашого `release-manifest.json`, `.sig`, `maintainer_keys.json` і `--artifact`.

Деталі: [`RUN_LOCAL.md`](./RUN_LOCAL.md) · [`SECURITY_HARDENING.md`](../security/SECURITY_HARDENING.md) (dev fixtures § PH-S85).

---

## Related

| Документ | Зміст |
|----------|--------|
| [`POOLAI_GALAXY_GRID.md` §9.2](../concept/POOLAI_GALAXY_GRID.md#92-signed-releases-канон-ph-s63) | Концепт signed releases |
| [`tests/fixtures/release/dev/README.md`](../../tests/fixtures/release/dev/README.md) | Signed dev fixtures + regenerate |
| [`src/bin/poolai_verify_release.rs`](../../src/bin/poolai_verify_release.rs) | CLI |

**Last updated:** 2026-05-27 (PH-S88)
