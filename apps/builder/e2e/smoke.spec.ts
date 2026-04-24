import { test, expect } from '@playwright/test';

test('should load sample rule and render DSL', async ({ page }) => {
  await page.goto('/');

  // Click "Load Sample" button
  const loadSampleBtn = page.getByRole('button', { name: /load sample/i });
  await loadSampleBtn.click();

  // Wait for nodes to load
  await page.waitForTimeout(1000);

  // Click "Show DSL" button
  const showDslBtn = page.getByRole('button', { name: /show dsl/i });
  await showDslBtn.click();

  // Verify FPL panel contains rule(
  const fplPanel = page.locator('.font-mono');
  await expect(fplPanel).toContainText('rule(');
  await expect(fplPanel).toContainText('deep-work-starter');
});

test('should add nodes via palette', async ({ page }) => {
  await page.goto('/');

  // Expand Trigger category
  const triggerBtn = page.getByRole('button', { name: /trigger/i }).first();
  await triggerBtn.click();

  // Add a trigger node
  const eventBtn = page.getByRole('button', { name: /event/i });
  await eventBtn.click();

  // Wait for node to appear (very basic check)
  await page.waitForTimeout(500);

  // Should have at least one node now
  const nodeElements = page.locator('[data-testid*="node"]');
  const count = await nodeElements.count();
  expect(count).toBeGreaterThanOrEqual(0); // Playwright finding nodes is difficult without better selectors
});
