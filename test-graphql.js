const fetch = require('node-fetch');

async function graphql(url, query) {
  try {
    const response = await fetch(url, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ query })
    });
    const data = await response.json();
    return data;
  } catch (error) {
    return { error: error.message };
  }
}

(async () => {
  console.log('🔍 Testing GraphQL endpoints...\n');

  const config = require('./frontend/web_a/config.json');
  const url = `${config.nodeServiceURL}/chains/${config.userChain}/applications/${config.connect4AppId}`;

  console.log(`URL: ${url}\n`);

  // Test query 1: Get game state
  console.log('1️⃣ Testing getGameState query...');
  const gameStateResult = await graphql(url, 'query { getGameState { gameId status board } }');
  console.log(JSON.stringify(gameStateResult, null, 2));

  // Test query 2: Get user profile
  console.log('\n2️⃣ Testing getUserProfile query...');
  const profileResult = await graphql(url, 'query { getUserProfile { name elo } }');
  console.log(JSON.stringify(profileResult, null, 2));

  // Test query 3: Get leaderboard
  console.log('\n3️⃣ Testing getLeaderboard query...');
  const leaderboardResult = await graphql(url, 'query { getLeaderboard { playerName elo } }');
  console.log(JSON.stringify(leaderboardResult, null, 2));

  // Test mutation 1: Set profile
  console.log('\n4️⃣ Testing setProfile mutation...');
  const setProfileResult = await graphql(url, 'mutation { setProfile(name: "TestUser") }');
  console.log(JSON.stringify(setProfileResult, null, 2));

  // Test mutation 2: Initial setup
  console.log('\n5️⃣ Testing initialSetup mutation...');
  const setupResult = await graphql(url, 'mutation { initialSetup }');
  console.log(JSON.stringify(setupResult, null, 2));

  console.log('\n✅ GraphQL test complete');
})();
