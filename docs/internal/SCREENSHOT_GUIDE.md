# Connect4 Battle - Screenshot Capture Guide
**Target**: 3-5 high-quality screenshots for README
**Purpose**: Professional presentation for judge evaluation

---

## Screenshot List

### 1. Lobby Screen with Blockchain Panel ✅ REQUIRED
**Filename**: `screenshots/01-lobby-blockchain-panel.png`

**What to Capture**:
- Full browser window at 1400px width
- Lobby screen showing:
  - "⚡ Connect4 Battle ⚡" title
  - "🎮 PLAY NOW" button prominently
  - RIGHT SIDEBAR: Blockchain Status panel visible
    - Network: Linera Testnet
    - Wallet address
    - Provably Fair Gaming message
  - LEFT SIDEBAR: Leaderboard visible

**Why Important**: Shows blockchain integration from first screen

**Steps**:
1. Open http://localhost:5173
2. Clear any existing data (Ctrl+Shift+Del)
3. Refresh page
4. Resize browser to 1400px wide
5. **BEFORE clicking anything**, take screenshot
6. Crop to remove browser chrome if needed

---

### 2. Active Game with Move Confirmation ✅ REQUIRED
**Filename**: `screenshots/02-game-move-blockchain-update.png`

**What to Capture**:
- Game board with at least 3-4 moves played
- Blockchain panel showing:
  - "Last Move: ✓ Column 4" (actual move)
  - "On-Chain Moves: 4" (actual count)
- Turn indicator visible
- Player names and ELO ratings visible
- Sound toggle (🔊) visible

**Why Important**: Proves blockchain is tracking every move

**Steps**:
1. Play 4 moves total (2 per player)
2. Wait for blockchain panel to update
3. Take screenshot right after a move completes
4. Ensure "Last Move" shows checkmark

---

### 3. Victory Screen with ELO Change ✅ REQUIRED
**Filename**: `screenshots/03-victory-elo-confetti.png`

**What to Capture**:
- Confetti animation in progress (timing is key!)
- Victory message with 🏆 trophy
- "+25 ELO" for winner
- Player names and updated ELO ratings
- Game board showing winning 4-in-a-row
- "Play Again" and "Return to Lobby" buttons

**Why Important**: Shows game completion and competitive features

**Steps**:
1. Play game until one player wins
2. **IMMEDIATELY** after win detected:
   - Wait 0.5 seconds for confetti to start
   - Take screenshot while particles are visible
3. Retry if you miss the timing (confetti lasts 3 seconds)

---

### 4. Mobile Responsive Layout ✅ REQUIRED
**Filename**: `screenshots/04-mobile-responsive-800px.png`

**What to Capture**:
- Browser at 800px width
- Full page scroll showing:
  - Game board (top)
  - Player panel (middle)
  - **Leaderboard VISIBLE** (bottom)
  - **Blockchain panel VISIBLE** (bottom)
- NO horizontal scrollbar

**Why Important**: Proves Phase 1 mobile fix worked

**Steps**:
1. Resize browser to exactly 800px wide
2. Scroll to show all sections
3. OR take tall screenshot showing full vertical layout
4. Highlight that panels are NOT hidden

**Alternative**: Take at 400px for true mobile view

---

### 5. Leaderboard & ELO Rankings (OPTIONAL)
**Filename**: `screenshots/05-leaderboard-rankings.png`

**What to Capture**:
- Left sidebar leaderboard panel
- Top 10 players with:
  - Gold/silver/bronze medal emojis
  - Player names
  - ELO ratings
  - Win/loss records

**Why Important**: Shows competitive gaming aspect

**Steps**:
1. Play several games to populate leaderboard
2. Zoom in on leaderboard panel
3. Ensure at least 3-5 entries visible

---

## Screenshot Quality Standards

### Technical Requirements
- **Resolution**: Minimum 1280x720, prefer 1920x1080
- **Format**: PNG (lossless)
- **File Size**: <500KB per image (optimize if needed)
- **No Compression Artifacts**: Use PNG, not JPEG

