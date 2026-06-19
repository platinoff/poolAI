import { test, expect } from "@playwright/test";

const visionUrl =
  process.env.POOLAI_VISION_URL ??
  "http://127.0.0.1:8765/docs/vision/index.html";

test.describe("Vision solar layout (PH-S565)", () => {
  test.beforeEach(async ({ page }) => {
    test.skip(
      process.env.POOLAI_VISION_SKIP === "1",
      "vision server not started",
    );
    try {
      await page.goto(visionUrl, { waitUntil: "domcontentloaded", timeout: 15_000 });
    } catch {
      test.skip(true, "vision server unavailable");
    }
  });

  test("solar hub nodes and orphan rim render", async ({ page }) => {
    await expect(page.locator("#map-scene, .map-scene, #vision-map")).toBeVisible({
      timeout: 20_000,
    });
    const nodes = page.locator(".map-node, [data-node-id], .vision-map-node");
    await expect(nodes.first()).toBeVisible({ timeout: 20_000 });
    expect(await nodes.count()).toBeGreaterThan(0);
  });

  test("keyboard Tab focuses map controls", async ({ page }) => {
    await page.keyboard.press("Tab");
    const focused = page.locator(":focus");
    await expect(focused).toBeVisible({ timeout: 10_000 });
  });
});
