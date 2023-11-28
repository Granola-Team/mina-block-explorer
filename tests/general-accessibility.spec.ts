import { test, expect } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright'; 

const pages = ['/summary', '/accounts/B62qoA5XwfEVnXbcrzphGH1TVuqxeJ5bhX7vTS3hcxpQFHnStG3MQk9'];
pages.forEach(page => {
  test.describe(`${page} Page`, () => { 
    test('should not have any automatically detectable accessibility issues', async ({ page }) => {
      await page.goto('/summary'); 
  
      const accessibilityScanResults = await new AxeBuilder({ page }).analyze(); 
  
      expect(accessibilityScanResults.violations).toEqual([]); 
    });
  });
});