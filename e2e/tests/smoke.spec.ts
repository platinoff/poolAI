import { test, expect } from "@playwright/test";
import { loginAsAdmin } from "./helpers";

test.describe("PoolAI UI smoke (S23)", () => {
  test("login → dashboard → admin users", async ({ page }) => {
    await loginAsAdmin(page);

    await page.goto("/ui/admin/users");
    await expect(page.locator("#users-list")).toBeVisible({ timeout: 20_000 });
    await expect(
      page.locator(".admin-table, #users-list .muted"),
    ).toBeVisible();
  });
});
