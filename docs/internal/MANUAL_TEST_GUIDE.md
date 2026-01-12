# Connect4 Battle - Manual Testing Guide

**Purpose**: Verify complete multiplayer flow including victory screen
**Time Required**: 10-15 minutes
**Updated**: January 12, 2026, 15:30 IST

---

## 🎮 How to Test the Full Game

### Step 1: Open Both Players (2 minutes)

1. **Open Player A (Red)**
   - Browser: http://localhost:5173
   - Press F12 to open Developer Console
   - Watch console logs throughout the test

2. **Open Player B (Yellow)**
   - Browser: http://localhost:5174
   - Press F12 to open Developer Console
   - Keep both browsers visible side-by-side

---

### Step 2: Matchmaking (1 minute)

**Player A**:
1. Enter name: "TestRed"
2. Click **PLAY NOW**
3. ✅ **EXPECTED**: "Finding opponent..." message appears
4. ✅ **CHECK CONSOLE**: Should see quickPlay() logs

**Player B**:
1. Enter name: "TestYellow"
2. Click **PLAY NOW**
3. ✅ **EXPECTED**: Both players transition to game screen
4. ✅ **CHECK CONSOLE**: Should see "MATCH FOUND!"

**If matchmaking fails**:
- Wait 10 seconds
- Check Docker is running: `docker compose ps`
- Refresh both browsers and try again

---

### Step 3: Play to Victory (5-7 minutes)

**IMPORTANT**: Wait 3-5 seconds between each move to allow blockchain processing!

#### Winning Strategy: Red Vertical in Column 4

| Move # | Player | Column to Click | Expected Result |
|--------|--------|----------------|-----------------|
| 1 | Red (A) | Column **4** | Red disc appears bottom row |
| 2 | Yellow (B) | Column **5** | Yellow disc appears bottom row |
| 3 | Red (A) | Column **4** | Red disc stacks on previous red |
| 4 | Yellow (B) | Column **5** | Yellow disc stacks on previous yellow |
| 5 | Red (A) | Column **4** | 3rd red disc in column 4 |
| 6 | Yellow (B) | Column **5** | 3rd yellow disc in column 5 |
| 7 | Red (A) | Column **4** | 🏆 **WINNING MOVE!** |

**What to Click**:
- Columns are numbered 1-7 from left to right
- Click the number button ABOVE the column
- OR click directly on the column itself

**After Each Move**:
- ✅ **CHECK**: Disc appears on both player screens
- ✅ **CHECK**: Turn indicator switches
- ✅ **CHECK**: Move counter increments
- ✅ **WAIT**: 3-5 seconds before next move

---

### Step 4: Victory Verification (2 minutes)

**After Move 7, you should see**:

#### Player A (Winner - Red) should show:
- 🏆 Trophy emoji
- "VICTORY!" text
- "+25 ELO" message
- 🎊 **Confetti animation** (200 particles falling)
- 🔊 Victory sound effect
- Buttons: "PLAY AGAIN" and "EXIT"

