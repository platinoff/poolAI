import { defineConfig } from "@playwright/test";

const baseURL = process.env.POOLAI_BASE_URL ?? "http://127.0.0.1:8080";

export default defineConfig({
  testDir: "./tests",
  timeout: 60_000,
  retries: process.env.CI ? 1 : 0,
  snapshotPathTemplate:
    "{testDir}/{testFileDir}/{testFileName}-snapshots/{arg}{ext}",
  expect: {
    toHaveScreenshot: {
      animations: "disabled",
      maxDiffPixelRatio: 0.02,
    },
  },
  use: {
    baseURL,
    trace: "on-first-retry",
    viewport: { width: 1280, height: 720 },
    deviceScaleFactor: 1,
  },
  reporter: [["list"]],
});
