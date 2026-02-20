import { test, expect } from '@playwright/test';

test.describe('Authentication Flow', () => {
    test.describe.configure({ mode: 'serial' });
    const username = `user_${Math.floor(Math.random() * 100000)}`;
    const password = 'password123';

    test('should register a new user', async ({ page }) => {
        await page.goto('/signup');
        await page.waitForLoadState('networkidle');
        await page.fill('#username', username);
        await page.fill('#password', password);
        await page.fill('#confirm', password);
        await page.click('.auth-btn');

        // Should redirect to login
        await expect(page).toHaveURL('/login');
    });

    test('should login with the new user', async ({ page }) => {
        await page.goto('/login');
        await page.waitForLoadState('networkidle');
        await page.fill('#username', username);
        await page.fill('#password', password);
        await page.click('.auth-btn');

        // Should redirect to home and show username
        await expect(page).toHaveURL('/');
        await expect(page.locator('.username')).toHaveText(username);
    });

    test('should logout successfully', async ({ page }) => {
        await page.goto('/login');
        await page.fill('#username', username);
        await page.fill('#password', password);
        await page.click('.auth-btn');

        await expect(page.locator('.username')).toHaveText(username);

        await page.click('.logout-btn');
        await expect(page.locator('text=Login')).toBeVisible();
        await expect(page.locator('.username')).not.toBeVisible();
    });

    test('should show error for invalid login', async ({ page }) => {
        await page.goto('/login');
        await page.fill('#username', 'wrong_user');
        await page.fill('#password', 'wrong_pass');
        await page.click('.auth-btn');

        await page.waitForTimeout(500);
        await expect(page.locator('.error')).toBeVisible();
        await expect(page.locator('.error')).toHaveText('Invalid username or password');
    });
});
