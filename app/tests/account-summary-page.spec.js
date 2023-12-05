// @ts-check
const { test, expect } = require('@playwright/test');

test('account page present', async ({ page }) => {
  await page.goto('/accounts/fake-data?f=true');

  await expect(page.getByText("Public Key")).toBeVisible();
  await expect(page.getByText("Username")).toBeVisible();
  await expect(page.getByText("Balance")).toBeVisible();
  await expect(page.getByText("Nonce")).toBeVisible();
  await expect(page.getByText("Receipt Chain Hash")).toBeVisible();
  await expect(page.getByText("Delegate")).toBeVisible();
  await expect(page.getByText("Voting For")).toBeVisible();
  await expect(page.getByText("Pending Transactions")).toBeVisible();

});


