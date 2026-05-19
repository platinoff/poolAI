import { test, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { loginAsAdmin } from "./helpers";

/** FM-019 S33: axe in Playwright (complements pa11y CI). */
const criticalAndSerious = (violations: { impact?: string | null }[]) =>
  violations.filter((v) => v.impact === "critical" || v.impact === "serious");

test.describe("axe accessibility (S33)", () => {
  test("login page — no critical/serious violations", async ({ page }) => {
    await page.goto("/ui/login");
    const results = await new AxeBuilder({ page })
      .withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa"])
      .analyze();
    expect(criticalAndSerious(results.violations)).toEqual([]);
  });

  test("admin users — no critical/serious violations", async ({ page }) => {
    await loginAsAdmin(page);
    await page.goto("/ui/admin/users");
    await page.locator("#users-list").waitFor({ state: "visible", timeout: 20_000 });
    const results = await new AxeBuilder({ page })
      .withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa"])
      .analyze();
    expect(criticalAndSerious(results.violations)).toEqual([]);
  });
});
