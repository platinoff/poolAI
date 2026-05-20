#!/usr/bin/env python3
"""DEV-ONLY: Compare Axum .route() paths with docs/openapi.yaml.

Not part of PoolAI runtime (see .cursor/rules/runtime-stack-policy.mdc).
Run manually for OpenAPI gap audits; product stack is Rust-only.
"""
from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
NETWORK = ROOT / "src" / "network"
OPENAPI = ROOT / "docs" / "openapi.yaml"


def collect_routes() -> set[str]:
    routes: set[str] = set()
    pat = re.compile(r'\.route\(\s*"([^"]+)"')
    for path in NETWORK.rglob("*.rs"):
        text = path.read_text(encoding="utf-8", errors="ignore")
        for m in pat.finditer(text):
            routes.add(m.group(1))
    return routes


def collect_openapi_paths() -> set[str]:
    text = OPENAPI.read_text(encoding="utf-8")
    return set(re.findall(r"^  (/[^:]+):", text, re.M))


# Routes registered under Router::nest(prefix, …) — openapi paths include the prefix.
NEST_PREFIX: dict[str, str] = {
    "/": "/ai-ml/",
    "/status": "/ai-ml/",
    "/optimization": "/ai-ml/",
    "/optimization/profile": "/ai-ml/",
    "/optimization/tuning": "/ai-ml/",
    "/optimization/quantization-result": "/ai-ml/",
    "/automl": "/ai-ml/",
    "/federated": "/ai-ml/",
    "/pipeline": "/ai-ml/",
    "/pipeline/demo": "/ai-ml/",
    "/pipeline/{id}": "/ai-ml/",
    "/pipeline/{id}/execute": "/ai-ml/",
}

# Redirects / examples — not product API surface.
IGNORE_ROUTES = {"/ui/", "/api/workers"}


def openapi_path_for_route(route: str) -> str:
    if route in NEST_PREFIX:
        return NEST_PREFIX[route] + route.lstrip("/")
    return route


def main() -> int:
    routes = collect_routes()
    paths = collect_openapi_paths()
    missing = sorted(
        r
        for r in routes
        if r not in IGNORE_ROUTES and openapi_path_for_route(r) not in paths
    )
    extra = sorted(
        p
        for p in paths
        if not any(
            p == r
            or (p.startswith("/api/") is False and r == p)
            for r in routes
        )
    )
    print(f"routes in src/network: {len(routes)}")
    print(f"paths in openapi.yaml: {len(paths)}")
    print("\n=== In code, missing from openapi.yaml ===")
    for m in missing:
        print(m)
    print(f"\nTotal missing: {len(missing)}")
    return 0 if not missing else 1


if __name__ == "__main__":
    sys.exit(main())
