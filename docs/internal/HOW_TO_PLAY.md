# 🎮 HOW TO PLAY CONNECT4 BATTLE

## 🚀 QUICK START (3 Steps)

### Step 1: Start Linera Node
Make sure your Linera blockchain node is running on port 8081:
```bash
linera service --port 8081
```

### Step 2: Start Game Servers

#### **Windows:**
Double-click **`START_GAME.bat`** in the project folder

#### **Mac/Linux:**
```bash
./START_GAME.sh
```

#### **Manual (Any OS):**
Open TWO terminal windows:

**Terminal 1 (Player A):**
```bash
cd frontend/web_a
python -m http.server 8000
```

**Terminal 2 (Player B):**
```bash
cd frontend/web_b
python -m http.server 8001
```

### Step 3: Open Browsers

🔴 **Player A (Red):** http://localhost:8000
🟡 **Player B (Yellow):** http://localhost:8001

---

## 🎯 GAMEPLAY

1. **Both players enter their names**
2. **Click "Quick Play"** on both browsers
3. **Wait 2-3 seconds** for matchmaking
4. **Play!** Red goes first, click any column
5. **First to connect 4 wins!** 🏆

---

## 📊 CONFIGURATION

### Endpoints
- **Linera Node:** http://localhost:8081
- **Player A Frontend:** http://localhost:8000
- **Player B Frontend:** http://localhost:8001

### Config Files
- `frontend/web_a/config.json` - Player A settings
- `frontend/web_b/config.json` - Player B settings

### Default Settings
```json
{
  "nodeServiceURL": "http://localhost:8081",
  "connect4AppId": "your-app-id",
  "masterChain": "your-master-chain",
  "lobbyChain": "your-lobby-chain",
  "userChain": "your-user-chain"
}
```

---

## ⚠️ TROUBLESHOOTING

### Problem: "Failed to fetch config.json"
**Solution:** Make sure you're using http://localhost URLs, not file:// URLs

### Problem: "Network Error" or "GraphQL Error"
**Solution:**
1. Check Linera node is running: `ps aux | grep linera`
2. Verify node is on port 8081
3. Check config.json has correct nodeServiceURL

### Problem: Matchmaking doesn't work
**Solution:**
1. Refresh both browsers
2. Make sure both players click "Quick Play"
3. Check browser console for errors (F12)

### Problem: Port already in use
**Solution:**
```bash
# Kill existing servers
pkill -f "python -m http.server"

# Or use different ports
python -m http.server 8002  # For Player A
python -m http.server 8003  # For Player B
```

---

## 🎬 DEMO VIDEO SETUP

For recording your demo video:

1. **Arrange windows side-by-side**
2. **Use screen recording software** (OBS Studio)
3. **Show both players simultaneously**
4. **Play a complete game** (6-7 moves)

**Pro Tip:** Use browser zoom to fit both windows nicely:
- Ctrl + Mouse Wheel (Windows)
- Cmd + Plus/Minus (Mac)

---

## 🔧 ADVANCED OPTIONS

### Custom Ports
Edit the port numbers in START_GAME.bat or START_GAME.sh if needed.

### Different Chains
Update config.json in each frontend folder with your specific chain IDs.

### Network Play
For demo purposes over network:
1. Replace `localhost` with your IP address in config.json
2. Ensure firewall allows ports 8000, 8001, 8081

---

## ✅ CHECKLIST BEFORE DEMO

- [ ] Linera node running (port 8081)
- [ ] Player A server running (port 8000)
- [ ] Player B server running (port 8001)
- [ ] Both browsers can access localhost:8000 and localhost:8001
- [ ] Config.json files are correct
- [ ] Test one complete game before recording

---

## 🎉 HAVE FUN!

You're now ready to play Connect4 Battle on the blockchain! 🚀
