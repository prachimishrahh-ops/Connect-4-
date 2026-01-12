# 🐳 DOCKER vs LOCAL SETUP

## Quick Decision Guide

### 🐳 **USE DOCKER IF:**
- ✅ You want **one-command setup** (easiest)
- ✅ You're recording a **demo video** (most reliable)
- ✅ You want **clean environment** every time
- ✅ You don't want to install Rust/Linera manually
- ✅ You're on **Windows** (fewer PATH issues)

### 💻 **USE LOCAL IF:**
- ✅ You already have **Linera installed** and working
- ✅ You want **faster iteration** (no container overhead)
- ✅ You want to **debug/develop** (easier access to logs)
- ✅ You have **limited Docker resources** (RAM/CPU)

---

## 🐳 OPTION 1: DOCKER (RECOMMENDED FOR DEMO)

### **Super Easy - One Command:**

```bash
cd "C:\Users\prate\Downloads\new prejt or buildahtin\connect4-battle"
docker-compose up
```

### **What This Does:**
1. ✅ Starts Linera network automatically
2. ✅ Builds and deploys Connect4 + Bankroll apps
3. ✅ Starts frontend servers
4. ✅ Opens all required ports

### **Your Localhost Links:**

🔴 **PLAYER A (RED):**
```
http://localhost:5173
```

🟡 **PLAYER B (YELLOW):**
```
http://localhost:5174
```

### **Ports Exposed:**
```
5173 - Player A Frontend
5174 - Player B Frontend
8081 - Linera Service A
8082 - Linera Service B
8083 - Lobby Service
8080 - Faucet
```

### **To Stop:**
```bash
docker-compose down
```

### **To Rebuild (if you change code):**
```bash
docker-compose down
docker-compose up --build
```

---

## 💻 OPTION 2: LOCAL SETUP

### **Prerequisites:**
- Rust + Cargo installed
- Linera CLI installed
- Python 3 (for frontend servers)

### **Step-by-Step:**

#### **1. Start Linera Network:**
```bash
cd "C:\Users\prate\Downloads\new prejt or buildahtin\connect4-battle"

# Create temp directory
export LINERA_TMP=$(mktemp -d)

# Start network
linera net up --testing-prng-seed 37 --other-initial-chains 3 --with-faucet --faucet-port 8080 --path "$LINERA_TMP"
```

#### **2. Build & Deploy Apps:**
```bash
# Build WASM
cargo build --release --target wasm32-unknown-unknown

# Get chain ID
CHAIN_ID=$(linera wallet show | grep -oE '[0-9a-f]{64}' | head -1)

# Deploy bankroll
linera project publish-and-create bankroll \
  --json-parameters "{\"master_chain\": \"$CHAIN_ID\", \"bonus\": \"25000\"}" \
  --json-argument "{\"master_chain\": \"$CHAIN_ID\", \"bonus\": \"25000\"}"

# Deploy connect4
linera project publish-and-create connect4 \
  --json-parameters "{\"bankroll_id\": \"$BANKROLL_ID\"}" \
  --json-argument "{\"bankroll_id\": \"$BANKROLL_ID\"}"
```

#### **3. Start Services:**
```bash
# Create wallets
linera wallet init --faucet http://localhost:8080

# Service A (Player A)
linera service --port 8081 &

# Service B (Player B)
linera service --port 8082 &

# Lobby service
linera service --port 8083 &
```

#### **4. Update Config Files:**
Edit `frontend/web_a/config.json` and `frontend/web_b/config.json` with your:
- App IDs (from deployment)
- Chain IDs (from wallet)

#### **5. Start Frontend Servers:**
```bash
# Terminal 1
cd frontend/web_a
python -m http.server 8000

# Terminal 2
cd frontend/web_b
python -m http.server 8001
```

#### **6. Open Browsers:**
- http://localhost:8000 (Player A)
- http://localhost:8001 (Player B)

---

## 🎯 RECOMMENDATION FOR YOUR DEMO

### **Use Docker! Here's why:**

✅ **One Command** - Just `docker-compose up`
✅ **Reliable** - Same environment every time
✅ **Clean** - No leftover state between runs
✅ **Professional** - Shows containerization skills
✅ **Easy to Record** - No manual setup on camera
✅ **Ports Mapped** - Everything just works

