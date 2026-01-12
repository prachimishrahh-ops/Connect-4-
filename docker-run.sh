#!/usr/bin/env bash
set -eu

echo "=== Starting Connect4 Battle Docker Deployment ==="

# Create a temp directory for linera network
export LINERA_TMP=$(mktemp -d)
echo "Using temp directory: $LINERA_TMP"

# Start network with faucet
echo "🚀 Starting linera network with faucet..."
linera net up --testing-prng-seed 37 --other-initial-chains 3 --with-faucet --faucet-port 8080 --path "$LINERA_TMP" > /tmp/linera_net.log 2>&1 &
LINERA_PID=$!

# Wait for READY! message in log
echo "⏳ Waiting for network to be ready..."
MAX_WAIT=180
WAITED=0
while ! grep -q "READY!" /tmp/linera_net.log 2>/dev/null && [ $WAITED -lt $MAX_WAIT ]; do
    sleep 2
    WAITED=$((WAITED + 2))
    if [ $((WAITED % 10)) -eq 0 ]; then
        echo "  Waiting... ($WAITED/$MAX_WAIT seconds)"
    fi
done

if ! grep -q "READY!" /tmp/linera_net.log; then
    echo "❌ Network did not become ready after $MAX_WAIT seconds"
    cat /tmp/linera_net.log
    exit 1
fi

echo "✅ Network is ready!"

# Create a SEPARATE wallet for deployment (network holds lock on wallet_0)
export DEPLOY_DIR=$(mktemp -d)
export LINERA_WALLET="$DEPLOY_DIR/wallet.json"
export LINERA_STORAGE="rocksdb:$DEPLOY_DIR/client.db"

echo "🔑 Creating deployment wallet via faucet..."
linera wallet init --faucet http://localhost:8080
linera wallet request-chain --faucet http://localhost:8080
echo "Wallet: $LINERA_WALLET"

# Verify wallet was created
if [ ! -f "$LINERA_WALLET" ]; then
    echo "❌ Wallet file not found at $LINERA_WALLET"
    ls -la "$DEPLOY_DIR"
    exit 1
fi

# Build apps
echo "📦 Building WASM contracts..."
cd /build
cargo build --release --target wasm32-unknown-unknown

# Get default chain ID from wallet
echo "🔗 Getting chain ID..."
# List wallet chains and extract the default chain
CHAIN_ID=$(linera wallet show 2>&1 | grep -oE '[0-9a-f]{64}' | head -1)

if [ -z "$CHAIN_ID" ]; then
    echo "❌ Could not get chain ID from wallet file"
    echo "Wallet contents:"
    cat "$LINERA_WALLET"
    exit 1
fi

echo "✅ Using chain ID: $CHAIN_ID"

# Verify linera network is still running
echo "🔍 Verifying network status..."
if ! kill -0 $LINERA_PID 2>/dev/null; then
    echo "❌ Linera network process died"
    echo "Network log:"
    cat /tmp/linera_net.log
    exit 1
fi
echo "✅ Network is still running"

# Deploy bankroll
echo "📤 Deploying bankroll app..."
set +e  # Don't exit on error, capture it
BANKROLL_OUTPUT=$(linera --wait-for-outgoing-messages project publish-and-create bankroll \
  --json-parameters "{\"master_chain\": \"$CHAIN_ID\", \"bonus\": \"25000\"}" \
  --json-argument "{\"master_chain\": \"$CHAIN_ID\", \"bonus\": \"25000\"}" 2>&1)
BANKROLL_EXIT=$?
set -e

echo "Bankroll deployment output:"
echo "$BANKROLL_OUTPUT"
echo "Exit code: $BANKROLL_EXIT"

if [ $BANKROLL_EXIT -ne 0 ]; then
    echo "❌ Bankroll deployment failed with exit code $BANKROLL_EXIT"
    exit 1
fi

# The app ID is printed on its own line (64-char hex without any prefix)
BANKROLL_ID=$(echo "$BANKROLL_OUTPUT" | grep -E '^[0-9a-f]{64}$' | tail -1)

if [ -z "$BANKROLL_ID" ]; then
    echo "❌ Could not get bankroll app ID from output"
    exit 1
