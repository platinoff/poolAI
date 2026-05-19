import type { Page } from "@playwright/test";

export const e2eUser = process.env.POOLAI_E2E_USER ?? "admin";
export const e2ePassword = process.env.POOLAI_E2E_PASSWORD ?? "admin123";

/** Log in via /ui/login and wait for dashboard. */
export async function loginAsAdmin(page: Page): Promise<void> {
  await page.goto("/ui/login");
  await page.locator("#username").fill(e2eUser);
  await page.locator("#password").fill(e2ePassword);
  await page.locator("#loginBtn").click();
  await page.waitForURL(/\/ui\/?$/, { timeout: 20_000 });
}
