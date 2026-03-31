---
name: auto-concurrency
description: "Concurrency discipline: race conditions, atomicity, lock ordering, shared state protection, and the specific concurrency anti-patterns Claude defaults to. Corrects TOCTOU bugs, non-atomic multi-step operations, unprotected shared state, and missing cancellation handling. Use when writing async code, using mutexes/locks, spawning tasks, or sharing state between threads/tasks. Triggers: race condition, mutex, lock, atomic, concurrent, parallel, async, spawn, thread, shared state, Arc, Mutex, RwLock, channel, TOCTOU, deadlock, cancellation, JoinSet, semaphore."
---

# Concurrency — What Claude Gets Wrong

Your training data is overwhelmingly single-threaded. When you write concurrent code, you pattern-match on what concurrent code *looks like* instead of reasoning about interleavings. The result: code that works in tests and races in production.

## The Three Questions

Before writing any concurrent code, answer:

1. **What state is shared?** — List every variable, struct field, or resource accessed by multiple tasks/threads.
2. **Is every multi-step mutation atomic?** — If two steps must happen together, they need a single lock or transaction.
3. **What happens on cancellation?** — If this task is cancelled mid-operation, is state consistent?

## Anti-Patterns You Default To

| Anti-pattern | Example | Fix |
|---|---|---|
| Check-then-act (TOCTOU) | `if !exists { create() }` | Use atomic `create_if_not_exists` or hold lock across check+act |
| Split read-modify-write | `let v = lock.read(); drop(lock); lock.write(v+1)` | Single lock: `let mut v = lock.write(); *v += 1;` |
| Lock per step, not per operation | Lock for read, unlock, lock for write | Hold one lock for the entire read-modify-write |
| Holding lock across .await | `let guard = mutex.lock().await; async_work().await;` | Collect data under lock, drop lock, then await |
| Shared state without sync | `static mut COUNTER: u64` or `let shared = Rc::new(...)` across tasks | `Arc<Mutex<T>>` or `Arc<AtomicU64>` |
| Fire-and-forget spawns | `tokio::spawn(work())` with no handle | Use `JoinSet` or store handles for shutdown |
| No cancellation handling | Long operation with no check for cancellation signal | Use `tokio::select!` with cancellation token |
| Lock ordering violation | Task A locks X then Y, Task B locks Y then X | Always acquire locks in consistent order |

## TOCTOU — Your Most Common Concurrency Bug

```rust
// You write:
async fn ensure_user(pool: &PgPool, email: &str) -> Result<User> {
    if let Some(user) = find_user(pool, email).await? {
        return Ok(user);  // Found existing
    }
    create_user(pool, email).await?  // Race: another task created between check and insert
}

// Senior writes:
async fn ensure_user(pool: &PgPool, email: &str) -> Result<User> {
    // Use INSERT ... ON CONFLICT — atomic at the DB level
    sqlx::query_as("INSERT INTO users (email) VALUES ($1)
                     ON CONFLICT (email) DO UPDATE SET email = EXCLUDED.email
                     RETURNING *")
        .bind(email)
        .fetch_one(pool)
        .await
        .context("ensuring user exists")
}
```

**Rule:** If you write `if !exists { create }`, it's almost always a TOCTOU bug under concurrency. Use atomic operations: `INSERT ON CONFLICT`, `create_if_not_exists`, `compare_and_swap`, `entry().or_insert_with()`.

## Holding Locks Across .await

This is the Rust-specific concurrency bug you make most often:

```rust
// You write — DEADLOCK RISK:
async fn update_state(state: Arc<Mutex<AppState>>) {
    let mut guard = state.lock().await;
    let data = fetch_from_db(guard.user_id).await; // Lock held during I/O!
    guard.cached_data = data;
}

// Senior writes:
async fn update_state(state: Arc<Mutex<AppState>>) {
    let user_id = {
        let guard = state.lock().await;
        guard.user_id  // Copy what you need, drop lock
    };
    let data = fetch_from_db(user_id).await;  // No lock held during I/O
    let mut guard = state.lock().await;
    guard.cached_data = data;
}
```

**Rule:** The critical section (code under lock) should contain only synchronous, fast operations. Never `.await` while holding a `MutexGuard`.

## Multi-Step Atomicity

```typescript
// You write — NOT ATOMIC:
async function transferFunds(from: string, to: string, amount: number) {
  const fromBalance = await getBalance(from);
  if (fromBalance < amount) throw new Error("Insufficient funds");
  await setBalance(from, fromBalance - amount);
  // If this fails, money vanished:
  await setBalance(to, (await getBalance(to)) + amount);
}

// Senior writes:
async function transferFunds(from: string, to: string, amount: number) {
  await db.transaction(async (tx) => {
    const fromBalance = await tx.getBalance(from);  // SELECT ... FOR UPDATE
    if (fromBalance < amount) throw new Error("Insufficient funds");
    await tx.setBalance(from, fromBalance - amount);
    await tx.setBalance(to, (await tx.getBalance(to)) + amount);
  });  // All-or-nothing
}
```

## Cancellation and Shutdown

```rust
// You write — no shutdown handling:
loop {
    let job = queue.poll().await?;
    process(job).await?;
}

// Senior writes:
loop {
    tokio::select! {
        job = queue.poll() => {
            let job = job?;
            process(job).await?;
        }
        _ = shutdown_signal.recv() => {
            info!("graceful shutdown initiated");
            break;
        }
    }
}
```

## The Concurrency Review Checklist

After writing concurrent code, verify:

1. **Shared state audit:** Every piece of shared state has explicit synchronization
2. **Atomicity:** Multi-step operations that must be consistent use transactions or single locks
3. **No TOCTOU:** No check-then-act without holding a lock or using atomic ops
4. **No locks across .await:** Critical sections are synchronous
5. **Lock ordering:** If multiple locks exist, acquisition order is consistent everywhere
6. **Shutdown:** All spawned tasks are tracked and cancelled/joined on shutdown
7. **Cancellation safety:** If cancelled mid-operation, no state is left inconsistent
