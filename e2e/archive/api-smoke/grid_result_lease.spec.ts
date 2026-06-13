import { test, expect } from "@playwright/test";

const standRoot = process.env.POOLAI_E2E_STAND_ROOT;

function gridJobEnvelope(jobId: string, sourcePeerId: string) {
  return {
    v: 1,
    sent_at: new Date().toISOString(),
    source_peer_id: sourcePeerId,
    type: "job" as const,
    job_id: jobId,
    task_kind: "inference",
    input_artifact_ids: [`artifact-${jobId}`],
  };
}

function gridResultEnvelope(jobId: string, leaseEpoch?: number) {
  return {
    v: 1,
    sent_at: new Date().toISOString(),
    type: "result" as const,
    job_id: jobId,
    status: "completed" as const,
    output_artifact_ids: [`out-${jobId}`],
    ...(leaseEpoch !== undefined ? { lease_epoch: leaseEpoch } : {}),
  };
}

test.describe("Grid Result lease_epoch CAS (PH-S117)", () => {
  test.skip(
    !standRoot,
    "requires bin/e2e-playwright.sh --start (POOLAI_E2E_STAND_ROOT)",
  );

  test("stale lease_epoch on Result → 409 lease_epoch_rejected", async ({
    request,
  }) => {
    const jobId = `ph-s117-grid-result-${Date.now()}`;
    const peerId = "e2e-grid-result-peer";

    const jobIngest = await request.post("/api/v1/grid/envelope", {
      data: gridJobEnvelope(jobId, peerId),
    });
    expect(jobIngest.status()).toBe(200);

    const getJob = await request.get(`/api/v1/jobs/${jobId}`);
    expect(getJob.status()).toBe(200);
    const epoch = (await getJob.json()).job.lease_epoch as number;

    const stale = await request.post("/api/v1/grid/envelope", {
      data: gridResultEnvelope(jobId, epoch > 0 ? epoch - 1 : 0),
    });
    expect(stale.status()).toBe(409);
    const err = await stale.json();
    expect(err.error?.code).toBe("lease_epoch_rejected");

    const afterReject = await request.get(`/api/v1/jobs/${jobId}`);
    expect((await afterReject.json()).job.status).toBe("leased");
  });

  test("matching lease_epoch on Result → completed", async ({ request }) => {
    const jobId = `ph-s117-grid-ok-${Date.now()}`;
    const peerId = "e2e-grid-result-ok";

    const jobIngest = await request.post("/api/v1/grid/envelope", {
      data: gridJobEnvelope(jobId, peerId),
    });
    expect(jobIngest.status()).toBe(200);

    const getJob = await request.get(`/api/v1/jobs/${jobId}`);
    const epoch = (await getJob.json()).job.lease_epoch as number;

    const result = await request.post("/api/v1/grid/envelope", {
      data: gridResultEnvelope(jobId, epoch),
    });
    expect(result.status()).toBe(200);
    const body = await result.json();
    expect(body.ok).toBe(true);
    expect(body.type).toBe("result");
    expect(body.status).toBe("completed");

    const finalJob = await request.get(`/api/v1/jobs/${jobId}`);
    expect((await finalJob.json()).job.status).toBe("completed");
  });
});
