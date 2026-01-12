# Connect4 Battle - Phase 2 Multiplayer Test Plan

**Objective**: Verify all frontend improvements work correctly in 2-player multiplayer scenario

---

## Prerequisites

1. **Start Docker deployment**:
   ```bash
   cd "C:\Users\prate\Downloads\new prejt or buildahtin\connect4-battle"
   docker compose up --build
   ```

2. **Wait for deployment** (~2 minutes):
   - Watch logs for: "Blackjack on Microchains READY!" (or Connect4 equivalent)
   - Verify services running: `docker ps`

3. **Open both players**:
   - Player A (Red): http://localhost:5173
   - Player B (Yellow): http://localhost:5174

---

## Test Suite 1: Blockchain Visibility

### Test 1.1: Wallet Address Display
**Steps:**
1. Open Player A frontend
2. Look at right sidebar "Blockchain Status" panel

**Expected:**
- ✅ Network shows "Linera Testnet"
- ✅ Wallet shows abbreviated address (e.g., "696250037...")
- ✅ Hover over wallet shows full address tooltip
- ✅ Last Move shows "Waiting for game..."
- ✅ On-Chain Moves shows "0"

### Test 1.2: Loading Messages
**Steps:**
1. Enter name in Player A
2. Click "🎮 PLAY NOW"
3. Observe loading overlay

**Expected:**
- ✅ Loading messages rotate through blockchain-themed messages
- ✅ Messages include: "Recording on blockchain...", "⛓️ Every move is provably fair!", etc.
- ✅ Messages feel professional, not generic

### Test 1.3: Move Confirmation
**Steps:**
1. Complete matchmaking (both players click PLAY NOW)
2. Player A makes a move (click column)
3. Check blockchain status panel

**Expected:**
- ✅ "Last Move" updates to "✓ Column X"
- ✅ "On-Chain Moves" increments by 1
- ✅ Loading shows "Recording on blockchain..."

---

## Test Suite 2: One-Click Onboarding

### Test 2.1: Lobby Screen
**Steps:**
1. Fresh browser window to http://localhost:5173
2. Observe lobby screen

**Expected:**
- ✅ Title: "⚡ Connect 4 Battle ⚡"
- ✅ Subtitle: "Every move recorded on Linera blockchain • Provably fair • Competitive ELO ranking"
- ✅ Single prominent "🎮 PLAY NOW" button (green)
- ✅ NO "Create Profile" or "Connect to Lobby" buttons visible
- ✅ "Cancel Search" button hidden initially

### Test 2.2: Quick Play Flow
**Steps:**
1. Player A: Enter name "Alice"
2. Player A: Click "🎮 PLAY NOW"
3. Player B: Enter name "Bob"
4. Player B: Click "🎮 PLAY NOW"

**Expected:**
- ✅ Player A: Matchmaking spinner appears immediately
- ✅ Player B: Matchmaking spinner appears
- ✅ Both: "Finding opponent..." message shows
- ✅ Both: Match found sound plays
- ✅ Both: Game screen appears within 1-2 seconds
- ✅ Total time from click to game: <3 seconds per player

### Test 2.3: Cancel Search
**Steps:**
1. Player A: Click PLAY NOW
2. Player A: Click "Cancel Search" button

**Expected:**
- ✅ Matchmaking spinner disappears
- ✅ Returns to lobby screen
- ✅ Can click PLAY NOW again

---

## Test Suite 3: Mobile Responsiveness

### Test 3.1: Desktop Layout (>1200px)
**Steps:**
1. Resize browser to 1400px wide
2. Start game with 2 players

**Expected:**
- ✅ 3-column layout: Players | Board | Blockchain Status
- ✅ All panels visible side-by-side
- ✅ Leaderboard visible in left panel
- ✅ Blockchain status visible in right panel

### Test 3.2: Tablet Layout (800-1200px)
**Steps:**
1. Resize browser to 1000px wide
2. Game should still be active

**Expected:**
- ✅ Single column layout (stacked vertically)
- ✅ Player panel STILL VISIBLE (not hidden)
- ✅ Game board centered
- ✅ Blockchain status STILL VISIBLE (not hidden)
- ✅ Leaderboard STILL VISIBLE
- ✅ All content readable

