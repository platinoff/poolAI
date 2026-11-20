#!/usr/bin/env bash
# Setup MSVC Environment for Rust Development (bash/MSYS2 helper).
# Bash equivalent of the former setup_msvc_environment.ps1 (GEN-3 format hygiene).
#
# NOTE: MSVC environment variables (INCLUDE/LIB/PATH for cl/link) must be set in
# the *current* shell to affect builds. From bash you can either:
#   1) Open "Developer PowerShell/Command Prompt for VS 2022" and build there, or
#   2) Source the generated vars: eval "$(bash scripts/setup_msvc_environment.sh --print-env)"
#
# This script locates Visual Studio + MSVC + Windows SDK and prints the env
# exports (with --print-env) or a human summary.
set -euo pipefail

PRINT_ENV=0
while [ $# -gt 0 ]; do
  case "$1" in
    --print-env) PRINT_ENV=1; shift ;;
    -h|--help)
      echo "Usage: bash scripts/setup_msvc_environment.sh [--print-env]"
      echo "  --print-env  emit 'export PATH=...; export INCLUDE=...; export LIB=...' for eval"
      exit 0
      ;;
    *) echo "unknown arg: $1" >&2; exit 2 ;;
  esac
done

VS_PATH=""
for p in \
  "/c/Program Files/Microsoft Visual Studio/2022/Community" \
  "/c/Program Files/Microsoft Visual Studio/2022/BuildTools" \
  "/c/Program Files/Microsoft Visual Studio/2022/Professional" \
  "/c/Program Files/Microsoft Visual Studio/2022/Enterprise"; do
  if [ -d "$p" ]; then VS_PATH="$p"; break; fi
done

if [ -z "$VS_PATH" ]; then
  echo "Visual Studio 2022 not found. Install VS 2022 with C++ workload." >&2
  exit 1
fi
echo "Found Visual Studio at: $VS_PATH" >&2

MSVC_ROOT="$VS_PATH/VC/Tools/MSVC"
if [ ! -d "$MSVC_ROOT" ]; then
  echo "MSVC Tools not found (install 'Desktop development with C++')." >&2
  exit 1
fi
MSVC_VERSION="$(ls -1 "$MSVC_ROOT" 2>/dev/null | sort -V | tail -n1 || true)"
if [ -z "$MSVC_VERSION" ]; then
  echo "No MSVC versions found under $MSVC_ROOT" >&2
  exit 1
fi
echo "Using MSVC version: $MSVC_VERSION" >&2

SDK_ROOT="/c/Program Files (x86)/Windows Kits/10"
SDK_VERSION=""
if [ -d "$SDK_ROOT/Lib" ]; then
  SDK_VERSION="$(ls -1 "$SDK_ROOT/Lib" 2>/dev/null | sort -V | tail -n1 || true)"
fi

MSVC_BIN="$MSVC_ROOT/$MSVC_VERSION/bin/Hostx64/x64"
MSVC_INCLUDE="$MSVC_ROOT/$MSVC_VERSION/include"
MSVC_LIB="$MSVC_ROOT/$MSVC_VERSION/lib/x64"

INCLUDE_VAL="$MSVC_INCLUDE"
LIB_VAL="$MSVC_LIB"
if [ -n "$SDK_VERSION" ]; then
  INCLUDE_VAL="$INCLUDE_VAL;$SDK_ROOT/Include/$SDK_VERSION/shared;$SDK_ROOT/Include/$SDK_VERSION/ucrt;$SDK_ROOT/Include/$SDK_VERSION/um;$SDK_ROOT/Include/$SDK_VERSION/winrt"
  LIB_VAL="$LIB_VAL;$SDK_ROOT/Lib/$SDK_VERSION/ucrt/x64;$SDK_ROOT/Lib/$SDK_VERSION/um/x64"
fi

# Prepend MSVC bin and cargo; drop msys64 from PATH to avoid cl/link conflicts.
CARGO_BIN="${USERPROFILE:-$HOME}/.cargo/bin"
NEW_PATH="$MSVC_BIN:$CARGO_BIN:$PATH"
# best-effort removal of msys64 segments when used for MSVC builds
NEW_PATH="$(printf '%s' "$NEW_PATH" | tr ':' '\n' | grep -vi '/c/msys64' | paste -sd: -)"

if [ "$PRINT_ENV" -eq 1 ]; then
  printf 'export PATH=%q\n' "$NEW_PATH"
  printf 'export INCLUDE=%q\n' "$INCLUDE_VAL"
  printf 'export LIB=%q\n' "$LIB_VAL"
else
  echo "MSVC environment (for eval):" >&2
  echo "  PATH += $MSVC_BIN" >&2
  echo "  INCLUDE has $(( $(printf '%s' "$INCLUDE_VAL" | tr ';' '\n' | wc -l) )) paths" >&2
  echo "  LIB has $(( $(printf '%s' "$LIB_VAL" | tr ';' '\n' | wc -l) )) paths" >&2
  echo "Run: eval \"\$(bash scripts/setup_msvc_environment.sh --print-env)\"" >&2
fi
