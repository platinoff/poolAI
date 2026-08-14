import { test, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { loginAsAdmin, primeUiPrefs, waitForAdminAxeReady } from "./helpers";

/** FM-019 S33 + FM-031: axe in Playwright (complements pa11y CI). */
const criticalAndSerious = (violations: { impact?: string | null }[]) =>
  violations.filter((v) => v.impact === "critical" || v.impact === "serious");

const AXE_TAGS = [
  "wcag2a",
  "wcag2aa",
  "wcag21a",
  "wcag21aa",
  "wcag22aa",
];

/** FM-031: admin shell routes (aligned with `bin/pa11y-ci.sh` ADMIN_URLS). */
const ADMIN_AXE_PAGES: { path: string; waitFor: string }[] = [
  { path: "/ui/admin/users", waitFor: "#users-list" },
  { path: "/ui/admin/security", waitFor: "#security-content" },
  { path: "/ui/admin/config", waitFor: "#config-content" },
  { path: "/ui/admin/tenants", waitFor: "#tenants-list" },
  /** PH-S45: audit filters + table enhance; settled via `waitForAdminAxeReady`. */
  { path: "/ui/admin/audit", waitFor: "#audit-events" },
  { path: "/ui/admin/monitoring", waitFor: "#monitoring-content" },
  { path: "/ui/admin/instances", waitFor: "#instances-list" },
  { path: "/ui/admin/topology", waitFor: "#topology-nodes-list" },
  { path: "/ui/admin/vm", waitFor: "#vm-instances" },
  { path: "/ui/admin/workers", waitFor: "#workers-list" },
  { path: "/ui/admin/jobs", waitFor: "#jobs-list" },
  {
    path: "/ui/admin/updates-compat",
    waitFor: "#updates-compat-panel",
  },
  {
    path: "/ui/admin/network-profiles",
    waitFor: "#network-profiles-panel",
  },
  {
    path: "/ui/admin/seed-inventory",
    waitFor: "#seed-inventory-panel",
  },
  {
    path: "/ui/admin/security-advisories",
    waitFor: "#security-advisories-panel",
  },
  {
    path: "/ui/admin/payout-batch",
    waitFor: "#payout-batch-panel",
  },
  { path: "/ui/admin/libs", waitFor: "#libraries-list" },
  { path: "/ui/admin/raid", waitFor: "#raid-admin" },
];

/** PH-S14: representative pages for high-contrast axe (login + admin shell). */
const HC_AXE_PAGES: {
  path: string;
  waitFor: string;
  auth?: boolean;
}[] = [
  { path: "/ui/login", waitFor: "#loginForm", auth: false },
  { path: "/ui/admin/users", waitFor: "#users-list", auth: true },
  { path: "/ui/admin/monitoring", waitFor: "#monitoring-content", auth: true },
  { path: "/ui/admin/config", waitFor: "#config-content", auth: true },
  { path: "/ui/admin/jobs", waitFor: "#jobs-list", auth: true },
  { path: "/ui/admin/tenants", waitFor: "#tenants-list", auth: true },
];

test.describe("axe accessibility (S33, FM-031)", () => {
  test("login page — no critical/serious violations", async ({ page }) => {
    await page.goto("/ui/login");
    const results = await new AxeBuilder({ page }).withTags(AXE_TAGS).analyze();
    expect(criticalAndSerious(results.violations)).toEqual([]);
  });

  for (const { path, waitFor } of ADMIN_AXE_PAGES) {
    test(`admin ${path} — no critical/serious violations`, async ({ page }) => {
      await loginAsAdmin(page);
      await page.goto(path);
      await page.locator(waitFor).waitFor({ state: "visible", timeout: 20_000 });
      await waitForAdminAxeReady(page, waitFor);
      const results = await new AxeBuilder({ page })
        .withTags(AXE_TAGS)
        .exclude('.modal[aria-hidden="true"], #poolai-bootstrap-banner-host[hidden]')
        .analyze();
      expect(criticalAndSerious(results.violations)).toEqual([]);
    });
  }
});

test.describe("axe high-contrast color-contrast (PH-S14)", () => {
  for (const { path, waitFor, auth = false } of HC_AXE_PAGES) {
    test(`${path} high-contrast — no critical/serious contrast violations`, async ({
      page,
    }) => {
      await primeUiPrefs(page, { theme: "high-contrast" });
      if (auth) {
        await loginAsAdmin(page, { theme: "high-contrast" });
      }
      await page.goto(path);
      await page.locator(waitFor).waitFor({ state: "visible", timeout: 20_000 });
      if (auth) {
        await page.evaluate(() => {
          const w = window as Window & {
            poolaiApplyTheme?: (t: string) => void;
          };
          w.poolaiApplyTheme?.("high-contrast");
        });
        await waitForAdminAxeReady(page, waitFor);
      }
      const results = await new AxeBuilder({ page })
        .withTags(AXE_TAGS)
        .withRules(["color-contrast"])
        .exclude(
          '.modal[aria-hidden="true"], #poolai-bootstrap-banner-host[hidden]',
        )
        .analyze();
      expect(criticalAndSerious(results.violations)).toEqual([]);
    });
  }
});

const visionUrl =
  process.env.POOLAI_VISION_URL ??
  "http://127.0.0.1:8765/GSV/docs/vision/index.html";

test.describe("axe vision map (PH-S1051)", () => {
  test("vision index — no critical/serious violations", async ({ page }) => {
    test.skip(
      process.env.POOLAI_VISION_SKIP === "1",
      "vision server not started",
    );
    try {
      await page.goto(visionUrl, { waitUntil: "domcontentloaded", timeout: 15_000 });
    } catch {
      test.skip(true, "vision server unavailable");
    }
    await expect(page.locator("h1")).toBeVisible({ timeout: 20_000 });
    const results = await new AxeBuilder({ page })
      .withTags(AXE_TAGS)
      .exclude("#map-starfield, .map-orbit-layer")
      .analyze();
    expect(criticalAndSerious(results.violations)).toEqual([]);
  });
});
