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

  test("monitoring page loads dashboards section (PH-S43/PH-S45)", async ({
    page,
  }) => {
    await gotoAdminReady(page, "/ui/admin/monitoring", "#monitoring-content");
    await expect(
      page.getByRole("button", { name: /create dashboard/i }),
    ).toBeVisible({ timeout: 15_000 });
  });

  test("security page loads secret rotation tab (PH-S27)", async ({ page }) => {
    await page.goto("/ui/admin/security");
    await page.locator("#security-tab-rotation").click();
    const content = page.locator("#security-content");
    await expect(content).toBeVisible({ timeout: 20_000 });
    await expect(content.locator(".admin-table")).toBeVisible({ timeout: 15_000 });
    await expect(
      page.getByRole("button", { name: /reload jwt|jwt.*env/i }),
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
    await gotoAdminReady(page, "/ui/admin/workers", "#workers-list");
    await expect(page.locator('[data-i18n="admin.wrk.createBtn"]')).toBeVisible({
      timeout: 15_000,
    });
  });

  test("jobs page loads list and store badge (PH-S53)", async ({ page }) => {
    await gotoAdminReady(page, "/ui/admin/jobs", "#jobs-list");
    await expect(page.locator("#jobs-store-badge")).toBeVisible({
      timeout: 15_000,
    });
    await expect(
      page.locator("#jobs-list .admin-table, #jobs-list .admin-empty-state, #jobs-list .muted").first(),
    ).toBeVisible({ timeout: 20_000 });
  });

  test("jobs page shows lease columns when present (PH-S96, PH-S152 wasm)", async ({
    page,
    request,
  }) => {
    const expires = "2026-12-31T23:59:59Z";
    const createRes = await request.post("/api/v1/jobs", {
      data: {
        kind: "inference",
        lease_owner: "e2e-lease-worker",
        lease_epoch: 42,
        lease_expires_at: expires,
      },
    });
    expect(createRes.status()).toBe(201);

    await gotoAdminReady(page, "/ui/admin/jobs", "#jobs-list");
    await expect(
      page.getByRole("columnheader", { name: /lease owner/i }),
    ).toBeVisible({ timeout: 15_000 });
    await expect(
      page.getByRole("columnheader", { name: /lease epoch/i }),
    ).toBeVisible();
    await expect(
      page.getByRole("columnheader", { name: /lease state/i }),
    ).toBeVisible();
    await expect(
      page.getByRole("columnheader", { name: /lease expires/i }),
    ).toBeVisible();
    await expect(page.locator("#jobs-list")).toContainText("e2e-lease-worker");
    await expect(page.locator("#jobs-list .lease-epoch-cell")).toContainText("#42");
    await expect(page.locator("#jobs-list .lease-owner-cell")).toHaveAttribute(
      "title",
      /Galaxy|lease|CAS/i,
    );
    await expect(page.locator("#jobs-list .lease-epoch-cell")).toHaveAttribute(
      "title",
      /epoch|CAS|renew/i,
    );
    await expect(page.locator("#jobs-list")).toContainText("Active");
    await expect(page.locator("#jobs-list")).toContainText(expires);
    await page.waitForFunction(
      () => {
        const wasm = document.documentElement.dataset.poolaiUiWasm;
        const failed = (window as Window & { poolaiUiWasm?: { failed?: boolean } })
          .poolaiUiWasm?.failed;
        return Boolean(wasm?.includes("poolai-ui-wasm")) || Boolean(failed);
      },
      undefined,
      { timeout: 10_000 },
    );
  });

  test("jobs page shows expired lease badge (PH-S105, PH-S152 wasm)", async ({
    page,
    request,
  }) => {
    const expiredAt = "2020-01-01T00:00:00Z";
    const createRes = await request.post("/api/v1/jobs", {
      data: {
        kind: "inference",
        lease_owner: "e2e-lease-expired",
        lease_epoch: 7,
        lease_expires_at: expiredAt,
      },
    });
    expect(createRes.status()).toBe(201);

    await gotoAdminReady(page, "/ui/admin/jobs", "#jobs-list");
    await expect(page.locator("#jobs-list")).toContainText("e2e-lease-expired");
    await expect(page.locator("#jobs-list")).toContainText("Expired");
  });

  test("jobs page shows migrating status badge (PH-S141)", async ({
    page,
    request,
  }) => {
    const createRes = await request.post("/api/v1/jobs", {
      data: { kind: "inference" },
    });
    expect(createRes.status()).toBe(201);
    const created = (await createRes.json()) as { id: string };

    const leaseRes = await request.post(
      `/api/v1/jobs/${created.id}/lease`,
      { data: { lease_owner: "e2e-migrate-badge" } },
    );
    expect(leaseRes.status()).toBe(200);

    const patchRes = await request.patch(`/api/v1/jobs/${created.id}`, {
      data: { status: "migrating" },
    });
    expect(patchRes.status()).toBe(200);

    await gotoAdminReady(page, "/ui/admin/jobs", "#jobs-list");
    const row = page.locator("#jobs-list").getByRole("row").filter({
      hasText: created.id,
    });
    await expect(row).toBeVisible({ timeout: 15_000 });
    const badge = row.locator('[data-job-status="migrating"]');
    await expect(badge).toBeVisible();
    await expect(badge).toContainText("Migrating");
    await expect(badge).toHaveAttribute("title", /re-migrate|handoff|PH-S104/i);
  });

  test("updates compatibility page shows protocol and doc blocks (PH-S93, PH-S197 wasm)", async ({
    page,
  }) => {
    await gotoAdminReady(
      page,
      "/ui/admin/updates-compat",
      "#updates-compat-panel",
    );
    await expect(page.locator("#updates-compat-protocol")).toBeVisible({
      timeout: 15_000,
    });
    await expect(
      page.locator("#updates-compat-coordinator-protocol"),
    ).toBeVisible();
    await expect(page.locator("#updates-compat-negotiation-status")).toContainText(
      "Accepted",
    );
    await expect(page.locator("#updates-compat-verify-release")).toBeVisible();
    await expect(page.locator("#updates-compat-verify-cmd")).toContainText(
      "poolai-verify-release",
    );
    await expect(page.locator("#updates-compat-matrix")).toBeVisible();
    await expect(
      page.getByRole("link", { name: /Galaxy §9\.3 compat matrix/i }),
    ).toBeVisible();
    await page.waitForFunction(
      () =>
        document.documentElement.dataset.poolaiUiWasm ||
        window.poolaiUiWasm?.ready ||
        window.poolaiUiWasm?.failed,
      { timeout: 15_000 },
    );
  });

  test("grid pricing page fetches snapshot query (PH-S82, PH-S151/PH-S152 wasm)", async ({
    page,
  }) => {
    await gotoAdminReady(page, "/ui/admin/grid-pricing", "#grid-pricing-panel");
    await expect(page.locator("#grid-pricing-form")).toBeVisible({
      timeout: 15_000,
    });
    await page.locator("#grid-pricing-task").fill("inference:text");
    await page.locator("#grid-pricing-model").fill("e2e-default");
    await page.locator("#grid-pricing-unit").selectOption("inference_blended_token");
    const pricingRespP = page.waitForResponse(
      (res) =>
        res.url().includes("/api/v1/grid/pricing") &&
        res.request().method() === "GET" &&
        res.url().includes("task_profile=inference%3Atext") &&
        res.url().includes("model_profile=e2e-default") &&
        res.url().includes("unit_key=inference_blended_token"),
      { timeout: 20_000 },
    );
    await page.locator("#grid-pricing-fetch-btn").click();
    const pricingResp = await pricingRespP;
    expect([200, 503]).toContain(pricingResp.status());
    await expect(
      page
        .locator(
          "#grid-pricing-panel #grid-pricing-result, #grid-pricing-panel .admin-fetch-error",
        )
        .first(),
    ).toBeVisible({ timeout: 15_000 });
    await page.waitForFunction(
      () => {
        const wasm = document.documentElement.dataset.poolaiUiWasm;
        const failed = (window as Window & { poolaiUiWasm?: { failed?: boolean } })
          .poolaiUiWasm?.failed;
        return Boolean(wasm?.includes("poolai-ui-wasm")) || Boolean(failed);
      },
      undefined,
      { timeout: 10_000 },
    );
  });

  test("payout batch panel loads (PH-S564)", async ({ page }) => {
    await gotoAdminReady(page, "/ui/admin/payout-batch", "#payout-batch-panel");
    await expect(page.locator("#payout-batch-panel .admin-card, #payout-batch-panel .admin-fetch-error")).toBeVisible({
      timeout: 15_000,
    });
  });

  test("network profiles panel loads (PH-S582)", async ({ page }) => {
    await gotoAdminReady(page, "/ui/admin/network-profiles", "#network-profiles-panel");
    await expect(
      page.locator("#network-profiles-panel .admin-table, #network-profiles-panel .muted, #network-profiles-panel .admin-fetch-error"),
    ).toBeVisible({ timeout: 15_000 });
  });

  test("seed inventory panel loads (PH-S584)", async ({ page }) => {
    await gotoAdminReady(page, "/ui/admin/seed-inventory", "#seed-inventory-panel");
    await expect(
      page.locator("#seed-inventory-panel .admin-table, #seed-inventory-panel .muted, #seed-inventory-panel .admin-fetch-error"),
    ).toBeVisible({ timeout: 15_000 });
  });

  test("vm page loads instances container", async ({ page }) => {
    await gotoAdminReady(page, "/ui/admin/vm", "#vm-instances");
    await expect(page.locator('[data-i18n="admin.vmadm.createBtn"]')).toBeVisible({
      timeout: 15_000,
    });
  });

  test("vm page creates instance via modal (PH-S03, PH-S45)", async ({
    page,
  }) => {
    const vmName = `e2e-vm-${Date.now()}`;
    await gotoAdminReady(page, "/ui/admin/vm", "#vm-instances");
    const modal = page.locator("#createVmModal");
    const createBtn = page.locator('[data-i18n="admin.vmadm.createBtn"]');
    await expect(createBtn).toBeVisible({ timeout: 15_000 });
    await createBtn.click();
    try {
      await expect(modal).toHaveAttribute("aria-hidden", "false", {
        timeout: 3_000,
      });
    } catch {
      await page.evaluate(() => {
        if (typeof showCreateVmModal === "function") {
          showCreateVmModal();
        } else if (typeof showModal === "function") {
          showModal("createVmModal");
        }
      });
      await expect(modal).toHaveAttribute("aria-hidden", "false", {
        timeout: 10_000,
      });
    }
    await modal.locator("#vmName").fill(vmName);
    const createRespP = page.waitForResponse(
      (res) =>
        res.url().includes("/api/v1/vm/instances") &&
        res.request().method() === "POST",
      { timeout: 30_000 },
    );
    await modal.locator("#createVmForm").evaluate((form) => {
      (form as HTMLFormElement).requestSubmit();
    });
    const createResp = await createRespP;
    expect(createResp.ok()).toBeTruthy();
    await expect(modal).toHaveAttribute("aria-hidden", "true", {
      timeout: 10_000,
    });
    await expect(
      page.locator("#vm-instances tr", { hasText: vmName }),
    ).toBeVisible({ timeout: 20_000 });

    const row = page.locator("#vm-instances tr", { hasText: vmName });
    const [deleteResp] = await Promise.all([
      page.waitForResponse(
        (res) =>
          res.url().includes("/api/v1/vm/instances/") &&
          res.request().method() === "DELETE" &&
          res.ok(),
        { timeout: 20_000 },
      ),
      row.getByRole("button", { name: /delete|видалити/i }).click(),
    ]);
    expect(deleteResp.ok()).toBeTruthy();
    await expect(
      page.locator("#vm-instances tr", { hasText: vmName }),
    ).toHaveCount(0, { timeout: 20_000 });
  });

  test("libs page loads libraries list", async ({ page }) => {
    await gotoAdminReady(page, "/ui/admin/libs", "#libraries-list");
    await expect(page.locator('[data-i18n="admin.lib.uploadBtn"]')).toBeVisible({
      timeout: 15_000,
    });
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