### Composition Guidelines
1. **Clean UI**: No browser extensions, bookmarks bar, or personal data visible
2. **Full Context**: Show enough UI to understand what's happening
3. **Highlight Key Features**: Blockchain panel should be prominent
4. **Professional**: No typos in player names, use clean test data
5. **Lighting**: If using dark theme, ensure text is readable

### What to AVOID
- ❌ Blurry or low-resolution images
- ❌ Personal browser data (bookmarks, extensions)
- ❌ Awkward cropping (missing key UI elements)
- ❌ Profanity or inappropriate test data
- ❌ Error messages or broken UI
- ❌ Screenshots without blockchain panel visible

---

## Tools for Screenshot Capture

### Windows
- **Snipping Tool**: Win+Shift+S (built-in, easy)
- **ShareX**: Free, powerful, auto-upload
- **Greenshot**: Free, annotation tools

### macOS
- **Command+Shift+4**: Native screenshot tool
- **CleanShot X**: Professional tool (paid)
- **Skitch**: Free, simple annotations

### Cross-Platform
- **Browser DevTools**: F12 → Responsive Design Mode → Screenshot
- **Lightshot**: Free, easy sharing
- **Nimbus Screenshot**: Browser extension

---

## Adding Screenshots to README

### Location in README
After "## 🎮 Key Features" section:

```markdown
## 📸 Screenshots

### Lobby with Blockchain Visibility
![Lobby Screen](screenshots/01-lobby-blockchain-panel.png)
*One-click "PLAY NOW" button with real-time blockchain status panel showing wallet address and provably fair messaging*

### Active Gameplay with Move Tracking
![Game in Progress](screenshots/02-game-move-blockchain-update.png)
*Every move recorded on-chain with instant confirmation in the Blockchain Status panel*

### Victory with ELO System
![Victory Screen](screenshots/03-victory-elo-confetti.png)
*Confetti celebration with competitive ELO rating updates (+25 for winner, -20 for loser)*

### Mobile Responsive Design
![Mobile Layout](screenshots/04-mobile-responsive-800px.png)
*Full features visible on all screen sizes - leaderboard and blockchain panel adapt gracefully*
```

---

## Screenshot Checklist

Before capturing:
- [ ] Docker deployment running
- [ ] Browser cache cleared for clean UI
- [ ] Browser window sized correctly (1400px for desktop, 800px for mobile)
- [ ] No personal browser data visible
- [ ] Test data is professional (names like "Alice" and "Bob", not "test123")

Screenshots to capture:
- [ ] 1. Lobby with blockchain panel (REQUIRED)
- [ ] 2. Active game with move confirmation (REQUIRED)
- [ ] 3. Victory with ELO and confetti (REQUIRED)
- [ ] 4. Mobile responsive layout (REQUIRED)
- [ ] 5. Leaderboard rankings (OPTIONAL)

After capturing:
- [ ] Create `/screenshots` folder in repository
- [ ] Optimize file sizes (<500KB each)
- [ ] Verify PNG format (not JPEG)
- [ ] Add to README with descriptive captions
- [ ] Commit and push to GitHub

---

## Pro Tips

1. **Timing is Everything**: For confetti screenshot, practice the timing (0.5-1s after win)
2. **Use Annotations**: Arrow pointing to blockchain panel in first screenshot helps
3. **Consistent Sizing**: All desktop screenshots at same width (1400px)
4. **Mobile Proof**: Mobile screenshot is CRITICAL - shows Phase 1 fix worked
5. **Update Blockchain Panel**: Make sure wallet address is visible, not "Connecting..."

---

**Guide Status**: ✅ READY FOR EXECUTION
**Estimated Time**: 10-15 minutes (including retakes)
**Required Screenshots**: 4 (Lobby, Game, Victory, Mobile)
**Optional Screenshots**: 1 (Leaderboard)
