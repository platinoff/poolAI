import { test, expect } from "@playwright/test";
import {
  gotoAdminReady,
  loginAsAdmin,
  waitForAdminContentReady,
} from "./helpers";

test.describe("PoolAI admin E2E (S27–S34, PH-S23)", () => {
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

  test("raid page shows cluster and raft status card (PH-S05)", async ({
    page,
  }) => {
    await page.goto("/ui/admin/raid");
    const cluster = page.locator("#raid-cluster-status");
    await expect(cluster).toBeVisible({ timeout: 20_000 });
    await expect(
      cluster.getByRole("heading", {
        name: /cluster status|статус кластера/i,
      }),
    ).toBeVisible();
    await expect(
      cluster.getByText(/raft consensus|raft консенсус/i),
    ).toBeVisible();
  });

  test("topology page loads stats, graph, and nodes table", async ({ page }) => {
    await page.goto("/ui/admin/topology");
    await expect(page.locator("#topology-node-count")).toBeVisible({
      timeout: 20_000,
    });
    await expect(page.locator("#topology-graph-svg")).toBeVisible();
    await expect(page.locator("#topology-latency-heatmap")).toBeVisible();
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

  test("vm page creates instance via modal (PH-S03)", async ({ page }) => {
    const vmName = `e2e-vm-${Date.now()}`;
    await page.goto("/ui/admin/vm");
    await page
      .getByRole("button", { name: /create vm instance/i })
      .click();
    const modal = page.locator("#createVmModal");
    await expect(modal).toBeVisible({ timeout: 10_000 });
    await page.locator("#vmName").fill(vmName);
    await page.locator("#createVmForm").evaluate((form) => {
      (form as HTMLFormElement).requestSubmit();
    });
    await expect(page.locator("#vm-instances")).toContainText(vmName, {
      timeout: 20_000,
    });

    const row = page.locator("#vm-instances tr", { hasText: vmName });
    await row
      .getByRole("button", { name: /delete|видалити/i })
      .click();
    await expect(page.locator("#vm-instances")).not.toContainText(vmName, {
      timeout: 20_000,
    });
  });

  test("libs page loads libraries list", async ({ page }) => {
    await page.goto("/ui/admin/libs");
    const list = page.locator("#libraries-list");
    await expect(list).toBeVisible({ timeout: 20_000 });
    await expect(
      list.locator(".admin-table, .muted, .admin-fetch-error").first(),
    ).toBeVisible();
    await expect(
      page.getByRole("button", { name: /upload library/i }),
    ).toBeVisible();
  });

  test("admin dashboard loads overview panels (PH-S23)", async ({ page }) => {
    await gotoAdminReady(page, "/ui/admin", "#system-overview");
    await expect(page.locator("#quick-stats")).toBeVisible();
    await expect(page.locator("#active-alerts")).toBeVisible();
    await expect(page.locator("#recent-activity")).toBeVisible();
    await expect(page.locator("#metrics-chart")).toBeVisible();
  });

  test("users page loads list and create action (PH-S23)", async ({ page }) => {
    await gotoAdminReady(page, "/ui/admin/users", "#users-list");
    await expect(
      page.getByRole("button", { name: /create user|створити користувача/i }),
    ).toBeVisible();
    await expect(
      page.locator("#users-list .admin-table, #users-list .muted").first(),
    ).toBeVisible();
  });

  test("users page opens create modal (PH-S23)", async ({ page }) => {
    await gotoAdminReady(page, "/ui/admin/users", "#users-list");
    await page.evaluate(() => {
      if (typeof showModal === "function") {
        showModal("createUserModal");
      }
    });
    await expect(page.locator("#createUserModal")).toHaveAttribute(
      "aria-hidden",
      "false",
    );
    await expect(page.locator("#userUsername")).toBeVisible();
    await expect(page.locator("#userPassword")).toBeVisible();
    await expect(page.locator("#userRole")).toBeVisible();
  });

  test("config page loads general tab form (PH-S23)", async ({ page }) => {
    await gotoAdminReady(page, "/ui/admin/config", "#config-content");
    await expect(page.locator("#config-tab-general")).toHaveAttribute(
      "aria-selected",
      "true",
    );
    await expect(page.locator("#generalConfigForm")).toBeVisible();
    await page.locator("#config-tab-performance").click();
    await expect(page.locator("#config-tab-performance")).toHaveAttribute(
      "aria-selected",
      "true",
    );
    await expect(page.locator("#performanceConfigForm")).toBeVisible();
  });

  test("instances page loads list and create action (PH-S23)", async ({
    page,
  }) => {
    await gotoAdminReady(page, "/ui/admin/instances", "#instances-list");
    await expect(
      page.getByRole("button", { name: /create instance|створити/i }),
    ).toBeVisible();
    await expect(page.locator("#instances-tbody")).toBeVisible();
    await expect(page.locator("#placement-previews")).toBeVisible();
  });

  test("topology refresh keeps graph visible (PH-S22/PH-S23)", async ({
    page,
  }) => {
    await page.goto("/ui/admin/topology");
    await waitForAdminContentReady(page, "#topology-nodes-list");
    await page
      .getByRole("button", { name: /refresh|оновити/i })
      .click();
    await expect(page.locator("#topology-graph-svg")).toBeVisible();
    await expect(page.locator("#topology-node-count")).not.toHaveText("-");
  });
});
