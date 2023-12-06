// @ts-check
const { test, expect } = require('@playwright/test');

test('blocks page is present', async ({ page }) => {
    await page.goto('/blocks');

    await page.waitForSelector('td', { state: 'attached'});

    await expect(page.getByText("Height")).toBeVisible()
    await expect(page.getByText("Date")).toBeVisible()
    await expect(page.getByText("Block Producer")).toBeVisible()
    await expect(page.getByText("Coinbase", { exact: true })).toBeVisible()
    await expect(page.getByText("Transactions")).toBeVisible()
    await expect(page.getByText("SNARKs")).toBeVisible()
    await expect(page.getByText("Slot")).toBeVisible()
    await expect(page.getByText("State Hash")).toBeVisible()
    await expect(page.getByText("Coinbase Receiver")).toBeVisible()

    await expect(await page.locator('tr').count()).toEqual(11);
    await expect(await page.locator('th').count()).toEqual(9);

});
