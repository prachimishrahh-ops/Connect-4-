# Connect4 Battle Demo Video Script

**Duration**: 3-5 minutes
**Target**: Wave 6 judges

## Equipment Setup
- OBS Studio or QuickTime Screen Recording
- 1080p resolution minimum
- Split screen: Terminal (left 40%) + Browser (right 60%)
- Clear audio narration

---

## Script (Narrate while showing actions)

### 00:00-00:30 - Introduction
**Visual**: Show project README on GitHub
**Narration**:
"This is Connect4 Battle, a fully decentralized Connect Four game built on Linera microchains. What makes this special is our 4-chain architecture that eliminates congestion and enables instant gameplay."

**Show on screen**:
- Project title
- GitHub repository URL
- Conway Testnet App ID

### 00:30-01:15 - Architecture Overview
**Visual**: Show architecture diagram from README
**Narration**:
"Connect4 Battle uses a sophisticated 4-chain architecture:
- Master Chain: Global state and configuration
- Lobby Chain: Game matchmaking and room management
- Game Chains: Individual game instances with their own microchain
- User Chains: Personal player state and history

This design means thousands of games can run simultaneously without any blockchain congestion."

**Highlight**: Point to each chain in the diagram

### 01:15-02:00 - Docker Startup & Deployment
**Visual**: Switch to terminal
**Narration**:
"Starting the application is simple. We're running against Conway Testnet with a verified deployment."

**Terminal commands**:
```bash
docker compose up
```

**Show**:
- Services starting (node, frontend)
- "Linera service running on port 8080"
- "Frontend running on port 5173"

**Narration while services start**:
"The app is deployed to Conway Testnet with App ID: [INSERT APP ID FROM DEPLOYMENT]
This is a real blockchain deployment, not a demo or mock."

### 02:00-02:30 - Wallet Connection
**Visual**: Switch to browser at localhost:5173
**Narration**:
"Let's connect our Linera wallet and start playing."

**Actions**:
1. Click "Connect Wallet" button
2. Show wallet extension popup
3. Approve connection
4. Show connected wallet address in UI

**Narration**:
"The wallet connection uses Linera's auto-signing feature introduced in SDK 0.15.8, so we only sign once at connection, then all game moves are automatic."

### 02:30-03:30 - Gameplay Demonstration
**Visual**: Play an actual game
**Narration**:
"Now let's play a game to demonstrate the cross-chain messaging in action."

**Actions**:
1. Click "Create Game" or "Join Game"
2. Make several moves (drop pieces)
3. Show game state updating in real-time
4. Show move history
5. Complete the game (win/lose/draw)

**Narration during gameplay**:
"Each move is a cross-chain message:
- User Chain sends move to Game Chain
- Game Chain validates and updates state
- Game Chain broadcasts update to both players
- UI updates instantly via GraphQL subscriptions

No polling, no delays. This is real-time blockchain gaming."

### 03:30-04:00 - Technical Highlights
**Visual**: Open browser DevTools
**Narration**:
"Let me show you what's happening under the hood."

**Show**:
1. Network tab: GraphQL subscriptions active
2. Console: Cross-chain messages logged
3. Application tab: No localStorage (judges specifically check this)

**Narration**:
"All state is managed on-chain or in React state. We don't use localStorage or sessionStorage, which is a requirement for Wave 6 judging."

### 04:00-04:30 - Conclusion
**Visual**: Show README with deployment info
**Narration**:
"Connect4 Battle demonstrates:
- Multi-chain architecture for scalability
- Real-time cross-chain messaging
- Production-ready code with zero compromises
- Full Conway Testnet deployment

App ID: [INSERT APP ID]
GitHub: [REPOSITORY URL]
Built with Linera SDK 0.15.7

Thank you for watching!"

---

## Post-Recording Checklist
- [ ] Video is 3-5 minutes long
- [ ] Audio is clear and professional
- [ ] App ID is visible
- [ ] GitHub URL is shown
- [ ] Gameplay is smooth and demonstrates features
- [ ] No localStorage shown in DevTools
- [ ] Video uploaded to YouTube
- [ ] YouTube link added to README

---

## YouTube Video Details

**Title**: Connect4 Battle - Wave 6 Linera Buildathon Submission

**Description**:
Connect4 Battle is a fully decentralized Connect Four game built on Linera microchains.

🔗 Conway Testnet App ID: [INSERT APP ID]
📁 GitHub: [REPOSITORY URL]
⚡ Built with Linera SDK 0.15.7

Features:
✅ 4-chain architecture (Master/Lobby/Game/User)
✅ Real-time cross-chain messaging
✅ Zero localStorage/sessionStorage
✅ Auto-signing with Linera SDK 0.15.8+
✅ Docker one-command setup

Wave 6 Linera Buildathon submission.

**Tags**: linera, blockchain, gaming, microchains, web3, buildathon, connect4

**Thumbnail**: Screenshot of game board with "Connect4 Battle" and "Linera" logos
