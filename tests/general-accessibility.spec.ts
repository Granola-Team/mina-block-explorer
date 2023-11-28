import { test, expect } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright'; 

test.describe('[/summary Page]', () => { 
  test('should not have any automatically detectable accessibility issues', async ({ page }) => {
    await page.goto('/summary'); 

    const accessibilityScanResults = await new AxeBuilder({ page }).analyze(); 

    expect(accessibilityScanResults.violations).toEqual([]); 
  });
});

test.describe('[/accounts Page]', () => { 
  test('should not have any automatically detectable accessibility issues', async ({ page }) => {
    await page.goto('/accounts/B62qoA5XwfEVnXbcrzphGH1TVuqxeJ5bhX7vTS3hcxpQFHnStG3MQk9/'); 

    const accessibilityScanResults = await new AxeBuilder({ page }).analyze(); 

    expect(accessibilityScanResults.violations).toEqual([]); 
  });
});