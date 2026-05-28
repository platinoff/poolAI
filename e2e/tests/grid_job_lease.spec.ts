import { test, expect } from "@playwright/test";

const standRoot = process.env.POOLAI_E2E_STAND_ROOT;

function gridJobEnvelope(jobId: string, sourcePeerId?: string) {
  return {
    v: 1,
    sent_at: new Date().toISOString(),
    ...(sourcePeerId ? { source_peer_id: sourcePeerId } : {}),
    type: "job" as const,
    job_id: jobId,
    task_kind: "inference",
    input_artifact_ids: [`artifact-${jobId}`],
  };
}

test.describe("Grid Job envelope lease (PH-S112)", () => {
  test.skip(
    !standRoot,
    "requires bin/e2e-playwright.sh --start (POOLAI_E2E_STAND_ROOT)",
  );

  test("POST /grid/envelope Job + peer → leased + lease fields", async ({
    request,
  }) => {
    const jobId = `ph-s112-grid-${Date.now()}`;
    const peerId = "e2e-grid-peer-a";

    const ingest = await request.post("/api/v1/grid/envelope", {
      data: gridJobEnvelope(jobId, peerId),
    });
    expect(ingest.status()).toBe(200);
    const body = await ingest.json();
    expect(body.ok).toBe(true);
    expect(body.type).toBe("job");
    expect(body.job_id).toBe(jobId);
    expect(body.status).toBe("leased");

    const getJob = await request.get(`/api/v1/jobs/${jobId}`);
    expect(getJob.status()).toBe(200);
    const job = (await getJob.json()).job;
    expect(job.status).toBe("leased");
    expect(job.worker_id).toBe(peerId);
    expect(job.lease_owner).toBe(peerId);
    expect(job.lease_epoch).toBe(1);
    expect(job.lease_expires_at).toBeTruthy();
  });

  test("POST /grid/envelope Job without peer → scheduled, no lease", async ({
    request,
  }) => {
    const jobId = `ph-s112-grid-nopeer-${Date.now()}`;

    const ingest = await request.post("/api/v1/grid/envelope", {
      data: gridJobEnvelope(jobId),
    });
    expect(ingest.status()).toBe(200);
    const body = await ingest.json();
    expect(body.status).toBe("scheduled");

    const getJob = await request.get(`/api/v1/jobs/${jobId}`);
    expect(getJob.status()).toBe(200);
    const job = (await getJob.json()).job;
    expect(job.status).toBe("scheduled");
    expect(job.lease_owner).toBeFalsy();
    expect(job.lease_epoch).toBeFalsy();
    expect(job.lease_expires_at).toBeFalsy();
  });
});
