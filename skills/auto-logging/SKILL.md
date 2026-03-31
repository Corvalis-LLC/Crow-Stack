---
name: auto-logging
description: "Logging discipline: log level selection, structured fields, what to log vs what not to log, and the specific logging anti-patterns Claude defaults to. Corrects flat log levels, over-logging, string interpolation, and log-and-propagate. Use when adding logging, reviewing log output, or designing observability. Triggers: log, logging, tracing, log level, info, debug, warn, error, trace, structured logging, observability, correlation id, span, instrument."
---

# Logging — What Claude Gets Wrong

You log everything at `info!`. You log in loops. You narrate success paths at the same verbosity as failures. You use string interpolation instead of structured fields. This skill fixes all of it.

## Log Level Selection

**The two tests:**
- **ERROR:** "Would I wake someone up for this?" — Yes → ERROR
- **INFO:** "Would I want this line in production at normal load?" — Yes → INFO

| Level | When | Example |
|---|---|---|
| **ERROR** | Operation failed, requires attention, actionable | `error!(job_id, err, "inference job failed after all retries")` |
| **WARN** | Degraded but working, approaching limits, recovered failure | `warn!(attempt = 3, "retry succeeded for whale_alert fetch")` |
| **INFO** | State transitions, service lifecycle, business milestones | `info!(consensus = "BULLISH", agreement = "2/3", "debate completed")` |
| **DEBUG** | Variable states, execution branches, API payloads | `debug!(retry_strategy = "exponential", attempt = 2, "retrying")` |
| **TRACE** | Function entry/exit, loop iterations, raw data | `trace!(raw_bytes = bytes.len(), "parsing sentinel response")` |

**Common misuses you make:**
- Expected conditions at WARN (cache miss, user not found → these are DEBUG)
- Actual errors at INFO (`info!("failed to connect")` → this is ERROR)
- Normal absence at ERROR (`error!("no new whale transactions")` → not an error, it's INFO or DEBUG)
- Function entry/exit at INFO → use spans or DEBUG

## Structured Fields, Not String Interpolation

```rust
// You write:
info!("Processed {} items in {}ms", count, elapsed);

// Senior writes:
info!(count, elapsed_ms = elapsed, "batch processed");
```

Structured fields are filterable, aggregatable, and machine-parseable. String interpolation is none of those.

**Consistent field names across the codebase:**
- `correlation_id` (not `corr_id`, `request_id`, `trace_id` interchangeably)
- `duration_ms` (not `elapsed`, `time`, `ms`)
- `sentinel` (not `crawler`, `source`)
- `err` or `error` (not `e`, `ex`, `exception`)

## What to Log

- **Decisions and why:** `info!(strategy = "exponential", reason = "rate_limited", "chose retry strategy")`
- **External interactions:** Method, target, status code, duration — not full bodies
- **Business events:** Sentinel report generated, debate completed, alert dispatched
- **State transitions:** Service started, connection established, worker disconnected
- **Failures with context:** What operation, what target, what error, what identifier

## What NOT to Log

| Don't log | Why | Instead |
|---|---|---|
| PII, tokens, API keys | Security breach vector | Use `skip` or redact |
| Every loop iteration | N log lines where 1 suffices | Log summary: count, failures, duration |
| Routine success of each operation | Noise that buries signals | DEBUG at most, or omit |
| Full request/response bodies | Volume, PII risk | Status code + content length + duration |
| Timestamps in message text | Framework adds them | Just use structured fields |
| The log level in the message | `"ERROR: An error occurred"` | The level IS the level |
| Data already in the database | Redundant, stale | Log the ID, read DB for details |

## Anti-Patterns You Default To

| Anti-pattern | Example | Fix |
|---|---|---|
| Everything at `info!` | `info!("starting")`, `info!("error: {e}")`, `info!("done")` | Use appropriate levels |
| Log-and-propagate | `error!("failed: {e}"); return Err(e)` | Either log (when handling) OR propagate (with context). Never both. |
| Paired entry/exit logs | `info!("starting X")` ... `info!("finished X")` | Use a span: `#[instrument]` or `info_span!` |
| Logging in loops | `for item in items { info!("processing {}", item.id); }` | Log before (count) and after (summary). Log individual failures only. |
| Redundant message text | `error!("Error occurred while processing: {}", err)` | `error!(err, "processing failed")` — "Error occurred" is redundant with ERROR level |
| Verbose messages | `"We have successfully completed initialization of the database connection pool"` | `"database pool initialized"` |
| Success at same level as failure | `info!("inserted report")` after every insert | Success = DEBUG. Failure = WARN/ERROR. |
| `#[instrument]` on everything | Every function traced at INFO | Use selectively. Most functions need no tracing. |
| Not considering volume | Log in hot path that fires 10k/sec | Ask: "how often will this fire?" before choosing level |

## Spans vs Events

- **Span**: Wraps an operation with duration. Use for things that start and end (HTTP handlers, inference jobs, sentinel runs).
- **Event**: Point-in-time occurrence. Use for decisions, state changes, errors.

**Rust** — use `tracing` spans natively:
```rust
let _span = info_span!("evaluate_triumvirate", correlation_id = %id).entered();
info!(judge = "TORUS", verdict = "BULLISH", "judge voted");
warn!(judge = "URSA", latency_ms = 2300, "inference slow");
```

**TypeScript/Python** — no first-class spans, so simulate with timing + single summary:
```typescript
// Don't do paired entry/exit. Instead:
const start = Date.now();
// ... do work ...
logger.info({ correlation_id, duration_ms: Date.now() - start, status }, 'request handled');
```
```python
t0 = time.monotonic()
# ... do work ...
log.info("batch complete", duration_ms=int((time.monotonic() - t0) * 1000), count=len(items))
```
The key: **one log line per operation** with duration as a structured field, not two lines ("starting" / "finished").

## The Production Volume Test

Before adding a log line, ask: "At production load, how many times per second/minute will this fire?"

- 1/hour → INFO is fine
- 1/second → INFO only if it's a key business event
- 100/second → DEBUG or TRACE only
- 10,000/second → TRACE only, or don't log at all (use metrics)
