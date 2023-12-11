// @ts-check
const { test, expect } = require('@playwright/test');

test('account page present', async ({ page }) => {
  await page.goto('/accounts/B62qpWaQoQoPL5AGta7Hz2DgJ9CJonpunjzCGTdw8KiCCD1hX8fNHuR');

  await expect(page.getByText("Public Key")).toBeVisible();
  await expect(page.getByText("Username")).toBeVisible();
  await expect(page.getByText("Balance")).toBeVisible();
  await expect(page.getByText("Nonce")).toBeVisible();
  await expect(page.getByText("Receipt Chain Hash")).toBeVisible();
  await expect(page.getByText("Delegate")).toBeVisible();
  await expect(page.getByText("Voting For")).toBeVisible();
  await expect(page.getByText("Pending Transactions")).toBeVisible();

});


