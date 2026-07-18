# Legacy Playwright API-smoke (archived PH-S144)

Migrated to Rust `tests/*_integration.rs` + `cargo test-ci` (канон).

| Archived spec | Rust replacement |
|---------------|------------------|
| `jobs_lease.spec.ts` | `tests/jobs_api_contracts.rs` + `poolai-http-stand-smoke --lease-renew` (PH-S196 stand) |
| `jobs_migrating.spec.ts` | `tests/jobs_api_contracts.rs` |
| `protocol_middleware.spec.ts` | `tests/protocol_middleware_integration.rs` |
| `telegram_wallet.spec.ts` | `tests/telegram_wallet_integration.rs` (PH-S990) + `virtual_node_telegram_binding_integration.rs` |
| `grid_pricing.spec.ts` | `tests/grid_pricing_integration.rs` |
| `grid_job_lease.spec.ts` | `tests/grid_envelope_lease_integration.rs` |
| `grid_result_lease.spec.ts` | `tests/grid_envelope_lease_integration.rs` |
| `jobs_raid.spec.ts` | `tests/job_store_raid_persistence.rs` + `poolai-http-stand-smoke --raid-restart` (PH-S156) |

Do not add new API-only Playwright specs — see [`.cursor/rules/poolai-testing-policy.mdc`](../../../.cursor/rules/poolai-testing-policy.mdc).
