#!/bin/bash
# Build script for poolAI with HTTPS feature enabled
# Requires MSYS2 GCC toolchain

export PATH="/c/msys64/ucrt64/bin:/c/msys64/usr/bin:$PATH"
export CC="gcc"
export CXX="g++"

cargo build --features enterprise,https,jwt
