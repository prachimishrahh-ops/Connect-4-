# Connect4 Battle - Conway Testnet Deployment Guide
**Target**: Conway Testnet (official Linera testnet)
**Purpose**: Production deployment for WaveHack Linera Buildathon 2025 submission
**Estimated Time**: 15-20 minutes

---

## Quick Start (TL;DR)

```bash
# 1. Install & configure
cargo install linera-service --git https://github.com/linera-io/linera-protocol.git --tag linera-v0.15.0
linera wallet init --with-new-chain --faucet https://faucet.conway.linera.io

# 2. Build
cd connect4-battle
cargo build --release --target wasm32-unknown-unknown

# 3. Deploy bankroll
linera publish-and-create target/wasm32-unknown-unknown/release/bankroll_{contract,service}.wasm --json-argument "null"
export BANKROLL_APP_ID="<copy-from-output>"

# 4. Deploy connect4
linera publish-and-create target/wasm32-unknown-unknown/release/liars_dice_{contract,service}.wasm --json-parameters "{\"bankroll_app_id\":\"$BANKROLL_APP_ID\"}" --json-argument "0"
export CONNECT4_APP_ID="<copy-from-output>"

# 5. Update README with Application ID
# 6. Test and submit!
```

---

## Prerequisites Checklist

- [ ] Rust 1.86.0+ installed (`rustup default 1.86.0`)
- [ ] wasm32 target added (`rustup target add wasm32-unknown-unknown`)
- [ ] Linera CLI installed (v0.15.0+)
- [ ] Conway testnet wallet configured
- [ ] Project compiles: `cargo build --release --target wasm32-unknown-unknown`
- [ ] Sufficient testnet tokens (>10.0)

---

## Step-by-Step Deployment

### Step 1: Install Linera CLI (if not installed)

```bash
# Install via cargo
cargo install linera-service --git https://github.com/linera-io/linera-protocol.git --tag linera-v0.15.0

# Verify installation
linera --version
```

### Step 2: Initialize Conway Testnet Wallet

```bash
# Initialize wallet and get initial chain + tokens
linera wallet init --with-new-chain --faucet https://faucet.conway.linera.io

# Verify wallet created
linera wallet show
```

### Step 3: Build Applications

```bash
cd "C:\Users\prate\Downloads\new prejt or buildahtin\connect4-battle"

# Build both applications for WASM
cargo build -p bankroll --release --target wasm32-unknown-unknown
cargo build -p liars_dice --release --target wasm32-unknown-unknown

# Verify WASM files exist
ls target/wasm32-unknown-unknown/release/*.wasm
```

### Step 4: Deploy Bankroll Application

```bash
linera publish-and-create \
  target/wasm32-unknown-unknown/release/bankroll_{contract,service}.wasm \
  --json-argument "null"

# SAVE THE APPLICATION ID FROM OUTPUT!
# Example: e476187f6ddfeb9d588c7b45d3df334d5501d6499b3f9ad5595cae86cce16a65...

export BANKROLL_APP_ID="<PASTE_APPLICATION_ID_HERE>"
echo "Bankroll App ID: $BANKROLL_APP_ID"
```

### Step 5: Deploy Connect4 Application

```bash
linera publish-and-create \
  target/wasm32-unknown-unknown/release/liars_dice_{contract,service}.wasm \
  --json-parameters "{\"bankroll_app_id\":\"$BANKROLL_APP_ID\"}" \
  --json-argument "0"

# SAVE THE APPLICATION ID FROM OUTPUT!
export CONNECT4_APP_ID="<PASTE_APPLICATION_ID_HERE>"
echo "Connect4 App ID: $CONNECT4_APP_ID"
```

### Step 6: Update README.md

Add to README.md near the top:

```markdown
**Application ID**: `<YOUR_CONNECT4_APP_ID>`
**Deployed to**: Conway Testnet
**Live Demo**: [Instructions Below](#playing-the-game)
```

### Step 7: Test Deployment (Optional Local Verification)

```bash
# Get your chain IDs
linera wallet show

# Start services locally
linera service --port 8081 &
linera service --port 8082 &

# Test GraphQL endpoints
curl http://localhost:8081/ | grep "GraphiQL"
curl http://localhost:8082/ | grep "GraphiQL"
```

---

## Troubleshooting

### "Insufficient balance"
```bash
linera faucet --amount 50.0
linera query-balance
```

### "Application already exists"
```bash
# Create new chain and deploy there
linera open-chain --to-new-chain
# Use new chain ID for deployment
```

### "Connection refused to testnet"
```bash
# Check testnet status
curl https://faucet.conway.linera.io/health

# If down, check Linera Discord for updates
```

---

## Final Checklist

Before submission:

- [ ] Bankroll app deployed to Conway
- [ ] Connect4 app deployed to Conway
- [ ] Application ID copied
- [ ] README.md updated with Application ID
- [ ] Local testing passed (optional)
- [ ] Docker deployment still works locally
- [ ] Video demo recorded
- [ ] Screenshots captured

---

## What to Submit

1. **README.md**: Must include Application ID
2. **Video Demo**: 3-5 minutes showing full game
3. **Screenshots**: 3-5 images of gameplay
4. **GitHub URL**: Link to repository

---

**Deployment Guide Status**: ✅ READY FOR EXECUTION
**Estimated Time**: 15-20 minutes
**Complexity**: Medium (straightforward if following steps)
