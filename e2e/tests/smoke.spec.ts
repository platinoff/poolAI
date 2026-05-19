import { test, expect } from "@playwright/test";

const user = process.env.POOLAI_E2E_USER ?? "admin";
const password = process.env.POOLAI_E2E_PASSWORD ?? "admin123";

test.describe("PoolAI UI smoke (S23)", () => {
  test("login → dashboard → admin users", async ({ page }) => {
    await page.goto("/ui/login");
    await page.locator("#username").fill(user);
    await page.locator("#password").fill(password);
    await page.locator("#loginBtn").click();

    await expect(page).toHaveURL(/\/ui\/?$/, { timeout: 20_000 });

    await page.goto("/ui/admin/users");
    await expect(page.locator("#users-list")).toBeVisible({ timeout: 20_000 });
    await expect(
      page.locator(".admin-table, #users-list .muted"),
    ).toBeVisible();
  });
});
