@echo off
echo ================================================
echo   CONNECT4 BATTLE - Starting Game Servers
echo ================================================
echo.

echo Starting Player A (Red) on http://localhost:8000
echo Starting Player B (Yellow) on http://localhost:8001
echo.
echo Press Ctrl+C to stop servers
echo ================================================
echo.

start "Player A Server" cmd /k "cd frontend\web_a && python -m http.server 8000"
timeout /t 2 /nobreak >nul

start "Player B Server" cmd /k "cd frontend\web_b && python -m http.server 8001"
timeout /t 2 /nobreak >nul

echo.
echo Servers started! Opening browsers...
echo.

start http://localhost:8000
timeout /t 1 /nobreak >nul
start http://localhost:8001

echo.
echo ================================================
echo   GAME SERVERS RUNNING
echo ================================================
echo.
echo Player A (Red):    http://localhost:8000
echo Player B (Yellow): http://localhost:8001
echo.
echo Keep this window open while playing.
echo Press any key to stop servers and exit...
pause >nul

taskkill /FI "WINDOWTITLE eq Player A Server*" /F
taskkill /FI "WINDOWTITLE eq Player B Server*" /F
