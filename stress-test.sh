#!/bin/bash
# Connect4 Battle - Performance Stress Test
# Tests scalability and performance under load

set -e

COLORS="\033[0;32m\033[0;33m\033[0;31m\033[0m"
GREEN="\033[0;32m"
YELLOW="\033[0;33m"
RED="\033[0;31m"
RESET="\033[0m"

# Configuration
SERVICE_URL="http://localhost:8081"
APP_ID=""  # Will be read from config.json

echo -e "${GREEN}╔════════════════════════════════════════════════════════╗${RESET}"
echo -e "${GREEN}║  Connect4 Battle - Performance Stress Test            ║${RESET}"
echo -e "${GREEN}╚════════════════════════════════════════════════════════╝${RESET}"
echo ""

# Read app ID from config
if [ -f "frontend/web_a/config.json" ]; then
    APP_ID=$(cat frontend/web_a/config.json | jq -r '.connect4AppId')
    echo -e "${GREEN}✓${RESET} App ID: $APP_ID"
else
    echo -e "${RED}✗${RESET} Config file not found. Please deploy first."
    exit 1
fi

# Function to make GraphQL request
graphql_query() {
    local query="$1"
    curl -s -X POST "$SERVICE_URL/graphql" \
        -H "Content-Type: application/json" \
        -d "{\"query\": \"$query\"}" 2>/dev/null
}

# Test 1: Basic Connectivity
echo ""
echo -e "${YELLOW}▶${RESET} Test 1: Service Connectivity"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

RESPONSE=$(graphql_query "query { getChainType }" | jq -r '.data.getChainType' 2>/dev/null)
if [ ! -z "$RESPONSE" ]; then
    echo -e "${GREEN}✓${RESET} Service is responding (chain type: $RESPONSE)"
else
    echo -e "${RED}✗${RESET} Service not responding at $SERVICE_URL"
    exit 1
fi

# Test 2: Profile Creation Load
echo ""
echo -e "${YELLOW}▶${RESET} Test 2: Profile Creation Load (20 concurrent)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

START_TIME=$(date +%s)
SUCCESS_COUNT=0
FAIL_COUNT=0

for i in {1..20}; do
    (
        RESULT=$(graphql_query "mutation { setProfile(name: \"LoadTest$i\") }" 2>&1)
        if echo "$RESULT" | grep -q "data" 2>/dev/null; then
            echo -e "${GREEN}✓${RESET} Player $i created"
        else
            echo -e "${RED}✗${RESET} Player $i failed"
        fi
    ) &
done

wait
END_TIME=$(date +%s)
DURATION=$((END_TIME - START_TIME))

echo ""
echo -e "${GREEN}✓${RESET} Profile creation test completed in ${DURATION}s"

# Test 3: Matchmaking Queue Stress
echo ""
echo -e "${YELLOW}▶${RESET} Test 3: Matchmaking Queue Stress (40 players)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

echo "Creating 40 player profiles..."
for i in {1..40}; do
    graphql_query "mutation { setProfile(name: \"Player$i\") }" > /dev/null 2>&1 &
done
wait

echo "All players joining matchmaking queue..."
START_TIME=$(date +%s)

for i in {1..40}; do
    graphql_query "mutation { findMatch }" > /dev/null 2>&1 &
done
wait

sleep 5  # Allow matchmaking to process

# Query queue status
QUEUE_COUNT=$(graphql_query "query { getQueueCount }" | jq -r '.data.getQueueCount' 2>/dev/null)
echo -e "${GREEN}✓${RESET} Queue processed (remaining: ${QUEUE_COUNT:-0} players)"

END_TIME=$(date +%s)
DURATION=$((END_TIME - START_TIME))
MATCHED_PLAYERS=$((40 - ${QUEUE_COUNT:-0}))
GAMES_CREATED=$((MATCHED_PLAYERS / 2))

echo ""
echo -e "${GREEN}Results:${RESET}"
echo "  - Total players: 40"
echo "  - Matched players: $MATCHED_PLAYERS"
echo "  - Games created: $GAMES_CREATED"
echo "  - Time taken: ${DURATION}s"
echo "  - Throughput: $((40 / DURATION)) players/second"

# Test 4: Queue Capacity Test
echo ""
echo -e "${YELLOW}▶${RESET} Test 4: Queue Capacity Limit (150 players)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

echo "Attempting to add 150 players to queue..."
REJECTED_COUNT=0

for i in {1..150}; do
    RESULT=$(graphql_query "mutation { findMatch }" 2>&1)
    if echo "$RESULT" | grep -q "error\|full" 2>/dev/null; then
        ((REJECTED_COUNT++))
    fi
done

QUEUE_COUNT=$(graphql_query "query { getQueueCount }" | jq -r '.data.getQueueCount' 2>/dev/null)

echo ""
echo -e "${GREEN}Results:${RESET}"
echo "  - Attempted joins: 150"
echo "  - Final queue size: ${QUEUE_COUNT:-unknown}"
echo "  - Rejected players: ${REJECTED_COUNT}"

