// @ts-check
const { test, expect } = require('@playwright/test');

test('blocks table is present', async ({ page }) => {
    await page.goto('/blocks');

    await page.waitForSelector('table', { state: 'attached'});

    await expect(await page.getByRole("cell", { name: "Height" })).toBeVisible()
    await expect(await page.getByRole("cell", { name: "Date" })).toBeVisible()
    await expect(await page.getByRole("cell", { name: "Block Producer" })).toBeVisible()
    await expect(await page.getByRole("cell", { name: "Coinbase", exact: true })).toBeVisible()
    await expect(await page.getByRole('cell', { name: "Transactions" })).toBeVisible()
    await expect(await page.getByRole("cell", { name: "SNARKs" })).toBeVisible()
    await expect(await page.getByRole("cell", { name: "Slot" })).toBeVisible()
    await expect(await page.getByRole("cell", { name: "State Hash" })).toBeVisible()
    await expect(await page.getByRole("cell", { name: "Coinbase Receiver" })).toBeVisible()

    await expect(await page.locator('tr').count()).toEqual(11);
    await expect(await page.locator('th').count()).toEqual(9);

});
