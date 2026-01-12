const playwright = require('playwright');

(async () => {
    const browser = await playwright.chromium.launch({ headless: false });
    const context = await browser.newContext();

    const pageA = await context.newPage();
    await pageA.goto('http://localhost:5173');

    console.log('✅ Player A browser loaded');
    console.log('⏳ Waiting 3 seconds for game state to sync...');
    await new Promise(r => setTimeout(r, 3000));

    console.log('\n🎯 Executing winning move: Red plays column 4 (index 3)');

    // Execute the winning move directly
    await pageA.evaluate(() => {
        makeMove(3);
    });

    console.log('✅ Winning move executed');
    console.log('⏳ Waiting 5 seconds for win detection and victory animation...');
    await new Promise(r => setTimeout(r, 5000));

    // Check for victory screen
    const victoryShown = await pageA.evaluate(() => {
        const victoryScreen = document.getElementById('victoryScreen');
        return victoryScreen && !victoryScreen.classList.contains('hidden');
    });

    console.log('\n🏆 Victory screen:', victoryShown ? 'SHOWING ✅' : 'NOT SHOWING ❌');

    // Get game status
    const gameStatus = await pageA.evaluate(() => {
        const moves = document.querySelector('.stat-item:nth-child(1) .stat-value')?.textContent || 'unknown';
        const turn = document.querySelector('.stat-item:nth-child(3) .stat-value')?.textContent || 'unknown';
        return { moves, turn };
    });

    console.log('📊 Game status:', gameStatus);

    // Count discs on board
    const discCount = await pageA.evaluate(() => {
        const cells = document.querySelectorAll('.cell');
        let red = 0, yellow = 0;
        cells.forEach(cell => {
            if (cell.classList.contains('red')) red++;
            if (cell.classList.contains('yellow')) yellow++;
        });
        return { red, yellow, total: red + yellow };
    });

    console.log('🎲 Disc count:', discCount);

    // Take screenshot
    await pageA.screenshot({ path: 'winning_move.png', fullPage: true });
    console.log('📸 Screenshot saved: winning_move.png');

    console.log('\n🔍 Keeping browser open for 30 seconds...');
    await new Promise(r => setTimeout(r, 30000));

    await browser.close();
    console.log('👋 Test complete');
})();
