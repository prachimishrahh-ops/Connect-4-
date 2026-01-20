const { chromium } = require('playwright');

(async () => {
  const browser = await chromium.launch({ headless: false });

  console.log('🎮 Starting full game playthrough test...\n');

  const contextA = await browser.newContext();
  const contextB = await browser.newContext();

  const pageA = await contextA.newPage();
  const pageB = await contextB.newPage();

  // Navigate
  await pageA.goto('http://localhost:5173');
  await pageB.goto('http://localhost:5174');
  await new Promise(r => setTimeout(r, 2000));

  // Create profiles
  console.log('🔴 Player A: Creating profile...');
  await pageA.fill('input#playerName', 'RedChampion');
  await pageA.click('button:has-text("PLAY NOW")');
  await new Promise(r => setTimeout(r, 2000));

  console.log('🟡 Player B: Creating profile...');
  await pageB.fill('input#playerName', 'YellowWarrior');
  await pageB.click('button:has-text("PLAY NOW")');
  await new Promise(r => setTimeout(r, 2000));

  // Find match
  console.log('🔴 Player A: Finding match...');
  await pageA.click('button:has-text("PLAY NOW")');

  console.log('🟡 Player B: Finding match...');
  await pageB.click('button:has-text("PLAY NOW")');

  console.log('⏳ Waiting for match...');
  await new Promise(r => setTimeout(r, 5000));

  // Check if game started
  const contentA = await pageA.content();
  if (!contentA.includes('YOUR TURN') && !contentA.includes("OPPONENT'S TURN")) {
    console.log('❌ Game did not start - matchmaking failed');
    await browser.close();
    process.exit(1);
  }

  console.log('✅ Game started!\n');
  console.log('🎲 Playing Connect 4 until someone wins...\n');

  // Game loop
  let gameOver = false;
  let moveCount = 0;
  const maxMoves = 42; // 6 rows * 7 columns = max possible moves

  // Strategy: Red plays column 0-6 in sequence, Yellow plays 6-0 in sequence
  let redColumn = 0;
  let yellowColumn = 6;

  while (!gameOver && moveCount < maxMoves) {
    await new Promise(r => setTimeout(r, 2000)); // Wait between moves

    // Check current turn
    const bodyA = await pageA.textContent('body');
    const bodyB = await pageB.textContent('body');

    if (bodyA.includes('YOU WIN') || bodyA.includes('YOU LOSE') || bodyA.includes('DRAW')) {
      gameOver = true;
      break;
    }

    // Determine whose turn it is
    if (bodyA.includes('YOUR TURN')) {
      // Red's turn
      const col = redColumn;
      console.log(`🔴 Red move #${moveCount + 1}: Column ${col + 1}`);

      // Click column on game board
      const columns = await pageA.$$('.column');
      if (columns[col]) {
        await columns[col].click();
        moveCount++;
        redColumn = (redColumn + 1) % 7; // Next column in sequence
      } else {
        console.log('   ❌ Column not found, trying backup click');
        await pageA.click(`#gameBoard`);
      }
    } else if (bodyB.includes('YOUR TURN')) {
      // Yellow's turn
      const col = yellowColumn;
      console.log(`🟡 Yellow move #${moveCount + 1}: Column ${col + 1}`);

      const columns = await pageB.$$('.column');
      if (columns[col]) {
        await columns[col].click();
        moveCount++;
        yellowColumn = yellowColumn - 1 < 0 ? 6 : yellowColumn - 1; // Reverse sequence
      } else {
        console.log('   ❌ Column not found, trying backup click');
        await pageB.click(`#gameBoard`);
      }
    } else {
      console.log('   ⏳ Waiting for turn...');
    }

    // Check for game end every few moves
    if (moveCount % 3 === 0) {
      const statusA = await pageA.textContent('body');
      if (statusA.includes('VICTORY') || statusA.includes('DEFEAT') || statusA.includes('DRAW')) {
        gameOver = true;
      }
    }
  }

  console.log('\n📊 Game Result:');
  await new Promise(r => setTimeout(r, 2000));

  const finalA = await pageA.textContent('body');
  const finalB = await pageB.textContent('body');

  if (finalA.includes('YOU WIN') || finalA.includes('VICTORY')) {
    console.log('✅ RED WINS!');
  } else if (finalB.includes('YOU WIN') || finalB.includes('VICTORY')) {
    console.log('✅ YELLOW WINS!');
  } else if (finalA.includes('DRAW')) {
    console.log('✅ GAME ENDED IN DRAW');
  } else if (finalA.includes('YOU LOSE')) {
    console.log('✅ YELLOW WINS (Red lost)');
  } else if (finalB.includes('YOU LOSE')) {
    console.log('✅ RED WINS (Yellow lost)');
  } else {
    console.log('⚠️  Game status unclear');
    console.log('Player A sees:', finalA.substring(0, 200));
    console.log('Player B sees:', finalB.substring(0, 200));
  }

  console.log(`\n📈 Total moves: ${moveCount}`);

  // Take final screenshots
  await pageA.screenshot({ path: 'game-end-red.png', fullPage: true });
  await pageB.screenshot({ path: 'game-end-yellow.png', fullPage: true });
  console.log('📸 Screenshots saved: game-end-red.png, game-end-yellow.png');

  console.log('\n⏸️  Keeping browsers open for 30 seconds to view results...');
  await new Promise(r => setTimeout(r, 30000));

  await browser.close();
  console.log('✅ Full game test complete!');
})();
