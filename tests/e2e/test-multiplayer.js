const playwright = require('playwright');

(async () => {
    const browser = await playwright.chromium.launch({ headless: false });
    const context = await browser.newContext();

    // Open both players
    const pageA = await context.newPage();
    const pageB = await context.newPage();

    await pageA.goto('http://localhost:5173');
    await pageB.goto('http://localhost:5174');

    console.log('✅ Both browsers loaded');
    console.log('⏳ Waiting 3 seconds for game state to sync...');
    await new Promise(r => setTimeout(r, 3000));

    // Game sequence: Create a winnable game for Red (Player A)
    // Red wins with column 3 (index 3) vertical: moves at indices [3,4,3,4,3,4,3]
    // Note: Column index 3 = UI displays as "4"
    const moves = [
        { player: 'A', colIndex: 3, displayCol: 4, name: 'Red' },
        { player: 'B', colIndex: 4, displayCol: 5, name: 'Yellow' },
        { player: 'A', colIndex: 3, displayCol: 4, name: 'Red' },
        { player: 'B', colIndex: 4, displayCol: 5, name: 'Yellow' },
        { player: 'A', colIndex: 3, displayCol: 4, name: 'Red' },
        { player: 'B', colIndex: 4, displayCol: 5, name: 'Yellow' },
        { player: 'A', colIndex: 3, displayCol: 4, name: 'Red (WINNING MOVE)' }
    ];

    console.log('\n🎮 Starting Connect4 game test...');
    console.log('📋 Planned moves:', moves.map(m => `column ${m.displayCol}`).join(', '));

    for (let i = 0; i < moves.length; i++) {
        const move = moves[i];
        const currentPage = move.player === 'A' ? pageA : pageB;
        const otherPage = move.player === 'A' ? pageB : pageA;

        console.log(`\n--- Move ${i + 1}/7: ${move.name} plays column ${move.displayCol} (index ${move.colIndex}) ---`);

        // Click using the column-indicator element (the numbered buttons at the top)
        // These have onclick="makeMove(N)" where N is the column index
        console.log(`🖱️  Clicking column indicator ${move.displayCol} on Player ${move.player}'s browser...`);

        try {
            // Use nth-child to select the right column indicator
            // nth-child is 1-based, so displayCol matches directly
            const indicatorSelector = `.column-indicators .column-indicator:nth-child(${move.displayCol})`;
            await currentPage.click(indicatorSelector, { timeout: 5000 });
            console.log('✅ Column indicator clicked successfully');
        } catch (e) {
            console.log('❌ Click failed:', e.message);
            // Try alternative: click the board column directly
            try {
                console.log('⚠️  Trying alternative: clicking board column...');
                await currentPage.evaluate((colIdx) => {
                    makeMove(colIdx);
                }, move.colIndex);
                console.log('✅ makeMove() called directly');
            } catch (e2) {
                console.log('❌ Direct call also failed:', e2.message);
                break;
            }
        }

        // Wait for move to process
        console.log('⏳ Waiting 2 seconds for move to process...');
        await new Promise(r => setTimeout(r, 2000));

        // Check board state on both players
        const boardA = await pageA.evaluate(() => {
            const cells = document.querySelectorAll('.cell');
            let filledCount = 0;
            cells.forEach(cell => {
                if (cell.classList.contains('red') || cell.classList.contains('yellow')) {
                    filledCount++;
                }
            });
            return filledCount;
        });

        const boardB = await pageB.evaluate(() => {
            const cells = document.querySelectorAll('.cell');
            let filledCount = 0;
            cells.forEach(cell => {
                if (cell.classList.contains('red') || cell.classList.contains('yellow')) {
                    filledCount++;
                }
            });
            return filledCount;
        });

        console.log(`🎯 Board state: Player A sees ${boardA} discs, Player B sees ${boardB} discs (expected: ${i + 1})`);

        if (boardA !== i + 1 || boardB !== i + 1) {
            console.log('⚠️  Board sync issue detected!');
        } else {
            console.log('✅ Board synchronized correctly');
        }

        // Wait for UI to update
        await new Promise(r => setTimeout(r, 1000));
    }

    console.log('\n⏳ Waiting 5 seconds for win detection...');
    await new Promise(r => setTimeout(r, 5000));

    // Check for victory screen
    const victoryA = await pageA.evaluate(() => {
        const victoryScreen = document.getElementById('victoryScreen');
        return victoryScreen && !victoryScreen.classList.contains('hidden');
    });

    const victoryB = await pageB.evaluate(() => {
        const victoryScreen = document.getElementById('victoryScreen');
        return victoryScreen && !victoryScreen.classList.contains('hidden');
    });

    console.log('\n🏆 Victory screen status:');
    console.log('  Player A:', victoryA ? '✅ SHOWING' : '❌ NOT SHOWING');
    console.log('  Player B:', victoryB ? '✅ SHOWING' : '❌ NOT SHOWING');

    // Take final screenshots
    await pageA.screenshot({ path: 'player_a_final.png', fullPage: true });
    await pageB.screenshot({ path: 'player_b_final.png', fullPage: true });
    console.log('\n📸 Screenshots saved: player_a_final.png, player_b_final.png');

    console.log('\n✅ Test sequence complete!');
    console.log('🔍 Keeping browsers open for 20 seconds for inspection...');

    // Keep browsers open briefly
    await new Promise(r => setTimeout(r, 20000));

    await browser.close();
    console.log('\n👋 Test complete - browsers closed');
})();
