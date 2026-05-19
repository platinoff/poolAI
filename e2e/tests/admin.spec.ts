import { test, expect } from "@playwright/test";
import { loginAsAdmin } from "./helpers";

test.describe("PoolAI admin E2E (S27 / S29 / S31 / S33)", () => {
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
      content.locator(".admin-table, .muted, .admin-fetch-error").first(),
    ).toBeVisible();
    await expect(
      page.getByRole("button", { name: /create dashboard/i }),
    ).toBeVisible();
  });

  test("security page loads OAuth2 tab panel", async ({ page }) => {
    await page.goto("/ui/admin/security");
    const content = page.locator("#security-content");
    await expect(content).toBeVisible({ timeout: 20_000 });
    await expect(
      content.locator(".admin-table, .muted, .admin-fetch-error").first(),
    ).toBeVisible();
    await expect(page.locator("#security-tab-oauth2")).toHaveAttribute(
      "aria-selected",
      "true",
    );
    await expect(page.locator("#oauth2-providers-list")).toBeVisible({
      timeout: 20_000,
    });
    await expect(
      content.getByRole("button", { name: /register|зареєстр/i }),
    ).toBeVisible();
  });

  test("audit page loads events container", async ({ page }) => {
    await page.goto("/ui/admin/audit");
    const events = page.locator("#audit-events");
    await expect(events).toBeVisible({ timeout: 20_000 });
    await expect(
      events.locator(".admin-table, .muted, .admin-fetch-error").first(),
    ).toBeVisible();
    await expect(
      page.getByRole("button", { name: /^query$/i }),
    ).toBeVisible();
  });

  test("raid page loads admin and artifacts panels", async ({ page }) => {
    await page.goto("/ui/admin/raid");
    const admin = page.locator("#raid-admin");
    const artifacts = page.locator("#raid-artifacts");
    await expect(admin).toBeVisible({ timeout: 20_000 });
    await expect(artifacts).toBeVisible();
    await expect(
      admin.locator(".admin-card, .muted, .admin-fetch-error").first(),
    ).toBeVisible();
    await expect(
      page.getByRole("button", { name: /upload artifact|завантажити/i }),
    ).toBeVisible();
  });

  test("topology page loads stats and nodes table", async ({ page }) => {
    await page.goto("/ui/admin/topology");
    await expect(page.locator("#topology-node-count")).toBeVisible({
      timeout: 20_000,
    });
    await expect(page.locator("#topology-nodes-list")).toBeVisible();
    await expect(
      page.locator("#topology-nodes-tbody .admin-table, tr, .muted").first(),
    ).toBeVisible({ timeout: 20_000 });
  });

  test("workers page loads list container", async ({ page }) => {
    await page.goto("/ui/admin/workers");
    const list = page.locator("#workers-list");
    await expect(list).toBeVisible({ timeout: 20_000 });
    await expect(
      list.locator(".admin-table, .muted, .admin-fetch-error").first(),
    ).toBeVisible();
    await expect(
      page.getByRole("button", { name: /create worker/i }),
    ).toBeVisible();
  });

  test("vm page loads instances container", async ({ page }) => {
    await page.goto("/ui/admin/vm");
    const instances = page.locator("#vm-instances");
    await expect(instances).toBeVisible({ timeout: 20_000 });
    await expect(
      instances.locator(".admin-table, .muted, .admin-fetch-error").first(),
    ).toBeVisible();
    await expect(
      page.getByRole("button", { name: /create vm instance/i }),
    ).toBeVisible();
  });
});
