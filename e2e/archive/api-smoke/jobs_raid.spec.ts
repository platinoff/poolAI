import { test, expect } from "@playwright/test";
import {
  createJobViaApi,
  getJobViaApi,
  restartPoolaiE2eStand,
} from "../../tests/helpers";

const standRoot = process.env.POOLAI_E2E_STAND_ROOT;

test.describe("Jobs RAID persistence smoke (PH-S52)", () => {
  test.skip(
    !standRoot,
    "requires bin/e2e-playwright.sh --start (POOLAI_E2E_STAND_ROOT)",
  );

  test("POST job → restart coordinator → GET job survives", async ({
    request,
  }) => {
    const created = await createJobViaApi(request, {
      kind: "inference",
      priority: 7,
      input_artifact_ids: ["ph-s52-raid-smoke"],
    });
    expect(created.id).toBeTruthy();
    expect(created.kind).toBe("inference");
    expect(created.status).toBe("scheduled");

    await restartPoolaiE2eStand(request);

    const detail = await getJobViaApi(request, created.id);
    expect(detail.job.spec.id).toBe(created.id);
    expect(detail.job.spec.kind).toBe("inference");
    expect(detail.job.status).toBe("scheduled");
  });
});
