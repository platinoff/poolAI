# Building with HTTPS Feature on Windows

The `https` feature requires a C compiler (GCC) for native dependencies (`aws-lc-sys`).

## Prerequisites

- MSYS2 installed with GCC toolchain
- GCC available in PATH

## Building

### In MSYS2 Bash:

```bash
export PATH="/c/msys64/ucrt64/bin:/c/msys64/usr/bin:$PATH"
export CC="gcc"
export CXX="g++"
cargo build --features enterprise,https,jwt
```

Or use the provided script:

```bash
chmod +x scripts/build-with-https.sh
./scripts/build-with-https.sh
```

### In PowerShell:

```powershell
$env:PATH = "C:\msys64\ucrt64\bin;C:\msys64\usr\bin;$env:PATH"
$env:CC = "gcc"
$env:CXX = "g++"
cargo build --features enterprise,https,jwt
```

## Note

- Without HTTPS feature, the project builds without C compiler requirements
- CI on Linux should work automatically as GCC is available in standard images
