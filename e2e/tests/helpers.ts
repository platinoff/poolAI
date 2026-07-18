import { execFileSync } from "node:child_process";
import { expect, type APIRequestContext, type Locator, type Page } from "@playwright/test";

export const e2eUser = process.env.POOLAI_E2E_USER ?? "admin";
export const e2ePassword = process.env.POOLAI_E2E_PASSWORD ?? "admin123";

export type UiTheme = "dark" | "light" | "high-contrast";
/** @deprecated use UiTheme */
export type VisualTheme = UiTheme;
export type VisualLang = "en" | "uk";

/** Set theme/locale in localStorage before the first navigation. */
export async function primeUiPrefs(
  page: Page,
  prefs: { theme?: UiTheme; lang?: VisualLang },
): Promise<void> {
  await page.addInitScript((p) => {
    if (p.theme) localStorage.setItem("poolai_theme", p.theme);
    if (p.lang) localStorage.setItem("poolai_ui_lang", p.lang);
    // Default-admin bootstrap banner adds noisy axe targets; ack for E2E.
    localStorage.setItem("poolai_bootstrap_admin_ack", "1");
  }, prefs);
}

export function matrixSnapshotName(
  pageName: string,
  theme: VisualTheme,
  lang: VisualLang,
): string {
  return `${pageName}-${theme}-${lang}.png`;
}

/** Log in via /ui/login; capture POST /api/v1/login before redirect (PH-S25, Windows-safe). */
export async function loginAsAdmin(
  page: Page,
  prefs?: { theme?: UiTheme; lang?: VisualLang },
): Promise<void> {
  await page.addInitScript(() => {
    localStorage.setItem("poolai_bootstrap_admin_ack", "1");
  });
  if (prefs) {
    await primeUiPrefs(page, prefs);
  }
  await page.goto("/ui/login");
  await page.locator("#username").fill(e2eUser);
  await page.locator("#password").fill(e2ePassword);
  const [loginResp] = await Promise.all([
    page.waitForResponse(
      (res) =>
        res.url().includes("/api/v1/login") &&
        res.request().method() === "POST" &&
        res.ok(),
      { timeout: 20_000 },
    ),
    page.locator("#loginForm").evaluate((form) => {
      (form as HTMLFormElement).requestSubmit();
    }),
  ]);
  expect(loginResp.ok()).toBeTruthy();
  await page.waitForURL(/\/ui\/?$/, { timeout: 20_000 });
  await page.waitForFunction(
    () => !!localStorage.getItem("poolai_token"),
    null,
    { timeout: 20_000 },
  );
  await page.evaluate((user) => {
    if (!localStorage.getItem("poolai_role")) {
      localStorage.setItem("poolai_role", "Admin");
    }
    if (!localStorage.getItem("poolai_user")) {
      localStorage.setItem("poolai_user", user);
    }
  }, e2eUser);
}

export async function expectVisualLang(
  page: Page,
  lang: VisualLang,
): Promise<void> {
  await expect(page.locator("html")).toHaveAttribute(
    "lang",
    lang === "uk" ? "uk" : "en",
  );
}

/** Wait until admin region finished loading and danger buttons have stable contrast. */
export async function waitForAdminAxeReady(
  page: Page,
  contentSelector: string,
): Promise<void> {
  const root = contentSelector.startsWith("#")
    ? contentSelector.slice(1)
    : contentSelector;
  await page.evaluate(() => document.fonts?.ready);
  await page.waitForFunction(
    (regionId: string) => {
      const region = document.getElementById(regionId);
      if (!region) return false;
      const section = region.closest(".admin-section") ?? region.parentElement;
      const settled =
        region.querySelector(".admin-table tbody tr") ||
        region.querySelector("#audit-events-table") ||
        region.querySelector(".admin-fetch-error") ||
        region.querySelector(".admin-card") ||
        region.querySelector(".admin-form") ||
        region.querySelector("form") ||
        (region.querySelector(".muted") &&
          !/loading/i.test(region.textContent ?? "")) ||
        (section?.querySelector(".admin-filters input, .admin-filters select") &&
          region.querySelector(".admin-table-container, .admin-table"));
      if (!settled) return false;
      const buttons = region.querySelectorAll<HTMLButtonElement>(
        "button.btn.btn-danger",
      );
      if (!buttons.length) return true;
      return Array.from(buttons).every((el) => {
        const s = getComputedStyle(el);
        return (
          s.color === "rgb(255, 255, 255)" &&
          s.webkitTextFillColor === "rgb(255, 255, 255)" &&
          Number(s.opacity) === 1
        );
      });
    },
    root,
    { timeout: 20_000 },
  );
  await page.evaluate(
    () =>
      new Promise<void>((resolve) => {
        requestAnimationFrame(() => requestAnimationFrame(() => resolve()));
      }),
  );
}

