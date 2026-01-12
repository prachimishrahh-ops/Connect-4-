const playwright = require('playwright');

(async () => {
    const browser = await playwright.chromium.launch({ headless: false });
    const context = await browser.newContext();

    console.log('\n🐌 SLOW & DELIBERATE VICTORY TEST');
    console.log('='.repeat(70));

    const pageA = await context.newPage();
    const pageB = await context.newPage();

    // Capture console logs
    pageA.on('console', msg => console.log(`[Player A] ${msg.text()}`));
    pageB.on('console', msg => console.log(`[Player B] ${msg.text()}`));

    await pageA.goto('http://localhost:5173');
    await pageB.goto('http://localhost:5174');

    console.log('✅ Both browsers loaded\n');

    // MATCHMAKING
    console.log('📋 MATCHMAKING PHASE');
    console.log('-'.repeat(70));

    await pageA.fill('#playerName', 'SlowRed');
    await new Promise(r => setTimeout(r, 500));
    console.log('👤 Player A entered name');

    await pageA.click('button:has-text("PLAY NOW")');
    console.log('🎮 Player A clicked PLAY NOW');

    await new Promise(r => setTimeout(r, 3000));

    await pageB.fill('#playerName', 'SlowYellow');
    await new Promise(r => setTimeout(r, 500));
    console.log('👤 Player B entered name');

    await pageB.click('button:has-text("PLAY NOW")');
    console.log('🎮 Player B clicked PLAY NOW');

    console.log('⏳ Waiting 8 seconds for match...\n');
    await new Promise(r => setTimeout(r, 8000));

    // GAMEPLAY
    console.log('📋 GAMEPLAY PHASE');
    console.log('-'.repeat(70));

    const moves = [
        { page: pageA, col: 3, player: 'Red (A)' },
        { page: pageB, col: 4, player: 'Yellow (B)' },
        { page: pageA, col: 3, player: 'Red (A)' },
        { page: pageB, col: 4, player: 'Yellow (B)' },
        { page: pageA, col: 3, player: 'Red (A)' },
        { page: pageB, col: 4, player: 'Yellow (B)' },
        { page: pageA, col: 3, player: 'Red (A) WINNING' }
    ];

    for (let i = 0; i < moves.length; i++) {
        const move = moves[i];
        const isWinning = i === 6;

        console.log(`\n${'='.repeat(70)}`);
        console.log(`MOVE ${i + 1}/7: ${move.player} → Column ${move.col + 1}`);
        console.log('='.repeat(70));

        // Get current state before move
        const stateBefore = await move.page.evaluate(() => {
            return {
                gameState: currentGameState,
                myColor,
                isMyTurn: currentGameState && myColor ? currentGameState.currentTurn === myColor : false
            };
        });

        console.log('📊 State before move:');
        console.log(`   Game Status: ${stateBefore.gameState?.status || 'NO GAME'}`);
        console.log(`   Current Turn: ${stateBefore.gameState?.currentTurn || 'N/A'}`);
        console.log(`   My Color: ${stateBefore.myColor || 'N/A'}`);
        console.log(`   Is My Turn: ${stateBefore.isMyTurn ? '✅ YES' : '❌ NO'}`);
        console.log(`   Move Count: ${stateBefore.gameState?.moveCount || 0}`);

        if (!stateBefore.isMyTurn && stateBefore.gameState) {
            console.log('⚠️  WARNING: Trying to move when it is not our turn!');
            console.log('   Refreshing state first...');
            await move.page.evaluate(() => refreshGameState());
            await new Promise(r => setTimeout(r, 2000));
        }

        console.log(`\n🎯 Executing makeMove(${move.col})...`);
        await move.page.evaluate((col) => {
            makeMove(col);
        }, move.col);

        console.log(`⏳ Waiting ${isWinning ? 10 : 5} seconds for processing...\n`);
        await new Promise(r => setTimeout(r, isWinning ? 10000 : 5000));

        // Get state after move
        const stateAfter = await move.page.evaluate(() => {
            const cells = document.querySelectorAll('.cell');
            let red = 0, yellow = 0;
            cells.forEach(cell => {
                if (cell.classList.contains('red')) red++;
                if (cell.classList.contains('yellow')) yellow++;
            });
            return {
                redDiscs: red,
                yellowDiscs: yellow,
                total: red + yellow,
                moveCount: currentGameState?.moveCount || 0,
                status: currentGameState?.status || 'UNKNOWN'
            };
        });

        console.log('📊 State after move:');
        console.log(`   Board: ${stateAfter.redDiscs} red, ${stateAfter.yellowDiscs} yellow (${stateAfter.total} total)`);
        console.log(`   Move Count: ${stateAfter.moveCount}`);
        console.log(`   Status: ${stateAfter.status}`);

        if (stateAfter.total !== i + 1) {
            console.log(`\n❌ ERROR: Expected ${i + 1} discs, but got ${stateAfter.total}!`);
            console.log('   Move may have failed!\n');
        } else {
            console.log('   ✅ Move successful!\n');
        }
    }

    // VICTORY CHECK
    console.log('\n' + '='.repeat(70));
    console.log('VICTORY VERIFICATION');
    console.log('='.repeat(70));

    console.log('⏳ Waiting 5 more seconds for victory screen...');
    await new Promise(r => setTimeout(r, 5000));

    const victoryA = await pageA.evaluate(() => {
        const victoryScreen = document.getElementById('victoryScreen');
        return victoryScreen && !victoryScreen.classList.contains('hidden');
    });

    const victoryB = await pageB.evaluate(() => {
        const victoryScreen = document.getElementById('victoryScreen');
        return victoryScreen && !victoryScreen.classList.contains('hidden');
    });

    console.log(`\n🏆 Victory Screen Player A: ${victoryA ? '✅ SHOWN' : '❌ HIDDEN'}`);
    console.log(`🏆 Victory Screen Player B: ${victoryB ? '✅ SHOWN' : '❌ HIDDEN'}`);

    await pageA.screenshot({ path: 'slow_victory_a.png', fullPage: true });
    await pageB.screenshot({ path: 'slow_victory_b.png', fullPage: true });
    console.log('\n📸 Screenshots saved');

    console.log('\n🔍 Keeping browsers open for 30 seconds...');
    await new Promise(r => setTimeout(r, 30000));

    await browser.close();
})();
