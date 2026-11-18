#!/usr/bin/env bash
# PoolAI — перевірка вільного місця на томі репозиторію та (опційно) розміру target/.
# Запускати з MSYS2 bash перед важкими `cargo test --all-features` / довгими збірками.
#
# Usage:
#   bash scripts/check_target_disk.sh              # лише попередження в stderr
#   bash scripts/check_target_disk.sh --enforce    # exit 1 при порушенні лімітів
#
# Env (опційно):
#   POOLAI_MIN_FREE_DISK_GB   — мінімум вільних ГБ на томі (default: 12; 0 = не перевіряти)
#   POOLAI_MAX_TARGET_DIR_GB  — попередження/фейл, якщо target більший за N ГБ (default: 48; 0 = не перевіряти)
#   POOLAI_MIN_FREE_RAM_GB    — мінімум вільної RAM (MemFree) у ГБ (default: 3; 0 = не перевіряти)
#   POOLAI_ENFORCE_DISK_LIMIT=1 — те саме, що --enforce
#   CARGO_TARGET_DIR          — каталог артефактів (як у Cargo); інакше <repo>/target

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

MIN_FREE_GB="${POOLAI_MIN_FREE_DISK_GB:-12}"
MAX_TARGET_GB="${POOLAI_MAX_TARGET_DIR_GB:-48}"
MIN_RAM_GB="${POOLAI_MIN_FREE_RAM_GB:-3}"
[[ "$MIN_FREE_GB" =~ ^[0-9]+$ ]] || MIN_FREE_GB=12
[[ "$MAX_TARGET_GB" =~ ^[0-9]+$ ]] || MAX_TARGET_GB=48
[[ "$MIN_RAM_GB" =~ ^[0-9]+$ ]] || MIN_RAM_GB=3
ENFORCE=0
if [ "${POOLAI_ENFORCE_DISK_LIMIT:-0}" = "1" ]; then
  ENFORCE=1
fi
for arg in "$@"; do
  case "$arg" in
    --enforce) ENFORCE=1 ;;
    --help|-h)
      sed -n '1,20p' "$0"
      exit 0
      ;;
  esac
done

if [ ! -f "$REPO_ROOT/Cargo.toml" ]; then
  echo "check_target_disk.sh: Cargo.toml not found under $REPO_ROOT" >&2
  exit 1
fi

TARGET_DIR="${CARGO_TARGET_DIR:-$REPO_ROOT/target}"

# df -Pk: Available in 1K blocks (POSIX; MSYS2 / Linux / macOS)
FREE_KB="$(df -Pk "$REPO_ROOT" 2>/dev/null | tail -1 | awk '{print $4}')"
if ! [[ "$FREE_KB" =~ ^[0-9]+$ ]]; then
  echo "check_target_disk.sh: could not read free disk space for $REPO_ROOT (df -Pk)" >&2
  FREE_KB=""
fi

FREE_GB=0
if [ -n "$FREE_KB" ]; then
  FREE_GB=$((FREE_KB / 1024 / 1024))
fi

TARGET_GB=0
if [ -d "$TARGET_DIR" ]; then
  SIZE_KB="$(du -sk "$TARGET_DIR" 2>/dev/null | cut -f1)"
  if [[ "$SIZE_KB" =~ ^[0-9]+$ ]]; then
    TARGET_GB=$((SIZE_KB / 1024 / 1024))
  fi
fi

# RAM guard: this box's hub dies silently when free memory drops to ~0.7 GiB
# (Firefox + Telegram + heavy builds). MemFree (not MemAvailable) is what
# MSYS2 exposes; the number is conservative on purpose.
FREE_RAM_GB=0
RAM_KNOWN=0
if [ -r /proc/meminfo ]; then
  RAM_KB="$(grep -m1 '^MemFree:' /proc/meminfo | awk '{print $2}')"
  if [[ "$RAM_KB" =~ ^[0-9]+$ ]]; then
    FREE_RAM_GB=$((RAM_KB / 1024 / 1024))
    RAM_KNOWN=1
  fi
fi

VIOLATION=0

echo "check_target_disk: repo=$REPO_ROOT" >&2
echo "check_target_disk: target_dir=$TARGET_DIR (size ~${TARGET_GB} GiB)" >&2
if [ -n "$FREE_KB" ]; then
  echo "check_target_disk: free on volume ~${FREE_GB} GiB (min configured: ${MIN_FREE_GB} GiB)" >&2
else
  echo "check_target_disk: free space unknown" >&2
fi
if [ "$RAM_KNOWN" -eq 1 ]; then
  echo "check_target_disk: free RAM ~${FREE_RAM_GB} GiB (min configured: ${MIN_RAM_GB} GiB)" >&2
else
  echo "check_target_disk: free RAM unknown (/proc/meminfo unreadable)" >&2
fi

if [ -n "$FREE_KB" ] && [ "${MIN_FREE_GB}" -gt 0 ] && [ "$FREE_GB" -lt "$MIN_FREE_GB" ]; then
  echo "check_target_disk: WARNING: free disk (${FREE_GB} GiB) < POOLAI_MIN_FREE_DISK_GB (${MIN_FREE_GB}). Risk: linker/test failures." >&2
  echo "check_target_disk: hint: cargo clean, or set CARGO_TARGET_DIR to a volume with more space." >&2
  VIOLATION=1
fi

if [ "${MAX_TARGET_GB}" -gt 0 ] && [ "$TARGET_GB" -gt "$MAX_TARGET_GB" ]; then
  echo "check_target_disk: WARNING: target dir (~${TARGET_GB} GiB) > POOLAI_MAX_TARGET_DIR_GB (${MAX_TARGET_GB})." >&2
  echo "check_target_disk: hint: cargo clean, or prune old feature builds." >&2
  VIOLATION=1
fi

if [ "$RAM_KNOWN" -eq 1 ] && [ "${MIN_RAM_GB}" -gt 0 ] && [ "$FREE_RAM_GB" -lt "$MIN_RAM_GB" ]; then
  echo "check_target_disk: WARNING: free RAM (${FREE_RAM_GB} GiB) < POOLAI_MIN_FREE_RAM_GB (${MIN_RAM_GB})." >&2
  echo "check_target_disk: hint: poolAI test-ci is NOT for this box (RAM/EDR) — it lives in GitHub Actions; close Firefox/Telegram or reboot before heavy builds." >&2
  VIOLATION=1
fi

if [ "$VIOLATION" -eq 1 ] && [ "$ENFORCE" -eq 1 ]; then
  echo "check_target_disk: ENFORCE: exiting 1 (--enforce or POOLAI_ENFORCE_DISK_LIMIT=1)" >&2
  exit 1
fi

exit 0
