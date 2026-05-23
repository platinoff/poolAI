import { expect, type Locator, type Page } from "@playwright/test";

export const e2eUser = process.env.POOLAI_E2E_USER ?? "admin";
export const e2ePassword = process.env.POOLAI_E2E_PASSWORD ?? "admin123";

export type VisualTheme = "dark" | "light";
export type VisualLang = "en" | "uk";

/** Set theme/locale in localStorage before the first navigation. */
export async function primeUiPrefs(
  page: Page,
  prefs: { theme?: VisualTheme; lang?: VisualLang },
): Promise<void> {
  await page.addInitScript((p) => {
    if (p.theme) localStorage.setItem("poolai_theme", p.theme);
    if (p.lang) localStorage.setItem("poolai_ui_lang", p.lang);
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
  prefs?: { theme?: VisualTheme; lang?: VisualLang },
): Promise<void> {
  if (prefs) {
    await primeUiPrefs(page, prefs);
  }
  await page.goto("/ui/login");
  await page.locator("#username").fill(e2eUser);
  await page.locator("#password").fill(e2ePassword);
  await page.locator("#loginForm").evaluate((form) => {
    (form as HTMLFormElement).requestSubmit();
  });
  await page.waitForURL(/\/ui\/?$/, { timeout: 20_000 });
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

/** Wait until admin shell and primary content region are ready (table, empty, or error). */
export async function waitForAdminContentReady(
  page: Page,
  contentSelector: string,
): Promise<void> {
  const content = page.locator(contentSelector);
  await expect(content).toBeVisible({ timeout: 20_000 });
  await expect(
    content.locator(".admin-table, .muted, .admin-fetch-error, .admin-card").first(),
  ).toBeVisible({ timeout: 20_000 });
  await page.evaluate(() => document.fonts?.ready);
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
