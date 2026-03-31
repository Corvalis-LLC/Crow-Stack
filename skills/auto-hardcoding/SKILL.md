---
name: auto-hardcoding
description: "Configuration discipline: no hardcoded URLs, ports, timeouts, or magic numbers in business logic. Corrects localhost assumptions, inline credentials, unnamed numeric constants, and missing config injection. Use when writing code that connects to services, sets timeouts, configures pools, or uses any tunable value. Triggers: config, configuration, environment, env var, localhost, hardcode, magic number, timeout, port, url, connection string, secret, api key, base url, pool size."
---

# Hardcoding — What Claude Gets Wrong

You write code that works on your machine. Every URL is `localhost`, every timeout is a bare `30`, every credential is inline. The code compiles, the tests pass, and it's undeployable.

## The Iron Rule

**No literal URLs, ports, durations, sizes, or credentials in business logic. Ever.**

All tunable values must come from a config struct/object that is:
1. Passed as a parameter (not read from env inside business logic)
2. Validated at startup (not on first use)
3. Documented with sensible defaults where appropriate

## Anti-Patterns You Default To

| Anti-pattern | Example | Fix |
|---|---|---|
| Hardcoded localhost | `"http://localhost:8080/api"` | `config.api_base_url` |
| Inline credentials | `"Bearer my-api-key"` | `config.api_key` from env, never in source |
| Magic timeout | `Duration::from_secs(30)` | `Duration::from_secs(config.timeout_secs)` |
| Magic pool size | `max_size=20` | `config.max_pool_size` |
| Magic retry count | `for attempt in range(5)` | `for attempt in range(config.max_retries)` |
| Unnamed backoff | `sleep(2 ** attempt)` | `sleep(config.base_backoff_secs * 2 ** attempt)` |
| Inline port | `.bind("0.0.0.0:3000")` | `.bind(format!("0.0.0.0:{}", config.port))` |
| Default database name | `database="myapp"` | `config.database_url` (DSN string) |
| Client per request | `Client::new()` inside a function | Accept `&Client` as parameter, create once at startup |

## The Config Pattern

### Rust
```rust
#[derive(Debug, Clone, Deserialize)]
pub struct ApiConfig {
    pub base_url: String,
    pub timeout_secs: u64,
    pub max_retries: u32,
}

impl ApiConfig {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            base_url: env::var("API_BASE_URL").context("API_BASE_URL is required")?,
            timeout_secs: env::var("API_TIMEOUT_SECS")
                .unwrap_or_else(|_| "10".into())
                .parse().context("API_TIMEOUT_SECS must be a number")?,
            max_retries: env::var("API_MAX_RETRIES")
                .unwrap_or_else(|_| "3".into())
                .parse().context("API_MAX_RETRIES must be a number")?,
        })
    }
}

// Business logic receives config, never reads env
pub struct PricingClient {
    http: Client,
    config: ApiConfig,
}
```

### TypeScript
```typescript
interface ServiceConfig {
  readonly baseUrl: string;
  readonly timeoutMs: number;
  readonly maxRetries: number;
}

// Validated at startup, passed to constructors
class PricingClient {
  constructor(private readonly config: ServiceConfig) {}
}
```

### Python
```python
@dataclass(frozen=True)
class ServiceConfig:
    base_url: str
    timeout_secs: float = 10.0
    max_retries: int = 3

    def __post_init__(self) -> None:
        if not self.base_url:
            raise ValueError("base_url must not be empty")
```

## What Gets a Default vs What Doesn't

| Value | Default OK? | Why |
|-------|-------------|-----|
| Page size | Yes — `25` is a sensible default | Non-critical, user can override |
| Log level | Yes — `"info"` is standard | Non-critical |
| Port | Maybe — `8080` is common for dev | Document it, make it overridable |
| Timeout | Yes — but name it | `DEFAULT_TIMEOUT_SECS = 10` not bare `10` |
| Database URL | **No** — must come from env | Empty string or localhost will hide misconfiguration |
| API keys | **No** — must come from env | Never in source code, never a default |
| Base URLs | **No** — differ per environment | localhost in prod = outage |
| Pool max size | Maybe — but name it | `DEFAULT_MAX_POOL_SIZE = 10` not bare `10` |

## Named Constants for Legitimate Defaults

When a default IS appropriate, name it:

```rust
const DEFAULT_TIMEOUT_SECS: u64 = 10;
const DEFAULT_MAX_RETRIES: u32 = 3;
const DEFAULT_PAGE_SIZE: i64 = 25;
const MAX_PAGE_SIZE: i64 = 100;
```

Not:
```rust
.timeout(Duration::from_secs(10))  // What is 10? Why 10?
.limit(25)                          // Where did 25 come from?
```

## The localhost Test

**If your code contains `localhost`, `127.0.0.1`, or `0.0.0.0` outside of test code or a default bind address, it's a bug.** These are development assumptions that will break in staging/production.

Exception: `0.0.0.0` as a bind address for a server is fine (listen on all interfaces). But the port should still come from config.
