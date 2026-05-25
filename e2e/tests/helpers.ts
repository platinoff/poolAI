import { expect, type Locator, type Page } from "@playwright/test";

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

/** Log in via /ui/login and wait for dashboard. */
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
  const loginResponse = page.waitForResponse(
    (res) =>
      res.url().includes("/api/v1/login") &&
      res.request().method() === "POST" &&
      res.ok(),
  );
  await page.locator("#loginForm").evaluate((form) => {
    (form as HTMLFormElement).requestSubmit();
  });
  const loginJson = await (await loginResponse).json() as {
    token?: string;
    role?: string;
  };
  await page.waitForURL(/\/ui\/?$/, { timeout: 20_000 });
  if (loginJson.token) {
    await page.evaluate(
      ({ token, role, user }) => {
        localStorage.setItem("poolai_token", token);
        if (role) localStorage.setItem("poolai_role", role);
        localStorage.setItem("poolai_user", user);
      },
      { token: loginJson.token, role: loginJson.role ?? "Admin", user: e2eUser },
    );
  }
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
      const settled =
        region.querySelector(".admin-table tbody tr") ||
        region.querySelector(".admin-fetch-error") ||
        region.querySelector(".admin-card") ||
        region.querySelector(".admin-form") ||
        region.querySelector("form") ||
        (region.querySelector(".muted") &&
          !/loading/i.test(region.textContent ?? ""));
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
  await expect(content).toBeVisible({ timeout: 20_000 });
  await expect(
    content.locator(".admin-table, .muted, .admin-fetch-error, .admin-card, .admin-form, form, .stat-item").first(),
  ).toBeVisible({ timeout: 20_000 });
  await page.evaluate(() => document.fonts?.ready);
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
