const { chromium } = require('playwright');

/**
 * JUDGE CHAOS TEST - Simulates a judge who:
 * 1. Has only 2 minutes
 * 2. Doesn't read docs
 * 3. Clicks randomly
 * 4. Reloads mid-flow
 * 5. Tries to break things
 */

(async () => {
  const browser = await chromium.launch({ headless: false });
  const startTime = Date.now();
  const results = {
    dockerStartup: 'ALREADY RUNNING',
    uiLoads: false,
    canStartWithoutDocs: false,
    pageReloadWorks: false,
    errorMessagesHelpful: true,
    multiplayerWorks: false,
    consoleErrors: [],
    criticalFailures: []
  };

  console.log('='.repeat(60));
  console.log('JUDGE CHAOS TEST - Connect4 Battle');
  console.log('='.repeat(60));
  console.log('Simulating: Judge with 2 minutes, no docs, chaos clicking\n');

  const contextA = await browser.newContext();
  const contextB = await browser.newContext();

  const pageA = await contextA.newPage();
  const pageB = await contextB.newPage();

  // Capture console errors
  pageA.on('console', msg => {
    if (msg.type() === 'error') results.consoleErrors.push(`A: ${msg.text()}`);
  });
  pageB.on('console', msg => {
    if (msg.type() === 'error') results.consoleErrors.push(`B: ${msg.text()}`);
  });

  try {
    // TEST 1: UI Loads without error
    console.log('TEST 1: UI loads without error');
    await pageA.goto('http://localhost:5173', { timeout: 10000 });
    await pageB.goto('http://localhost:5174', { timeout: 10000 });
    await new Promise(r => setTimeout(r, 2000));

    const bodyA = await pageA.textContent('body');
    if (bodyA.includes('Connect') || bodyA.includes('Battle') || bodyA.includes('Player')) {
      results.uiLoads = true;
      console.log('   PASS: UI loaded successfully\n');
    } else {
      results.criticalFailures.push('UI did not load properly');
      console.log('   FAIL: UI did not load\n');
    }

    // TEST 2: Can start game WITHOUT reading docs
    console.log('TEST 2: Can start game without reading docs');

    // Look for obvious buttons
    const buttons = await pageA.$$('button');
    console.log(`   Found ${buttons.length} buttons`);

    // Find input and enter name (Connect4 uses #playerName)
    try {
      await pageA.fill('#playerName', 'ChaosRed');
      console.log('   Entered name: ChaosRed');
    } catch {
      const inputs = await pageA.$$('input');
      if (inputs.length > 0) {
        await inputs[0].fill('ChaosRed');
        console.log('   Entered name: ChaosRed (via input)');
      }
    }

    // Click PLAY NOW button
    try {
      await pageA.click('button:has-text("PLAY NOW")', { timeout: 3000 });
      console.log('   Clicked PLAY NOW');
      await new Promise(r => setTimeout(r, 2000));
      results.canStartWithoutDocs = true;
    } catch {
      results.criticalFailures.push('Could not find obvious PLAY NOW button');
      console.log('   Could not find PLAY NOW button');
    }

    // Same for player B (Yellow)
    try {
      await pageB.fill('#playerName', 'ChaosYellow');
    } catch {
      await pageB.fill('input', 'ChaosYellow').catch(() => {});
    }
    await pageB.click('button:has-text("PLAY NOW")').catch(() => {});
    await new Promise(r => setTimeout(r, 2000));

    // TEST 3: Page reload mid-flow
    console.log('\nTEST 3: Page reload mid-flow');
    await pageA.reload();
    await new Promise(r => setTimeout(r, 3000));

    const bodyAfterReload = await pageA.textContent('body');
    if (bodyAfterReload.includes('Connect') || bodyAfterReload.includes('Battle')) {
      results.pageReloadWorks = true;
      console.log('   PASS: Page still works after reload\n');
    } else {
      results.criticalFailures.push('Page broke after reload');
      console.log('   FAIL: Page broke after reload\n');
    }

    // TEST 4: Matchmaking and multiplayer
    console.log('TEST 4: Multiplayer matchmaking');

    // Re-enter name after reload
    try {
      await pageA.fill('#playerName', 'ChaosRed');
      await pageA.click('button:has-text("PLAY NOW")').catch(() => {});
    } catch {}
    await new Promise(r => setTimeout(r, 2000));

    // Wait for game board to appear
    console.log('   Waiting for match (max 20 seconds)...');
    for (let i = 0; i < 20; i++) {
      await new Promise(r => setTimeout(r, 1000));

      // Check for game board visibility
      const boardA = await pageA.$('#gameBoard');
      const boardB = await pageB.$('#gameBoard');

      if (boardA && boardB) {
        const visibleA = await boardA.isVisible().catch(() => false);
        const visibleB = await boardB.isVisible().catch(() => false);
        if (visibleA || visibleB) {
          results.multiplayerWorks = true;
          console.log(`   PASS: Match found in ${i + 1} seconds!\n`);
          break;
        }
      }

      // Also check for turn indicator
      const bodyCheck = await pageA.textContent('body').catch(() => '');
      if (bodyCheck.includes('YOUR TURN') || bodyCheck.includes('OPPONENT')) {
        results.multiplayerWorks = true;
        console.log(`   PASS: Match found in ${i + 1} seconds!\n`);
        break;
      }
    }

    if (!results.multiplayerWorks) {
      results.criticalFailures.push('Matchmaking failed within 20 seconds');
      console.log('   FAIL: Matchmaking did not work\n');
    }

    // TEST 5: Random clicking (chaos) - try to play a few moves
    console.log('TEST 5: Chaos clicking (trying to break things)');

    // Click random columns
    const columns = await pageA.$$('.column');
    for (let i = 0; i < Math.min(3, columns.length); i++) {
      try {
        await columns[i].click({ timeout: 1000 });
        await new Promise(r => setTimeout(r, 1000));
      } catch {}
    }

    // Check if page is still functional
    const bodyAfterChaos = await pageA.textContent('body');
    if (bodyAfterChaos.length > 100) {
      console.log('   PASS: App survived chaos clicking\n');
    } else {
      results.criticalFailures.push('App crashed during chaos clicking');
      console.log('   FAIL: App crashed\n');
    }

    // Take screenshots
    await pageA.screenshot({ path: 'tests/e2e/chaos_red.png' });
    await pageB.screenshot({ path: 'tests/e2e/chaos_yellow.png' });

  } catch (e) {
    results.criticalFailures.push(`Test error: ${e.message}`);
    console.log(`ERROR: ${e.message}`);
  }

  // Calculate elapsed time
  const elapsed = Math.round((Date.now() - startTime) / 1000);

  // FINAL REPORT
  console.log('\n' + '='.repeat(60));
  console.log('JUDGE CHAOS TEST RESULTS');
  console.log('='.repeat(60));
  console.log(`Time elapsed: ${elapsed} seconds`);
  console.log('');
  console.log(`UI Loads:              ${results.uiLoads ? 'PASS' : 'FAIL'}`);
  console.log(`Start Without Docs:    ${results.canStartWithoutDocs ? 'PASS' : 'FAIL'}`);
  console.log(`Page Reload Works:     ${results.pageReloadWorks ? 'PASS' : 'FAIL'}`);
  console.log(`Multiplayer Works:     ${results.multiplayerWorks ? 'PASS' : 'FAIL'}`);
  console.log(`Console Errors:        ${results.consoleErrors.length === 0 ? 'NONE' : results.consoleErrors.length}`);
  console.log('');

  if (results.criticalFailures.length > 0) {
    console.log('CRITICAL FAILURES:');
    results.criticalFailures.forEach(f => console.log(`  - ${f}`));
  }

  const passCount = [
    results.uiLoads,
    results.canStartWithoutDocs,
    results.pageReloadWorks,
    results.multiplayerWorks,
    results.consoleErrors.length === 0
  ].filter(Boolean).length;

  console.log('');
  console.log(`OVERALL SCORE: ${passCount}/5 tests passed`);
  console.log(passCount >= 4 ? 'VERDICT: JUDGE-PROOF' : 'VERDICT: NEEDS WORK');
  console.log('='.repeat(60));

  await new Promise(r => setTimeout(r, 3000));
  await browser.close();
})();
