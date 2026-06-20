#!/usr/bin/env bash
# Generate docs/development/PH_S_MASTER_BACKLOG_351.md — 351 pending sprints PH-S660…PH-S1010
# Theme-aware slots 1–2 (concept/roadmap aligned); slots 3–10 shared ops close band.
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

# Slots 1–2 follow band theme (Galaxy § / RUST_RATIO §5.13); 3–10 ops close band.
slot_focus() {
  local start=$1 slot=$2 b=$3
  local s=$((start + slot - 1))
  local ti=$(( (b - 1) % 10 ))

  if (( slot <= 2 )); then
    case $ti in
      0)
        if (( slot == 1 )); then
          echo "GET /api/v1/grid/prefetch-metrics HTTP wire (Galaxy §5.5)"
        else
          echo "GET /api/v1/grid/locality-metrics HTTP wire (Galaxy §5.2)"
        fi
        ;;
      1)
        if (( slot == 1 )); then
          echo "GET /api/v1/grid/verification-metrics HTTP wire (Galaxy §6.2)"
        else
          echo "GET /api/v1/grid/replay-metrics HTTP wire (Galaxy §6.3)"
        fi
        ;;
      2)
        if (( slot == 1 )); then
          echo "GET /api/v1/grid/settlement-metrics HTTP wire (Galaxy §6.4)"
        else
          echo "GET /api/v1/grid/trust-metrics HTTP wire (Galaxy §6.5)"
        fi
        ;;
      3)
        if (( slot == 1 )); then
          echo "GET /api/v1/grid/replication-metrics HTTP wire (Galaxy §6.4 replication)"
        else
          echo "GET /api/v1/grid/pricing-metrics HTTP wire (Galaxy §4.2 oracle snapshot)"
        fi
        ;;
      4)
        if (( slot == 1 )); then
          echo "Admin wasm slim panel #1 (poolai-ui-core → poolai-ui-wasm)"
        else
          echo "Admin wasm slim panel #2 (admin_charts.js canvas glue → wasm)"
        fi
        ;;
      5)
        if (( slot == 1 )); then
          echo "Stand smoke — JSON metric API export shape #1 (band parity)"
        else
          echo "Stand smoke — JSON metric API export shape #2 (band parity)"
        fi
        ;;
      6)
        if (( slot == 1 )); then
          echo "Galaxy concept helper stub #1 (Galaxy §4–§8)"
        else
          echo "Galaxy concept helper stub #2 (Galaxy §4–§8)"
        fi
        ;;
      7)
        if (( slot == 1 )); then
          echo "poolai-ui-core maintain / warning cleanup (RUST_RATIO §5.13)"
        else
          echo "poolai-loc-audit ratio snapshot PH-S${s}"
        fi
        ;;
      8)
        if (( slot == 1 )); then
          echo "INDEX canon sync — step 8 + §7 ratio pointer"
        else
          echo "HANDOFF/NEXT/STABLE/GALAXY canon pointer sync"
        fi
        ;;
      9)
        if (( slot == 1 )); then
          echo "Horizon band metric wire #1 (integration scaffold)"
        else
          echo "Horizon band metric wire #2 (integration scaffold)"
        fi
        ;;
    esac
    return
  fi

  case $slot in
    3)
      case $ti in
        4) echo "Admin wasm glue regression test (admin/mod.rs wasm render/parsePrometheusGauge)" ;;
        8) echo "FM §5.14 master backlog cross-check + vision rev pointer" ;;
        *) echo "Admin panel wasm glue (poolai-ui-core + admin JSON/metrics fetch)" ;;
      esac
      ;;
    4)
      case $ti in
        5) echo "poolai-http-stand-smoke — extend runner for band metric APIs" ;;
        *) echo "poolai-http-stand-smoke /metrics + JSON metric API export shape" ;;
      esac
      ;;
    5)
      case $ti in
        6) echo "Galaxy concept helper stub + unit test (band theme)" ;;
        7) echo "Ratio advisory pre-check gate (ui-core or stand smoke)" ;;
        *) echo "Galaxy concept helper stub + unit test (Galaxy §4–§8 horizon)" ;;
      esac
      ;;
    6) echo "poolai-loc-audit → rust_ratio.json sprint zriz PH-S${s}" ;;
    7)
      case $ti in
        8) echo "INDEX §7 rust_ratio + GALAXY_GRID_ROADMAP horizon table sync" ;;
        *) echo "INDEX/HANDOFF/NEXT/STABLE/GALAXY canon pointer sync" ;;
      esac
      ;;
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
  echo "**Band slot map (theme-aware):** slots **1–2** = band theme (Galaxy JSON metric APIs §4–§6, wasm slim §5.13, stand smoke, concept stubs, ops, docs, horizon). Slots **3–10** = shared ops close (wasm glue, stand smoke, stub, loc-audit, docs, vision, advisory, \`galaxy_horizon_sNN0_integration\`). Regen: \`bash scripts/generate-ph-s-master-backlog-351.sh\`."
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

  echo "## Band 1 — PH-S660…S669 (drained ✅ · ui-core + network_profile)"
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
    status=""
    if (( b <= 3 )); then
      status=" (drained ✅)"
    elif (( b == 4 )); then
      status=" (**active §5.12**)"
    fi
    echo "## Band ${b} — PH-S${start}…S${end} (${theme})${status}"
    echo ""
    echo "| Sprint | Focus | Acceptance |"
    echo "|--------|-------|------------|"
    for slot in $(seq 1 10); do
      s=$((start + slot - 1))
      focus=$(slot_focus "$start" "$slot" "$b")
      printf "| **PH-S%d** | %s | scope test green |\n" "$s" "$focus"
    done
    echo ""
  done

  echo "## Band 36 — PH-S1010 (tail)"
  echo ""
  echo "| Sprint | Focus | Acceptance |"
  echo "|--------|-------|------------|"
  echo "| **PH-S1010** | Master backlog replenish marker + FM §5.14 sync | FM/HANDOFF/NEXT note 351 backlog complete; next scan (signed caps, full network_profile persist, §8.2 payout) |"
  echo ""
  echo "---"
  echo ""
  echo "**Після PH-S1010 ✅:** новий project scan → наступний master backlog або FM-horizon (Galaxy §6.6 signed capabilities, §8.2 payout wire, pricing live fetch PH-S102)."
} > "$OUT"
echo "Wrote $OUT ($(wc -l < "$OUT") lines)"
