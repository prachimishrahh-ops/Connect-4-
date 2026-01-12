const { chromium } = require('playwright');

(async () => {
  const browser = await chromium.launch({ headless: false });

  console.log('🔍 Debug: Checking frontend structure...\n');

  const context = await browser.newContext();
  const page = await context.newPage();

  await page.goto('http://localhost:5173');
  console.log('📍 Loaded http://localhost:5173');

  await new Promise(r => setTimeout(r, 5000));

  // Take screenshot
  await page.screenshot({ path: 'frontend-debug.png', fullPage: true });
  console.log('📸 Screenshot saved: frontend-debug.png');

  // Get page content
  const content = await page.content();
  console.log('\n📄 Page HTML (first 2000 chars):');
  console.log(content.substring(0, 2000));

  // Get body text
  const bodyText = await page.textContent('body');
  console.log('\n📝 Page text content:');
  console.log(bodyText);

  // Try to find input fields
  const inputs = await page.$$('input');
  console.log(`\n🔍 Found ${inputs.length} input elements`);

  for (let i = 0; i < inputs.length; i++) {
    const placeholder = await inputs[i].getAttribute('placeholder');
    const type = await inputs[i].getAttribute('type');
    const id = await inputs[i].getAttribute('id');
    console.log(`  Input ${i + 1}: type="${type}", id="${id}", placeholder="${placeholder}"`);
  }

  // Try to find buttons
  const buttons = await page.$$('button');
  console.log(`\n🔍 Found ${buttons.length} button elements`);

  for (let i = 0; i < buttons.length; i++) {
    const text = await buttons[i].textContent();
    const id = await buttons[i].getAttribute('id');
    console.log(`  Button ${i + 1}: id="${id}", text="${text}"`);
  }

  console.log('\n⏸️  Keeping browser open for 30 seconds...');
  await new Promise(r => setTimeout(r, 30000));

  await browser.close();
})();
