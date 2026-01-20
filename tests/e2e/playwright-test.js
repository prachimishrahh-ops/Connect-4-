const { chromium } = require('playwright');

(async () => {
  const browser = await chromium.launch({ headless: false });

  console.log('🎮 Starting multiplayer matchmaking test...\n');

  // Create two browser contexts (two separate players)
  const contextA = await browser.newContext();
  const contextB = await browser.newContext();

  const pageA = await contextA.newPage();
  const pageB = await contextB.newPage();

  // Navigate both players
  console.log('📍 Navigating players to frontends...');
  await pageA.goto('http://localhost:5173');
  await pageB.goto('http://localhost:5174');
  await new Promise(r => setTimeout(r, 2000));

  // Player A: Create profile
  console.log('🔴 Player A: Creating profile...');
  await pageA.fill('input#playerName', 'GamerRed');
  await pageA.click('button:has-text("PLAY NOW")');
  await new Promise(r => setTimeout(r, 3000));

  // Player B: Create profile
  console.log('🟡 Player B: Creating profile...');
  await pageB.fill('input#playerName', 'GamerYellow');
  await pageB.click('button:has-text("PLAY NOW")');
  await new Promise(r => setTimeout(r, 3000));

  // Both players: Find match
  console.log('🔴 Player A: Finding match...');
  await pageA.click('button:has-text("PLAY NOW")');

  console.log('🟡 Player B: Finding match...');
  await pageB.click('button:has-text("PLAY NOW")');

  console.log('⏳ Waiting for matchmaking (up to 60 seconds)...');

  // Wait for game to start - check for game board or match completion
  let matched = false;
  let attempts = 0;
  const maxAttempts = 60; // 60 seconds

  while (!matched && attempts < maxAttempts) {
    attempts++;
    await new Promise(r => setTimeout(r, 1000));

    // Check if game board is visible on either page
    const gameBoardA = await pageA.$('#gameBoard');
    const gameBoardB = await pageB.$('#gameBoard');

    // Check for "Your turn" or game started indicators
    const contentA = await pageA.content();
    const contentB = await pageB.content();

    if ((gameBoardA || contentA.includes('Your turn') || contentA.includes('game started')) ||
        (gameBoardB || contentB.includes('Your turn') || contentB.includes('game started'))) {
      matched = true;
      console.log(`✅ Match found! (${attempts} seconds)`);
      break;
    }

    // Log status every 10 seconds
    if (attempts % 10 === 0) {
      console.log(`  Still waiting... (${attempts}/${maxAttempts} seconds)`);

      // Check current status on both pages
      const statusA = await pageA.textContent('body').catch(() => '');
      const statusB = await pageB.textContent('body').catch(() => '');

      if (statusA.includes('Searching')) console.log('  Player A: Still searching...');
      if (statusB.includes('Searching')) console.log('  Player B: Still searching...');
    }
  }

  if (!matched) {
    console.log('\n❌ MATCHMAKING FAILED - Players did not match after 60 seconds');
    console.log('\n📸 Taking screenshots for debugging...');
    await pageA.screenshot({ path: 'player-a-stuck.png', fullPage: true });
    await pageB.screenshot({ path: 'player-b-stuck.png', fullPage: true });
    console.log('Screenshots saved: player-a-stuck.png, player-b-stuck.png');

    // Get page content for debugging
    console.log('\n🔍 Player A page content:');
    const bodyA = await pageA.textContent('body');
    console.log(bodyA.substring(0, 500));

    console.log('\n🔍 Player B page content:');
    const bodyB = await pageB.textContent('body');
    console.log(bodyB.substring(0, 500));
  } else {
    console.log('\n✅ MATCHMAKING SUCCESS!');
    console.log('🎮 Game started - players matched successfully');

    // Take success screenshots
    await pageA.screenshot({ path: 'player-a-matched.png' });
    await pageB.screenshot({ path: 'player-b-matched.png' });
    console.log('Screenshots saved: player-a-matched.png, player-b-matched.png');

    // Keep browsers open for manual gameplay testing
    console.log('\n⏸️  Browsers will stay open for 60 seconds for manual testing...');
    await new Promise(r => setTimeout(r, 60000));
  }

  await browser.close();
  console.log('\n✅ Test complete');
  process.exit(matched ? 0 : 1);
})();
