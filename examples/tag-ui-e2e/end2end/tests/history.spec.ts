import { test, expect, seedAuth, waitForHydrated } from "./fixtures";

test.describe("pw-tag-history", () => {
  test("pw-tag-history-created-on-detail-happy", async ({ page }) => {
    const { fixtures } = await seedAuth(page, "owner", true);
    await page.goto(`/tag/${fixtures.tag_id}`, { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("tag-detail-page")).toBeVisible({
      timeout: 60_000,
    });
    await expect(page.getByTestId("record-history-timeline")).toBeVisible({
      timeout: 60_000,
    });
    await expect(page.getByTestId("history-empty-default")).toHaveCount(0);
    await expect(page.locator("[data-history-entry-id]").first()).toBeVisible({
      timeout: 60_000,
    });
  });

  test("pw-tag-history-after-update-happy", async ({ page }) => {
    const { fixtures } = await seedAuth(page, "owner", true);
    await page.goto(`/tag/${fixtures.tag_id}`, { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("tag-detail-page")).toBeVisible({
      timeout: 60_000,
    });
    const updated = `${fixtures.tag_name}-hist-edit`;
    await page.getByRole("textbox", { name: /^Name$/ }).fill(updated);
    await page.getByTestId("tag-detail-save").click();
    await expect(page.getByRole("textbox", { name: /^Name$/ })).toHaveValue(updated, {
      timeout: 60_000,
    });
    await expect(page.getByTestId("record-history-timeline")).toBeVisible({
      timeout: 60_000,
    });
    await expect(page.getByTestId("history-empty-default")).toHaveCount(0);
    await expect(page.getByText(updated, { exact: false }).first()).toBeVisible({
      timeout: 60_000,
    });
    await expect(page.locator("[data-history-entry-id]").first()).toBeVisible({
      timeout: 60_000,
    });
  });

  test("pw-tag-create-writes-history-happy", async ({ page }) => {
    await seedAuth(page, "owner");
    await page.goto("/tag/create", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("tag-create-page")).toBeVisible({
      timeout: 60_000,
    });
    const name = `E2E-Hist-Create-${Date.now()}`;
    await page.getByRole("textbox", { name: /^Name/ }).fill(name);
    await page.getByRole("textbox", { name: /^Taxonomy/ }).fill("spend");
    await page.getByTestId("tag-create-submit").click();
    await expect(page).toHaveURL(/\/tag\/[^/]+$/, { timeout: 60_000 });
    await waitForHydrated(page);
    await expect(page.getByTestId("tag-detail-page")).toBeVisible({
      timeout: 60_000,
    });
    await expect(page.getByTestId("record-history-timeline")).toBeVisible({
      timeout: 60_000,
    });
    await expect(page.getByTestId("history-empty-default")).toHaveCount(0);
    await expect(page.locator("[data-history-entry-id]").first()).toBeVisible({
      timeout: 60_000,
    });
  });
});