fi
echo "✅ Bankroll app ID: $BANKROLL_ID"

# Create player wallets FIRST (before deploying apps) so we can create user-chain instances
echo "🔑 Creating Player A wallet..."
PLAYER_A_DIR=$(mktemp -d)
export PLAYER_A_WALLET="$PLAYER_A_DIR/wallet.json"
export PLAYER_A_KEYSTORE="$PLAYER_A_DIR/keystore.json"
export PLAYER_A_STORAGE="rocksdb:$PLAYER_A_DIR/client.db"
LINERA_WALLET="$PLAYER_A_WALLET" LINERA_KEYSTORE="$PLAYER_A_KEYSTORE" LINERA_STORAGE="$PLAYER_A_STORAGE" \
  linera wallet init --faucet http://localhost:8080
LINERA_WALLET="$PLAYER_A_WALLET" LINERA_KEYSTORE="$PLAYER_A_KEYSTORE" LINERA_STORAGE="$PLAYER_A_STORAGE" \
  linera wallet request-chain --faucet http://localhost:8080
PLAYER_A_CHAIN=$(LINERA_WALLET="$PLAYER_A_WALLET" LINERA_KEYSTORE="$PLAYER_A_KEYSTORE" LINERA_STORAGE="$PLAYER_A_STORAGE" \
  linera wallet show 2>&1 | grep -oE '[0-9a-f]{64}' | head -1)
echo "Player A chain: $PLAYER_A_CHAIN"

echo "🔑 Creating Player B wallet..."
PLAYER_B_DIR=$(mktemp -d)
export PLAYER_B_WALLET="$PLAYER_B_DIR/wallet.json"
export PLAYER_B_KEYSTORE="$PLAYER_B_DIR/keystore.json"
export PLAYER_B_STORAGE="rocksdb:$PLAYER_B_DIR/client.db"
LINERA_WALLET="$PLAYER_B_WALLET" LINERA_KEYSTORE="$PLAYER_B_KEYSTORE" LINERA_STORAGE="$PLAYER_B_STORAGE" \
  linera wallet init --faucet http://localhost:8080
LINERA_WALLET="$PLAYER_B_WALLET" LINERA_KEYSTORE="$PLAYER_B_KEYSTORE" LINERA_STORAGE="$PLAYER_B_STORAGE" \
  linera wallet request-chain --faucet http://localhost:8080
PLAYER_B_CHAIN=$(LINERA_WALLET="$PLAYER_B_WALLET" LINERA_KEYSTORE="$PLAYER_B_KEYSTORE" LINERA_STORAGE="$PLAYER_B_STORAGE" \
  linera wallet show 2>&1 | grep -oE '[0-9a-f]{64}' | head -1)
echo "Player B chain: $PLAYER_B_CHAIN"

# Deploy connect4 on MASTER chain (type 0) - Master now handles lobby functionality
echo "📤 Deploying connect4 app on Master chain (with built-in lobby)..."
CONNECT4_OUTPUT=$(linera --wait-for-outgoing-messages project publish-and-create liars_dice \
  --required-application-ids "$BANKROLL_ID" \
  --json-parameters "{\"master_chain\": \"$CHAIN_ID\", \"lobby_chain\": \"$CHAIN_ID\", \"bankroll\": \"$BANKROLL_ID\"}" \
  --json-argument "0" 2>&1)

echo "$CONNECT4_OUTPUT"
CONNECT4_ID=$(echo "$CONNECT4_OUTPUT" | grep -E '^[0-9a-f]{64}$' | tail -1)

if [ -z "$CONNECT4_ID" ]; then
    echo "❌ Could not get connect4 app ID"
    exit 1
fi
echo "✅ Connect4 app ID: $CONNECT4_ID"
echo "✅ Master chain handles both admin and lobby functions"

# Set lobby chain to master chain since master handles lobby functionality
LOBBY_CHAIN="$CHAIN_ID"

# Create frontend configs with player-specific chains
echo "📝 Creating frontend configs..."
mkdir -p /build/frontend/web_a /build/frontend/web_b

