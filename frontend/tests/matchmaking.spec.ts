import { test, expect } from '@playwright/test';

test('should pair two players in matchmaking', async ({ browser }) => {
    const user1 = `p1_${Math.floor(Math.random() * 10000)}`;
    const user2 = `p2_${Math.floor(Math.random() * 10000)}`;
    const password = 'password';

    // Context for Player 1
    const context1 = await browser.newContext();
    const page1 = await context1.newPage();

    // Context for Player 2
    const context2 = await browser.newContext();
    const page2 = await context2.newPage();

    // Player 1 Signup & Login
    await page1.goto('/signup');
    await page1.waitForLoadState('networkidle');
    await page1.fill('#username', user1);
    await page1.fill('#password', password);
    await page1.fill('#confirm', password);
    await page1.click('.auth-btn');
    await page1.waitForURL('**/login');
    await page1.fill('#username', user1);
    await page1.fill('#password', password);
    await page1.click('.auth-btn');

    // Player 2 Signup & Login
    await page2.goto('/signup');
    await page2.waitForLoadState('networkidle');
    await page2.fill('#username', user2);
    await page2.fill('#password', password);
    await page2.fill('#confirm', password);
    await page2.click('.auth-btn');
    await page2.waitForURL('**/login');
    await page2.fill('#username', user2);
    await page2.fill('#password', password);
    await page2.click('.auth-btn');

    // Both Join Matchmaking
    await page1.click('text=Find a Match');
    await page2.click('text=Find a Match');

    // Wait for redirect to game page (timeout 15s)
    await expect(page1).toHaveURL(/\/game\//, { timeout: 15000 });
    await expect(page2).toHaveURL(/\/game\//, { timeout: 15000 });

    // Verify same game ID
    const url1 = page1.url();
    const url2 = page2.url();
    expect(url1).toBe(url2);

    await context1.close();
    await context2.close();
});
