import { test, expect } from "@playwright/test";

/** Matches default in `bin/e2e-playwright.sh --start` (POOLAI_GALAXY_PRICING_FALLBACK_JSON). */
const E2E_FALLBACK_QUOTE_USD_MICRO = 470_000;

const standRoot = process.env.POOLAI_E2E_STAND_ROOT;

function pricingQuery(modelProfile: string): string {
  const params = new URLSearchParams({
    task_profile: "inference:text",
    model_profile: modelProfile,
    unit_key: "inference_blended_token",
  });
  return `/api/v1/grid/pricing?${params}`;
}

test.describe("Grid pricing API smoke (PH-S86)", () => {
  test.skip(
    !standRoot,
    "requires bin/e2e-playwright.sh --start (POOLAI_E2E_STAND_ROOT + POOLAI_GALAXY_PRICING_FALLBACK_JSON)",
  );

  test("GET /api/v1/grid/pricing returns L2 fallback snapshot", async ({
    request,
  }) => {
    const model = `e2e-pricing-${Date.now()}`;
    const res = await request.get(pricingQuery(model));
    expect(res.status()).toBe(200);
    const body = await res.json();
    expect(body.ok).toBe(true);
    expect(body.source).toBe("oracle");
    expect(body.freshness).toBe("fresh");
    expect(body.snapshot.task_profile).toBe("inference:text");
    expect(body.snapshot.model_profile).toBe(model);
    expect(body.snapshot.unit_key).toBe("inference_blended_token");
    expect(body.snapshot.poolai_quote_usd_micro).toBe(
      E2E_FALLBACK_QUOTE_USD_MICRO,
    );
    expect(body.snapshot.provider_id_at_min).toBe("fallback_l2_config");
  });

  test("second GET serves cached snapshot", async ({ request }) => {
    const model = `e2e-pricing-cache-${Date.now()}`;
    const first = await request.get(pricingQuery(model));
    expect(first.status()).toBe(200);
    const second = await request.get(pricingQuery(model));
    expect(second.status()).toBe(200);
    const body = await second.json();
    expect(body.source).toBe("cache");
    expect(body.snapshot.poolai_quote_usd_micro).toBe(
      E2E_FALLBACK_QUOTE_USD_MICRO,
    );
  });

  test("reject invalid unit_key", async ({ request }) => {
    const params = new URLSearchParams({
      task_profile: "inference:text",
      model_profile: "e2e-invalid-unit",
      unit_key: "not_a_valid_unit",
    });
    const res = await request.get(`/api/v1/grid/pricing?${params}`);
    expect(res.status()).toBe(400);
  });
});
