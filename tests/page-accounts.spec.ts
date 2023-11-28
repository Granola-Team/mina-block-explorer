import { test, expect } from '@playwright/test';

test.describe('[/accounts/[public-key] Page]', () => { 
  test('should have all essential elements on page', async ({ page }) => {
    await page.goto('/accounts/B62qpge4uMq4Vv5Rvc8Gw9qSquUYd6xoW1pz7HQkMSHm6h1o7pvLPAN'); 

    await expect(page.getByText('Public Key')).toBeVisible();
    await expect(page.getByText('Username')).toBeVisible();
    await expect(page.getByText('Balance')).toBeVisible();
    await expect(page.getByText('Nonce')).toBeVisible();
    await expect(page.getByText('Receipt Chain Hash')).toBeVisible();
    await expect(page.getByText('Delegate')).toBeVisible();
    await expect(page.getByText('Voting For')).toBeVisible();
    await expect(page.getByText('Total Transactions')).toBeVisible();
  });
});