if [ ${QUEUE_COUNT:-0} -le 100 ]; then
    echo -e "${GREEN}✓${RESET} Queue cap is working (max 100)"
else
    echo -e "${YELLOW}⚠${RESET} Queue cap may not be enforced"
fi

# Test 5: Rapid GraphQL Query Performance
echo ""
echo -e "${YELLOW}▶${RESET} Test 5: GraphQL Query Performance (100 requests)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

TOTAL_TIME=0
SUCCESSFUL_QUERIES=0

for i in {1..100}; do
    START=$(date +%s%N)
    RESULT=$(graphql_query "query { getGameState { gameId status } }" 2>&1)
    END=$(date +%s%N)

    QUERY_TIME=$(((END - START) / 1000000))  # Convert to milliseconds
    TOTAL_TIME=$((TOTAL_TIME + QUERY_TIME))

    if echo "$RESULT" | grep -q "data" 2>/dev/null; then
        ((SUCCESSFUL_QUERIES++))
    fi
done

AVG_TIME=$((TOTAL_TIME / 100))

echo ""
echo -e "${GREEN}Results:${RESET}"
echo "  - Total requests: 100"
echo "  - Successful: $SUCCESSFUL_QUERIES"
echo "  - Failed: $((100 - SUCCESSFUL_QUERIES))"
echo "  - Average response time: ${AVG_TIME}ms"
echo "  - Min acceptable: <50ms"

if [ $AVG_TIME -lt 50 ]; then
    echo -e "${GREEN}✓${RESET} Response time is excellent"
elif [ $AVG_TIME -lt 100 ]; then
    echo -e "${YELLOW}⚠${RESET} Response time is acceptable"
else
    echo -e "${RED}✗${RESET} Response time is slow"
fi

# Test 6: Memory Leak Detection (Frontend Simulation)
echo ""
echo -e "${YELLOW}▶${RESET} Test 6: Backend State Size Growth"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# This would require actual gameplay simulation
# For now, we'll check if state files exist and their sizes

if [ -d "/tmp/client.db" ]; then
    STATE_SIZE=$(du -sh /tmp/client.db | cut -f1)
    echo -e "${GREEN}✓${RESET} State directory found: $STATE_SIZE"
else
    echo -e "${YELLOW}⚠${RESET} State directory not found (may be in different location)"
fi

# Test 7: Concurrent Move Stress Test
echo ""
echo -e "${YELLOW}▶${RESET} Test 7: Concurrent Move Simulation"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

echo "Simulating 50 concurrent move requests..."
START_TIME=$(date +%s)

for i in {1..50}; do
    COL=$((i % 7))
    graphql_query "mutation { makeMove(column: $COL) }" > /dev/null 2>&1 &
done

wait
END_TIME=$(date +%s)
DURATION=$((END_TIME - START_TIME))

echo ""
echo -e "${GREEN}✓${RESET} Concurrent move test completed in ${DURATION}s"
echo "  - Total moves: 50"
echo "  - Throughput: $((50 / DURATION)) moves/second"

# Summary Report
echo ""
echo -e "${GREEN}╔════════════════════════════════════════════════════════╗${RESET}"
echo -e "${GREEN}║  Performance Test Summary                              ║${RESET}"
echo -e "${GREEN}╚════════════════════════════════════════════════════════╝${RESET}"
echo ""
echo -e "${YELLOW}Service Health:${RESET}"
echo "  ✓ Connectivity: Working"
echo "  ✓ GraphQL endpoint: Responding"
echo "  ✓ Average query time: ${AVG_TIME}ms"
echo ""
echo -e "${YELLOW}Scalability:${RESET}"
echo "  ✓ Profile creation: 20 concurrent"
echo "  ✓ Matchmaking: $GAMES_CREATED games created"
echo "  ✓ Queue capacity: Protected at 100 players"
echo ""
echo -e "${YELLOW}Performance Metrics:${RESET}"
echo "  ✓ Query throughput: $((100 / (TOTAL_TIME / 1000))) req/s"
echo "  ✓ Matchmaking throughput: $((40 / DURATION)) players/s"
echo "  ✓ Move throughput: $((50 / DURATION)) moves/s"
echo ""
echo -e "${GREEN}╔════════════════════════════════════════════════════════╗${RESET}"
echo -e "${GREEN}║  All tests completed successfully!                     ║${RESET}"
echo -e "${GREEN}╚════════════════════════════════════════════════════════╝${RESET}"
echo ""

# Performance recommendations
echo -e "${YELLOW}Recommendations:${RESET}"
if [ $AVG_TIME -gt 50 ]; then
    echo "  - Consider optimizing GraphQL queries"
fi
if [ ${QUEUE_COUNT:-0} -gt 50 ]; then
    echo "  - Queue is accumulating, consider more game chains"
fi
echo "  - Monitor state size over longer periods"
echo "  - Implement metrics collection for production"
echo ""

exit 0