cat > /build/frontend/web_a/config.json <<EOF
{
  "nodeServiceURL": "http://localhost:8081",
  "connect4AppId": "$CONNECT4_ID",
  "bankrollAppId": "$BANKROLL_ID",
  "masterChain": "$CHAIN_ID",
  "lobbyChain": "$LOBBY_CHAIN",
  "userChain": "$PLAYER_A_CHAIN"
}
EOF

cat > /build/frontend/web_b/config.json <<EOF
{
  "nodeServiceURL": "http://localhost:8082",
  "connect4AppId": "$CONNECT4_ID",
  "bankrollAppId": "$BANKROLL_ID",
  "masterChain": "$CHAIN_ID",
  "lobbyChain": "$LOBBY_CHAIN",
  "userChain": "$PLAYER_B_CHAIN"
}
EOF

# Frontend files already exist in web_a and web_b directories
echo "✅ Frontend files ready (web_a/index.html and web_b/index.html)"

# Start services with separate wallets (already created above)
echo "🌐 Starting linera services..."

# CRITICAL FIX: Start lobby/master chain service (port 8083)
# This service handles cross-chain messages to the lobby chain
echo "🏛️ Starting Lobby chain service (port 8083)..."
LINERA_WALLET="$LINERA_WALLET" LINERA_STORAGE="$LINERA_STORAGE" \
  linera service --port 8083 > /tmp/service_8083.log 2>&1 &
LOBBY_SERVICE_PID=$!

# Player A service
LINERA_WALLET="$PLAYER_A_WALLET" LINERA_KEYSTORE="$PLAYER_A_KEYSTORE" LINERA_STORAGE="$PLAYER_A_STORAGE" \
  linera service --port 8081 > /tmp/service_8081.log 2>&1 &
SERVICE1_PID=$!

# Player B service
LINERA_WALLET="$PLAYER_B_WALLET" LINERA_KEYSTORE="$PLAYER_B_KEYSTORE" LINERA_STORAGE="$PLAYER_B_STORAGE" \
  linera service --port 8082 > /tmp/service_8082.log 2>&1 &
SERVICE2_PID=$!

sleep 3

# Verify services started
if ! kill -0 $LOBBY_SERVICE_PID 2>/dev/null; then
    echo "❌ Lobby service on port 8083 failed to start"
    cat /tmp/service_8083.log
else
    echo "✅ Lobby service running on port 8083"
fi
if ! kill -0 $SERVICE1_PID 2>/dev/null; then
    echo "❌ Service on port 8081 failed to start"
    cat /tmp/service_8081.log
fi
if ! kill -0 $SERVICE2_PID 2>/dev/null; then
    echo "❌ Service on port 8082 failed to start"
    cat /tmp/service_8082.log
fi

# Start web servers for frontends
echo "🖥️ Starting frontend web servers..."
cd /build/frontend/web_a && python3 -m http.server 5173 > /tmp/web_5173.log 2>&1 &
cd /build/frontend/web_b && python3 -m http.server 5174 > /tmp/web_5174.log 2>&1 &

echo ""
echo "==================================="
echo "🎮 Connect4 Battle is ready!"
echo "==================================="
echo "Player A Frontend (Red):  http://localhost:5173"
echo "Player B Frontend (Yellow): http://localhost:5174"
echo "Service A GraphQL:         http://localhost:8081"
echo "Service B GraphQL:         http://localhost:8082"
echo "Lobby Service:             http://localhost:8083"
echo "==================================="
echo ""
echo "Master Chain: $CHAIN_ID"
echo "Lobby Chain:  $LOBBY_CHAIN"
echo "Player A Chain: $PLAYER_A_CHAIN"
echo "Player B Chain: $PLAYER_B_CHAIN"
echo "Bankroll App: $BANKROLL_ID"
echo "Connect4 App: $CONNECT4_ID"
echo "==================================="
echo ""
echo "📋 INSTRUCTIONS:"
echo "1. Open http://localhost:5173 (Player A - Red)"
echo "2. Open http://localhost:5174 (Player B - Yellow)"
echo "3. Both players: Enter name and click 'Create Profile'"
echo "4. Both players: Click 'Find Match'"
echo "5. Game starts automatically when matched!"
echo "==================================="

# Keep container running
tail -f /dev/null
