---
name: auto-job-queue
description: "Job queue discipline: idempotent processing, poison pill protection, bounded retries with backoff, dead letter handling, backpressure, and graceful shutdown. Corrects fire-and-forget processing, unbounded retries, missing dedup, and crash-unsafe dequeue patterns. Use when designing job queues, background workers, task processors, or message consumers. Triggers: job, queue, worker, task, background, dequeue, enqueue, SKIP LOCKED, retry, dead letter, DLQ, poison pill, idempotent, backpressure, graceful shutdown, consumer, processor, celery, bull, sidekiq."
---

# Job Queues — What Claude Gets Wrong

You write job processors that work when everything goes right. In production: workers crash mid-job, malformed jobs block the queue forever, the same job runs twice producing duplicate side effects, and nobody notices when the dead letter queue fills up.

## The Six Rules

1. **Idempotent processing** — reprocessing a job MUST produce the same result, not duplicates
2. **Poison pill protection** — malformed jobs get quarantined, not retried forever
3. **Bounded retries with backoff** — retry N times with exponential delay, then dead-letter
4. **Dead letter handling** — permanently failed jobs go somewhere observable with alerting
5. **Backpressure** — monitor queue depth, shed load when saturated
6. **Graceful shutdown** — in-progress jobs complete or are released on SIGTERM

## Anti-Patterns You Default To

| Anti-pattern | Example | Fix |
|---|---|---|
| No idempotency | Process job → side effect fires again on retry | Check result exists before processing |
| Unbounded retry | Failed → requeue → fail → requeue forever | `max_attempts` + dead letter after exhaustion |
| No backoff on retry | Fail → immediately retry → fail → immediately retry | Exponential backoff: `retry_after = now + base * 2^attempt` |
| Poison pill | Malformed JSON → crash → requeue → crash forever | Catch deserialization errors → dead letter immediately |
| No backpressure | Enqueue unlimited jobs while worker is down | Check queue depth before enqueue, reject when saturated |
| Unsafe dequeue | `BRPOP` → crash → job lost forever | Use `BRPOPLPUSH` or transaction-based claim |
| No graceful shutdown | `SIGTERM` → abandon in-flight job | Drain: stop accepting, finish current, release unclaimed |
| Silent dead letter | Failed jobs pile up with no alerting | Monitor DLQ depth, alert when non-empty |

## Idempotent Processing

```rust
// WRONG: side effects fire on every attempt
async fn process_job(pool: &PgPool, job: &InferenceJob) -> Result<()> {
    let result = run_inference(&job.prompt).await?;
    insert_result(pool, job.id, &result).await?; // Duplicate on retry
    send_alert(pool, &result).await?;             // Duplicate alert
}

// RIGHT: check-before-act + upsert
async fn process_job(pool: &PgPool, job: &InferenceJob) -> Result<()> {
    // Idempotency guard: skip if already processed
    if result_exists(pool, job.id).await? {
        tracing::info!(job_id = %job.id, "already processed, skipping");
        return Ok(());
    }
    let result = run_inference(&job.prompt).await?;
    // Upsert: safe on concurrent/duplicate execution
    upsert_result(pool, job.id, &result).await?;
    // Side effects use their own idempotency check
    send_alert_if_not_sent(pool, job.id, &result).await?;
}
```

## Poison Pill Protection

```rust
// WRONG: deserialization panic requeues the same broken job
let job: InferenceJob = serde_json::from_value(row.payload)?; // panics on malformed

// RIGHT: catch deserialization, dead-letter immediately
match serde_json::from_value::<InferenceJob>(row.payload.clone()) {
    Ok(job) => process_job(pool, &job).await,
    Err(e) => {
        tracing::error!(job_id = %row.id, error = %e, "malformed job payload — dead lettering");
        dead_letter_job(pool, row.id, &format!("deserialization: {e}")).await?;
        Ok(())
    }
}
```

## Bounded Retries with Backoff

```rust
// In the job claim query, set retry_after for backoff
async fn handle_failure(pool: &PgPool, job_id: Uuid, error: &str, attempt: i32) -> Result<()> {
    if attempt >= MAX_ATTEMPTS {
        dead_letter_job(pool, job_id, error).await?;
        tracing::error!(job_id = %job_id, attempts = attempt, "job exhausted retries — dead lettered");
        return Ok(());
    }
    let backoff_secs = BASE_BACKOFF_SECS * 2_i64.pow(attempt as u32);
    sqlx::query(
        "UPDATE jobs SET status = 'pending', attempts = $1, last_error = $2,
         retry_after = NOW() + make_interval(secs => $3)
         WHERE id = $4"
    )
    .bind(attempt)
    .bind(error)
    .bind(backoff_secs as f64)
    .bind(job_id)
    .execute(pool).await?;
    Ok(())
}

// Job claim respects retry_after
// "WHERE status = 'pending' AND (retry_after IS NULL OR retry_after <= NOW())"
```

## Backpressure

```rust
// WRONG: enqueue without checking capacity
async fn enqueue(pool: &PgPool, job: NewJob) -> Result<Uuid> {
    insert_job(pool, &job).await
}

// RIGHT: check depth, reject when saturated
const MAX_QUEUE_DEPTH: i64 = 1000;

async fn enqueue(pool: &PgPool, job: NewJob) -> Result<Uuid> {
    let depth: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM jobs WHERE status = 'pending'")
        .fetch_one(pool).await?;
    if depth >= MAX_QUEUE_DEPTH {
        tracing::error!(depth, max = MAX_QUEUE_DEPTH, "queue saturated — rejecting job");
        return Err(QueueSaturated { depth }.into());
    }
    insert_job(pool, &job).await
}
```

## Graceful Shutdown

```rust
// RIGHT: drain pattern with CancellationToken
async fn worker_loop(pool: PgPool, cancel: CancellationToken) {
    loop {
        tokio::select! {
            _ = cancel.cancelled() => {
                tracing::info!("shutdown signal received — draining");
                break;
            }
            result = claim_and_process(&pool) => {
                if let Err(e) = result {
                    tracing::error!(error = %e, "job processing error");
                }
            }
        }
    }
    // In-flight job completes naturally because select! doesn't cancel it
    // — the branch runs to completion before checking cancellation again
    tracing::info!("worker drained, exiting");
}
```

## Dead Letter Monitoring

Dead letters are not a trash can — they're an alert:

```rust
// Check DLQ depth periodically or on every dead-letter insertion
async fn dead_letter_job(pool: &PgPool, job_id: Uuid, reason: &str) -> Result<()> {
    sqlx::query(
        "UPDATE jobs SET status = 'dead', last_error = $1, dead_at = NOW() WHERE id = $2"
    ).bind(reason).bind(job_id).execute(pool).await?;

    let dead_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM jobs WHERE status = 'dead' AND dead_at > NOW() - INTERVAL '1 hour'"
    ).fetch_one(pool).await?;

    if dead_count > DEAD_LETTER_ALERT_THRESHOLD {
        tracing::error!(dead_count, threshold = DEAD_LETTER_ALERT_THRESHOLD,
            "dead letter rate elevated — investigate");
    }
    Ok(())
}
```

## Job Visibility

Every job row should have these queryable columns:
```sql
status       -- pending | processing | completed | failed | dead
attempts     -- current attempt count
max_attempts -- retry limit
last_error   -- last failure reason
created_at   -- when enqueued
claimed_at   -- when a worker picked it up
completed_at -- when finished (success or dead)
retry_after  -- earliest next attempt (for backoff)
worker_id    -- which worker claimed it
```
