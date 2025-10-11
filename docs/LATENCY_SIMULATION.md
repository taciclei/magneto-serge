# Latency Simulation

Magneto-Serge supports realistic network latency simulation during cassette replay, enabling you to test how your application behaves under different network conditions without relying on real network delays.

## Overview

When recording HTTP/WebSocket interactions, Magneto can capture the actual response times. During replay, you can choose to:

- **Ignore latency** (instant responses)
- **Replay recorded latency** (realistic simulation)
- **Use fixed latency** (consistent predictable delays)
- **Scale latency** (adjust speed, e.g., 50% slower, 2x faster)

This feature is essential for:
- Testing timeout handling
- Simulating slow networks
- Performance testing under realistic conditions
- Verifying retry logic

## Latency Modes

### `LatencyMode::None` (Default)

No latency simulation. Responses are returned instantly.

```rust
use magneto_serge::player::{Player, LatencyMode};

let player = Player::new();
// or explicitly:
let player = Player::new().with_latency(LatencyMode::None);
```

**Use case**: Fast test execution when latency doesn't matter.

### `LatencyMode::Recorded`

Replays the actual response time that was captured during recording.

```rust
let player = Player::load(cassette_dir, "my-cassette")
    .unwrap()
    .with_latency(LatencyMode::Recorded);
```

**Use case**: Most realistic simulation of production network conditions.

### `LatencyMode::Fixed(u64)`

All responses use a fixed delay in milliseconds, regardless of recorded time.

```rust
// All responses delayed by 100ms
let player = Player::new().with_latency(LatencyMode::Fixed(100));
```

**Use case**: Testing timeout thresholds, consistent behavior verification.

### `LatencyMode::Scaled(u64)`

Multiplies recorded times by a percentage factor (100 = 1.0x, 200 = 2.0x).

```rust
// 50% of recorded time (2x faster)
let player = Player::new().with_latency(LatencyMode::Scaled(50));

// 200% of recorded time (2x slower)
let player = Player::new().with_latency(LatencyMode::Scaled(200));
```

**Use case**: Speeding up slow tests or simulating degraded network conditions.

## How It Works

### Recording Response Times

When recording cassettes, Magneto captures the actual network response time:

```rust
use magneto_serge::cassette::{Cassette, InteractionKind};

let mut cassette = Cassette::new("my-cassette".to_string());

// Record with timing (500ms response time)
cassette.add_interaction_with_timing(
    InteractionKind::Http { request, response },
    500  // milliseconds
);

// Record without timing (latency not simulated)
cassette.add_interaction(
    InteractionKind::Http { request, response }
);
```

### Cassette Format

Response times are stored in the `response_time_ms` field:

```json
{
  "version": "1.0",
  "name": "api-cassette",
  "interactions": [
    {
      "type": "Http",
      "recorded_at": "2025-10-11T10:30:00Z",
      "response_time_ms": 342,
      "request": { ... },
      "response": { ... }
    }
  ]
}
```

If `response_time_ms` is absent or `null`, latency simulation is skipped for that interaction.

### Applying Delays

The `Player` calculates the delay to apply before returning a response:

```rust
use magneto_serge::player::{Player, LatencyMode};

let player = Player::load(cassette_dir, "cassette")
    .unwrap()
    .with_latency(LatencyMode::Recorded);

// Get interaction from cassette
let interaction = player.get_interaction(0).unwrap();

// Calculate delay based on latency mode
if let Some(delay_ms) = player.calculate_delay(interaction) {
    tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms)).await;
}

// Return response after delay
```

## Examples

### Example 1: Testing Timeout Logic

```rust
use magneto_serge::{MagnetoProxy, ProxyMode};
use magneto_serge::player::LatencyMode;

// Record slow API responses
let proxy = MagnetoProxy::new("./cassettes")
    .unwrap()
    .with_port(8888)
    .with_mode(ProxyMode::Record);

proxy.start_recording("slow-api").unwrap();
// Make API calls (they take 5 seconds each)
proxy.stop_recording().unwrap();

// Replay with actual latency
let proxy = MagnetoProxy::new("./cassettes")
    .unwrap()
    .with_latency(LatencyMode::Recorded);

proxy.replay("slow-api").unwrap();
// Your app times out correctly after 3 seconds
```

### Example 2: Speeding Up Slow Tests

```rust
// Original cassette has 10 requests averaging 2 seconds each = 20 seconds total

// Replay at 10% speed (10x faster)
let player = Player::load(cassette_dir, "slow-integration-test")
    .unwrap()
    .with_latency(LatencyMode::Scaled(10));  // 2 seconds total
```

