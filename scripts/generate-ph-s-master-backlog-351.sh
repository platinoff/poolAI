#!/usr/bin/env bash
# Generate docs/development/PH_S_MASTER_BACKLOG_351.md — 351 pending sprints PH-S660…PH-S1010
set -euo pipefail
OUT="${1:-docs/development/PH_S_MASTER_BACKLOG_351.md}"
TODAY="2026-06-20"

band_theme() {
  local b=$1
  local themes=(
    "Galaxy prefetch/locality wire depth"
    "Galaxy verification/replay wire depth"
    "Galaxy settlement/trust wire depth"
    "Galaxy replication/pricing wire depth"
    "Admin wasm slim (ui-core + poolai-ui-wasm)"
    "Stand smoke /metrics parity"
    "Concept wire stub (Galaxy §4–§8)"
    "Ops loc-audit + ratio advisory"
    "Docs canon sync band"
    "Horizon integration close band"
  )
  echo "${themes[$(( (b - 1) % 10 ))]}"
}

slot_focus() {
  local start=$1 slot=$2 theme=$3
  local s=$((start + slot - 1))
  case $slot in
    1|2) echo "Galaxy metric HTTP wire + integration test (band theme: ${theme})" ;;
    3) echo "Admin panel wasm glue (poolai-ui-core + admin_charts.js)" ;;
    4) echo "poolai-http-stand-smoke /metrics export shape" ;;
    5) echo "Galaxy concept helper stub + unit test" ;;
    6) echo "poolai-loc-audit → rust_ratio.json sprint zriz PH-S${s}" ;;
    7) echo "INDEX/HANDOFF/NEXT/STABLE/GALAXY canon pointer sync" ;;
    8) echo "poolai-vision-sync --check green" ;;
    9) echo "Ratio hold advisory --min-ratio 0.95 --advisory snapshot" ;;
    10) echo "Horizon close galaxy_horizon_s${start}_integration + docs close band" ;;
  esac
}

{
  echo "# PH-S master backlog (351 pending)"
  echo ""
  echo "**Generated:** ${TODAY} · **Range:** PH-S660…PH-S1010 · **Total:** **351** · **Active §5.12:** max **10**"
  echo ""
  echo "**VDT:** один \`абракадабра\` = drain **10** відкритих з FM §5.12 → vision close → push → promote **наступні 10** з цього реєстру."
  echo ""
  echo "**Джерела scan:** concept POOLAI_GALAXY_GRID §TBD/horizon · GALAXY_GRID_ROADMAP · RUST_RATIO_STRATEGY §5.13 · FM §5.3 · src/ui/*.js wasm slim · ui-core blockers · **BLOCKED/Deferred excluded** (FM-003, FM-041)."
  echo ""
  echo "| Band | Sprints | Theme |"
  echo "|------|---------|-------|"

  for b in $(seq 1 35); do
    start=$((660 + (b - 1) * 10))
    end=$((start + 9))
    theme=$(band_theme "$b")
    printf "| %d | PH-S%d…S%d | %s |\n" "$b" "$start" "$end" "$theme"
  done
  echo "| 36 | PH-S1010 | Master backlog tail / replenish marker |"
  echo ""
  echo "---"
  echo ""

  echo "## Band 1 — PH-S660…S669 (active drain)"
  echo ""
  echo "| Sprint | Focus | Source | Acceptance |"
  echo "|--------|-------|--------|------------|"
  cat <<'B1'
| **PH-S660** | ui-core format timestamp UTC fix | PH-S655 / format.rs | `format_unix_timestamp_display_ph_s628` green |
| **PH-S661** | ui-core ML metric URL encode fix | PH-S655 / ml.rs | `build_metric_history_url_ph_s314/s334` green |
| **PH-S662** | ui-core full test gate | Rust test policy | `cargo test -p poolai-ui-core` 0 failed |
| **PH-S663** | Shared layout datetime wasm-only | RUST_RATIO §5.13 | drop `toLocaleString` in `src/ui/mod.rs` |
| **PH-S664** | network_profile persist stub | Galaxy §8 L916 | heartbeat metadata persist stub + unit test |
| **PH-S665** | Rust ratio loc-audit refresh | §5.13 fallback | `rust_ratio.json` sprint zriz |
| **PH-S666** | Docs INDEX canon sync | docs canon | INDEX §7 + ratio + vision rev |
| **PH-S667** | poolai-vision-sync drift gate | ops | `--check` green |
| **PH-S668** | Ratio hold advisory snapshot | §5.13 / PH-S351 pattern | `--min-ratio 0.95 --advisory` |
| **PH-S669** | Horizon close band S660–S668 | §5.12 fallback | `galaxy_horizon_s660_integration` + docs sync |
B1
  echo ""

  for b in $(seq 2 35); do
    start=$((660 + (b - 1) * 10))
    end=$((start + 9))
    theme=$(band_theme "$b")
    echo "## Band ${b} — PH-S${start}…S${end} (${theme})"
    echo ""
    echo "| Sprint | Focus | Acceptance |"
    echo "|--------|-------|------------|"
    for slot in $(seq 1 10); do
      s=$((start + slot - 1))
      focus=$(slot_focus "$start" "$slot" "$theme")
      printf "| **PH-S%d** | %s | scope test green |\n" "$s" "$focus"
    done
    echo ""
  done

  echo "## Band 36 — PH-S1010 (tail)"
  echo ""
  echo "| Sprint | Focus | Acceptance |"
  echo "|--------|-------|------------|"
  echo "| **PH-S1010** | Master backlog replenish marker + FM §5.14 sync | FM/HANDOFF/NEXT note 351 backlog complete; next scan after S1010 ✅ |"
  echo ""
  echo "---"
  echo ""
  echo "**Після PH-S1010 ✅:** новий project scan → наступний master backlog або FM-horizon."
} > "$OUT"
echo "Wrote $OUT ($(wc -l < "$OUT") lines)"
