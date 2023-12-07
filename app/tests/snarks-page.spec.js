// @ts-check
const { test, expect } = require('@playwright/test');

test('snarks table is present', async ({ page }) => {
  await page.goto('/snarks');
  await page.waitForSelector('table', { state: 'attached'});

  await expect(await page.getByRole("cell", { name: "Height" })).toBeVisible()
  await expect(await page.getByRole("cell", { name: "Date" })).toBeVisible()
  await expect(await page.getByRole("cell", { name: "Prover" })).toBeVisible()
  await expect(await page.getByRole("cell", { name: "Work Ids", exact: true })).toBeVisible()
  await expect(await page.getByRole("cell", { name: "State Hash" })).toBeVisible()
  await expect(await page.getByRole("cell", { name: "Fee" })).toBeVisible()

  await expect(await page.locator('tr').count()).toEqual(26);
  await expect(await page.locator('th').count()).toEqual(6);

});


