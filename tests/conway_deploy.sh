#!/usr/bin/env bash
source ~/.cargo/env 2>/dev/null || true
export PATH="$HOME/.cargo/bin:$PATH"

# Check if arguments are provided
if [ "$#" -lt 2 ]; then
    echo "Usage: $0 <FAUCET_URL> <GRAPHQL_URL>"
    echo "Example: $0 https://faucet.testnet-conway.linera.net http://localhost:8080"
    exit 1
fi

FAUCET_URL=$1
GRAPHQL_URL=$2

set -e
set -o pipefail

echo "=== Starting Connect4 Battle Conway Deployment ==="

# Create a clean temp directory for the wallet
export LINERA_TMP_DIR=$(mktemp -d)
echo "Using temp directory: $LINERA_TMP_DIR"

export LINERA_WALLET="$LINERA_TMP_DIR/wallet.json"
export LINERA_STORAGE="rocksdb:$LINERA_TMP_DIR/client.db"
export LINERA_KEYSTORE="$LINERA_TMP_DIR/keystore.json"

# Helper function to run linera with explicit wallet and keystore
function l_cmd() {
    linera --wallet "$LINERA_WALLET" --storage "$LINERA_STORAGE" --keystore "$LINERA_KEYSTORE" "$@"
}

# 1. Initialize Wallet
echo "🔑 Initializing wallet from deployment faucet..."
l_cmd wallet init --faucet "$FAUCET_URL"

# 2. Open Chains
echo "🔗 Requesting Master Chain..."
DEFAULT_CHAIN_ID=$(l_cmd wallet request-chain --faucet "$FAUCET_URL" | grep -oE '[0-9a-f]{64}' | head -1)
echo "✅ Master Chain ID: $DEFAULT_CHAIN_ID"

echo "🔗 Requesting Player A Chain..."
PLAYER_A_CHAIN=$(l_cmd wallet request-chain --faucet "$FAUCET_URL" | grep -oE '[0-9a-f]{64}' | head -1)
echo "✅ Player A Chain: $PLAYER_A_CHAIN"

echo "🔗 Requesting Player B Chain..."
PLAYER_B_CHAIN=$(l_cmd wallet request-chain --faucet "$FAUCET_URL" | grep -oE '[0-9a-f]{64}' | head -1)
echo "✅ Player B Chain: $PLAYER_B_CHAIN"

l_cmd sync && l_cmd query-balance

# 3. Build Contracts
echo "📦 Building WASM contracts..."
cd ..
cargo build --release --target wasm32-unknown-unknown
cd tests

# 4. Deploy Bankroll
echo "📤 Deploying Bankroll App..."
BANKROLL_APP_ID=$(l_cmd --wait-for-outgoing-messages project publish-and-create ../bankroll \
  --json-parameters "{\"master_chain\": \"$DEFAULT_CHAIN_ID\", \"bonus\": \"25000\"}" \
  --json-argument "{\"master_chain\": \"$DEFAULT_CHAIN_ID\", \"bonus\": \"25000\"}")

echo "✅ Bankroll App ID: $BANKROLL_APP_ID"
sleep 5

# 5. Deploy Connect4 Battle (using liars_dice module name from workspace)
echo "📤 Deploying Connect4 Battle App..."
CONNECT4_APP_ID=$(l_cmd --wait-for-outgoing-messages project publish-and-create ../liars_dice \
  --required-application-ids "$BANKROLL_APP_ID" \
  --json-parameters "{\"master_chain\": \"$DEFAULT_CHAIN_ID\", \"lobby_chain\": \"$DEFAULT_CHAIN_ID\", \"bankroll\": \"$BANKROLL_APP_ID\"}" \
  --json-argument "0")

echo "✅ Connect4 Battle App ID: $CONNECT4_APP_ID"
sleep 5

# 5b. Request App for Players
echo "🔗 Requesting application for Players..."
# Start a temporary service to sync/request
l_cmd service --port 8089 &
SERVICE_PID=$!
sleep 5

# Set default to Player A and request
l_cmd wallet set-default "$PLAYER_A_CHAIN"
l_cmd request-application "$CONNECT4_APP_ID"
l_cmd sync

# Set default to Player B and request
l_cmd wallet set-default "$PLAYER_B_CHAIN"
l_cmd request-application "$CONNECT4_APP_ID"
l_cmd sync

# Set back to master
l_cmd wallet set-default "$DEFAULT_CHAIN_ID"

kill $SERVICE_PID || true

# 6. Generate Configs
echo "📝 Creating frontend configs..."
mkdir -p ../frontend/web_a ../frontend/web_b

cat > ../frontend/web_a/config_conway.json <<EOF
{
  "nodeServiceURL": "http://localhost:8080",
  "liarsDiceAppId": "$CONNECT4_APP_ID",
  "bankrollAppId": "$BANKROLL_APP_ID",
  "masterChain": "$DEFAULT_CHAIN_ID",
  "lobbyChain": "$DEFAULT_CHAIN_ID",
  "userChain": "$PLAYER_A_CHAIN"
}
EOF

cat > ../frontend/web_b/config_conway.json <<EOF
{
  "nodeServiceURL": "http://localhost:8081",
  "liarsDiceAppId": "$CONNECT4_APP_ID",
  "bankrollAppId": "$BANKROLL_APP_ID",
  "masterChain": "$DEFAULT_CHAIN_ID",
  "lobbyChain": "$DEFAULT_CHAIN_ID",
  "userChain": "$PLAYER_B_CHAIN"
}
EOF

echo ""
echo "==================================="
echo "✅ Deployment Complete!"
echo "==================================="
echo "Bankroll ID:     $BANKROLL_APP_ID"
echo "Connect4 App ID: $CONNECT4_APP_ID"
echo "Master Chain:    $DEFAULT_CHAIN_ID"
echo "Player A Chain:  $PLAYER_A_CHAIN"
echo "Player B Chain:  $PLAYER_B_CHAIN"
echo "==================================="
