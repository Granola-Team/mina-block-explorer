// @ts-check
const { test, expect } = require('@playwright/test');

test('account page present', async ({ page }) => {
  await page.goto('/accounts/fake-data?f=true');


  await expect(page.getByText('Nonce')).toBeVisible();

  expect(await page.screenshot()).toMatchSnapshot();

});


