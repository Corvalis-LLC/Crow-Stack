---
name: auto-caching
description: "Caching discipline: stampede protection, invalidation strategy, error handling, stale-while-revalidate, and cache observability. Corrects naive get-or-fetch without coalescing, cached errors, arbitrary TTLs, and invisible cache behavior. Use when adding caching to API calls, database queries, computed results, or external service responses. Triggers: cache, caching, TTL, invalidate, stale, memoize, lru, redis cache, in-memory cache, cache miss, cache hit, stampede, thundering herd, coalesce, cache key, expiry, eviction."
---

# Caching — What Claude Gets Wrong

You add caching as a simple if-cached-return-else-fetch pattern. This works for a single user on localhost. In production, 50 concurrent requests all miss simultaneously, all fetch, and you've amplified load instead of reducing it. Then when the API errors, you cache the error for 5 minutes and serve garbage.

## The Five Rules

1. **Stampede protection** — concurrent misses for the same key must coalesce into one fetch
2. **Never cache errors** — or cache with very short TTL (5s) as negative cache
3. **Invalidation strategy** — know HOW and WHEN cached data becomes stale
4. **Stale-while-revalidate** — serve slightly stale data while refreshing in background
5. **Observability** — track hit rate, miss rate, evictions, and cache size

## Anti-Patterns You Default To

| Anti-pattern | Example | Fix |
|---|---|---|
| No stampede protection | 50 misses → 50 fetches | Single-flight: 50 misses → 1 fetch, 49 await |
| Cached errors | `cache.set(key, null)` on API failure | Only cache successful results |
| Arbitrary TTL | `ttl: 300` with no justification | TTL matches data freshness requirement |
| No invalidation plan | Set-and-forget, hope TTL covers it | Write-through invalidation + TTL as safety net |
| No observability | Silent cache, no idea of hit rate | Counter for hits/misses/evictions |
| Cache-everything | Cache mutable user-specific data | Only cache: immutable, expensive, shared, tolerates staleness |
| Stale data on write | Update DB, serve cached old value | Invalidate cache on write |

## Stampede Protection (Single-Flight)

```rust
// WRONG: N concurrent requests all miss and all fetch
async fn get_price(&self, symbol: &str) -> Result<f64> {
    if let Some(cached) = self.cache.get(symbol) {
        return Ok(cached);
    }
    let price = self.fetch_from_api(symbol).await?;
    self.cache.set(symbol, price, ttl);
    Ok(price)
}

// RIGHT: single-flight — one fetcher, others await
use tokio::sync::Mutex;
use std::collections::HashMap;
use tokio::sync::broadcast;

// Option A: use a crate like `moka` with `get_with` (built-in coalescing)
let price = cache.get_with(symbol.to_string(), async {
    fetch_from_api(symbol).await.unwrap_or_default()
}).await;

// Option B: manual coalescing with in-flight map
async fn get_price(&self, symbol: &str) -> Result<f64> {
    if let Some(cached) = self.cache.get(symbol) {
        return Ok(cached);
    }
    // Only one caller proceeds; others wait on the same future
    self.in_flight
        .get_or_insert_with(symbol, || self.fetch_and_cache(symbol))
        .await
}
```

```typescript
// WRONG: Promise.all with individual cache checks
const prices = await Promise.all(symbols.map(s => getPrice(s)));

// RIGHT: deduplicate in-flight requests
const inFlight = new Map<string, Promise<number>>();

async function getPrice(symbol: string): Promise<number> {
    const cached = cache.get(symbol);
    if (cached !== undefined) return cached;

    if (!inFlight.has(symbol)) {
        const promise = fetchPrice(symbol)
            .then(price => { cache.set(symbol, price, TTL); return price; })
            .finally(() => inFlight.delete(symbol));
        inFlight.set(symbol, promise);
    }
    return inFlight.get(symbol)!;
}
```

## Error Handling

```python
# WRONG: caches None on failure, serves None for TTL duration
result = api.fetch(key)
cache.set(key, result, ttl=300)  # result might be None from error

# RIGHT: only cache success, short negative cache for known-missing
result = api.fetch(key)
if result.ok:
    cache.set(key, result.data, ttl=300)
elif result.status == 404:
    cache.set(key, NOT_FOUND_SENTINEL, ttl=10)  # Short negative cache
# On transient error: don't cache, let next request retry
```

## Stale-While-Revalidate

```rust
// Serve stale data immediately, refresh in background
async fn get_with_swr(&self, key: &str) -> Option<Value> {
    if let Some(entry) = self.cache.get(key) {
        if entry.is_fresh() {
            return Some(entry.value);
        }
        // Stale but usable — serve it AND trigger background refresh
        tokio::spawn({
            let cache = self.cache.clone();
            let key = key.to_string();
            async move {
                if let Ok(fresh) = fetch_fresh(&key).await {
                    cache.set(key, fresh, ttl);
                }
            }
        });
        return Some(entry.value); // Return stale data immediately
    }
    // True miss — must wait for fetch
    let value = fetch_fresh(key).await.ok()?;
    self.cache.set(key, value.clone(), ttl);
    Some(value)
}
```

## TTL Selection Guide

| Data Type | TTL | Why |
|-----------|-----|-----|
| Crypto prices | 5-30s | High volatility, real-time matters |
| Config/settings | 60-300s | Changes infrequently, admin can wait |
| User profiles | 60-300s | Changes infrequently |
| Static assets | 3600s+ | Immutable once deployed |
| Negative cache (404) | 5-10s | Might become available soon |
| Error/failure | **Don't cache** | Next request should retry |

## Cache Observability

Every cache MUST track:
```
cache.hits      — counter
cache.misses    — counter
cache.evictions — counter (if bounded cache)
cache.size      — gauge (current entry count)
cache.latency   — histogram (fetch duration on miss)
```

Log cache configuration at startup:
```
info!(cache_name = "prices", max_entries = 1000, ttl_secs = 30, "cache initialized");
```
