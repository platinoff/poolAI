import { test, expect } from "@playwright/test";

const standRoot = process.env.POOLAI_E2E_STAND_ROOT;

function registerPayload(peerId: string, protocolVersion = "1.2") {
  return {
    peer_id: peerId,
    address: "10.0.0.1",
    port: 9091,
    protocol_version: protocolVersion,
    build_id: "e2e-protocol-middleware",
    metadata: { role: "virtual_node" },
  };
}

test.describe("Protocol middleware E2E smoke (PH-S134)", () => {
  test.skip(
    !standRoot,
    "requires bin/e2e-playwright.sh --start (POOLAI_E2E_STAND_ROOT)",
  );

  test("register-remote with X-PoolAI-Protocol 1.2 adds compat response headers", async ({
    request,
  }) => {
    const peerId = `e2e-proto-accept-${Date.now()}`;
    const res = await request.post("/api/v1/discovery/register-remote", {
      headers: { "X-PoolAI-Protocol": "1.2" },
      data: registerPayload(peerId, "1.2"),
    });
    expect(res.status()).toBe(200);
    const body = await res.json();
    expect(body.registered).toBe(true);
    expect(body.compat_status).toBe("accepted");
    expect(res.headers()["x-poolai-protocol-coordinator"]).toBeTruthy();
    expect(res.headers()["x-poolai-protocol-compat"]).toBe("accepted");
    expect(res.headers()["x-poolai-protocol-docs"]).toContain("POOLAI_GALAXY_GRID");
  });

  test("register-remote with unsupported X-PoolAI-Protocol returns 403", async ({
    request,
  }) => {
    const peerId = `e2e-proto-reject-${Date.now()}`;
    const res = await request.post("/api/v1/discovery/register-remote", {
      headers: { "X-PoolAI-Protocol": "1.0" },
      data: registerPayload(peerId, "1.2"),
    });
    expect(res.status()).toBe(403);
    const err = await res.json();
    expect(err.error?.code).toBe("protocol_unsupported");
    expect(res.headers()["x-poolai-protocol-compat"]).toBe("unsupported");
  });
});