### Test 3.3: Mobile Layout (<800px)
**Steps:**
1. Resize to 375px wide (iPhone size)
2. Verify all features accessible

**Expected:**
- ✅ Vertical stack: Players → Board → Blockchain
- ✅ NO horizontal scrolling
- ✅ Buttons touch-friendly (>44px tap targets)
- ✅ Text readable (not too small)
- ✅ Leaderboard scrollable but visible

---

## Test Suite 4: Full Game Flow

### Test 4.1: Complete Game Win
**Steps:**
1. Both players: PLAY NOW
2. Player A (Red): Make moves to create 4-in-a-row
3. Observe victory screen

**Expected:**
- ✅ Turn indicator shows "YOUR TURN" / "OPPONENT'S TURN"
- ✅ Disc drop animation smooth (0.5s cubic-bezier)
- ✅ Particle burst on disc landing (8 particles radial)
- ✅ Sound plays on each move
- ✅ Blockchain panel updates after each move
- ✅ Win detected correctly
- ✅ Confetti animation on winner's screen (200 particles)
- ✅ Victory sound plays
- ✅ Trophy emoji shows
- ✅ "VICTORY!" text displays
- ✅ ELO change shows "+25"
- ✅ Loser sees "DEFEAT" and "-20 ELO"

### Test 4.2: Full Game Draw
**Steps:**
1. Both players: Fill board with no 4-in-a-row

**Expected:**
- ✅ "DRAW!" banner appears
- ✅ No confetti
- ✅ "No ELO change" message
- ✅ Both players see draw state

### Test 4.3: Surrender
**Steps:**
1. During active game, Player A clicks "Surrender"
2. Confirm dialog

**Expected:**
- ✅ Confirmation dialog appears
- ✅ After confirm, Player B wins immediately
- ✅ Victory/defeat screens show
- ✅ ELO changes applied

---

## Test Suite 5: Sound System

### Test 5.1: First Interaction Audio
**Steps:**
1. Fresh browser window
2. Click PLAY NOW button (first click)
3. Hover over board column

**Expected:**
- ✅ Click sound plays immediately
- ✅ Hover sound plays
- ✅ No console errors about suspended AudioContext

### Test 5.2: All Sounds
**Steps:**
During a game, trigger each sound:
1. Hover board column (hover sound)
2. Click column (click + drop sound)
3. Match found (match sound)
4. Win game (win sound)
5. Lose game (lose sound)

**Expected:**
- ✅ All 6 sounds play clearly
- ✅ Volume appropriate (30% by default)
- ✅ No distortion or clipping

### Test 5.3: Sound Toggle
**Steps:**
1. Click sound icon (top right, second status)
2. Make a move
3. Click sound icon again
4. Make another move

**Expected:**
- ✅ Icon changes: 🔊 → 🔇
- ✅ Text changes: "Sound On" → "Sound Off"
- ✅ Sounds actually mute/unmute
- ✅ Preference persists during session

---

## Test Suite 6: Professional Polish

### Test 6.1: No Developer UI Visible
**Steps:**
1. Play complete game
2. Check all screens (lobby, game, victory)
3. Open browser console

**Expected:**
- ✅ NO "Connected to localhost:8081" messages visible to user
- ✅ NO technical error messages in UI
- ✅ NO "log" function console spam
- ✅ Only user-friendly messages visible

### Test 6.2: Visual Cleanliness
**Steps:**
1. Observe game board during active game

**Expected:**
- ✅ NO column number indicators (1-7) below board
- ✅ Preview disc shows on hover (sufficient guidance)
- ✅ Clean, uncluttered design
- ✅ Professional aesthetic

### Test 6.3: CSS Validity
**Steps:**
1. Open browser DevTools
2. Check for CSS errors

**Expected:**
- ✅ NO invalid CSS warnings (e.g., "20%%" instead of "20%")
- ✅ Floating disc backgrounds render correctly
- ✅ All gradients and animations smooth

---

## Test Suite 7: Performance

### Test 7.1: Animation Smoothness
**Steps:**
1. Make 10 rapid moves (fill columns)
2. Observe animations

**Expected:**
- ✅ Disc drop maintains 60fps
- ✅ Confetti doesn't cause frame drops
- ✅ Particle bursts smooth
- ✅ No visible lag or stutter

### Test 7.2: Memory Leaks
**Steps:**
1. Play 3 complete games in a row
2. Check browser DevTools Memory tab

