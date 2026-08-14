import { test, expect } from "@playwright/test";

const visionUrl =
  process.env.POOLAI_VISION_URL ??
  "http://127.0.0.1:8765/GSV/docs/vision/index.html";

test.describe("Vision legacy pointer page (band 117)", () => {
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

  test("deactivated — GSV pointer page, no legacy map UI", async ({ page }) => {
    await expect(page.locator("h1")).toHaveText("PoolAI Galaxy Starwalker Vision", {
      timeout: 15_000,
    });
    await expect(page.getByText(/Legacy vision UI деактивовано/)).toBeVisible();
    await expect(
      page.locator("#map-scene-3d, .vision-skip-link, #file-tree"),
    ).toHaveCount(0);
    await expect(
      page.getByRole("link", { name: "http://127.0.0.1:8891/" }),
    ).toBeVisible();
  });
});
