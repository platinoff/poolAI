import { test, expect, type APIRequestContext } from "@playwright/test";

const standRoot = process.env.POOLAI_E2E_STAND_ROOT;

/** Job that schedules without worker/vm binding (no auto lease on schedule tick). */
async function createUnboundJob(
  request: APIRequestContext,
  inputArtifactId: string,
) {
  const res = await request.post("/api/v1/jobs", {
    data: {
      kind: "inference",
      priority: 5,
      input_artifact_ids: [inputArtifactId],
      resources: { gpu_memory_mb: 9_007_199_254_740_991 },
    },
  });
  expect(res.status()).toBe(201);
  return (await res.json()) as { id: string };
}

test.describe("Jobs lease API smoke (PH-S107)", () => {
  test.skip(
    !standRoot,
    "requires bin/e2e-playwright.sh --start (POOLAI_E2E_STAND_ROOT)",
  );

  test("POST /jobs/{id}/lease acquires lease and sets leased status", async ({
    request,
  }) => {
    const created = await createUnboundJob(
      request,
      "ph-s107-lease-acquire",
    );

    const acquire = await request.post(`/api/v1/jobs/${created.id}/lease`, {
      data: { lease_owner: "e2e-worker-lease" },
    });
    expect(acquire.status()).toBe(200);
    const body = await acquire.json();
    const job = body.job;
    expect(job.status).toBe("leased");
    expect(job.lease_owner).toBe("e2e-worker-lease");
    expect(job.lease_epoch).toBe(1);
    expect(job.lease_expires_at).toBeTruthy();
  });

  test("second acquire returns 409 lease_already_active", async ({ request }) => {
    const created = await createUnboundJob(
      request,
      "ph-s107-lease-conflict",
    );

    const first = await request.post(`/api/v1/jobs/${created.id}/lease`, {
      data: { lease_owner: "e2e-worker-a" },
    });
    expect(first.status()).toBe(200);

    const second = await request.post(`/api/v1/jobs/${created.id}/lease`, {
      data: { lease_owner: "e2e-worker-b" },
    });
    expect(second.status()).toBe(409);
    const err = await second.json();
    expect(err.error?.code).toBe("lease_already_active");
  });

  test("POST /jobs/{id}/lease/renew extends lease_expires_at", async ({
    request,
  }) => {
    const created = await createUnboundJob(request, "ph-s107-lease-renew");

    const acquired = await request.post(`/api/v1/jobs/${created.id}/lease`, {
      data: { lease_owner: "e2e-worker-renew" },
    });
    expect(acquired.status()).toBe(200);
    const acquiredBody = await acquired.json();
    const epoch = acquiredBody.job.lease_epoch as number;
    const expiresBefore = acquiredBody.job.lease_expires_at as string;

    const renewed = await request.post(
      `/api/v1/jobs/${created.id}/lease/renew`,
      { data: { lease_epoch: epoch } },
    );
    expect(renewed.status()).toBe(200);
    const renewedBody = await renewed.json();
    expect(renewedBody.job.lease_epoch).toBe(epoch);
    expect(renewedBody.job.lease_expires_at).not.toBe(expiresBefore);
  });

  test("renew with stale epoch returns 409 lease_epoch_rejected", async ({
    request,
  }) => {
    const created = await createUnboundJob(
      request,
      "ph-s107-lease-renew-reject",
    );

    const acquired = await request.post(`/api/v1/jobs/${created.id}/lease`, {
      data: { lease_owner: "e2e-worker-cas" },
    });
    expect(acquired.status()).toBe(200);
    const epoch = (await acquired.json()).job.lease_epoch as number;

    const rejected = await request.post(
      `/api/v1/jobs/${created.id}/lease/renew`,
      { data: { lease_epoch: epoch > 0 ? epoch - 1 : 0 } },
    );
    expect(rejected.status()).toBe(409);
    const err = await rejected.json();
    expect(err.error?.code).toBe("lease_epoch_rejected");
  });
});

test.describe("Jobs lease negative paths (PH-S118)", () => {
  test.skip(
    !standRoot,
    "requires bin/e2e-playwright.sh --start (POOLAI_E2E_STAND_ROOT)",
  );

  test("renew without acquire returns 400 validation", async ({ request }) => {
    const created = await createUnboundJob(
      request,
      "ph-s118-renew-no-acquire",
    );

    const renew = await request.post(
      `/api/v1/jobs/${created.id}/lease/renew`,
      { data: { lease_epoch: 1 } },
    );
    expect(renew.status()).toBe(400);
    const err = await renew.json();
    const msg = String(err.error?.message || err.message || "");
    expect(msg).toMatch(/acquire lease/i);
  });

  test("renew after lease TTL returns 409 lease_expired", async ({
    request,
  }) => {
    const created = await createUnboundJob(request, "ph-s118-lease-expired");

    const acquired = await request.post(`/api/v1/jobs/${created.id}/lease`, {
      data: { lease_owner: "e2e-worker-expired" },
    });
    expect(acquired.status()).toBe(200);
    const epoch = (await acquired.json()).job.lease_epoch as number;

    await new Promise((r) => setTimeout(r, 2600));

    const expired = await request.post(
      `/api/v1/jobs/${created.id}/lease/renew`,
      { data: { lease_epoch: epoch } },
    );
    expect(expired.status()).toBe(409);
    const err = await expired.json();
    expect(err.error?.code).toBe("lease_expired");
  });

  test("second acquire by different owner returns 409 lease_already_active", async ({
    request,
  }) => {
    const created = await createUnboundJob(request, "ph-s118-wrong-owner");

    const first = await request.post(`/api/v1/jobs/${created.id}/lease`, {
      data: { lease_owner: "e2e-owner-a" },
    });
    expect(first.status()).toBe(200);

    const second = await request.post(`/api/v1/jobs/${created.id}/lease`, {
      data: { lease_owner: "e2e-owner-b" },
    });
    expect(second.status()).toBe(409);
    const err = await second.json();
    expect(err.error?.code).toBe("lease_already_active");
  });
});