**Expected:**
- ✅ Memory doesn't grow unbounded
- ✅ Particle effects clean up properly
- ✅ Confetti elements removed after animation
- ✅ No orphaned event listeners

---

## Test Suite 8: Cross-Browser Compatibility

### Test 8.1: Chrome
**Expected:** ✅ All features work

### Test 8.2: Firefox
**Expected:** ✅ All features work

### Test 8.3: Safari
**Expected:** ✅ All features work (Web Audio API might differ)

### Test 8.4: Edge
**Expected:** ✅ All features work

---

## Critical Path Test (2-Minute Judge Simulation)

**Scenario**: Judge evaluating project with 2-minute time limit

**Steps:**
1. **0:00** - Open http://localhost:5173
2. **0:05** - See "⚡ Connect 4 Battle ⚡" with blockchain messaging
3. **0:10** - Notice blockchain status panel (wallet address visible)
4. **0:15** - Enter name "Judge"
5. **0:20** - Click "🎮 PLAY NOW" button
6. **0:22** - Matchmaking starts (spinner shows)
7. **0:25** - Open second tab http://localhost:5174
8. **0:30** - Enter name "Player2", click PLAY NOW
9. **0:32** - Match found! Game screen appears
10. **0:35** - Notice blockchain panel: wallet, on-chain moves = 0
11. **0:40** - Judge makes first move (column 3)
12. **0:41** - Loading: "Recording on blockchain..."
13. **0:42** - Disc drops with animation + particle burst + sound
14. **0:43** - Blockchain panel updates: "Last Move: ✓ Column 3", "On-Chain Moves: 1"
15. **0:45** - Turn banner changes to "OPPONENT'S TURN"
16. **0:50** - Opponent makes move (visible in other tab)
17. **0:55** - Notice: smooth animations, clean UI, blockchain always visible
18. **1:00** - Resize to mobile size (~400px wide)
19. **1:05** - Verify: Leaderboard STILL VISIBLE (not hidden like before)
20. **1:10** - Verify: Blockchain panel STILL VISIBLE
21. **1:15** - Make winning moves (demonstrates game logic)
22. **1:45** - Win achieved, confetti animation plays
23. **1:50** - See "+25 ELO" and "💎 Your move is permanent on-chain" message
24. **1:55** - Click "Play Again"
25. **2:00** - **Judge verdict**: ✅ Blockchain visible ✅ Smooth UX ✅ Mobile works ✅ Professional

**Expected Judge Score**: 70-80 points (vs 50-60 before improvements)

---

## Bug Tracking

### Known Issues from Previous Session:
1. ✅ **FIXED**: Matchmaking stuck at "Searching..." (queue_count initialization)
2. ✅ **FIXED**: Leaderboard GraphQL field mismatch (name → playerName)
3. ✅ **FIXED**: Undefined LOBBY_CHAIN variable

### Potential New Issues to Watch:
1. ⚠️ Audio context might still need user gesture on some browsers
2. ⚠️ Confetti performance on low-end devices (200 DOM elements)
3. ⚠️ Polling interval (1.5s) might feel laggy vs real-time events

---

## Success Criteria

To proceed to PHASE 3 (Judge Criteria Verification), ALL must pass:

- [ ] Blockchain panel shows wallet address
- [ ] Blockchain panel updates on every move
- [ ] One-click PLAY NOW button works end-to-end
- [ ] Mobile layout shows leaderboard and blockchain status
- [ ] No developer UI visible to users
- [ ] Column indicators NOT visible
- [ ] Loading messages are blockchain-themed
- [ ] Audio works on first interaction
- [ ] Full game playable from start to finish
- [ ] 2-browser multiplayer synchronizes correctly
- [ ] Victory screen shows ELO changes
- [ ] Performance smooth (no lag or crashes)

**If ANY test fails**: Document in bug report, fix immediately, re-test.

---

## Test Execution Log

_To be filled out during testing:_

| Test ID | Status | Notes | Tester | Date |
|---------|--------|-------|--------|------|
| 1.1 | ⏳ | | | |
| 1.2 | ⏳ | | | |
| 1.3 | ⏳ | | | |
| ... | | | | |

---

**Next Phase**: After all tests pass → PHASE 3: Judge Criteria Verification
