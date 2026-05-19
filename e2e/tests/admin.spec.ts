import { test, expect } from "@playwright/test";
import { loginAsAdmin } from "./helpers";

test.describe("PoolAI admin E2E (S27)", () => {
  test.beforeEach(async ({ page }) => {
    await loginAsAdmin(page);
  });

  test("tenants page loads list container", async ({ page }) => {
    await page.goto("/ui/admin/tenants");
    const list = page.locator("#tenants-list");
    await expect(list).toBeVisible({ timeout: 20_000 });
    await expect(
      list.locator(".admin-table, .muted, .admin-fetch-error"),
    ).toBeVisible();
  });

  test("monitoring page loads dashboards section", async ({ page }) => {
    await page.goto("/ui/admin/monitoring");
    const content = page.locator("#monitoring-content");
    await expect(content).toBeVisible({ timeout: 20_000 });
    await expect(
      content.locator(".admin-table, .muted, .admin-fetch-error"),
    ).toBeVisible();
    await expect(
      page.getByRole("button", { name: /create dashboard/i }),
    ).toBeVisible();
  });
});
