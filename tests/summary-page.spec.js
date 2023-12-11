// @ts-check
const { test, expect } = require('@playwright/test');

test('summary is present', async ({ page }) => {
  await page.goto('/');

  await expect(page.getByText("Height")).toBeVisible();
  await expect(page.getByText("Slot")).toBeVisible();
  await expect(page.getByText("Epoch")).toBeVisible();
  await expect(page.getByText("Circulating Supply")).toBeVisible();
  await expect(page.getByText("Total Currency")).toBeVisible();
});


