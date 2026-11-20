#!/usr/bin/env bash
# Setup MSYS2 PATH for Rust compilation (bash equivalent of former .ps1).
# Source this to affect the current shell:  source scripts/setup_msys2_path.sh
#
# Adds MSYS2 UCRT64/usr bin dirs and exports CC for gcc/dlltool toolchain.

_msys2_added=()
for _msys2_path in "/c/msys64/ucrt64/bin" "/c/msys64/usr/bin"; do
  if [ -d "$_msys2_path" ]; then
    case ":$PATH:" in
      *":${_msys2_path}:"*) echo "MSYS2 already in PATH: ${_msys2_path}" ;;
      *) export PATH="${_msys2_path}:$PATH"; _msys2_added+=("${_msys2_path}"); echo "Added MSYS2 to PATH: ${_msys2_path}" ;;
    esac
  else
    echo "MSYS2 path not found: ${_msys2_path}"
  fi
done

if [ ${#_msys2_added[@]} -gt 0 ]; then
  echo
  echo "MSYS2 paths added to PATH for this session"
  echo "Note: for permanence, add to ~/.bashrc or your shell profile"
fi

echo
echo "Verifying tools..."
for _tool in gcc dlltool; do
  if command -v "$_tool" >/dev/null 2>&1; then
    echo "  ${_tool} found at: $(command -v "$_tool")"
  else
    echo "  Warning: ${_tool} not found in PATH"
  fi
done

export CC="gcc"
export CC_x86_64_pc_windows_gnu="gcc"
echo
echo "CC environment variables set:"
echo "  CC=$CC"
echo "  CC_x86_64_pc_windows_gnu=$CC_x86_64_pc_windows_gnu"

unset _msys2_added _msys2_path _tool
