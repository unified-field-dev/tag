import { test as base, expect, type Page } from "@playwright/test";

export type SeedAuthKind = "anonymous" | "owner" | "peer" | "unverified";

export type SeedFixtures = {
  tag_id: string;
  tag_name: string;
  peer_tag_id: string;
  peer_tag_name: string;
};

export async function seedAuth(
  page: Page,
  auth: SeedAuthKind,
  seedCatalog = false,
) {
  const res = await page.request.post("/api/test/seed-data", {
    data: {
      auth,
      seed_catalog: seedCatalog,
    },
  });
  const body = await res.text();
  expect(
    res.ok(),
    `seed-data failed (HTTP ${res.status()}): ${body || "(empty body)"}. Is the server up at ${process.env.PLAYWRIGHT_BASE_URL ?? "http://127.0.0.1:3160"}?`,
  ).toBeTruthy();
  return JSON.parse(body) as {
    ok: boolean;
    auth: string;
    fixtures: SeedFixtures;
  };
}

/**
 * Wait for Orbital boot overlay to finish and hydrate to mark the document ready.
 */
export async function waitForHydrated(page: Page, timeoutMs = 240_000) {
  await expect
    .poll(
      async () =>
        page.evaluate(() => {
          const html = document.documentElement;
          if (html.getAttribute("data-orbital-boot-state") === "error") {
            return "error";
          }
          if (html.getAttribute("data-orbital-hydrated") === "true") {
            return "ready";
          }
          return "loading";
        }),
      { timeout: timeoutMs },
    )
    .not.toBe("error");
  await expect
    .poll(
      async () =>
        page.evaluate(
          () => document.documentElement.getAttribute("data-orbital-hydrated") === "true",
        ),
      { timeout: timeoutMs },
    )
    .toBe(true);
  await expect(page.getByTestId("orbital-boot-overlay")).toHaveCount(0, {
    timeout: 60_000,
  });
  await expect(page.getByTestId("e2e-auth-bootstrap")).toBeAttached({
    timeout: 30_000,
  });
}

export const test = base;
export { expect };
