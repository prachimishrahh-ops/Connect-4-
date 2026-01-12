const playwright = require('playwright');

(async () => {
    const browser = await playwright.chromium.launch({ headless: false });
    const context = await browser.newContext();

    console.log('\n🎮 COMPREHENSIVE MULTIPLAYER + VICTORY TEST');
    console.log('='.repeat(60));

    // Open both players
    const pageA = await context.newPage();
    const pageB = await context.newPage();

    await pageA.goto('http://localhost:5173');
    await pageB.goto('http://localhost:5174');

    console.log('✅ Both browsers loaded');

    // Wait for page load
    await new Promise(r => setTimeout(r, 2000));

    // PHASE 1: MATCHMAKING
    console.log('\n📋 PHASE 1: MATCHMAKING');
    console.log('-'.repeat(60));

    // Player A enters name and starts matchmaking
    console.log('👤 Player A: Entering name "RedChampion"...');
    await pageA.fill('#playerName', 'RedChampion');
    await new Promise(r => setTimeout(r, 500));

    console.log('🎮 Player A: Clicking PLAY NOW...');
    await pageA.click('button:has-text("PLAY NOW")');

    console.log('⏳ Waiting 2 seconds...');
    await new Promise(r => setTimeout(r, 2000));

    // Player B enters name and starts matchmaking
    console.log('👤 Player B: Entering name "YellowWinner"...');
    await pageB.fill('#playerName', 'YellowWinner');
    await new Promise(r => setTimeout(r, 500));

    console.log('🎮 Player B: Clicking PLAY NOW...');
    await pageB.click('button:has-text("PLAY NOW")');

    console.log('⏳ Waiting 5 seconds for match creation...');
    await new Promise(r => setTimeout(r, 5000));

    // Check if game screen loaded
    const gameLoadedA = await pageA.evaluate(() => {
        return !document.getElementById('matchmakingStatus') ||
               document.getElementById('matchmakingStatus').classList.contains('hidden');
    });

    const gameLoadedB = await pageB.evaluate(() => {
        return !document.getElementById('matchmakingStatus') ||
               document.getElementById('matchmakingStatus').classList.contains('hidden');
    });

    console.log('📊 Match Status:');
    console.log('  Player A game loaded:', gameLoadedA ? '✅ YES' : '❌ NO');
    console.log('  Player B game loaded:', gameLoadedB ? '✅ YES' : '❌ NO');

    if (!gameLoadedA || !gameLoadedB) {
        console.log('\n❌ ERROR: Game did not load for both players');
        await pageA.screenshot({ path: 'error_player_a.png' });
        await pageB.screenshot({ path: 'error_player_b.png' });
        await browser.close();
        return;
    }

    // PHASE 2: PLAY TO VICTORY
    console.log('\n📋 PHASE 2: PLAYING TO VICTORY');
    console.log('-'.repeat(60));
    console.log('🎯 Strategy: Red wins with vertical in column 3');
    console.log('📝 Move sequence: [3,4,3,4,3,4,3]');

    const moves = [
        { player: 'A', page: pageA, col: 3, name: 'Red' },
        { player: 'B', page: pageB, col: 4, name: 'Yellow' },
        { player: 'A', page: pageA, col: 3, name: 'Red' },
        { player: 'B', page: pageB, col: 4, name: 'Yellow' },
        { player: 'A', page: pageA, col: 3, name: 'Red' },
        { player: 'B', page: pageB, col: 4, name: 'Yellow' },
        { player: 'A', page: pageA, col: 3, name: 'Red (WINNING)' }
    ];

    for (let i = 0; i < moves.length; i++) {
        const move = moves[i];
        const isWinningMove = i === 6;

        console.log(`\n--- Move ${i + 1}/7: ${move.name} plays column ${move.col} ${isWinningMove ? '🏆' : ''} ---`);

        // Execute move
        await move.page.evaluate((colIdx) => {
            makeMove(colIdx);
        }, move.col);

        console.log(`✅ Move executed: Column ${move.col}`);

        // Wait for move to process and state to update
        const waitTime = isWinningMove ? 8000 : 3500;
        console.log(`⏳ Waiting ${waitTime/1000}s for ${isWinningMove ? 'victory animation' : 'move to process'}...`);
        await new Promise(r => setTimeout(r, waitTime));

        // Check board state on both players
        const stateA = await pageA.evaluate(() => {
            const cells = document.querySelectorAll('.cell');
            let red = 0, yellow = 0;
            cells.forEach(cell => {
                if (cell.classList.contains('red')) red++;
                if (cell.classList.contains('yellow')) yellow++;
            });
            const movesDisplay = document.querySelector('.stat-item:nth-child(1) .stat-value')?.textContent || '0';
            return { red, yellow, total: red + yellow, moves: parseInt(movesDisplay) };
        });

        const stateB = await pageB.evaluate(() => {
            const cells = document.querySelectorAll('.cell');
            let red = 0, yellow = 0;
            cells.forEach(cell => {
                if (cell.classList.contains('red')) red++;
                if (cell.classList.contains('yellow')) yellow++;
            });
            const movesDisplay = document.querySelector('.stat-item:nth-child(1) .stat-value')?.textContent || '0';
            return { red, yellow, total: red + yellow, moves: parseInt(movesDisplay) };
        });

        console.log(`📊 Board State:`);
        console.log(`  Player A: ${stateA.red} red, ${stateA.yellow} yellow (${stateA.total} total) | Moves: ${stateA.moves}`);
        console.log(`  Player B: ${stateB.red} red, ${stateB.yellow} yellow (${stateB.total} total) | Moves: ${stateB.moves}`);

        if (stateA.total === stateB.total && stateA.red === stateB.red && stateA.yellow === stateB.yellow) {
            console.log('✅ Board synchronized correctly');
        } else {
            console.log('⚠️  SYNC ISSUE DETECTED!');
        }
    }

    // PHASE 3: VERIFY VICTORY SCREEN
    console.log('\n📋 PHASE 3: VICTORY SCREEN VERIFICATION');
    console.log('-'.repeat(60));

    console.log('⏳ Waiting additional 3 seconds for victory animations...');
    await new Promise(r => setTimeout(r, 3000));

    // Check victory screen on both players
    const victoryA = await pageA.evaluate(() => {
        const victoryScreen = document.getElementById('victoryScreen');
        const isShowing = victoryScreen && !victoryScreen.classList.contains('hidden');
        const winnerText = document.getElementById('victoryWinner')?.textContent || 'N/A';
        const confetti = document.querySelector('.confetti');
        const hasConfetti = confetti !== null;
        return { showing: isShowing, winner: winnerText, confetti: hasConfetti };
    });

    const victoryB = await pageB.evaluate(() => {
        const victoryScreen = document.getElementById('victoryScreen');
        const isShowing = victoryScreen && !victoryScreen.classList.contains('hidden');
        const winnerText = document.getElementById('victoryWinner')?.textContent || 'N/A';
        const confetti = document.querySelector('.confetti');
        const hasConfetti = confetti !== null;
        return { showing: isShowing, winner: winnerText, confetti: hasConfetti };
    });

    console.log('\n🏆 VICTORY SCREEN STATUS:');
    console.log('  Player A:');
    console.log(`    Showing: ${victoryA.showing ? '✅ YES' : '❌ NO'}`);
    console.log(`    Winner Text: ${victoryA.winner}`);
    console.log(`    Confetti: ${victoryA.confetti ? '✅ YES' : '❌ NO'}`);
    console.log('  Player B:');
    console.log(`    Showing: ${victoryB.showing ? '✅ YES' : '❌ NO'}`);
    console.log(`    Winner Text: ${victoryB.winner}`);
    console.log(`    Confetti: ${victoryB.confetti ? '✅ YES' : '❌ NO'}`);

    // Take victory screenshots
    await pageA.screenshot({ path: 'victory_player_a.png', fullPage: true });
    await pageB.screenshot({ path: 'victory_player_b.png', fullPage: true });
    console.log('\n📸 Victory screenshots saved');

    // PHASE 4: UX ANALYSIS
    console.log('\n📋 PHASE 4: UX ISSUES DETECTION');
    console.log('-'.repeat(60));

    const issuesFound = [];

    if (!victoryA.showing || !victoryB.showing) {
        issuesFound.push('❌ Victory screen not showing on one or both players');
    }

    if (victoryA.showing && !victoryA.confetti) {
        issuesFound.push('⚠️  Victory confetti missing on Player A');
    }

    if (victoryB.showing && !victoryB.confetti) {
        issuesFound.push('⚠️  Victory confetti missing on Player B');
    }

    if (victoryA.winner === 'N/A' || victoryB.winner === 'N/A') {
        issuesFound.push('⚠️  Winner name not displaying correctly');
    }

    // Check for console errors
    const errorsA = await pageA.evaluate(() => {
        return window.consoleErrors || [];
    });

    const errorsB = await pageB.evaluate(() => {
        return window.consoleErrors || [];
    });

    if (errorsA.length > 0) {
        issuesFound.push(`⚠️  ${errorsA.length} console errors on Player A`);
    }

    if (errorsB.length > 0) {
        issuesFound.push(`⚠️  ${errorsB.length} console errors on Player B`);
    }

    console.log('\n📊 UX ISSUES FOUND:', issuesFound.length);
    if (issuesFound.length > 0) {
        issuesFound.forEach(issue => console.log(`  ${issue}`));
    } else {
        console.log('  ✅ NO ISSUES DETECTED - PERFECT EXECUTION!');
    }

    // FINAL SUMMARY
    console.log('\n' + '='.repeat(60));
    console.log('📊 TEST SUMMARY');
    console.log('='.repeat(60));
    console.log(`Matchmaking: ${gameLoadedA && gameLoadedB ? '✅ SUCCESS' : '❌ FAILED'}`);
    console.log(`Moves Executed: 7/7 ✅`);
    console.log(`Victory Screen: ${victoryA.showing || victoryB.showing ? '✅ SHOWN' : '❌ NOT SHOWN'}`);
    console.log(`Victory Confetti: ${victoryA.confetti || victoryB.confetti ? '✅ ANIMATED' : '❌ MISSING'}`);
    console.log(`Issues Found: ${issuesFound.length}`);
    console.log('='.repeat(60));

    console.log('\n🔍 Keeping browsers open for 30 seconds for manual inspection...');
    await new Promise(r => setTimeout(r, 30000));

    await browser.close();
    console.log('\n👋 Test complete!');
})();
