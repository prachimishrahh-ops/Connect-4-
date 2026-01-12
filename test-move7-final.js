const playwright = require('playwright');

(async () => {
    console.log('\n🎯 TESTING MOVE 7 FIX - Post-Mutation Validation + Aggressive Retry\n');
    console.log('='.repeat(70));

    const browser = await playwright.chromium.launch({
        headless: false,
        args: ['--start-maximized']
    });
    const context = await browser.newContext({ viewport: null });

    const pageA = await context.newPage();
    const pageB = await context.newPage();

    // Console logging
    pageA.on('console', msg => {
        const text = msg.text();
        if (text.includes('🎯') || text.includes('✅') || text.includes('❌') ||
            text.includes('🔄') || text.includes('⏳') || text.includes('🏆') ||
            text.includes('📊') || text.includes('⚠️')) {
            console.log(`[RED] ${text}`);
        }
    });
    pageB.on('console', msg => {
        const text = msg.text();
        if (text.includes('🎯') || text.includes('✅') || text.includes('❌') ||
            text.includes('🔄') || text.includes('⏳') || text.includes('🏆') ||
            text.includes('📊') || text.includes('⚠️')) {
            console.log(`[YLW] ${text}`);
        }
    });

    await pageA.goto('http://localhost:5173');
    await pageB.goto('http://localhost:5174');

    console.log('✅ Both browsers loaded\n');

    // MATCHMAKING
    console.log('━'.repeat(70));
    console.log('MATCHMAKING PHASE');
    console.log('━'.repeat(70) + '\n');

    await pageA.fill('#playerName', 'FinalTestRed');
    await pageB.fill('#playerName', 'FinalTestYellow');
    await new Promise(r => setTimeout(r, 500));

    await Promise.all([
        pageA.click('button:has-text("PLAY NOW")', { timeout: 5000 }).catch(() => {}),
        new Promise(r => setTimeout(r, 200)).then(() =>
            pageB.click('button:has-text("PLAY NOW")', { timeout: 5000 }).catch(() => {})
        )
    ]);

    console.log('⏳ Waiting for match...\n');
    await new Promise(r => setTimeout(r, 8000));

    // Verify game loaded
    const gameLoadedA = await pageA.evaluate(() => {
        return document.getElementById('gameScreen') &&
               !document.getElementById('gameScreen').classList.contains('hidden');
    });
    const gameLoadedB = await pageB.evaluate(() => {
        return document.getElementById('gameScreen') &&
               !document.getElementById('gameScreen').classList.contains('hidden');
    });

    if (!gameLoadedA || !gameLoadedB) {
        console.log('\n❌ ERROR: Game did not load for both players');
        await browser.close();
        return;
    }

    console.log('✅ MATCH FOUND!\n');

    // GAMEPLAY - ALL 7 MOVES
    console.log('━'.repeat(70));
    console.log('GAMEPLAY PHASE - Testing Move 7 with Post-Mutation Validation');
    console.log('━'.repeat(70) + '\n');

    const moves = [
        { page: pageA, col: 3, player: 'Red' },
        { page: pageB, col: 4, player: 'Yellow' },
        { page: pageA, col: 3, player: 'Red' },
        { page: pageB, col: 4, player: 'Yellow' },
        { page: pageA, col: 3, player: 'Red' },
        { page: pageB, col: 4, player: 'Yellow' },
        { page: pageA, col: 3, player: 'Red - WINNING MOVE 🏆' }
    ];

    for (let i = 0; i < moves.length; i++) {
        const move = moves[i];
        const isWinning = i === 6;

        console.log(`\n${'='.repeat(70)}`);
        console.log(`MOVE ${i + 1}/7: ${move.player} → Column ${move.col + 1}`);
        console.log('='.repeat(70));

        // Execute move
        console.log('🎯 Executing move...');
        await move.page.evaluate((col) => {
            makeMove(col);
        }, move.col);

        // Wait for completion - longer for Move 7 since it may retry
        const waitTime = isWinning ? 30000 : 10000;
        console.log(`⏳ Waiting ${waitTime/1000}s for processing${isWinning ? ' (includes retry time)' : ''}...\n`);
        await new Promise(r => setTimeout(r, waitTime));

        // Check disc count
        const discsA = await pageA.evaluate(() => {
            return document.querySelectorAll('.cell.red, .cell.yellow').length;
        });
        const discsB = await pageB.evaluate(() => {
            return document.querySelectorAll('.cell.red, .cell.yellow').length;
        });

        console.log(`📊 Disc Count: Player A sees ${discsA}, Player B sees ${discsB}`);

        if (discsA === i + 1 && discsB === i + 1) {
            console.log(`✅ SYNCHRONIZED! (${i + 1} discs on both boards)`);
            if (isWinning) {
                console.log('\n🎉 MOVE 7 SUCCEEDED! Fix is working!');
            }
        } else {
            console.log(`⚠️  SYNC ISSUE - Expected ${i + 1} discs!`);
            if (isWinning) {
                console.log(`\n❌ MOVE 7 FAILED - Still only ${discsA} discs`);
            }
        }
    }

    // VICTORY VERIFICATION
    console.log('\n' + '='.repeat(70));
    console.log('VICTORY SCREEN VERIFICATION');
    console.log('='.repeat(70) + '\n');

    console.log('⏳ Waiting 5 more seconds for victory screen...');
    await new Promise(r => setTimeout(r, 5000));

    const victoryA = await pageA.evaluate(() => {
        const overlay = document.getElementById('victoryOverlay');
        return overlay && overlay.classList.contains('active');
    });

    const victoryB = await pageB.evaluate(() => {
        const overlay = document.getElementById('victoryOverlay');
        return overlay && overlay.classList.contains('active');
    });

    console.log('\n🏆 RESULTS:');
    console.log('━'.repeat(70));
    console.log(`Victory Screen Player A: ${victoryA ? '✅ SHOWN' : '❌ HIDDEN'}`);
    console.log(`Victory Screen Player B: ${victoryB ? '✅ SHOWN' : '❌ HIDDEN'}`);
    console.log('━'.repeat(70));

    if (victoryA || victoryB) {
        console.log('\n🎉 SUCCESS! Move 7 fix WORKED! Victory screen triggered!');
    } else {
        console.log('\n❌ FAILED: Victory screen still not showing');
    }

    await pageA.screenshot({ path: 'move7-final-test-red.png', fullPage: true });
    await pageB.screenshot({ path: 'move7-final-test-yellow.png', fullPage: true });
    console.log('\n📸 Screenshots saved\n');

    console.log('🔍 Keeping browsers open for 30 seconds...');
    await new Promise(r => setTimeout(r, 30000));

    await browser.close();
    console.log('\n✅ Test complete!\n');
})();
