@echo off
echo ================================================
echo   CONNECT4 BATTLE - DOCKER DEPLOYMENT
echo ================================================
echo.

cd /d "%~dp0"

echo Starting Docker container...
echo This will:
echo   - Start Linera blockchain network
echo   - Deploy Connect4 and Bankroll apps
echo   - Start frontend servers
echo.
echo Please wait 30-60 seconds for deployment...
echo ================================================
echo.

docker-compose up -d

echo.
echo Waiting for services to start...
timeout /t 30 /nobreak >nul

echo.
echo Opening browsers...
start http://localhost:5173
timeout /t 2 /nobreak >nul
start http://localhost:5174

echo.
echo ================================================
echo   GAME IS READY!
echo ================================================
echo.
echo Player A (Red):    http://localhost:5173
echo Player B (Yellow): http://localhost:5174
echo.
echo Check Docker logs: docker-compose logs -f
echo Stop servers:      docker-compose down
echo.
echo ================================================
pause
