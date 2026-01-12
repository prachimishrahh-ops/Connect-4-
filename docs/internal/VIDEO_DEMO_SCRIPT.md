# Connect4 Battle - Video Demo Script
**Target Length**: 3-5 minutes
**Purpose**: Showcase blockchain integration, smooth UX, and competitive gameplay for judges

---

## Recording Setup

### Tools Needed
- **Screen Recorder**: OBS Studio (free) or Loom (easy)
- **Browser**: Chrome or Firefox (clean profile, no extensions showing)
- **Resolution**: 1920x1080 or 1280x720 minimum
- **Frame Rate**: 30fps minimum, 60fps preferred
- **Audio**: Optional narration (can add text overlays instead)

### Before Recording
- [ ] Docker deployment running (`docker ps` shows connect4-battle)
- [ ] Both frontend URLs accessible (http://localhost:5173, http://localhost:5174)
- [ ] Clear browser cache for clean demo
- [ ] Close unnecessary tabs/windows
- [ ] Prepare two browser windows side-by-side
- [ ] Test sound toggle works
- [ ] Practice run (timing)

---

## Video Script (3-5 minutes)

### Scene 1: Introduction (0:00 - 0:20)
**Show**: Title screen or README

**Text Overlay/Narration**:
> "Connect4 Battle - A decentralized competitive Connect4 game built on Linera blockchain.
> Every move is recorded on-chain with provably fair gameplay and sub-second finality."

**Actions**:
- Show project README in browser
- Scroll to Key Features section
- Highlight blockchain-first design points

---

### Scene 2: One-Command Deployment (0:20 - 0:40)
**Show**: Terminal

**Text Overlay/Narration**:
> "Deploying to Linera blockchain is as simple as: docker compose up --build
> The entire system starts in under 2 minutes."

**Actions**:
```bash
# Show command (already running, so just show docker ps)
docker ps

# Show output
# Expected: 1 container running with ports 5173, 5174, 8081-8083
```

**Highlight**:
- Container status: Up
- Multiple ports exposed
- One-command simplicity

---

### Scene 3: Blockchain Visibility (0:40 - 1:10)
**Show**: Browser - http://localhost:5173

**Text Overlay/Narration**:
> "Unlike traditional games, Connect4 Battle makes blockchain integration VISIBLE.
> Notice the Blockchain Status panel showing your wallet address and on-chain activity."

**Actions**:
1. Open Player A frontend
2. Point to/circle Blockchain Status panel (right sidebar)
3. Zoom in on:
   - "Network: Linera Testnet"
   - "Wallet: dbd42532..." (abbreviated address)
   - Hover to show full address tooltip
   - "Last Move: Waiting for game..."
   - "On-Chain Moves: 0"
4. Highlight provably fair messaging at bottom

**Key Point**: This is what judges want to see - blockchain isn't hidden!

---

### Scene 4: One-Click Matchmaking (1:10 - 1:40)
**Show**: Split screen - both http://localhost:5173 and http://localhost:5174

**Text Overlay/Narration**:
> "Onboarding is streamlined to ONE click. No complex wallet connections,
> no confusing multi-step process. Just enter your name and PLAY NOW."

**Actions**:
1. **Player A Window**:
   - Enter name "Alice"
   - Show single prominent "🎮 PLAY NOW" button
   - Click button

2. **Show matchmaking spinner**:
   - "Finding opponent..." message
   - Professional loading animation

3. **Player B Window**:
   - Enter name "Bob"
   - Click "🎮 PLAY NOW"

4. **Both windows**:
   - Match found animation
   - Sound plays (🔊 icon visible)
   - Game screen appears

**Timing**: Show matchmaking completes in <3 seconds

---

### Scene 5: Blockchain-Recorded Gameplay (1:40 - 3:00)
**Show**: Split screen - active game

**Text Overlay/Narration**:
> "Every move is recorded on the Linera blockchain with instant finality.
> Watch the Blockchain Status panel update in real-time as players make moves."

**Actions**:
1. **Alice's turn (Red)**:
   - Click column 3
   - Show loading: "Recording on blockchain..."
   - Disc drops with smooth animation
   - Particle burst on landing
   - Sound effect plays
   - **Zoom in on Blockchain panel update**:
     - "Last Move: ✓ Column 3"
     - "On-Chain Moves: 1"

2. **Bob's turn (Yellow)**:
   - Click column 4
   - Show same blockchain confirmation
   - Turn indicator switches: "YOUR TURN" ↔ "OPPONENT'S TURN"
   - Move count increments: 1 → 2

3. **Continue playing** until Alice wins (4 in a row):
   - Show win detection
   - Confetti animation (200 particles)
   - Victory sound
   - Trophy emoji 🏆
   - "+25 ELO" display
   - Bob sees "DEFEAT -20 ELO"

4. **Highlight blockchain final state**:
   - "On-Chain Moves: X"
   - All moves permanently recorded

**Key Point**: Every move is provably fair and verifiable on-chain

---

### Scene 6: Mobile Responsiveness (3:00 - 3:30)
**Show**: Browser resize demonstration

**Text Overlay/Narration**:
> "The game is fully responsive. Notice how the blockchain panel and leaderboard
> remain visible even on mobile - a fix from our recent frontend improvements."

**Actions**:
1. Start with desktop view (1400px)
2. Resize to tablet (1000px):
   - Show leaderboard STILL visible
   - Show blockchain panel STILL visible
   - Layout shifts to single column

3. Resize to mobile (400px):
   - Show vertical stack
   - All features still accessible
   - No horizontal scrolling
   - Touch-friendly buttons

**Before/After note**: "Previous version hid these panels on mobile - now fixed!"

---

### Scene 7: Technical Architecture (3:30 - 4:00)
**Show**: README architecture diagram

**Text Overlay/Narration**:
> "Connect4 Battle uses Linera's unique 4-chain microchains architecture:
> Master chain for global state, Lobby for matchmaking,
> Game chains for active matches, and User chains for player profiles."

**Actions**:
- Scroll to architecture diagram in README
- Highlight 4-chain design
- Show message flow diagram
- Emphasize cross-chain messaging

---

### Scene 8: Conclusion & Call-to-Action (4:00 - 4:30)
**Show**: README features + Application ID

**Text Overlay/Narration**:
> "Connect4 Battle demonstrates Web3 gaming done right:
> - Blockchain is VISIBLE, not hidden
> - User experience rivals Web2 games
> - Every move is provably fair and permanent
> - Sub-second finality with Linera
>
> Deployed to Conway Testnet. Application ID in the README.
> Try it yourself - it's just one Docker command!"

**Actions**:
- Show Application ID in README
- Show "Quick Start" section
- End with both players playing another game

**Final Frame**:
```
Connect4 Battle on Linera
Application ID: [SHOWN ON SCREEN]
Built for WaveHack Linera Buildathon 2025
```

---

## Recording Checklist

Before starting:
- [ ] OBS/Loom configured (1920x1080, 30fps+)
- [ ] Audio tested (if doing narration)
- [ ] Browser windows arranged side-by-side
- [ ] Docker running, both frontends accessible
- [ ] Script timing practiced (should be 3-5 min)
- [ ] Zoom/highlight tools ready

During recording:
- [ ] Show blockchain panel prominently
- [ ] Emphasize one-click onboarding
- [ ] Demonstrate full game win
- [ ] Show mobile responsive layout
- [ ] Mention Conway testnet deployment
- [ ] Display Application ID clearly

After recording:
- [ ] Trim any dead air
- [ ] Add text overlays if no narration
- [ ] Export as MP4 (H.264 codec)
- [ ] Upload to YouTube (unlisted) or Loom
- [ ] Add link to README

---

## Alternative: Quick Demo (2 minutes)

If 5 minutes is too long, focus on:

1. **30s**: One-command deployment
2. **30s**: Blockchain panel close-up + PLAY NOW button
3. **60s**: Complete game showing move confirmations
4. **30s**: Application ID + architecture diagram

---

## Tips for Great Demo

1. **Show, Don't Tell**: Let the UI speak for itself
2. **Highlight Blockchain**: This is what judges want to see
3. **Smooth Pacing**: Not too fast, not too slow
4. **No Errors**: Practice run to avoid mistakes
5. **Professional**: Clean browser, no personal bookmarks visible
6. **Clear Audio**: If narrating, use good mic (or use text overlays)

---

**Script Status**: ✅ READY FOR RECORDING
**Estimated Recording Time**: 5-10 minutes (including retakes)
**Editing Time**: 5-10 minutes (trimming, overlays)
**Total Time**: 15-20 minutes for polished demo
