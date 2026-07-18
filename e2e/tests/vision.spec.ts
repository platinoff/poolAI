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
    await expect(page.locator("#map-scene-3d, #map-scene, .map-scene, #vision-map")).toBeVisible({
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

  test("auto-orbit toggle and fit-all controls (PH-S585)", async ({ page }) => {
    const orbitBtn = page.locator("#map-orbit-auto");
    await expect(orbitBtn).toBeVisible({ timeout: 10_000 });
    await expect(orbitBtn).toHaveAttribute("aria-pressed", /true|false/);
    const resetBtn = page.locator("#map-zoom-reset");
    await expect(resetBtn).toBeVisible();
    await orbitBtn.click();
    await expect(orbitBtn).toHaveAttribute("aria-pressed", /true|false/);
    await resetBtn.click();
    await expect(page.locator("#map-svg, .map-scene")).toBeVisible();
  });

  test("auto-orbit rotY changes on play and holds on pause (PH-S590)", async ({ page }) => {
    await expect(page.locator("#map-scene-3d, #map-scene, .map-scene, #vision-map")).toBeVisible({
      timeout: 20_000,
    });
    const orbitBtn = page.locator("#map-orbit-auto");
    await expect(orbitBtn).toBeVisible({ timeout: 10_000 });
    const readRotY = () =>
      page.evaluate(() => {
        const raw = localStorage.getItem("poolai-vision-map-prefs");
        if (!raw) return null;
        const parsed = JSON.parse(raw) as { mapOrbit?: { rotY?: number } };
        return parsed.mapOrbit?.rotY ?? null;
      });
    if ((await orbitBtn.getAttribute("aria-pressed")) === "false") {
      await orbitBtn.click();
      await expect(orbitBtn).toHaveAttribute("aria-pressed", "true");
    }
    await page.waitForTimeout(800);
    const rotY1 = await readRotY();
    await page.waitForTimeout(700);
    const rotY2 = await readRotY();
    expect(rotY1).not.toBeNull();
    expect(rotY2).not.toBeNull();
    expect(Math.abs((rotY2 as number) - (rotY1 as number))).toBeGreaterThan(0.05);
    await orbitBtn.click();
    await expect(orbitBtn).toHaveAttribute("aria-pressed", "false");
    const rotYPause = await readRotY();
    await page.waitForTimeout(700);
    const rotYAfter = await readRotY();
    expect(Math.abs((rotYAfter as number) - (rotYPause as number))).toBeLessThan(0.05);
  });

  test("skip links and map orbit aria-pressed (PH-S1047)", async ({ page }) => {
    const skip = page.locator(".vision-skip-link").first();
    await expect(skip).toBeAttached();
    await skip.focus();
    await expect(skip).toBeFocused();
    const orbitBtn = page.locator("#map-orbit-auto");
    await expect(orbitBtn).toBeVisible({ timeout: 10_000 });
    await expect(orbitBtn).toHaveAttribute("aria-pressed", /true|false/);
    const tree = page.locator("#file-tree[role='tree']");
    await expect(tree).toBeVisible();
  });

  test("vision map shell visual snapshot (PH-S1052)", async ({ page }) => {
    const map = page.locator("#map-scene-3d, #map-scene, .map-scene, #vision-map").first();
    await expect(map).toBeVisible({ timeout: 20_000 });
    await page.evaluate(() => document.fonts?.ready);
    await expect(page.locator("body")).toHaveScreenshot("vision-map-shell.png", {
      animations: "disabled",
      mask: [
        page.locator("#map-starfield"),
        page.locator(".map-orbit-layer"),
        page.locator("#map-svg"),
      ],
    });
  });
});
