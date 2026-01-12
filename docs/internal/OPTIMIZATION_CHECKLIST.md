# Connect4 Battle - Performance Optimization Checklist

**Quick Reference Guide for Implementation**

---

## 🔴 CRITICAL (Fix Immediately)

### 1. Memory Leaks
- [ ] Add cleanup for all setInterval calls
- [ ] Store interval IDs in array for cleanup
- [ ] Add beforeunload event listener
- [ ] Close AudioContext on cleanup
- [ ] Implement GameManager lifecycle class

**File**: Lines 1441, 598-619
**Time**: 2 hours
**Impact**: Prevents memory growth

---

### 2. Reduce Polling Frequency
- [ ] Change matchmaking polling from 300ms to 1000ms
- [ ] Change gameplay polling from 500ms to 2000ms
- [ ] Implement adaptive polling based on game state
- [ ] Add Visibility API to stop polling when tab hidden

**File**: Lines 1422-1441
**Time**: 4 hours
**Impact**: 70% network reduction

---

### 3. Cache DOM References
- [ ] Create BoardRenderer class
- [ ] Pre-cache all 42 cell elements at initialization
- [ ] Store references in Map for O(1) lookup
- [ ] Replace getElementById calls with cache lookups

**File**: Lines 1133-1202
**Time**: 3 hours
**Impact**: 95% fewer DOM queries

---

## ⚠️ HIGH PRIORITY (Week 1)

### 4. Batch GraphQL Queries
- [ ] Combine getUserGameChain, getUserColor, getUserProfile into one query
- [ ] Only query game chain if game exists
- [ ] Remove duplicate leaderboard fetches

**File**: Lines 1045-1070
**Time**: 2 hours
**Impact**: 50% fewer requests

---

### 5. Incremental DOM Updates
- [ ] Implement board diffing algorithm
- [ ] Only update changed cells
- [ ] Skip updates if state unchanged
- [ ] Use requestAnimationFrame for batching

**File**: Lines 1155-1202
**Time**: 6 hours
**Impact**: 90% fewer DOM operations

---

### 6. Fix Layout Thrashing
- [ ] Remove offsetHeight reads during animation
- [ ] Separate read/write operations
- [ ] Batch all reads, then batch all writes
- [ ] Use CSS classes instead of inline styles

**File**: Lines 1170-1172
**Time**: 3 hours
**Impact**: 60% reduction in reflows

---

## 🟡 MEDIUM PRIORITY (Week 2)

### 7. Code Splitting
- [ ] Extract CSS to separate file
- [ ] Extract JavaScript to separate file
- [ ] Lazy load sound system
- [ ] Lazy load particle effects

**Time**: 4 hours
**Impact**: 77% smaller initial load

---

### 8. Canvas Particle System
- [ ] Replace DOM particles with Canvas
- [ ] Create ParticleRenderer class
- [ ] Use requestAnimationFrame loop
- [ ] Implement object pooling

**File**: Lines 1349-1389
**Time**: 8 hours
**Impact**: 90% particle overhead reduction

---

### 9. Optimize Confetti
- [ ] Pre-create 200 confetti elements
- [ ] Implement ConfettiPool class
- [ ] Use Document Fragment for batch insertion
- [ ] Reuse elements instead of recreating

**File**: Lines 1307-1346
**Time**: 4 hours
**Impact**: 85% confetti overhead reduction

---

### 10. Add Service Worker
- [ ] Create service-worker.js
- [ ] Cache static assets
- [ ] Implement offline-first strategy
- [ ] Add cache versioning

**Time**: 3 hours
**Impact**: Instant repeat loads

---

## 🟢 OPTIMIZATION (Week 3)

### 11. Request Deduplication
- [ ] Create RequestDeduplicator class
- [ ] Track pending requests
- [ ] Return existing promise if request in flight
- [ ] Clear tracking when request completes

**Time**: 3 hours
**Impact**: 40% duplicate reduction

---

### 12. Response Caching
- [ ] Create ResponseCache class
- [ ] Cache leaderboard (TTL: 10s)
- [ ] Cache user profile (TTL: 30s)
- [ ] Implement cache invalidation

**Time**: 3 hours
**Impact**: 60% leaderboard reduction

---

### 13. State Memoization
- [ ] Create StateCache class
- [ ] Memoize expensive calculations
- [ ] Add LRU eviction (max 100 entries)
- [ ] Cache serialization results

**Time**: 4 hours
**Impact**: Reduces computation overhead

---

### 14. WebSocket Integration
- [ ] Create GameStateSubscription class
- [ ] Connect to WebSocket endpoint
- [ ] Handle real-time state updates
- [ ] Fallback to polling if WebSocket fails
- [ ] Implement reconnection logic

