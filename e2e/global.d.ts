/** Minimal Node stubs for IDE only (Playwright runtime provides Node; no @types/node required). */
declare const process: {
  env: Record<string, string | undefined>;
};

declare module "node:child_process" {
  export function execFileSync(
    file: string,
    args?: readonly string[],
    options?: { stdio?: "inherit" | "pipe"; env?: Record<string, string | undefined> },
  ): Buffer | string;
}
