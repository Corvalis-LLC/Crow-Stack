---
name: auto-resilience
description: "Network resilience discipline: timeouts, retries with backoff and jitter, circuit breaking, idempotency, and partial failure handling. Corrects bare network calls with no timeout, no retry, no degradation strategy. Use when making HTTP requests, calling external APIs, delivering webhooks, or aggregating data from multiple services. Triggers: timeout, retry, backoff, jitter, circuit breaker, idempotency, idempotent, resilience, resilient, network, fetch, request, webhook, delivery, partial failure, degraded, fallback, rate limit, 429, 503."
---

# Network Resilience — What Claude Gets Wrong

You write network calls as if the network is reliable. No timeouts, no retries, no circuit breaking. Your code works in development and fails on the first packet drop in production.

## The Five Requirements

Every network call in production code must have:

1. **Timeout** — explicit, per-request
2. **Retry** — with exponential backoff + jitter, only for transient failures
3. **Idempotency** — safe to retry (GET is free; POST needs an idempotency key)
4. **Partial failure handling** — degrade gracefully when some calls fail
5. **Observability** — log attempt, duration, status, failure reason

## Anti-Patterns You Default To

| Anti-pattern | Example | Fix |
|---|---|---|
| No timeout | `client.get(url).send().await?` | `.timeout(Duration::from_secs(10))` |
| No retry | Single attempt, propagate error | Retry 2-3 times with backoff |
| Linear backoff | `sleep(1s)`, `sleep(1s)`, `sleep(1s)` | Exponential: 500ms, 1s, 2s with jitter |
| Retry everything | Retry on 404 | Only retry transient: timeout, 429, 5xx, connection refused |
| No idempotency key | `POST /webhooks` without dedup | `X-Idempotency-Key` header or DB dedup |
| Hammer failing service | Retry forever, no circuit break | Track failures, back off after N consecutive |
| Silent failure | `catch { return [] }` | Log the failure, return typed error or degraded result |
| No jitter | All clients retry at the same time | Add random jitter to prevent thundering herd |
| Fire-and-forget delivery | Single `fetch()` for webhook | Persist job, retry from queue |

## Retry Pattern

```rust
// You write:
let resp = client.get(&url).send().await?;

// Senior writes:
async fn fetch_with_retry(
    client: &Client,
    url: &str,
    config: &RetryConfig,
) -> Result<Response> {
    let mut last_err = None;

    for attempt in 0..=config.max_retries {
        if attempt > 0 {
            let backoff = config.base_backoff * 2u32.pow(attempt - 1);
            let jitter = Duration::from_millis(rand::random::<u64>() % backoff.as_millis() as u64);
            tokio::time::sleep(backoff + jitter).await;
        }

        match client.get(url).timeout(config.timeout).send().await {
            Ok(resp) if resp.status().is_success() => return Ok(resp),
            Ok(resp) if resp.status() == 429 || resp.status().is_server_error() => {
                last_err = Some(anyhow!("HTTP {}", resp.status()));
            }
            Ok(resp) => return Err(anyhow!("non-retryable: HTTP {}", resp.status())),
            Err(e) => last_err = Some(e.into()),
        }
    }

    Err(last_err.unwrap_or_else(|| anyhow!("request failed")))
}
```

## What's Retryable vs Not

| Status/Error | Retryable? | Why |
|---|---|---|
| Timeout | Yes | Transient network issue |
| Connection refused | Yes | Service may be restarting |
| 429 Too Many Requests | Yes | Rate limited, respect `Retry-After` |
| 500, 502, 503, 504 | Yes | Server-side transient |
| 400 Bad Request | **No** | Your request is wrong, retrying won't help |
| 401/403 | **No** | Auth issue, retrying won't help |
| 404 Not Found | **No** | Resource doesn't exist |
| 409 Conflict | **No** | State conflict, needs resolution |

## Circuit Breaker (Simplified)

```rust
// Track consecutive failures per service
if consecutive_failures >= threshold {
    if last_failure_time.elapsed() < cooldown {
        return Err("circuit breaker open");
    }
    // Allow one probe request after cooldown
}
```

Don't over-engineer this. A simple counter + cooldown prevents hammering a dead service. Reset to 0 on success.

## Partial Failure Pattern

When aggregating from multiple services:

```typescript
// You write:
const [users, orders, metrics] = await Promise.all([
  fetchUsers(),   // If this throws, everything fails
  fetchOrders(),
  fetchMetrics(),
]);

// Senior writes:
const [users, orders, metrics] = await Promise.allSettled([
  fetchUsers(),
  fetchOrders(),
  fetchMetrics(),
]);

// Process results individually — one failure doesn't kill the rest
const result = {
  users: users.status === 'fulfilled' ? users.value : null,
  orders: orders.status === 'fulfilled' ? orders.value : null,
  metrics: metrics.status === 'fulfilled' ? metrics.value : null,
  degraded: [users, orders, metrics]
    .filter(r => r.status === 'rejected')
    .map((r, i) => ['users', 'orders', 'metrics'][i]),
};
```

## Webhook/Delivery Pattern

For operations that MUST eventually succeed:

1. **Persist the job** — DB row or queue entry before attempting delivery
2. **Attempt with timeout** — don't wait forever
3. **On failure: schedule retry** — exponential backoff, max attempts
4. **On permanent failure (4xx)** — mark as failed, don't retry
5. **Include idempotency key** — receiver can deduplicate
6. **Log every attempt** — attempt number, status, duration

## The Production Readiness Checklist

Before any network call goes to production, verify:

- [ ] Explicit timeout set
- [ ] Retries with exponential backoff + jitter for transient failures
- [ ] Non-retryable errors fail fast (4xx)
- [ ] Circuit breaker or failure tracking for frequently-called services
- [ ] Partial failure doesn't crash the entire operation
- [ ] Every attempt logged with duration and status
