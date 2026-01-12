#!/bin/bash

echo "================================================"
echo "  CONNECT4 BATTLE - Starting Game Servers"
echo "================================================"
echo ""

echo "Starting Player A (Red) on http://localhost:8000"
echo "Starting Player B (Yellow) on http://localhost:8001"
echo ""
echo "Press Ctrl+C to stop servers"
echo "================================================"
echo ""

# Start Player A server in background
cd frontend/web_a
python3 -m http.server 8000 &
PID_A=$!
cd ../..

# Wait a moment
sleep 2

# Start Player B server in background
cd frontend/web_b
python3 -m http.server 8001 &
PID_B=$!
cd ../..

# Wait a moment
sleep 2

echo ""
echo "Servers started! Opening browsers..."
echo ""

# Open browsers (works on most systems)
if command -v xdg-open &> /dev/null; then
    xdg-open http://localhost:8000 &
    sleep 1
    xdg-open http://localhost:8001 &
elif command -v open &> /dev/null; then
    open http://localhost:8000 &
    sleep 1
    open http://localhost:8001 &
else
    echo "Please manually open:"
    echo "  http://localhost:8000 (Player A)"
    echo "  http://localhost:8001 (Player B)"
fi

echo ""
echo "================================================"
echo "  GAME SERVERS RUNNING"
echo "================================================"
echo ""
echo "Player A (Red):    http://localhost:8000"
echo "Player B (Yellow): http://localhost:8001"
echo ""
echo "Press Ctrl+C to stop servers..."
echo ""

# Wait for Ctrl+C
trap "kill $PID_A $PID_B; exit" INT TERM
wait
