const { chromium } = require('playwright');

(async () => {
  const browser = await chromium.launch({ headless: false });

  console.log('🎮 Complete multiplayer game test\n');

  const contextA = await browser.newContext();
  const contextB = await browser.newContext();

  const pageA = await contextA.newPage();
  const pageB = await contextB.newPage();

  try {
    // Navigate
    console.log('📍 Navigating to frontends...');
    await pageA.goto('http://localhost:5173');
    await pageB.goto('http://localhost:5174');
    await new Promise(r => setTimeout(r, 3000));

    // Create profiles
    console.log('🔴 Red: Creating profile...');
    await pageA.fill('input#playerName', 'RedPlayer');
    await pageA.click('button:has-text("Create Profile")');
    await new Promise(r => setTimeout(r, 2000));

    console.log('🟡 Yellow: Creating profile...');
    await pageB.fill('input#playerName', 'YellowPlayer');
    await pageB.click('button:has-text("Create Profile")');
    await new Promise(r => setTimeout(r, 2000));

    // Start matchmaking
    console.log('\n🔍 Starting matchmaking...');
    await pageA.click('button:has-text("Find Match")');
    await pageB.click('button:has-text("Find Match")');

    // Wait for match with better detection
    console.log('⏳ Waiting for match (up to 30 seconds)...');
    let matched = false;
    for (let i = 0; i < 30; i++) {
      await new Promise(r => setTimeout(r, 1000));

      // Check for game board visibility
      const boardA = await pageA.$('#gameBoard');
      const boardB = await pageB.$('#gameBoard');

      if (boardA && boardB) {
        const visibleA = await boardA.isVisible();
        const visibleB = await boardB.isVisible();
        if (visibleA || visibleB) {
          matched = true;
          console.log(`✅ Match found after ${i + 1} seconds!`);
          break;
        }
      }
    }

    if (!matched) {
      console.log('❌ Matchmaking failed after 30 seconds');
      await browser.close();
      return;
    }

    // Wait for game to fully load
    await new Promise(r => setTimeout(r, 3000));

    console.log('\n🎲 Starting gameplay...\n');

    // Play the game
    let gameOver = false;
    let moveNum = 0;
    const maxMoves = 42;

    while (!gameOver && moveNum < maxMoves) {
      await new Promise(r => setTimeout(r, 2000));

      // Check for game end
      const bodyA = await pageA.textContent('body');
      const bodyB = await pageB.textContent('body');

      if (bodyA.includes('VICTORY') || bodyA.includes('DEFEAT') || bodyA.includes('DRAW')) {
        console.log('\n✅ Game ended naturally!');
        gameOver = true;
        break;
      }

      // Check whose turn
      const turnBannerA = await pageA.textContent('#turnBanner').catch(() => '');
      const turnBannerB = await pageB.textContent('#turnBanner').catch(() => '');

      if (turnBannerA.includes('YOUR TURN')) {
        // Red's turn - play in next available column
        const col = moveNum % 7;
        console.log(`🔴 Red move ${moveNum + 1}: Column ${col + 1}`);

        const columns = await pageA.$$('.column');
        if (columns[col]) {
          await columns[col].click();
          moveNum++;
        }
      } else if (turnBannerB.includes('YOUR TURN')) {
        // Yellow's turn
        const col = (6 - (moveNum % 7));
        console.log(`🟡 Yellow move ${moveNum + 1}: Column ${col + 1}`);

        const columns = await pageB.$$('.column');
        if (columns[col]) {
          await columns[col].click();
          moveNum++;
        }
      } else {
        console.log(`   ⏳ Waiting for turn indicator... (move ${moveNum})`);
      }
    }

    // Final results
    console.log(`\n📊 Game Summary:`);
    console.log(`   Total moves: ${moveNum}`);

    await new Promise(r => setTimeout(r, 2000));

    const finalA = await pageA.textContent('body');
    const finalB = await pageB.textContent('body');

    if (finalA.includes('VICTORY')) {
      console.log('   🏆 RED WINS!');
    } else if (finalB.includes('VICTORY')) {
      console.log('   🏆 YELLOW WINS!');
    } else if (finalA.includes('DRAW')) {
      console.log('   🤝 DRAW!');
    } else {
      console.log('   ❓ Game status unclear');
    }

    // Screenshots
    await pageA.screenshot({ path: 'final-red.png', fullPage: true });
    await pageB.screenshot({ path: 'final-yellow.png', fullPage: true });
    console.log('\n📸 Screenshots saved: final-red.png, final-yellow.png');

    console.log('\n⏸️  Keeping browsers open for inspection (20 seconds)...');
    await new Promise(r => setTimeout(r, 20000));

  } catch (error) {
    console.error('❌ Test error:', error.message);
  } finally {
    await browser.close();
    console.log('\n✅ Test complete');
  }
})();
