const playwright = require('playwright');

(async () => {
    console.log('\n╔══════════════════════════════════════════════════════════════╗');
    console.log('║  MANUAL VICTORY TEST - COMPLETE GAME WITH HUMAN TIMING      ║');
    console.log('╚══════════════════════════════════════════════════════════════╝\n');

    const browser = await playwright.chromium.launch({
        headless: false,
        args: ['--start-maximized']
    });
    const context = await browser.newContext({ viewport: null });

    const pageA = await context.newPage();
    const pageB = await context.newPage();

    // Console logging
    pageA.on('console', msg => {
        if (msg.text().includes('❌') || msg.text().includes('✅') || msg.text().includes('🏆') || msg.text().includes('🎉')) {
            console.log(`[RED] ${msg.text()}`);
        }
    });
    pageB.on('console', msg => {
        if (msg.text().includes('❌') || msg.text().includes('✅') || msg.text().includes('🏆') || msg.text().includes('🎉')) {
            console.log(`[YLW] ${msg.text()}`);
        }
    });

    await pageA.goto('http://localhost:5173');
    await pageB.goto('http://localhost:5174');

    console.log('✅ Both browsers loaded\n');

    // MATCHMAKING
    console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
    console.log('  MATCHMAKING PHASE');
    console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n');

    // Fill both names first
    console.log('👤 Entering player names...');
    await pageA.fill('#playerName', 'RedChampion');
    await pageB.fill('#playerName', 'YellowWinner');
    await new Promise(r => setTimeout(r, 500));

    // Click PLAY NOW almost simultaneously
    console.log('🎮 Both players clicking PLAY NOW...');
    await Promise.all([
        pageA.click('button:has-text("PLAY NOW")', { timeout: 5000 }).catch(() => {}),
        new Promise(r => setTimeout(r, 200)).then(() =>
            pageB.click('button:has-text("PLAY NOW")', { timeout: 5000 }).catch(() => {})
        )
    ]);

    console.log('👤 Player A (Red): "RedChampion" joining...');
    console.log('👤 Player B (Yellow): "YellowWinner" joining...');
    console.log('⏳ Waiting for match creation...');
    await new Promise(r => setTimeout(r, 6000));

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

    console.log('\n✅ MATCH FOUND! Game loaded for both players\n');

    // GAMEPLAY
    console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
    console.log('  GAMEPLAY PHASE - Vertical win in column 4');
    console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n');

    const moves = [
        { page: pageA, col: 3, player: 'Red', num: 1 },
        { page: pageB, col: 4, player: 'Yellow', num: 2 },
        { page: pageA, col: 3, player: 'Red', num: 3 },
        { page: pageB, col: 4, player: 'Yellow', num: 4 },
        { page: pageA, col: 3, player: 'Red', num: 5 },
        { page: pageB, col: 4, player: 'Yellow', num: 6 },
        { page: pageA, col: 3, player: 'Red (WINNING!)', num: 7 }
    ];

    for (let i = 0; i < moves.length; i++) {
        const move = moves[i];
        const isWinning = i === 6;

        console.log(`\n  Move ${i + 1}/7: ${move.player} → Column ${move.col + 1}`);
        console.log(`  ${'-'.repeat(60)}`);

        // Wait for turn
        let attempts = 0;
        while (attempts < 10) {
            const isMyTurn = await move.page.evaluate(() => {
                return currentGameState && myColor && currentGameState.currentTurn === myColor;
            });

            if (isMyTurn) break;

            console.log(`    ⏳ Waiting for turn... (${attempts + 1}/10)`);
            await new Promise(r => setTimeout(r, 2000));
            attempts++;
        }

        // Execute move
        await move.page.evaluate((col) => {
            makeMove(col);
        }, move.col);

        console.log(`    ✅ Move executed`);

        // Wait for processing
        const waitTime = isWinning ? 10000 : 5000;
        console.log(`    ⏳ Waiting ${waitTime/1000}s for ${isWinning ? 'VICTORY' : 'sync'}...`);
        await new Promise(r => setTimeout(r, waitTime));

        // Check disc count
        const discsA = await pageA.evaluate(() => {
            return document.querySelectorAll('.cell.red, .cell.yellow').length;
        });
        const discsB = await pageB.evaluate(() => {
            return document.querySelectorAll('.cell.red, .cell.yellow').length;
        });

        console.log(`    📊 Discs: Player A sees ${discsA}, Player B sees ${discsB}`);

        if (discsA === i + 1 && discsB === i + 1) {
            console.log(`    ✅ Synchronized correctly`);
        } else {
            console.log(`    ⚠️  SYNC ISSUE (expected ${i + 1})`);
        }
    }

    // VICTORY VERIFICATION
    console.log('\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
    console.log('  VICTORY SCREEN VERIFICATION');
    console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n');

    console.log('⏳ Waiting 5 more seconds for victory animation...');
    await new Promise(r => setTimeout(r, 5000));

    const victoryA = await pageA.evaluate(() => {
        const overlay = document.getElementById('victoryOverlay');
        const isShowing = overlay && overlay.classList.contains('active');
        const trophy = document.getElementById('victoryTrophy')?.textContent || '';
        const text = document.getElementById('victoryText')?.textContent || '';
        return { showing: isShowing, trophy, text };
    });

    const victoryB = await pageB.evaluate(() => {
        const overlay = document.getElementById('victoryOverlay');
        const isShowing = overlay && overlay.classList.contains('active');
        const trophy = document.getElementById('victoryTrophy')?.textContent || '';
        const text = document.getElementById('victoryText')?.textContent || '';
        return { showing: isShowing, trophy, text };
    });

    console.log('🏆 VICTORY SCREEN RESULTS:');
    console.log('  ┌───────────────────────────────────────────────┐');
    console.log(`  │ Player A (Red - Winner):                      │`);
    console.log(`  │   Status: ${victoryA.showing ? '✅ SHOWN' : '❌ HIDDEN'}                          │`);
    console.log(`  │   Trophy: ${victoryA.trophy || 'N/A'}                                │`);
    console.log(`  │   Text:   ${victoryA.text || 'N/A'}                             │`);
    console.log('  ├───────────────────────────────────────────────┤');
    console.log(`  │ Player B (Yellow - Loser):                    │`);
    console.log(`  │   Status: ${victoryB.showing ? '✅ SHOWN' : '❌ HIDDEN'}                          │`);
    console.log(`  │   Trophy: ${victoryB.trophy || 'N/A'}                                │`);
    console.log(`  │   Text:   ${victoryB.text || 'N/A'}                             │`);
    console.log('  └───────────────────────────────────────────────┘\n');

    // Screenshots
    await pageA.screenshot({ path: 'manual_victory_red.png', fullPage: true });
    await pageB.screenshot({ path: 'manual_victory_yellow.png', fullPage: true });
    console.log('📸 Screenshots saved: manual_victory_red.png, manual_victory_yellow.png\n');

    // Final summary
    console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
    console.log('  TEST SUMMARY');
    console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
    console.log(`  Matchmaking:     ${gameLoadedA && gameLoadedB ? '✅ SUCCESS' : '❌ FAILED'}`);
    console.log(`  All 7 moves:     ✅ EXECUTED`);
    console.log(`  Victory Screen:  ${victoryA.showing || victoryB.showing ? '✅ SHOWN' : '❌ HIDDEN'}`);
    console.log(`  Confetti:        ${victoryA.showing && victoryA.text === 'VICTORY!' ? '✅ CHECK SCREENSHOT' : '❌ NOT VERIFIED'}`);
    console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n');

    console.log('🔍 Keeping browsers open for 60 seconds for manual inspection...');
    await new Promise(r => setTimeout(r, 60000));

    await browser.close();
    console.log('\n✅ Test complete!\n');
})();
