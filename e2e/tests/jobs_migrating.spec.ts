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

test.describe("Jobs migrating lifecycle E2E (PH-S133)", () => {
  test.skip(
    !standRoot,
    "requires bin/e2e-playwright.sh --start (POOLAI_E2E_STAND_ROOT)",
  );

  test("PATCH leased → migrating → executing roundtrip", async ({ request }) => {
    const created = await createUnboundJob(
      request,
      "ph-s133-migrate-roundtrip",
    );

    const acquired = await request.post(`/api/v1/jobs/${created.id}/lease`, {
      data: { lease_owner: "e2e-worker-migrate" },
    });
    expect(acquired.status()).toBe(200);
    const acquiredBody = await acquired.json();
    expect(acquiredBody.job.status).toBe("leased");

    const migrating = await request.patch(`/api/v1/jobs/${created.id}`, {
      data: { status: "migrating" },
    });
    expect(migrating.status()).toBe(200);
    const migratingBody = await migrating.json();
    expect(migratingBody.job.status).toBe("migrating");

    const executing = await request.patch(`/api/v1/jobs/${created.id}`, {
      data: { status: "executing" },
    });
    expect(executing.status()).toBe(200);
    const executingBody = await executing.json();
    expect(executingBody.job.status).toBe("executing");
  });

  test("PATCH executing ↔ migrating roundtrip", async ({ request }) => {
    const created = await createUnboundJob(
      request,
      "ph-s133-exec-migrate-exec",
    );

    const acquired = await request.post(`/api/v1/jobs/${created.id}/lease`, {
      data: { lease_owner: "e2e-worker-migrate-b" },
    });
    expect(acquired.status()).toBe(200);

    const toMigrating = await request.patch(`/api/v1/jobs/${created.id}`, {
      data: { status: "migrating" },
    });
    expect(toMigrating.status()).toBe(200);

    const toExecuting = await request.patch(`/api/v1/jobs/${created.id}`, {
      data: { status: "executing" },
    });
    expect(toExecuting.status()).toBe(200);
    expect((await toExecuting.json()).job.status).toBe("executing");

    const backToMigrating = await request.patch(`/api/v1/jobs/${created.id}`, {
      data: { status: "migrating" },
    });
    expect(backToMigrating.status()).toBe(200);
    expect((await backToMigrating.json()).job.status).toBe("migrating");
  });
});
