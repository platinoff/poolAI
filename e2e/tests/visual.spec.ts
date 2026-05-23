import { test, expect } from "@playwright/test";
import {
  loginAsAdmin,
  matrixSnapshotName,
  primeUiPrefs,
  expectVisualLang,
  visualMaskLocators,
  waitForAdminContentReady,
  type VisualLang,
  type VisualTheme,
} from "./helpers";

/** Admin routes with stable shell; dynamic charts/SVG masked where noted. */
const ADMIN_VISUAL_PAGES: Array<{
  path: string;
  name: string;
  content: string;
  masks?: string[];
  afterReady?: string;
}> = [
  {
    path: "/ui/admin",
    name: "dashboard",
    content: "#admin_main_content",
    masks: ["#metrics-chart"],
    afterReady: "#system-overview .stat-item, #system-overview .admin-fetch-error",
  },
  {
    path: "/ui/admin/users",
    name: "users",
    content: "#users-list",
  },
  {
    path: "/ui/admin/tenants",
    name: "tenants",
    content: "#tenants-list",
  },
  {
    path: "/ui/admin/monitoring",
    name: "monitoring",
    content: "#monitoring-content",
    masks: [".metrics-charts-grid", ".metric-chart-svg"],
  },
  {
    path: "/ui/admin/security",
    name: "security",
    content: "#security-content",
  },
  {
    path: "/ui/admin/audit",
    name: "audit",
    content: "#audit-events",
  },
  {
    path: "/ui/admin/raid",
    name: "raid",
    content: "#raid-admin",
    masks: ["#raid-artifacts", "#raid-admin"],
  },
  {
    path: "/ui/admin/workers",
    name: "workers",
    content: "#workers-list",
  },
  {
    path: "/ui/admin/vm",
    name: "vm",
    content: "#vm-instances",
  },
  {
    path: "/ui/admin/libs",
    name: "libs",
    content: "#libraries-list",
  },
];

test.describe("PoolAI admin visual regression (PH-S11)", () => {
  test.beforeEach(async ({ page }) => {
    await loginAsAdmin(page);
  });

  for (const { path, name, content, masks, afterReady } of ADMIN_VISUAL_PAGES) {
    test(`snapshot ${name}`, async ({ page }) => {
      await page.goto(path);
      await waitForAdminContentReady(page, content);
      if (afterReady) {
        await expect(page.locator(afterReady).first()).toBeVisible({
          timeout: 20_000,
        });
      }
      const mask = masks?.length ? visualMaskLocators(page, masks) : undefined;
      await expect(page.locator("main.admin-main")).toHaveScreenshot(
        `${name}.png`,
        {
          animations: "disabled",
          mask,
        },
      );
    });
  }
});

test.describe("PoolAI login visual (PH-S11)", () => {
  test("snapshot login page", async ({ page }) => {
    await page.goto("/ui/login");
    await expect(page.locator("#loginForm, form")).toBeVisible({
      timeout: 15_000,
    });
    await page.evaluate(() => document.fonts?.ready);
    await expect(page).toHaveScreenshot("login.png", {
      animations: "disabled",
      fullPage: true,
    });
  });
});

const VISUAL_THEMES: VisualTheme[] = ["dark", "light"];
const VISUAL_LANGS: VisualLang[] = ["en", "uk"];

/** Representative pages for theme × i18n matrix (PH-S12). */
const VISUAL_MATRIX_ADMIN: Array<{
  path: string;
  name: string;
  content: string;
  masks?: string[];
  afterReady?: string;
}> = [
  {
    path: "/ui/admin",
    name: "dashboard",
    content: "#admin_main_content",
    masks: ["#metrics-chart"],
    afterReady: "#system-overview .stat-item, #system-overview .admin-fetch-error",
  },
  {
    path: "/ui/admin/users",
    name: "users",
    content: "#users-list",
  },
];

test.describe("PoolAI theme/i18n visual matrix (PH-S12)", () => {
  for (const theme of VISUAL_THEMES) {
    for (const lang of VISUAL_LANGS) {
      test(`login ${theme} ${lang}`, async ({ page }) => {
        await primeUiPrefs(page, { theme, lang });
        await page.goto("/ui/login");
        await expect(page.locator("#loginForm")).toBeVisible({
          timeout: 15_000,
        });
        await expectVisualLang(page, lang);
        await page.evaluate(() => document.fonts?.ready);
        await expect(page).toHaveScreenshot(
          matrixSnapshotName("login", theme, lang),
          { animations: "disabled", fullPage: true },
        );
      });

      test(`admin users ${theme} ${lang}`, async ({ page }) => {
        await loginAsAdmin(page, { theme, lang });
        const spec = VISUAL_MATRIX_ADMIN[1];
        await page.goto(spec.path);
        await waitForAdminContentReady(page, spec.content);
        await expectVisualLang(page, lang);
        const mask = spec.masks?.length
          ? visualMaskLocators(page, spec.masks)
          : undefined;
        await expect(page.locator("main.admin-main")).toHaveScreenshot(
          matrixSnapshotName("users", theme, lang),
          { animations: "disabled", mask },
        );
      });

      test(`admin dashboard ${theme} ${lang}`, async ({ page }) => {
        await loginAsAdmin(page, { theme, lang });
        const spec = VISUAL_MATRIX_ADMIN[0];
        await page.goto(spec.path);
        await waitForAdminContentReady(page, spec.content);
        if (spec.afterReady) {
          await expect(page.locator(spec.afterReady).first()).toBeVisible({
            timeout: 20_000,
          });
        }
        await expectVisualLang(page, lang);
        const mask = spec.masks?.length
          ? visualMaskLocators(page, spec.masks)
          : undefined;
        await expect(page.locator("main.admin-main")).toHaveScreenshot(
          matrixSnapshotName("dashboard", theme, lang),
          { animations: "disabled", mask },
        );
      });
    }
  }
});