### **Docker Demo Workflow:**

```bash
# 1. Start Docker
docker-compose up

# 2. Wait for "READY!" message (30-60 seconds)

# 3. Open browsers:
#    - http://localhost:5173 (Player A)
#    - http://localhost:5174 (Player B)

# 4. Record your demo!

# 5. Clean stop
docker-compose down
```

---

## 🎬 DEMO VIDEO SCRIPT (DOCKER VERSION)

**[Before Recording]:**
```bash
cd connect4-battle
docker-compose up
# Wait for READY message
```

**[On Camera]:**
> "Hi! I've already started the Docker container which automatically:
> - Deploys the Linera blockchain
> - Builds and publishes the Connect4 app
> - Starts frontend servers
>
> Now I'll show you the game in action..."

**[Open browsers]:**
- http://localhost:5173 ← Player A (Red)
- http://localhost:5174 ← Player B (Yellow)

**[Play the game and explain Linera features]**

---

## 📊 COMPARISON TABLE

| Feature | Docker 🐳 | Local 💻 |
|---------|-----------|----------|
| **Setup Time** | 1 min | 10+ min |
| **Commands** | 1 command | 10+ commands |
| **Reliability** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ |
| **Debug Access** | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| **Performance** | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| **Clean Restart** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ |
| **Demo Ready** | ✅ YES | ⚠️ Manual |
| **Resource Usage** | Higher | Lower |

---

## 🚀 QUICK START SCRIPTS

### Docker Start Script
```bash
#!/bin/bash
# start-docker.sh

cd "C:\Users\prate\Downloads\new prejt or buildahtin\connect4-battle"

echo "🐳 Starting Connect4 Battle with Docker..."
docker-compose up

# Wait for services
echo "⏳ Waiting for services to start..."
sleep 30

# Open browsers
echo "🌐 Opening browsers..."
start http://localhost:5173  # Player A
start http://localhost:5174  # Player B

echo "✅ Ready to play!"
```

### Local Start Script
```bash
#!/bin/bash
# start-local.sh

cd "C:\Users\prate\Downloads\new prejt or buildahtin\connect4-battle"

# Run the full deployment script
bash docker-run.sh

# Or use START_GAME.bat for just frontends
# (if blockchain is already running)
```

---

## ⚠️ TROUBLESHOOTING

### Docker Issues

**Problem: "Port already in use"**
```bash
docker-compose down
# Kill any processes using ports
netstat -ano | findstr :5173
taskkill /PID <process-id> /F
```

**Problem: "Container exits immediately"**
```bash
# Check logs
docker-compose logs

# Run interactively to debug
docker-compose run --rm connect4-battle bash
```

**Problem: "Image not found: kvozt/linera-dev:latest"**
```bash
# Pull the image
docker pull kvozt/linera-dev:latest

# Or build locally (if you have Dockerfile)
docker build -t kvozt/linera-dev:latest .
```

### Local Issues

**Problem: "linera: command not found"**
```bash
# Install Linera CLI
cargo install linera-service --locked
```

**Problem: "Network not ready"**
```bash
# Check if network is running
ps aux | grep linera

# Check logs
tail -f /tmp/linera_net.log
```

---

## 🎯 FINAL RECOMMENDATION

### **FOR DEMO VIDEO:**
```bash
# Use Docker - It's the easiest!
docker-compose up

# Then open:
# http://localhost:5173 (Player A)
# http://localhost:5174 (Player B)
```

### **FOR DEVELOPMENT:**
```bash
# Use local setup for faster iteration
bash docker-run.sh  # Initial setup
# Then modify code and test
```

---

## ✅ CHECKLIST

### Before Demo Recording:

**Docker Approach:**
- [ ] Docker Desktop is running
- [ ] Run `docker-compose up`
- [ ] Wait for "READY!" message
- [ ] Test both URLs work
- [ ] Test one complete game
- [ ] Ready to record! 🎬

**Local Approach:**
- [ ] Linera CLI installed
- [ ] Rust/Cargo installed
- [ ] Python 3 installed
- [ ] Run deployment script
- [ ] Start frontend servers
- [ ] Test both URLs work
- [ ] Test one complete game
- [ ] Ready to record! 🎬

---

Good luck with your demo! 🚀🎮
