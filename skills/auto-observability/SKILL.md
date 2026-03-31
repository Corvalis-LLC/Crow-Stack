---
name: auto-observability
description: "Observability discipline: metrics vs logs vs traces, distributed trace propagation, health check depth, SLO-oriented measurements, and degraded state detection. Corrects logging-as-metrics, shallow health checks, missing trace context propagation, and unobservable degradation. Use when adding monitoring, health endpoints, metrics, tracing spans, or alerting. Triggers: observability, metrics, health check, health endpoint, tracing, span, trace, distributed tracing, SLO, SLI, latency, p99, histogram, counter, gauge, prometheus, opentelemetry, degraded, liveness, readiness, monitor, alert, dashboard metric."
---

# Observability — What Claude Gets Wrong

You add `tracing::info!` and a `/health` that returns `200 OK` and call the system "observable." In production: you can't distinguish slow from broken, your health check lies because it doesn't test downstream dependencies, you have no idea which service introduced latency because trace context isn't propagated, and your metrics are just structured logs you'll never query.

## The Five Rules

1. **Three pillars serve different purposes** — logs for events, metrics for aggregates, traces for request flow
2. **Health checks must test dependencies** — a process that's up but can't reach the DB is not healthy
3. **Propagate trace context across boundaries** — HTTP headers, job queue metadata, message headers
4. **Measure what matters for SLOs** — latency histograms, error rates, saturation — not vanity counts
5. **Detect degradation, not just failure** — slow is often worse than down

## Anti-Patterns You Default To

| Anti-pattern | Example | Fix |
|---|---|---|
| Logging as metrics | `info!("processed {} jobs", count)` | Increment a counter: `jobs_processed.inc()` |
| Shallow health check | `GET /health → 200 "ok"` | Check DB, cache, downstream services, queue depth |
| No trace propagation | Service A calls B, traces are disconnected | Inject/extract trace context in headers |
| Counting instead of measuring | `requests_total` counter only | Histogram for latency, counter for errors, gauge for queue depth |
| Boolean health | Healthy or unhealthy, nothing between | Degraded state: healthy / degraded / unhealthy with reasons |
| Vanity metrics | `logins_total` with no error breakdown | `logins_total{result="success|failure|timeout"}` |
| Missing saturation signals | No queue depth, no connection pool usage | Gauge for pool active/idle, queue pending count |

## Health Check Depth

```rust
// WRONG: process is up, but is it useful?
async fn health() -> StatusCode {
    StatusCode::OK
}

// RIGHT: check every dependency, report degraded state
#[derive(Serialize)]
struct HealthResponse {
    status: &'static str, // "healthy" | "degraded" | "unhealthy"
    checks: BTreeMap<String, CheckResult>,
}

#[derive(Serialize)]
struct CheckResult {
    status: &'static str,
    latency_ms: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

async fn health(State(ctx): State<AppState>) -> (StatusCode, Json<HealthResponse>) {
    let mut checks = BTreeMap::new();

    // Database
    let db_start = Instant::now();
    let db_check = sqlx::query("SELECT 1").execute(&ctx.pool).await;
    checks.insert("database".into(), CheckResult {
        status: if db_check.is_ok() { "pass" } else { "fail" },
        latency_ms: db_start.elapsed().as_millis() as u64,
        error: db_check.err().map(|e| e.to_string()),
    });

    // Queue depth (saturation signal)
    let depth: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM jobs WHERE status = 'pending'")
        .fetch_one(&ctx.pool).await.unwrap_or(-1);
    checks.insert("queue_depth".into(), CheckResult {
        status: if depth < 100 { "pass" } else if depth < 500 { "warn" } else { "fail" },
        latency_ms: 0,
        error: if depth >= 500 { Some(format!("queue depth {depth} exceeds threshold")) } else { None },
    });

    let any_fail = checks.values().any(|c| c.status == "fail");
    let any_warn = checks.values().any(|c| c.status == "warn");
    let status = if any_fail { "unhealthy" } else if any_warn { "degraded" } else { "healthy" };
    let code = if any_fail { StatusCode::SERVICE_UNAVAILABLE } else { StatusCode::OK };

    (code, Json(HealthResponse { status, checks }))
}
```

## Trace Context Propagation

```rust
// WRONG: each service creates independent traces — can't follow a request
async fn call_downstream(url: &str) -> Result<Response> {
    reqwest::get(url).await?.error_for_status().map_err(Into::into)
}

// RIGHT: inject current span context into outgoing request
use tracing_opentelemetry::OpenTelemetrySpanExt;
use opentelemetry::global;

async fn call_downstream(client: &reqwest::Client, url: &str) -> Result<Response> {
    let mut headers = HeaderMap::new();
    // Inject current trace context into HTTP headers
    global::get_text_map_propagator(|propagator| {
        let cx = tracing::Span::current().context();
        propagator.inject_context(&cx, &mut HeaderInjector(&mut headers));
    });
    client.get(url).headers(headers).send().await?.error_for_status().map_err(Into::into)
}

// For job queues: store trace context in job metadata
let mut trace_ctx = HashMap::new();
global::get_text_map_propagator(|propagator| {
    propagator.inject_context(&tracing::Span::current().context(), &mut trace_ctx);
});
enqueue_job(pool, payload, trace_ctx).await?;
```

## Metrics: The Right Type for the Right Signal

```rust
// Counters — monotonically increasing totals (use for: requests, errors, jobs processed)
let requests = counter!("http_requests_total", "method" => method, "status" => status);
requests.increment(1);

// Histograms — distributions (use for: latency, response size, batch size)
let latency = histogram!("http_request_duration_seconds");
latency.record(elapsed.as_secs_f64());

// Gauges — point-in-time values (use for: queue depth, pool size, active connections)
let depth = gauge!("queue_pending_jobs");
depth.set(pending_count as f64);
```

| Signal | Metric Type | Example |
|--------|------------|---------|
| Request count | Counter | `http_requests_total{method, status}` |
| Error count | Counter | `http_errors_total{method, error_type}` |
| Latency | Histogram | `http_request_duration_seconds{method, route}` |
| Queue depth | Gauge | `queue_pending_jobs{queue_name}` |
| Pool usage | Gauge | `db_pool_active_connections` |
| Job duration | Histogram | `job_processing_seconds{job_type}` |

## Degradation Detection

Don't just check "is it up?" — check "is it performing?"

```rust
// RIGHT: latency-based degradation detection
async fn check_inference_health(pool: &PgPool) -> &'static str {
    let p99: Option<f64> = sqlx::query_scalar(
        "SELECT PERCENTILE_CONT(0.99) WITHIN GROUP (ORDER BY duration_ms)
         FROM inference_results WHERE created_at > NOW() - INTERVAL '5 minutes'"
    ).fetch_one(pool).await.ok().flatten();

    match p99 {
        Some(ms) if ms < 5000.0 => "healthy",
        Some(ms) if ms < 15000.0 => "degraded",
        Some(_) => "unhealthy",
        None => "no_data", // No recent inferences — might be fine, might be broken
    }
}
```

## SLO-Oriented Measurement

Measure what your users care about, not what's easy to count:

| SLI (what to measure) | Bad metric | Good metric |
|------------------------|-----------|-------------|
| Availability | Uptime percentage | Successful requests / total requests |
| Latency | Average response time | p50, p95, p99 response time |
| Freshness | "Last updated" timestamp | Age of newest data vs SLO threshold |
| Correctness | Error count | Error rate (errors / total) by category |