/** Wait until admin shell and primary content region are ready (table, empty, or error). */
export async function waitForAdminContentReady(
  page: Page,
  contentSelector: string,
): Promise<void> {
  const content = page.locator(contentSelector);
  await content.waitFor({ state: "attached", timeout: 20_000 });
  await expect(
    content
      .locator(
        ".admin-table, .admin-fetch-error, .admin-card, .admin-form, form, .stat-item, .admin-panel-body",
      )
      .first()
      .or(content.locator(".muted").filter({ hasNot: page.locator("#dash-refreshed-at") }).first()),
  ).toBeVisible({ timeout: 30_000 });
  await page.evaluate(() => document.fonts?.ready);
}

/** Settle layout/fonts before visual snapshots (PH-S1054). */
export async function waitForVisualSnapshotReady(page: Page): Promise<void> {
  await page.evaluate(() => document.fonts?.ready);
  await page.evaluate(
    () =>
      new Promise<void>((resolve) => {
        requestAnimationFrame(() => requestAnimationFrame(() => resolve()));
      }),
  );
}

/** Visual snapshot readiness — optional `afterReady` for async grid panels (PH-S1050). */
export async function waitForAdminVisualReady(
  page: Page,
  contentSelector: string,
  afterReady?: string,
): Promise<void> {
  await page.locator(contentSelector).waitFor({ state: "attached", timeout: 20_000 });
  if (afterReady) {
    await expect(page.locator(afterReady).first()).toBeVisible({ timeout: 45_000 });
  } else {
    await waitForAdminContentReady(page, contentSelector);
  }
  await waitForVisualSnapshotReady(page);
}

/** Navigate to an admin route and wait for primary content (PH-S23). */
export async function gotoAdminReady(
  page: Page,
  path: string,
  contentSelector: string,
): Promise<void> {
  await page.goto(path);
  await waitForAdminContentReady(page, contentSelector);
}

/** Accept the next native confirm/alert dialog (delete flows). */
export function acceptNextDialog(page: Page): void {
  page.once("dialog", (dialog) => dialog.accept());
}

/**
 * Mask force-layout SVG and live topology data (PH-S13).
 * Shell (headers, graph frame, table columns, Refresh) stays in the baseline.
 */
export const TOPOLOGY_VISUAL_MASKS = [
  "#topology-graph-svg",
  "#topology-latency-heatmap",
  "#topology-nodes-tbody",
  "#topology-latency-tbody",
  ".admin-stats-grid",
] as const;

/** Mask live charts / dynamic regions before visual snapshots (PH-S11 / PH-S13). */
export function visualMaskLocators(page: Page, selectors: string[]): Locator[] {
  return selectors.map((sel) => page.locator(sel));
}

export type JobSummary = {
  id: string;
  kind: string;
  status: string;
  created_at: string;
};

export type JobDetail = {
  job: {
    spec: { id: string; kind: string };
    status: string;
  };
};

const baseURL = process.env.POOLAI_BASE_URL ?? "http://127.0.0.1:8080";

/** Wait until coordinator health endpoint responds (PH-S52 restart smoke). */
export async function waitForCoordinatorHealth(
  request: APIRequestContext,
  timeoutMs = 90_000,
): Promise<void> {
  const deadline = Date.now() + timeoutMs;
  let lastStatus = 0;
  while (Date.now() < deadline) {
    const res = await request.get("/api/v1/health");
    lastStatus = res.status();
    if (res.ok()) return;
    await new Promise((r) => setTimeout(r, 2000));
  }
  throw new Error(`health not ready (${lastStatus}) at ${baseURL}`);
}

/** POST /api/v1/jobs — returns created job summary (PH-S52). */
export async function createJobViaApi(
  request: APIRequestContext,
  body: { kind?: string; priority?: number; input_artifact_ids?: string[] } = {},
): Promise<JobSummary> {
  const res = await request.post("/api/v1/jobs", {
    data: {
      kind: body.kind ?? "inference",
      priority: body.priority ?? 5,
      input_artifact_ids: body.input_artifact_ids ?? ["e2e-artifact-a"],
    },
  });
  expect(res.status()).toBe(201);
  return (await res.json()) as JobSummary;
}

/** GET /api/v1/jobs/{id} — job detail payload (PH-S52). */
export async function getJobViaApi(
  request: APIRequestContext,
  id: string,
): Promise<JobDetail> {
  const res = await request.get(`/api/v1/jobs/${id}`);
  expect(res.ok()).toBeTruthy();
  return (await res.json()) as JobDetail;
}

/**
 * Restart poolai started by `bin/e2e-playwright.sh --start`.
 * Requires POOLAI_E2E_STAND_ROOT with generated restart.sh.
 */
export async function restartPoolaiE2eStand(
  request: APIRequestContext,
): Promise<void> {
  const standRoot = process.env.POOLAI_E2E_STAND_ROOT;
  if (!standRoot) {
    throw new Error("POOLAI_E2E_STAND_ROOT is required to restart the e2e stand");
  }
  execFileSync("bash", [`${standRoot}/restart.sh`], {
    stdio: "inherit",
    env: process.env,
  });
  await waitForCoordinatorHealth(request);
}
