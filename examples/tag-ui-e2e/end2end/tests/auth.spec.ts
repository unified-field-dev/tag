import { test, expect, seedAuth, waitForHydrated } from "./fixtures";

test.describe("pw-tag-auth", () => {
  test("pw-tag-anon-gate-sad", async ({ page }) => {
    await seedAuth(page, "anonymous");
    await page.goto("/tag", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("auth-required-empty-state")).toBeAttached({
      timeout: 60_000,
    });
    await expect(page.getByTestId("tag-list-page")).toHaveCount(0);
  });

  test("pw-tag-unverified-gate-sad", async ({ page }) => {
    await seedAuth(page, "unverified");
    await page.goto("/tag", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("tag-list-page")).toHaveCount(0);
    await expect(
      page.getByTestId("email-verification-required-empty-state"),
    ).toBeAttached({ timeout: 30_000 });
  });
});