### Example 3: Simulating Network Degradation

```rust
// Simulate a network that's 5x slower than recorded
let player = Player::load(cassette_dir, "production-api")
    .unwrap()
    .with_latency(LatencyMode::Scaled(500));  // 5x slower

// Test retry logic and backoff strategies
```

### Example 4: Fixed Delay for Consistency

```rust
// Every API call takes exactly 100ms
let player = Player::new().with_latency(LatencyMode::Fixed(100));

// Useful for testing rate limiters, queues, etc.
```

## Multi-Language Bindings

### Python

```python
from magneto_serge import MagnetoProxy, ProxyMode, LatencyMode

proxy = MagnetoProxy("./cassettes")
proxy.set_latency_mode(LatencyMode.RECORDED)
proxy.set_mode(ProxyMode.REPLAY)
proxy.replay("api-cassette")
```

### JavaScript/TypeScript

```typescript
import { MagnetoProxy, ProxyMode, LatencyMode } from 'magneto-serge';

const proxy = new MagnetoProxy('./cassettes');
proxy.setLatencyMode(LatencyMode.Fixed(200));
proxy.setMode(ProxyMode.REPLAY);
proxy.replay('api-cassette');
```

### Java/Kotlin

```kotlin
import magneto.serge.MagnetoProxy
import magneto.serge.ProxyMode
import magneto.serge.LatencyMode

val proxy = MagnetoProxy("./cassettes")
proxy.setLatencyMode(LatencyMode.Scaled(50))  // 2x faster
proxy.setMode(ProxyMode.REPLAY)
proxy.replay("api-cassette")
```

## Performance Considerations

### Overhead

Latency simulation adds minimal overhead:
- **None mode**: 0μs (no-op)
- **Recorded/Fixed/Scaled**: <1μs calculation + async sleep
- **Total impact**: <0.01% on overall request latency

### Memory

Response times are stored as `Option<u64>` (9 bytes per interaction):
- 1000 interactions = ~9 KB
- Negligible compared to HTTP bodies

### Backward Compatibility

Old cassettes without `response_time_ms` fields:
- Continue to work normally
- Latency simulation is skipped (treated as `None`)
- No migration required

## Best Practices

### 1. Record in Production-Like Environments

For realistic latency data, record cassettes from environments that match your production network characteristics.

```rust
// Record from staging environment (similar latency to production)
let proxy = MagnetoProxy::new("./cassettes")
    .unwrap()
    .with_mode(ProxyMode::Record);
```

### 2. Use Scaled Mode for Fast CI/CD

```rust
// In CI: Run tests 10x faster
let player = Player::new().with_latency(LatencyMode::Scaled(10));
```

### 3. Test Timeout Edge Cases with Fixed Mode

```rust
// Test timeout handling at exactly the threshold
let player = Player::new().with_latency(LatencyMode::Fixed(5001));  // Just over 5s timeout
```

### 4. Combine with Strict Mode

```rust
// Realistic latency + strict request matching
let player = Player::load_strict(cassette_dir, "cassette")
    .unwrap()
    .with_latency(LatencyMode::Recorded);
```

### 5. Document Latency Expectations

Add comments to cassettes or tests indicating expected latency ranges:

```rust
// Expected latency: 200-500ms (API gateway + database)
let player = Player::new().with_latency(LatencyMode::Recorded);
```

## WebSocket Latency

✅ **WebSocket latency simulation is fully supported!**

WebSocket messages use relative timestamps (timestamp_ms from connection start) for replay timing. All four latency modes work with WebSocket sessions:

### How It Works

WebSocket messages are timestamped relative to the connection start:

```json
{
  "type": "WebSocket",
  "url": "wss://api.example.com/stream",
  "messages": [
    {
      "direction": "Sent",
      "timestamp_ms": 0,
      "msg_type": "Text",
      "data": "Hello"
    },
    {
      "direction": "Received",
      "timestamp_ms": 6000,
      "msg_type": "Text",
      "data": "Block #1"
    },
    {
      "direction": "Received",
      "timestamp_ms": 12000,
      "msg_type": "Text",
      "data": "Block #2"
    }
  ]
}
```

### LatencyMode::None (Instant Mode)

Perfect for testing blockchain applications or any WebSocket streams where you don't want to wait for timed delays:

