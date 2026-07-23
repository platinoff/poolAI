#!/usr/bin/env bash
# Append project-close extension bands 151–163 (PH-S2149…S2278) to master backlog.
# Canon: docs/development/PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md
set -euo pipefail
OUT="${1:-docs/development/PH_S_MASTER_BACKLOG_1000.md}"
TODAY="2026-07-22"

phase_for_band() {
  local b=$1
  if (( b >= 151 && b <= 153 )); then echo "K Memory"; return; fi
  if (( b >= 154 && b <= 156 )); then echo "L JobDepth"; return; fi
  if (( b >= 157 && b <= 159 )); then echo "M Solana"; return; fi
  if (( b >= 160 && b <= 161 )); then echo "N WasmUI"; return; fi
  if (( b >= 162 && b <= 163 )); then echo "O ProjectClose"; return; fi
  echo "unknown"
}

slug_for_band() {
  local b=$1
  if (( b >= 151 && b <= 153 )); then echo "memory"; return; fi
  if (( b >= 154 && b <= 156 )); then echo "job_depth"; return; fi
  if (( b >= 157 && b <= 159 )); then echo "solana"; return; fi
  if (( b >= 160 && b <= 161 )); then echo "wasm_ui"; return; fi
  if (( b >= 162 && b <= 163 )); then echo "project_close"; return; fi
  echo "extension"
}

slice_name() {
  local sub=$1
  local slices=(
    "depth scaffold"
    "store wire"
    "API contracts"
    "admin/ops glue"
    "stand smoke"
    "loc-audit"
    "docs canon"
    "vision-sync"
    "ratio advisory"
    "horizon close"
  )
  echo "${slices[$sub]}"
}

# Remove previous extension marker if re-run
if grep -q 'PROJECT COMPLETION EXTENSION' "$OUT" 2>/dev/null; then
  # keep file up to line before marker
  awk '/^<!-- PROJECT COMPLETION EXTENSION/{exit} {print}' "$OUT" > "${OUT}.tmp"
  mv "${OUT}.tmp" "$OUT"
fi

{
  echo ""
  echo "<!-- PROJECT COMPLETION EXTENSION 2026-07-22 -->"
  echo ""
  echo "---"
  echo ""
  echo "# Project completion extension (bands 151–163 · PH-S2149…S2278)"
  echo ""
  echo "**Added:** ${TODAY} · **+130 sprints** · closes FM **§5.18** @ PH-S2278 · plan: [\`PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md\`](./PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md)"
  echo ""
  echo "| Band | Sprints | Theme |"
  echo "|------|---------|-------|"
  for b in $(seq 151 163); do
    # band 151 start = 2149
    start=$((2149 + (b - 151) * 10))
    end=$((start + 9))
    phase=$(phase_for_band "$b")
    # sub-index within phase group
    case "$phase" in
      "K Memory") sub=$((b - 151)) ;;
      "L JobDepth") sub=$((b - 154)) ;;
      "M Solana") sub=$((b - 157)) ;;
      "N WasmUI") sub=$((b - 160)) ;;
      "O ProjectClose") sub=$((b - 162)) ;;
      *) sub=0 ;;
    esac
    # map sub 0..2 onto first three slice names for short themes; full 10-slice still in rows
    slice=$(slice_name $((sub % 10)))
    printf "| %d | PH-S%d…S%d | %s · %s |\n" "$b" "$start" "$end" "$phase" "$slice"
  done
  echo ""

  for b in $(seq 151 163); do
    start=$((2149 + (b - 151) * 10))
    end=$((start + 9))
    phase=$(phase_for_band "$b")
    slug=$(slug_for_band "$b")
    echo "## Band ${b} — PH-S${start}…S${end} (${phase})"
    echo ""
    echo "| Sprint | Focus | Acceptance | Status |"
    echo "|--------|-------|------------|--------|"
    if (( b == 163 )); then
      cat <<EOF
| **PH-S${start}** | DIGEST project-complete-v3 truth | no overclaim vs code | **[ ]** |
| **PH-S$((start + 1))** | STABLE §5.18 section | checklist mirrored | **[ ]** |
| **PH-S$((start + 2))** | INDEX + STRUCTURE pointers | completion roadmap link | **[ ]** |
| **PH-S$((start + 3))** | OpenAPI final gap-audit | exit 0 | **[ ]** |
| **PH-S$((start + 4))** | Integration suite gate | \`cargo test-ci\` green | **[ ]** |
| **PH-S$((start + 5))** | loc-audit final zriz | \`rust_ratio.json\` | **[ ]** |
| **PH-S$((start + 6))** | FM §5.18 closure draft | checklist rows | **[ ]** |
| **PH-S$((start + 7))** | poolai-vision-sync --check | green | **[ ]** |
| **PH-S$((start + 8))** | Ratio hold advisory final | \`--min-ratio 0.95 --advisory\` | **[ ]** |
| **PH-S$((start + 9))** | FM §5.18 project-complete | PH-S2278; HANDOFF owner-scan only | **[ ]** |
EOF
    else
      cat <<EOF
| **PH-S${start}** | \`${slug}_depth\` scaffold | ui-core depth enum + criteria registry | **[ ]** |
| **PH-S$((start + 1))** | \`${slug}\` store/wire slice | durable path or verify stub + unit test | **[ ]** |
| **PH-S$((start + 2))** | \`${slug}\` API contracts | \`tests/*_integration.rs\` | **[ ]** |
| **PH-S$((start + 3))** | \`${slug}\` admin/ops glue | verify-dev-stand or admin strip | **[ ]** |
| **PH-S$((start + 4))** | Stand smoke \`${slug}\` export | export shape unit test | **[ ]** |
| **PH-S$((start + 5))** | poolai-loc-audit PH-S$((start + 5)) | \`rust_ratio.json\` zriz | **[ ]** |
| **PH-S$((start + 6))** | Docs canon sync | RUN_LOCAL/INDEX/HANDOFF/NEXT | **[ ]** |
| **PH-S$((start + 7))** | poolai-vision-sync --check | drift gate green | **[ ]** |
| **PH-S$((start + 8))** | Ratio hold advisory | \`--min-ratio 0.95 --advisory\` | **[ ]** |
| **PH-S$((start + 9))** | galaxy_horizon_s${start}_integration | band close | **[ ]** |
EOF
    fi
    echo ""
  done

  cat <<'TAIL'
**Після PH-S2278 ✅:** FM §5.18 project development complete · FM-003 LAN / FM-041 Cloud SDK / ZK/TEE поза closure · новий scan лише за запитом власника.
TAIL
} >> "$OUT"

echo "Appended extension to $OUT ($(wc -l < "$OUT") lines)"
