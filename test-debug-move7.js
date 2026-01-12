const playwright = require('playwright');

(async () => {
    const browser = await playwright.chromium.launch({ headless: false });
    const context = await browser.newContext();

    const pageA = await context.newPage();

    // Capture all console messages
    const consoleLogs = [];
    pageA.on('console', msg => {
        const text = msg.text();
        console.log(`[BROWSER] ${text}`);
        consoleLogs.push(text);
    });

    await pageA.goto('http://localhost:5173');

    console.log('✅ Browser loaded');
    console.log('⏳ Waiting 5 seconds for any existing game...');
    await new Promise(r => setTimeout(r, 5000));

    // Try to make a move and see what happens
    console.log('\n🎯 Attempting to execute makeMove(3) directly...');

    const result = await pageA.evaluate(() => {
        console.log('='.repeat(60));
        console.log('CURRENT GAME STATE:');
        console.log('currentGameState:', currentGameState);
        console.log('myColor:', myColor);
        console.log('isMyTurn():', currentGameState && myColor ? currentGameState.currentTurn === myColor : false);
        console.log('='.repeat(60));

        if (currentGameState) {
            makeMove(3);
            return {
                hadState: true,
                status: currentGameState.status,
                currentTurn: currentGameState.currentTurn,
                myColor: myColor,
                moveCount: currentGameState.moveCount
            };
        }
        return { hadState: false };
    });

    console.log('\n📊 Game state before move attempt:');
    console.log(JSON.stringify(result, null, 2));

    console.log('\n⏳ Waiting 5 seconds to see console logs...');
    await new Promise(r => setTimeout(r, 5000));

    console.log('\n📝 All console logs captured:');
    console.log(consoleLogs.join('\n'));

    console.log('\n🔍 Keeping browser open for 20 seconds...');
    await new Promise(r => setTimeout(r, 20000));

    await browser.close();
})();