```rust
use magneto_serge::websocket::player::WebSocketPlayer;
use magneto_serge::player::LatencyMode;

// Create player with instant mode
let player = WebSocketPlayer::new()
    .with_latency(LatencyMode::None);

// Messages delivered instantly, no 6-second waits!
```

**Use case**: Fast test execution, blockchain tests, real-time event processing tests.

### LatencyMode::Recorded (Realistic Timing)

Replays with the exact timing from the recording:

```rust
let player = WebSocketPlayer::new()
    .with_latency(LatencyMode::Recorded);

// Message at 0ms: instant
// Message at 6000ms: waits 6 seconds
// Message at 12000ms: waits 12 seconds
```

**Use case**: Testing real-world timing behavior, timeout handling, sequence dependencies.

### LatencyMode::Fixed (Consistent Delays)

All messages delayed by the same amount:

```rust
let player = WebSocketPlayer::new()
    .with_latency(LatencyMode::Fixed(100));

// Every message delayed by exactly 100ms
```

**Use case**: Testing rate limiting, consistent behavior verification.

### LatencyMode::Scaled (Adjust Speed)

Speed up or slow down message timing:

```rust
// 10x faster (blockchain blocks every 600ms instead of 6s)
let player = WebSocketPlayer::new()
    .with_latency(LatencyMode::Scaled(10));

// 2x slower (more time to test processing)
let player = WebSocketPlayer::new()
    .with_latency(LatencyMode::Scaled(200));
```

**Use case**: Fast CI/CD tests, stress testing under slow networks.

### Blockchain Example (Issue #5)

Testing blockchain interactions with 6-second block times:

```rust
// Without instant mode: Test takes 60+ seconds for 10 blocks
// With instant mode: Test completes instantly!

let player = WebSocketPlayer::new()
    .with_latency(LatencyMode::None);

player.load(cassette_dir, "blockchain-test").unwrap();
let (messages, _) = player.replay_session("wss://blockchain.example.com").unwrap();

// All 10 blocks delivered instantly
assert_eq!(messages.len(), 10);
```

### API Reference

```rust
impl WebSocketPlayer {
    /// Set latency simulation mode
    pub fn with_latency(mut self, mode: LatencyMode) -> Self;

    /// Get current latency mode
    pub fn latency_mode(&self) -> LatencyMode;

    /// Calculate delay for a message based on its timestamp
    pub fn calculate_message_delay(&self, timestamp_ms: u64, base_timestamp: u64) -> Option<u64>;
}
```

## Troubleshooting

### Issue: "No delay applied during replay"

**Cause**: Cassette was recorded without timing information.

**Solution**: Re-record cassette or use `Fixed` or `Scaled` mode.

### Issue: "Tests too slow with Recorded mode"

**Cause**: Production API has high latency.

**Solution**: Use `Scaled` mode to speed up tests:

```rust
let player = Player::new().with_latency(LatencyMode::Scaled(10));  // 10x faster
```

### Issue: "Inconsistent latency in tests"

**Cause**: Using `Recorded` mode with variable network conditions during recording.

**Solution**: Use `Fixed` mode for deterministic behavior:

```rust
let player = Player::new().with_latency(LatencyMode::Fixed(200));
```

## API Reference

### `LatencyMode` Enum

```rust
pub enum LatencyMode {
    None,           // No latency simulation
    Recorded,       // Use recorded response times
    Fixed(u64),     // Fixed delay in milliseconds
    Scaled(u64),    // Scale factor as percentage (100 = 1.0x)
}
```

### `Player` Methods

```rust
impl Player {
    /// Set latency simulation mode
    pub fn with_latency(mut self, mode: LatencyMode) -> Self;

    /// Get current latency mode
    pub fn latency_mode(&self) -> LatencyMode;

    /// Calculate delay for an interaction
    pub fn calculate_delay(&self, interaction: &Interaction) -> Option<u64>;
}
```

### `Cassette` Methods

```rust
impl Cassette {
    /// Add interaction without timing
    pub fn add_interaction(&mut self, kind: InteractionKind);

    /// Add interaction with response time (ms)
    pub fn add_interaction_with_timing(&mut self, kind: InteractionKind, response_time_ms: u64);
}
```

## Related Documentation

- [ARCHITECTURE.md](./ARCHITECTURE.md) - Player component design
- [ROADMAP.md](./ROADMAP.md) - Feature roadmap (Phase 5.5)
- [EXAMPLES.md](./EXAMPLES.md) - Complete usage examples

---

**Last updated**: 2025-10-11
**Version**: 0.1.0