**Time**: 12 hours
**Impact**: 95% polling elimination

---

## 📦 Build Optimization

### 15. Minification
- [ ] Install terser for JS minification
- [ ] Install cssnano for CSS minification
- [ ] Create build script
- [ ] Add source maps

**Time**: 2 hours
**Impact**: 55% JS reduction, 44% CSS reduction

---

### 16. Compression
- [ ] Enable gzip on server
- [ ] Enable Brotli compression
- [ ] Add Cache-Control headers
- [ ] Test compression ratios

**Time**: 1 hour
**Impact**: 67% additional reduction

---

## 🧪 Testing & Monitoring

### 17. Performance Testing
- [ ] Set up Lighthouse CI
- [ ] Define performance budget
- [ ] Add custom performance monitoring
- [ ] Track metrics:
  - Polling latency
  - Render time
  - Memory usage
  - Network requests/min

**Time**: 4 hours

---

### 18. Chrome DevTools Profiling
- [ ] Record performance timeline
- [ ] Identify long tasks (> 50ms)
- [ ] Detect memory leaks
- [ ] Analyze network waterfall

**Time**: 2 hours

---

## 📊 Success Metrics

### Before Optimization
- [ ] Baseline: First Contentful Paint
- [ ] Baseline: Time to Interactive
- [ ] Baseline: Network requests/min
- [ ] Baseline: Bundle size
- [ ] Baseline: Memory usage
- [ ] Baseline: Frame rate

### After Phase 1
- [ ] Verify: 40% overall improvement
- [ ] Verify: 70% network reduction
- [ ] Verify: 95% DOM query reduction
- [ ] Verify: No memory leaks

### After Phase 2
- [ ] Verify: 70% total improvement
- [ ] Verify: 60fps animations
- [ ] Verify: 77% bundle reduction
- [ ] Verify: Instant repeat loads

### After Phase 3
- [ ] Verify: 85% total improvement
- [ ] Verify: WebSocket working
- [ ] Verify: < 30 requests/min
- [ ] Verify: Performance score > 90

---

## 🎯 Quick Implementation Guide

### Step 1: Setup (30 min)
```bash
# Install dependencies
npm install terser cssnano

# Create directory structure
mkdir -p frontend/js frontend/css
```

### Step 2: Extract Files (1 hour)
1. Move CSS to `frontend/css/app.css`
2. Move JS to `frontend/js/game.js`
3. Update index.html references

### Step 3: Implement Critical Fixes (11 hours)
1. Adaptive polling (4h)
2. Request batching (2h)
3. DOM caching (3h)
4. Memory cleanup (2h)

### Step 4: Test & Validate (2 hours)
1. Run Lighthouse
2. Check network tab
3. Profile memory
4. Verify metrics

### Step 5: Deploy & Monitor (1 hour)
1. Build optimized bundle
2. Deploy to production
3. Monitor performance
4. Gather user feedback

---

## 🔧 Code Templates

### Adaptive Polling
```javascript
const intervals = {
    matchmaking: 1000,
    myTurn: 2000,
    opponentTurn: 3000,
    idle: 5000,
    backgroundTab: 10000
};

function startAdaptivePolling(mode) {
    stopPolling();
    pollInterval = setInterval(refreshGameState, intervals[mode]);
}
```

### DOM Caching
```javascript
class BoardRenderer {
    constructor() {
        this.cellCache = new Map();
        for (let r = 0; r < ROWS; r++) {
            for (let c = 0; c < COLS; c++) {
                const cell = document.getElementById(`cell-${r}-${c}`);
                this.cellCache.set(`${r},${c}`, cell);
            }
        }
    }
    getCell(row, col) { return this.cellCache.get(`${row},${col}`); }
}
```

### Memory Cleanup
```javascript
class GameManager {
    constructor() {
        this.intervals = [];
    }
    addInterval(fn, delay) {
        const id = setInterval(fn, delay);
        this.intervals.push(id);
        return id;
    }
    cleanup() {
        this.intervals.forEach(clearInterval);
        this.intervals = [];
    }
}
window.addEventListener('beforeunload', () => gameManager.cleanup());
```

---

## 📝 Progress Tracking

**Phase 1 Complete**: [ ] Yes [ ] No
**Phase 2 Complete**: [ ] Yes [ ] No
**Phase 3 Complete**: [ ] Yes [ ] No

**Performance Score**: ___/10
**Load Time**: ___s
**Network Requests/min**: ___
**Bundle Size**: ___KB
**Memory Usage**: ___MB
**Frame Rate**: ___fps

---

**Last Updated**: January 12, 2026
**Next Review**: After Phase 1 completion