#### Player B (Loser - Yellow) should show:
- 😞 Sad emoji
- "DEFEAT" text
- "-20 ELO" message
- **NO confetti** (losers don't get confetti)
- 🔊 Defeat sound effect
- Buttons: "PLAY AGAIN" and "EXIT"

#### Both Players should show:
- Game board with all 7 discs placed
- 4 red discs in column 4 (vertical line)
- 3 yellow discs in column 5

---

## 🐛 Troubleshooting

### Issue: Victory Screen Doesn't Appear

**Possible Causes**:
1. **Move 7 didn't execute**
   - Check console: Should see "✅ Move validation passed"
   - Check browser: Should see 7 total discs on board
   - **FIX**: Refresh state, wait longer between moves

2. **Backend didn't detect win**
   - Test backend directly:
     ```bash
     curl -s "http://localhost:8083/chains/1b0a6e2d8f362e4322227779916fcf55634b0a6a79e94330487254978829f94c/applications/c2bad7b457c04e6da461120b5f92b460fc795cea0628e219ca91c196a0b57c4d" -X POST -H "Content-Type: application/json" -d '{"query":"query { getCurrentGame { status winner } }"}'
     ```
   - **EXPECTED**: `{"data":{"getCurrentGame":{"status":"Finished","winner":"Red"}}}`

3. **Frontend polling not catching update**
   - **FIX**: Click browser refresh after move 7
   - **CHECK**: Game state should update

### Issue: Move Rejected "Not Your Turn"

**Console will show**:
```
❌ Move rejected: Not your turn (currentTurn: Yellow, myColor: Red)
```

**Solution**:
- Wait 3-5 more seconds
- The blockchain is still processing the previous move
- Check "Turn:" indicator at top of screen

### Issue: Move Rejected "Column Full"

**Console will show**:
```
❌ Move rejected: Column 4 is full
```

**Solution**:
- You clicked the wrong column (each column holds max 6 discs)
- Click a different column number

---

## ✅ Success Checklist

Use this to verify everything works:

### Matchmaking Phase
- [ ] Player A enters name successfully
- [ ] Player B enters name successfully
- [ ] Both players click PLAY NOW
- [ ] "Finding opponent..." message shows
- [ ] Game screen loads for both players within 5 seconds
- [ ] Console logs show "MATCH FOUND!" on both

### Gameplay Phase
- [ ] Move 1 executes successfully (Red column 4)
- [ ] Move 2 executes successfully (Yellow column 5)
- [ ] Move 3 executes successfully (Red column 4)
- [ ] Move 4 executes successfully (Yellow column 5)
- [ ] Move 5 executes successfully (Red column 4)
- [ ] Move 6 executes successfully (Yellow column 5)
- [ ] Move 7 executes successfully (Red column 4 - WINNING)

### Board Synchronization
- [ ] Both players see identical board state
- [ ] Move counter shows same number on both (0→1→2→...→7)
- [ ] Turn indicator switches correctly after each move
- [ ] No desyncs or missing discs

### Victory Phase
- [ ] Victory screen appears automatically after move 7
- [ ] Winner sees "VICTORY!" message
- [ ] Winner sees +25 ELO
- [ ] Winner sees 🏆 trophy emoji
- [ ] **Winner sees confetti animation**
- [ ] **Victory sound plays** (if sound is on)
- [ ] Loser sees "DEFEAT" message
- [ ] Loser sees -20 ELO
- [ ] Loser sees 😞 sad emoji
- [ ] **Defeat sound plays** (if sound is on)

### Console Logs (Optional but Helpful)
- [ ] Each move shows "🎯 makeMove called: column X"
- [ ] Each move shows "✅ Move validation passed"
- [ ] Each move shows "✅ Move mutation successful"
- [ ] No "❌ Move rejected" errors (unless intentional wrong turn)
- [ ] Game state updates show "🎮 Game state from game chain: Found!"

---

## 📝 Report Template

If you encounter issues, please provide:

```
**Issue**: [Brief description]
**Step**: [Which step from the guide]
**Move Number**: [1-7, or matchmaking]
**Player Affected**: [A, B, or both]
**Console Errors**: [Copy relevant console messages]
**Screenshot**: [Optional but helpful]
```

Example:
```
**Issue**: Victory screen didn't appear
**Step**: Step 4 - Victory Verification
**Move Number**: After move 7
**Player Affected**: Both
**Console Errors**:
  Player A: "❌ Move rejected: Not your turn (currentTurn: Yellow, myColor: Red)"
  Player B: [No errors]
**Screenshot**: [Attached]
```

---

## 🎯 Expected Total Test Time

- Matchmaking: 1 minute
- 7 Moves (with 4s wait each): 5-6 minutes
- Victory verification: 1 minute
- **Total: ~8-10 minutes**

---

## 💡 Pro Tips

1. **Keep Console Open**: Press F12 in both browsers - the console logs are incredibly helpful!

2. **Wait Between Moves**: The blockchain needs 3-5 seconds to process each move. Don't rush!

3. **Watch Both Screens**: Keep both browsers visible to see synchronization in real-time.

4. **Sound On**: Turn on sound (click 🔊 icon) to hear victory/defeat effects.

5. **Take Screenshots**: If victory screen appears, take screenshots - they're great for documentation!

6. **Test Twice**: Run the test twice to verify consistency:
   - First time: Follow the guide exactly
   - Second time: Try different column combinations

---

## 🚀 Advanced Testing (Optional)

### Test Different Win Conditions

1. **Horizontal Win** (Row 1, columns 1-2-3-4):
   - Harder to set up, requires specific move sequence

2. **Diagonal Win**:
   - Most complex, requires careful planning

3. **Draw** (Board fills without winner):
   - Very rare in Connect4, requires 42 moves

### Test Edge Cases

1. **Click wrong column during opponent's turn**
   - **EXPECTED**: Console shows "❌ Move rejected: Not your turn"

2. **Try to fill a full column**
   - **EXPECTED**: Console shows "❌ Move rejected: Column X is full"

3. **Rapid clicking**
   - **EXPECTED**: Only one move executes, others rejected

---

## ✨ Success Indicators

If you see ALL of these, the game is working perfectly:

✅ Matchmaking completes in < 10 seconds
✅ All 7 moves execute successfully
✅ Board stays synchronized between players
✅ Victory screen appears automatically
✅ Confetti animation plays for winner
✅ Victory/defeat sounds play
✅ No console errors during normal gameplay

**If ALL checkboxes pass: The game is production-ready!** 🎉

---

*Happy Testing!*
*For issues or questions, check the UX_IMPROVEMENTS_REPORT.md document*
