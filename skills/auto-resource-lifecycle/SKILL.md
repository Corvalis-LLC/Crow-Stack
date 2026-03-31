---
name: auto-resource-lifecycle
description: "Resource lifecycle discipline: guaranteed cleanup on all paths (including error), RAII/context managers/try-with-resources, and the specific resource leak patterns Claude defaults to. Corrects manual open/close without error-path coverage, missing cleanup on teardown, and spawned tasks without join/cancel. Use when opening files, database connections, HTTP clients, event listeners, or spawning tasks. Triggers: resource, cleanup, close, drop, dispose, finally, context manager, with, RAII, defer, leak, connection pool, file handle, event listener, subscription, spawn, JoinSet, teardown, shutdown."
---

# Resource Lifecycle — What Claude Gets Wrong

You open resources and close them on the happy path. Then an error occurs between open and close, and the resource leaks. You do this in every language, every time.

## The Iron Rule

**Every resource acquisition must have a paired release in a construct that guarantees execution regardless of control flow.**

| Language | Guaranteed Cleanup |
|----------|-------------------|
| Rust | `Drop` trait (RAII) — compiler enforces it |
| Python | `with` statement (context managers) |
| TypeScript/JS | `try/finally`, `using` (TC39 Stage 3) |
| Go | `defer` |
| Java | try-with-resources |

If you're manually calling `.close()` or `.dispose()` in a non-finally/non-Drop path, it's a bug.

## Anti-Patterns You Default To

| Anti-pattern | Example | Fix |
|---|---|---|
| Manual close on happy path | `f = open(p); data = read(f); f.close()` | `with open(p) as f: data = read(f)` |
| Close after early return | `conn = pool.get(); if bad { return Err(...) }; conn.close()` | Use RAII/Drop or try-finally |
| No cleanup on error path | `let file = File::create(path)?; write!(file, ...)?;` (file leaks if write fails) | File's Drop handles this in Rust, but verify for non-RAII resources |
| HTTP client per request | `reqwest::get(url).await?` in a loop | Create client once, reuse: `let client = Client::new();` |
| Event listener without removal | `el.addEventListener("click", handler)` | Store ref, remove in cleanup: `el.removeEventListener("click", handler)` |
| Subscription without unsubscribe | `store.subscribe(callback)` | `const unsub = store.subscribe(cb); onDestroy(unsub);` |
| Spawned task without join | `tokio::spawn(background_work())` | Use `JoinSet` or store `JoinHandle` for shutdown |
| Connection held across await | `let conn = pool.acquire().await?; long_work().await; drop(conn);` | Acquire late, release early. Minimize hold time. |

## Rust Patterns

```rust
// You write:
async fn process(pool: &PgPool) -> Result<()> {
    let mut conn = pool.acquire().await?;
    let rows = sqlx::query("SELECT ...").fetch_all(&mut *conn).await?;
    // conn dropped here — fine for happy path, but you often manually manage
    // connections in ways that defeat Drop
    Ok(())
}

// Senior writes — prefer query directly on pool:
async fn process(pool: &PgPool) -> Result<()> {
    let rows = sqlx::query("SELECT ...").fetch_all(pool).await?;
    // No manual connection management — pool handles lifecycle
    Ok(())
}
```

### Task Cleanup with JoinSet
```rust
// You write:
for item in items {
    tokio::spawn(process(item)); // Fire and forget — no shutdown cleanup
}

// Senior writes:
let mut tasks = JoinSet::new();
for item in items {
    tasks.spawn(process(item));
}
while let Some(result) = tasks.join_next().await {
    result.context("background task panicked")?;
}
```

## Python Patterns

```python
# You write:
def fetch_data(url):
    session = aiohttp.ClientSession()
    response = await session.get(url)
    data = await response.json()
    await session.close()  # Never reached if get() or json() throws
    return data

# Senior writes:
async def fetch_data(url):
    async with aiohttp.ClientSession() as session:
        async with session.get(url) as response:
            return await response.json()
```

## TypeScript Patterns

```typescript
// You write:
async function readConfig(path: string) {
  const handle = await fs.open(path);
  const content = await handle.readFile('utf-8');
  await handle.close(); // Skipped if readFile throws
  return JSON.parse(content);
}

// Senior writes:
async function readConfig(path: string) {
  const handle = await fs.open(path);
  try {
    const content = await handle.readFile('utf-8');
    return JSON.parse(content);
  } finally {
    await handle.close();
  }
}
```

## The Lifecycle Checklist

When you write code that acquires a resource, verify:

1. **Is cleanup guaranteed on ALL paths?** (error, early return, panic)
2. **Is the resource held for minimum duration?** (acquire late, release early)
3. **Are spawned tasks tracked for shutdown?** (JoinSet, AbortController, cleanup registry)
4. **Is the resource reused where appropriate?** (HTTP clients, DB pools — create once)
5. **Are subscriptions/listeners removed on component teardown?**
